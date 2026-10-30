//! Import a private export of installed-client metadata. Never download assets.
use anyhow::{ensure, Context, Result};
use clap::Parser;
use serde::Deserialize;
use std::path::PathBuf;
use zwift_race_finder::database::{ClientRouteRecord, Database};

#[derive(Parser)]
#[command(
    about = "Import client routes as the supported route source, preserving fractional climbing and provenance"
)]
struct Args {
    /// Private JSON produced by scripts/export_client_routes.ps1
    #[arg(long)]
    input: PathBuf,
    /// Preview route changes; database initialization/schema migration may still occur
    #[arg(long)]
    dry_run: bool,
}

#[derive(Deserialize)]
struct Export {
    schema_version: u32,
    routes: Vec<ClientRouteRecord>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let export: Export = serde_json::from_slice(
        &std::fs::read(&args.input).context("Cannot read client route export")?,
    )
    .context("Invalid client route export; regenerate with export_client_routes.ps1")?;
    ensure!(
        export.schema_version == 1,
        "Unsupported client export schema {}",
        export.schema_version
    );
    ensure!(
        !export.routes.is_empty(),
        "Client export contains no routes"
    );
    for record in &export.routes {
        record.validate()?;
    }
    let db = Database::new()?;
    let summary = db.import_client_routes(&export.routes, args.dry_run)?;
    println!(
        "{}Client routes: {} written; {} unchanged/lower-priority skipped",
        if args.dry_run { "Preview: " } else { "" },
        summary.written,
        summary.skipped
    );
    println!("Sources: selected client cache > extracted client XML. Existing legacy records are replaced only by matching client IDs.");
    Ok(())
}
