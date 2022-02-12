#![allow(unused)]

use human_panic::setup_panic;
use clap::Parser;
use serde_json::json;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The pattern to look for
    #[clap()]
    pattern: String,

    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,

    #[clap(long = "json")]
    json: bool,
}

fn main() {
    setup_panic!();

    let args = Args::parse();

    let result = std::fs::read_to_string(&args.path);

    let content = match result {
        Ok(content) => { content }
        Err(error) => {
            panic!("Error: {}", error);
        }
    };

    grrs::print_matches(&content, &args.pattern, &mut std::io::stdout(), &args.json);
}


