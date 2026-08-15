// bcc Ben's C Compiler for C89 (ANSI C)
// Tokenizing inputs.

use super::tokens::{Token, TokenType};

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let line: u32 = 1;
    let column: u32 = 1;

    // loop through input string char by char.
    // process the following:
    //      - single char tokens
    //      - keywords
    //      - numbers
    //      - identifiers
    //      - other stuff in tokens.rs
    //      - whitespace

    tokens
}

// TODO implement test functions
