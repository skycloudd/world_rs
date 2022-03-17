use clap::Parser;
use std::process;

use world::{run, Args};

fn main() {
    let config = Args::parse();

    match run(&config) {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("[ERROR] {}", e);
            process::exit(1);
        }
    }
}
