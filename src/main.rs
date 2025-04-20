use clap::Parser;
use std::io::{self, BufRead, Write};
use std::process;

mod config;
mod generator;

use config::Config;
use generator::generate_variations;

fn main() {
    // Parse command-line arguments
    let config = Config::parse();
    
    // Handle input from file, stdin, or command-line
    let passwords = if let Some(password) = &config.password {
        vec![password.clone()]
    } else if config.stdin {
        let stdin = io::stdin();
        let mut passwords = Vec::new();
        
        for line in stdin.lock().lines() {
            match line {
                Ok(password) => {
                    if !password.trim().is_empty() {
                        passwords.push(password.trim().to_string());
                    }
                },
                Err(e) => {
                    eprintln!("Error reading from stdin: {}", e);
                    process::exit(1);
                }
            }
        }
        
        passwords
    } else {
        eprintln!("Error: No password provided. Use -p or --stdin");
        process::exit(1);
    };
    
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    for password in passwords {
        // First output the original password if requested
        if config.include_original {
            writeln!(stdout_lock, "{}", password).expect("Failed to write to stdout");
        }
        
        // Generate and stream the variations
        for variation in generate_variations(&password, config.max_distance) {
            writeln!(stdout_lock, "{}", variation).expect("Failed to write to stdout");
        }
    }
}