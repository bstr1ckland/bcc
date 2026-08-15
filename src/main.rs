// bcc Ben's C Compiler for C89 (ANSI C)

mod lexical;

use lexical::tokens::Token;
use lexical::tokenize::tokenize;

use std::env;
use std::fs::File;
use std::io::Read;
use std::result::Result;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Iterate through each argument for main, and read file as a byte array.
    for file in env::args().skip(1) {
        let mut file = File::open(&file)?;
        let mut bytes: Vec<u8> = Vec::new();
        file.read_to_end(&mut bytes)?;

        // Tokenize byte array, 
        // Transfer ownership of bytes since (I don't think) we need it anymore.
        let tokens: Vec<Token> = tokenize(bytes);

        // Parse tokens and generate a parse tree.

        // Apply semantic rules to parse tree.
    }

    Ok(())
}
