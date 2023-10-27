use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use clap::Parser;

mod lexer;
use lexer::Token;

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

    let mut lexer = lexer::Lexer::new("1 + 2     *3-4/5");
    let tokens = lexer.tokenize();
    println!("{:#?}", tokens);

    assert_eq!(
        tokens,
        vec![
            Token::Number(1),
            Token::Plus,
            Token::Number(2),
            Token::Multiply,
            Token::Number(3),
            Token::Minus,
            Token::Number(4),
            Token::Divide,
            Token::Number(5)
        ]
    );

    Ok(())
}
