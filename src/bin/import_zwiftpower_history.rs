//! Import an explicitly approved private ZwiftPower export. No authentication/network.
use anyhow::{Context, Result};
use clap::Parser;
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use zwift_race_finder::{database::Database, history::parse_zwiftpower_export};

#[derive(Parser)]
#[command(about = "Import measured ZwiftPower history; unknown historical scores remain NULL")]
struct Args {
    /// Private versioned export from the profile results DataTable
    #[arg(long)]
    input: PathBuf,
    /// Explicit target, otherwise the normal application database
    #[arg(long)]
    database: Option<PathBuf>,
    /// Validate migration and import on an in-memory copy; target is not changed
    #[arg(long)]
    dry_run: bool,
}
fn main() -> Result<()> {
    let args = Args::parse();
    let bytes = std::fs::read(&args.input).context("Cannot read private history export")?;
    let sha = format!("{:x}", Sha256::digest(&bytes));
    let records = parse_zwiftpower_export(&bytes, &sha)?;
    let path = args
        .database
        .map(Ok)
        .unwrap_or_else(Database::default_path)?;
    // Always preflight the complete migration/batch without changing the target.
    let preview = Database::preview(&path)?;
    let summary = preview.import_history(&records)?;
    if args.dry_run {
        println!(
            "Preview (target unchanged): {} new; {} enriched; {} unchanged",
            summary.inserted, summary.enriched, summary.unchanged
        );
    } else {
        let db = Database::open(&path)?;
        let summary = db.import_history(&records)?;
        println!(
            "History: {} new; {} enriched; {} unchanged",
            summary.inserted, summary.enriched, summary.unchanged
        );
    }
    println!("No scores inferred; confirmed existing scores are preserved. Back up before applying migration/import.");
    Ok(())
}
