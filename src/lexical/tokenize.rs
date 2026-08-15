// bcc Ben's C Compiler for C89 (ANSI C)
// Tokenizing inputs (via byte array).

use super::tokens::{Token, TokenType};

pub fn tokenize(bytes: Vec<u8>) -> Vec<Token<'static>> {
    let tokens: Vec<Token> = Vec::new();

    let line: u32 = 1;
    let column: u32 = 1;

    for char in bytes {
        // process the following:
        //      - single char tokens
        //      - keywords
        //      - numbers
        //      - identifiers
        //      - other stuff in tokens.rs
        //      - whitespace
    }

    tokens
}
