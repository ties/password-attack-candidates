use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "Generate hashcat password candidates with 1-2 character distance", long_about = None)]
pub struct Config {
    /// Input password to generate variations from
    #[arg(short, long)]
    pub password: Option<String>,
    
    /// Read passwords from stdin, one per line
    #[arg(long)]
    pub stdin: bool,
    
    /// Maximum edit distance (1 or 2)
    #[arg(short, long, default_value = "2")]
    pub max_distance: u8,
    
    /// Include the original password in the output
    #[arg(short, long)]
    pub include_original: bool,
}