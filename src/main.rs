use clap::Parser;
use std::io::{self, BufRead, Write};
use std::process;

mod config;
mod generator;

use config::Config;
use generator::generate_variations;

// Helper function to count typeable characters
fn count_typeable_chars(s: &str) -> usize {
    s.chars().filter(|&c| {
        c.is_ascii_alphanumeric() ||
        "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".contains(c) // Common symbols
    }).count()
}

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

    if config.count {
        let mut total_count: u64 = 0;
        for password in &passwords {
            if config.include_original {
                total_count += 1;
            }
            total_count += generate_variations(password, config.max_distance, config.transposition_distance).iter().count() as u64;
        }
        println!("{}", total_count);
    } else {
        let mut all_variations: Vec<String> = Vec::new();

        // Collect all variations first
        for password in &passwords {
            if config.include_original {
                all_variations.push(password.clone());
            }
            all_variations.extend(generate_variations(password, config.max_distance, config.transposition_distance));
        }

        // Sort the collected variations
        all_variations.sort_by_key(|variation| count_typeable_chars(variation));

        // Print the sorted variations
        let stdout = io::stdout();
        let mut stdout_lock = stdout.lock();
        for variation in all_variations {
             writeln!(stdout_lock, "{}", variation).expect("Failed to write to stdout");
        }
    }
}