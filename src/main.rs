use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    path: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
