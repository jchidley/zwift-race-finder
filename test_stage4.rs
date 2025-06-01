// Test to debug Stage 4 event data
// Compile with: cargo build --bin zwift-race-finder
// Run with: cargo run -- --debug 2>&1 | grep -A50 "Stage 4" > stage4_debug.txt

fn main() {
    println!("This is a test file to examine Stage 4 event data.");
    println!("Please run the main program with --debug flag and grep for Stage 4 events.");
    println!("Example: cargo run -- --debug 2>&1 | grep -A50 'Stage 4: Makuri May'");
}
