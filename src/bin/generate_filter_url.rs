use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    
    if args.is_empty() {
        eprintln!("Usage: generate_filter_url [options]");
        eprintln!("Example: generate_filter_url --tags ranked,zracing --duration 30 --tolerance 15");
        return;
    }
    
    let mut params = Vec::new();
    let mut i = 0;
    
    while i < args.len() {
        match args[i].as_str() {
            "--tags" => {
                if i + 1 < args.len() {
                    params.push(format!("tags={}", args[i + 1]));
                    i += 1;
                }
            }
            "--exclude-tags" => {
                if i + 1 < args.len() {
                    params.push(format!("exclude_tags={}", args[i + 1]));
                    i += 1;
                }
            }
            "--duration" | "-d" => {
                if i + 1 < args.len() {
                    params.push(format!("duration={}", args[i + 1]));
                    i += 1;
                }
            }
            "--tolerance" | "-t" => {
                if i + 1 < args.len() {
                    params.push(format!("tolerance={}", args[i + 1]));
                    i += 1;
                }
            }
            "--event-type" | "-e" => {
                if i + 1 < args.len() {
                    params.push(format!("event_type={}", args[i + 1]));
                    i += 1;
                }
            }
            "--zwift-score" | "-s" => {
                if i + 1 < args.len() {
                    params.push(format!("zwift_score={}", args[i + 1]));
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    
    if !params.is_empty() {
        println!("Shareable URL parameters: ?{}", params.join("&"));
        println!("\nExample usage:");
        println!("zwift-race-finder --from-url \"{}\"", params.join("&"));
    }
}