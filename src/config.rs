use clap::Parser;

/// Generates password variations based on Levenshtein distance
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// The password to generate variations for (use --stdin for multiple)
    #[arg(short, long)]
    pub password: Option<String>,

    /// Read passwords from standard input (one per line)
    #[arg(long)]
    pub stdin: bool,

    /// Maximum Levenshtein distance for variations
    #[arg(short, long, default_value_t = 1)]
    pub max_distance: usize,

    /// Maximum transposition distance for variations
    #[arg(short = 't', long, default_value_t = 1)]
    pub transposition_distance: usize,

    /// Include the original password(s) in the output
    #[arg(long)]
    pub include_original: bool,

    /// Count the number of variations instead of printing them
    #[arg(long)]
    pub count: bool,
}