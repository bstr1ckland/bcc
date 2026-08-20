// bcc Ben's C Compiler for C89 (ANSI C)
// Tokenizing inputs.

use std::process::exit;

use super::tokens::{Token, TokenType};

pub fn tokenize(s: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut curr_l: u32 = 0;
    let mut curr_c: u32 = 0;

    for c in s.chars() {
        let mut t_type= Some(TokenType::Unknown);

        // Character literal
        if c == '\'' {
            t_type = TokenType::single_chars(c);
        }

        // String || Number literal
        else if c == '"' {

        }

        // Advance to next token
        if c == '\n' {
            curr_l += 1;
            curr_c = 1;
        } else {
            curr_c += 1;
        }

        if t_type == Some(TokenType::Unknown) {
            println!("Error: Unknown token found {}", c);
            exit(1);
        }

        let t = Token {
            token:  t_type,
            value:  c.to_string(),
            line:   curr_l,
            column: curr_c,
        };

        tokens.push(t);
        
    }

    tokens
}
