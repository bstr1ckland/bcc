// bcc - Ben's C Compiler for C89 (ANSI C)

mod lexical;

use lexical::tokenize::tokenize;
use lexical::tokens::Token;

use std::result::Result;
use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Iterate through each argument for main, and read file as a string.
    for file in env::args().skip(1) {
        let file: String = fs::read_to_string(file)?;

        // We can transfer ownership of file string here, don't need it again
        let tokens: Vec<Token> = tokenize(file);

        // testing
        for t in tokens {
            println!("{:#?}", t);
        }

        // Parse tokens and generate a parse tree.

        // Apply semantic rules to parse tree.
    }

    Ok(())
}
