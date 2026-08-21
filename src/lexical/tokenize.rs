// bcc Ben's C Compiler for C89 (ANSI C)
// Tokenizing inputs.

use std::process::exit;

use super::tokens::{Token, TokenType, KEYWORDS};

pub fn tokenize(s: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut curr_l: u32 = 0; // line
    let mut curr_c: u32 = 0; // column

    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        let mut t_type= Some(TokenType::Unknown);
        let mut value = "".to_string();

        // Character literal, ex: 'c'
        if c == '\'' {
            t_type = TokenType::single_chars(c);
            value = c.to_string();
        }

        // String || Number literal, ex: "hello"
        else if c == '"' {
            // store a string
        }

        // Keywords and identifiers
        else if c.is_alphabetic() {
            // Build the rest of the found word
            value.push(c);
            while let Some(next_c) = chars.next() {
                if c.is_whitespace() {
                    break;
                }
                curr_c += 1;
                value.push(next_c);
            }

            // Check for keyword, else it is an identifier
            if KEYWORDS.contains(&value.as_str()) {
                t_type = TokenType::multi_chars(&value);
            } else {
                t_type = Some(TokenType::Identifier);
            }            
        }

        // Update index
        if c == '\n' {
            curr_l += 1;
            curr_c = 1;
        } else {
            curr_c += 1;
        }

        // Fail if token type hasn't been updated yet
        if t_type == Some(TokenType::Unknown) {
            println!("Error: Unknown token found {}", c);
            exit(1);
        }

        let t = Token {
            token:  t_type,
            value:  value,
            line:   curr_l,
            column: curr_c,
        };

        tokens.push(t);
        
    }

    tokens
}
