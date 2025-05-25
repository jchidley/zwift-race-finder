#!/usr/bin/env python3
"""
Scrape Jack's ZwiftPower profile to get actual race results for regression testing
Uses Playwright for proper JavaScript rendering
"""

import asyncio
import json
import sqlite3
from datetime import datetime
from pathlib import Path
from playwright.async_api import async_playwright
import re

# Jack's ZwiftPower profile URL
PROFILE_URL = "https://zwiftpower.com/profile.php?z=YOUR_PROFILE_ID"

async def scrape_race_results():
    """Scrape race results from ZwiftPower profile"""
    results = []
    
    async with async_playwright() as p:
        # Launch browser with a viewport
        browser = await p.chromium.launch(headless=True)
        context = await browser.new_context(
            viewport={"width": 1920, "height": 1080},
            user_agent="Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"
        )
        page = await context.new_page()
        
        print(f"Loading ZwiftPower profile: {PROFILE_URL}")
        await page.goto(PROFILE_URL, wait_until="networkidle")
        
        # Wait for the results table to load
        await page.wait_for_selector("table.table-striped", timeout=30000)
        
        # Get the race results table
        # ZwiftPower typically shows recent results in a table
        results_table = await page.query_selector("table.table-striped")
        
        if results_table:
            # Extract all rows from the results table
            rows = await results_table.query_selector_all("tbody tr")
            print(f"Found {len(rows)} race results")
            
            for row in rows:
                try:
                    # Extract data from each cell
                    cells = await row.query_selector_all("td")
                    if len(cells) >= 7:  # Ensure we have enough columns
                        # Typical columns: Date, Event, Cat, Position, Time, W/kg, etc.
                        date_text = await cells[0].text_content()
                        event_name = await cells[1].text_content()
                        category = await cells[2].text_content()
                        position = await cells[3].text_content()
                        time_text = await cells[4].text_content()
                        
                        # Try to get the event link which might contain route info
                        event_link = await cells[1].query_selector("a")
                        event_href = await event_link.get_attribute("href") if event_link else None
                        
                        # Extract event ID from the link
                        event_id = None
                        if event_href and "id=" in event_href:
                            match = re.search(r'id=(\d+)', event_href)
                            if match:
                                event_id = match.group(1)
                        
                        # Parse time (format: "1:23:45" or "23:45")
                        time_parts = time_text.strip().split(":")
                        if len(time_parts) == 3:
                            minutes = int(time_parts[0]) * 60 + int(time_parts[1])
                        elif len(time_parts) == 2:
                            minutes = int(time_parts[0])
                        else:
                            continue
                        
                        result = {
                            "date": date_text.strip(),
                            "event_name": event_name.strip(),
                            "category": category.strip(),
                            "position": position.strip(),
                            "time_minutes": minutes,
                            "time_text": time_text.strip(),
                            "event_id": event_id,
                            "event_href": event_href
                        }
                        
                        results.append(result)
                        print(f"  - {date_text}: {event_name} - {time_text}")
                        
                except Exception as e:
                    print(f"Error parsing row: {e}")
                    continue
        
        # Also try to get the full results history link if available
        history_link = await page.query_selector("a[href*='results']")
        if history_link:
            print("\nFound results history link, following...")
            await history_link.click()
            await page.wait_for_load_state("networkidle")
            
            # Parse additional results from history page
            # ... (similar parsing logic)
        
        await browser.close()
    
    return results

async def get_route_info_for_event(event_id):
    """Try to get route information for a specific event"""
    # This would need to navigate to the event page and extract route details
    # For now, we'll return None and handle this mapping separately
    return None

def save_results_to_database(results):
    """Save scraped results to SQLite database"""
    db_path = Path.home() / ".local/share/zwift-race-finder/races.db"
    
    if not db_path.parent.exists():
        db_path.parent.mkdir(parents=True)
    
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()
    
    # Create a table for scraped results if it doesn't exist
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS zwiftpower_results (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date TEXT NOT NULL,
            event_name TEXT NOT NULL,
            category TEXT,
            position TEXT,
            time_minutes INTEGER NOT NULL,
            time_text TEXT,
            event_id TEXT,
            event_href TEXT,
            route_id INTEGER,
            scraped_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
    """)
    
    # Insert results
    for result in results:
        cursor.execute("""
            INSERT INTO zwiftpower_results 
            (date, event_name, category, position, time_minutes, time_text, event_id, event_href)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        """, (
            result["date"],
            result["event_name"],
            result["category"],
            result["position"],
            result["time_minutes"],
            result["time_text"],
            result["event_id"],
            result["event_href"]
        ))
    
    conn.commit()
    conn.close()
    
    print(f"\nSaved {len(results)} results to database")

def export_results_to_json(results):
    """Export results to JSON for analysis"""
    output_path = Path("zwiftpower_results.json")
    with open(output_path, "w") as f:
        json.dump(results, f, indent=2)
    print(f"\nExported results to {output_path}")

async def main():
    """Main function"""
    print("Scraping ZwiftPower race results...")
    print("=" * 60)
    
    results = await scrape_race_results()
    
    if results:
        print(f"\nSuccessfully scraped {len(results)} race results!")
        
        # Save to database
        save_results_to_database(results)
        
        # Export to JSON for analysis
        export_results_to_json(results)
        
        # Show summary
        print("\nSummary of race times:")
        print("-" * 40)
        times = [r["time_minutes"] for r in results]
        if times:
            print(f"Shortest race: {min(times)} minutes")
            print(f"Longest race: {max(times)} minutes")
            print(f"Average time: {sum(times) / len(times):.1f} minutes")
            
            # Show most common events
            event_counts = {}
            for r in results:
                event = r["event_name"]
                event_counts[event] = event_counts.get(event, 0) + 1
            
            print("\nMost frequent events:")
            for event, count in sorted(event_counts.items(), key=lambda x: x[1], reverse=True)[:5]:
                print(f"  - {event}: {count} times")
    else:
        print("No results found. Check if the page loaded correctly.")

if __name__ == "__main__":
    asyncio.run(main())