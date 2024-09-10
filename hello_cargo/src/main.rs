use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

const VERSION: &str = "1.0.0";
fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    // Check if any arguments are provided
    if args.len() < 2 {
        eprintln!("Usage: {} [-v | -t]", args[0]);
        std::process::exit(1);
    }

    // Handle the arguments
    match args[1].as_str() {
        "-v" => {
            println!("Version: {}", VERSION);
        },
        "-t" => {
            let start = SystemTime::now();
            match start.duration_since(UNIX_EPOCH) {
                Ok(n) => println!("Current time (seconds since UNIX EPOCH): {}", n.as_secs()),
                Err(_) => eprintln!("SystemTime before UNIX EPOCH!"),
            }
        },
        _ => {
            eprintln!("Invalid option: {}", args[1]);
            eprintln!("Usage: {} [-v | -t]", args[0]);
            std::process::exit(1);
        }
    }
   

}
