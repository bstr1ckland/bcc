// bcc Ben's C Compiler for C89 (ANSI C)
// Tokenizing inputs (via byte array).

use super::tokens::{Token, TokenType};

// TODO
// Maybe change it to Vec<String>,
// since one String is a file, and we want
// to compile multiple files at once

pub fn tokenize(s: String) -> Vec<Token<'static>> {
    let tokens: Vec<Token> = Vec::new();

    let line: u32 = 1;
    let column: u32 = 1;

    for c in s.chars() {
        // process the following:
        //      - single char tokens
        //      - keywords
        //      - numbers
        //      - identifiers
        //      - whitespace
        //      - other stuff in tokens.rs
    }

    tokens
}
