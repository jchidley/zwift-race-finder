use anyhow::Result;
use colored::*;

use crate::database::Database;

pub fn show_unknown_routes() -> Result<()> {
    let db = Database::new()?;
    let routes = db.get_unknown_routes()?;

    if routes.is_empty() {
        println!("No unknown routes found.");
    } else {
        println!("\n{}", "Unknown Routes (missing route metadata):".yellow().bold());
        println!("{}", "=".repeat(60));
        println!("{:<12} {:<8} {}", "Route ID", "Seen", "Event Name");
        println!("{}", "-".repeat(60));

        for (route_id, event_name, times_seen) in routes {
            println!(
                "{:<12} {:<8} {}",
                route_id.to_string().yellow(),
                times_seen,
                event_name
            );
        }

        println!(
            "\n{}: Refresh the selected installed-client route export",
            "Tip".yellow()
        );
        println!("Then import it with import_client_routes --input /path/to/client-routes.json.");
    }
    Ok(())
}
