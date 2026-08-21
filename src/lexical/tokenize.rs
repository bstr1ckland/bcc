// bcc - Ben's C Compiler for C89 (ANSI C)
// Tokenizing inputs

use std::process::exit;

use super::tokens::{ Token, TokenType, KEYWORDS };

pub fn tokenize(s: String) -> Vec<Token> {
    // Returned at end of method
    let mut tokens: Vec<Token> = Vec::new(); 
    // File (Function param) as array of chars
    let mut chars = s.chars();
    // Peekable array of chars, allows peeking at next char in iteration
    let peekable = chars.clone().peekable();

    let mut curr_l: u32 = 0; // line
    let mut curr_c: u32 = 0; // column

    // Iterate through chars
    while let Some(c) = chars.next() {
        let mut t_type = Some(TokenType::Unknown);
        let mut val = "".to_string();

        // Handle single && mutli char tokens


        // Character literal, ex: 'c'
        if c == '\'' {
            val = c.to_string();

            // THIS IS IMPLEMENTED WRONG.
            // NEED TO MOVE TO NEXT CHAR,
            // ADD THAT TO VAL
            // AND ADD THE OTHER ' TOO.

            // WILL GET UNWRAPPED LATER
            t_type = Some(TokenType::CharLiteral);
        }

        // String || Number literal, ex: "hello"
        else if c == '"' {
            // Will unwrap string later
            val.push(c);
            while let Some(next_c) = chars.next() {
                if c == '"' {
                    break;
                }
                curr_c += 1;
                val.push(next_c);
            }

            // Check if value is numeric, 
            // otherwise it is string literal
            if val.parse::<f64>().is_ok() {
                t_type = Some(TokenType::NumberLiteral);
            } else {
                t_type = Some(TokenType::StringLiteral);
            }
        }

        // Keywords and identifiers
        else if c.is_alphabetic() {
            // Build the rest of the found word
            val.push(c);
            while let Some(next_c) = chars.next() {
                if c.is_whitespace() {
                    break;
                }
                curr_c += 1;
                val.push(next_c);
            }

            // Check for keyword, else it is an identifier
            if KEYWORDS.contains(&val.as_str()) {
                t_type = TokenType::multi_chars(&val);
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
            value:  val,
            line:   curr_l,
            column: curr_c,
        };

        tokens.push(t);
        
    }

    tokens
}
