use std::{env, fs};

use token::Token;

mod token;

fn main() {
    let args = env::args();
    // TODO: Implement more complicated compile options
    let Some(file) = args.skip(1).next() else {
        eprintln!("Error: Filepath not given\n\tExpected: xenon <source_path>");
        panic!();
    };

    let source_code =
        fs::read_to_string(file).expect("Should have been able to open specified file path");

    let tokens = Token::tokenize(source_code);

    println!("Tokens: \n{tokens:#?}");
}
