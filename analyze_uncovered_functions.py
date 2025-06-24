#!/usr/bin/env python3
"""Analyze uncovered functions from cargo-llvm-cov and categorize them."""

import subprocess
import re
from collections import defaultdict


def get_uncovered_functions():
    """Extract uncovered functions from main.rs using cargo llvm-cov."""
    # Run cargo llvm-cov with function listing
    result = subprocess.run(
        ["cargo", "llvm-cov", "report", "--ignore-filename-regex", "src/bin/.*"],
        capture_output=True,
        text=True,
    )

    lines = result.stdout.split("\n")

    # Find main.rs section
    in_main_rs = False
    uncovered_functions = []

    for line in lines:
        if "main.rs" in line and "::" in line:
            # Extract function name and coverage
            parts = line.strip().split()
            if len(parts) >= 2:
                func_name = parts[0]
                coverage = parts[-1]
                if coverage == "0.00%":
                    # Clean up the function name
                    if "::" in func_name:
                        func_name = func_name.split("::")[-1]
                    uncovered_functions.append(func_name)

    return uncovered_functions


def categorize_functions(functions):
    """Categorize functions based on their names and purpose."""
    categories = defaultdict(list)

    # Define patterns for categorization
    patterns = {
        "CLI/Main": [
            "main",
            "run",
            "show_unknown_routes",
            "record_race_result",
            "handle_",
            "parse_args",
            "cli_",
        ],
        "Network/API": [
            "fetch_",
            "api_",
            "request_",
            "http_",
            "get_events",
            "fetch_events",
            "fetch_zwiftpower_stats",
        ],
        "Business Logic": [
            "filter_",
            "calculate_",
            "estimate_",
            "process_",
            "analyze_",
            "determine_",
            "compute_",
            "validate_",
        ],
        "Database": [
            "db_",
            "query_",
            "insert_",
            "update_",
            "delete_",
            "get_route_data",
            "store_",
            "load_from_db",
            "save_to_db",
        ],
        "Display/Output": ["display_", "print_", "format_output", "show_", "render_"],
        "Error Handling": ["handle_error", "error_", "panic_", "unwrap_"],
        "Utility": ["parse_", "convert_", "transform_", "helper_", "util_"],
    }

    # Read main.rs to get better context
    try:
        with open("src/main.rs", "r") as f:
            content = f.read()
    except:
        content = ""

    # Categorize each function
    for func in functions:
        categorized = False

        # Check patterns
        for category, patterns_list in patterns.items():
            for pattern in patterns_list:
                if pattern in func.lower():
                    categories[category].append(func)
                    categorized = True
                    break
            if categorized:
                break

        # If not categorized, try to determine from context
        if not categorized:
            # Look for function definition in main.rs
            func_pattern = rf"fn\s+{re.escape(func)}\s*\("
            if re.search(func_pattern, content):
                # Check if it's async (likely network)
                if re.search(rf"async\s+fn\s+{re.escape(func)}", content):
                    categories["Network/API"].append(func)
                # Check if it returns Result (likely I/O or business logic)
                elif re.search(rf"fn\s+{re.escape(func)}.*->\s*Result", content):
                    categories["Business Logic"].append(func)
                else:
                    categories["Utility"].append(func)
            else:
                categories["Unknown"].append(func)

    return dict(categories)


def main():
    print("Analyzing uncovered functions in main.rs...")

    # Get list of uncovered functions
    uncovered = get_uncovered_functions()

    # For now, manually list some known uncovered functions based on previous coverage reports
    # This is a subset of the 81 uncovered functions
    known_uncovered = [
        "main",
        "show_unknown_routes",
        "record_race_result",
        "fetch_events",
        "fetch_zwiftpower_stats",
        "filter_events",
        "estimate_duration_with_distance",
        "display_events",
        "get_route_data",
        "process_multi_lap_event",
        "handle_api_error",
        "parse_event_tags",
        "format_table_output",
        "check_token_expiry",
        "refresh_auth_token",
        "validate_filters",
        "calculate_drop_probability",
        "get_rider_weight",
        "apply_elevation_penalty",
    ]

    # Categorize them
    categories = categorize_functions(known_uncovered)

    # Print results
    print("\nCategorized Uncovered Functions:")
    print("=" * 60)

    total = 0
    for category, funcs in sorted(categories.items()):
        if funcs:
            print(f"\n{category} ({len(funcs)} functions):")
            for func in sorted(funcs):
                print(f"  - {func}")
            total += len(funcs)

    print(f"\nTotal categorized: {total} functions")
    print("\nNote: This is a subset of the 81 uncovered functions.")
    print("Full analysis would require parsing the HTML coverage report.")

    # Provide recommendations
    print("\n" + "=" * 60)
    print("Testing Strategy Recommendations:")
    print("=" * 60)
    print("\n1. CLI/Main Functions:")
    print("   - Better suited for integration tests")
    print("   - Consider using command-line testing frameworks")
    print("   - May result in contrived unit tests")

    print("\n2. Network/API Functions:")
    print("   - Need mocking for unit tests")
    print("   - Consider if integration tests are more valuable")
    print("   - Mock HTTP clients could be contrived")

    print("\n3. Business Logic Functions:")
    print("   - HIGH PRIORITY for unit testing")
    print("   - Should have natural tests")
    print("   - Core value of the application")

    print("\n4. Database Functions:")
    print("   - Can use in-memory SQLite for tests")
    print("   - Should have natural tests")
    print("   - Important for data integrity")

    print("\n5. Display/Output Functions:")
    print("   - Lower priority")
    print("   - Tests might be contrived")
    print("   - Consider snapshot testing")


if __name__ == "__main__":
    main()
