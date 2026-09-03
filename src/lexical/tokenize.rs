// bcc - Ben's C Compiler for C89 (ANSI C)
// Tokenizing inputs

use std::process::exit;
use crate::lexical::tokens::TokenType::Percent;
use super::tokens::{Token, TokenType, KEYWORDS };

pub fn tokenize(s: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new(); 
    let mut chars = s.chars().peekable();

    let mut curr_line: u32 = 1;
    let mut curr_column: u32 = 1;

    while let Some(c) = chars.next() {
        let mut t_type = Some(TokenType::Unknown);
        let mut val = "".to_string();
        
        if c == '\n' {
            t_type = Some(TokenType::NewLine);
            val.push('\n');
            curr_line += 1;
            curr_column = 0;
        }

        else if c.is_whitespace() {
            curr_column += 1;
            continue;
        }

        // Character literal, ex: 'c'
        else if c == '\'' {
            val.push(c); // push opening '
            if let Some(next_c) = chars.next() {
                val.push(next_c); // push the literal
                
                if let Some(next_next_c) = chars.next() {
                    val.push(next_next_c); // push closing ;
                }
            }
            curr_column += 3;
            t_type = Some(TokenType::CharLiteral);
        }

        // Identifiers & Keywords
        else if c.is_alphabetic() || c == '_' {
            val.push(c);
            curr_column += 1;

            while let Some(peek_c) = chars.peek() {
                if peek_c.is_whitespace() || is_symbol(*peek_c) {
                    break;
                }

                if let Some(next_c) = chars.next() {
                    val.push(next_c);
                    curr_column += 1;
                }
            }

            if is_identifier(&val) {
                t_type = Some(TokenType::Identifier);
            }
            if KEYWORDS.contains(&val.as_str()) {
                t_type = TokenType::multi_chars(&val);
            }
        }

        // String literal, ex: "hello"
        else if c == '"' {
            val.push(c);
            curr_column += 1;

            while let Some(next_c) = chars.next() {
                if next_c == '"' {
                    break;
                }
                curr_column += 1;
                val.push(next_c);
            }

            t_type = Some(TokenType::StringLiteral);
        }

        // Handle comments, C89 only supports /* */.
        else if c == '/' {

        }

        // Number literals
        else if c.is_numeric() || c == '-' {
            val.push(c);
            curr_column += 1;

            let mut decimal_count = 0;
            let mut is_hex = false;

            while let Some(peek_c) = chars.peek() {
                if c == '0' && *peek_c == 'x' { // Hex number
                    // Push 'x' into val
                    if let Some(next_c) = chars.next() {
                        val.push(next_c);
                        curr_column += 1;
                        is_hex = true;
                    }
                }
                else if peek_c.is_numeric() { // Integer number
                    if let Some(next_c) = chars.next() {
                        val.push(next_c);
                        curr_column += 1;
                    }
                }
                else if *peek_c == '.' && decimal_count == 0 { // Floating point num
                    decimal_count += 1;

                    // Push '.' and remaining nums
                    if let Some(next_c) = chars.next() {
                        val.push(next_c);
                        curr_column += 1;
                        while let Some(peek_c) = chars.peek() {
                            if peek_c.is_numeric() {
                                if let Some(next_c) = chars.next() {
                                    val.push(next_c);
                                    curr_column += 1;
                                }
                            }
                            else {
                                break;
                            }
                        }
                    }
                }
                // Invalid numerical character
                else {
                    break;
                }

                // Process hex number. Special case because it can contain letters
                if is_hex {
                    while let Some(peek_c) = chars.peek() {
                        // TODO: Implement hex number constraints
                        if peek_c.is_alphabetic() || peek_c.is_numeric() {
                            if let Some(next_c) = chars.next() {
                                val.push(next_c);
                                curr_column += 1;
                            }
                        }
                        else {
                            is_hex = false;
                            break;
                        }
                    }
                }
            }

            if val.parse::<f64>().is_ok() {
                t_type = Some(TokenType::NumberLiteral);
            }
            // Parse hex number
            else if i64::from_str_radix(&val.strip_prefix("0x").unwrap(), 16).is_ok() {
                t_type = Some(TokenType::NumberLiteral);
            }
        }

        // #include<>
        else if c == '#' {

        }

        // Symbols like =, > , +
        else if is_symbol(c) {
            val.push(c);
            curr_column += 1;

            let mut has_next_char = true;

            // Check the char after the current in case it's a possible -
            // multi char combo token
            if let Some(peek_c) = chars.peek() {
                if is_symbol(*peek_c) {
                    let mut temp = val.clone();
                    temp.push(*peek_c);

                    // See if we get a valid token with the combo of two chars
                    t_type = TokenType::multi_chars(&temp);
                    if t_type == Some(TokenType::Unknown) {
                        has_next_char = false;
                        t_type = TokenType::single_chars(c);
                    }
                }
                else {
                    t_type = TokenType::single_chars(c);
                    has_next_char = false;
                }
            }
            // No next char to process, must be a single char
            else {
                has_next_char = false;
                t_type = TokenType::single_chars(c);
            }

            if has_next_char {
                if let Some(next_c) = chars.next() {
                    val.push(next_c);
                    t_type = TokenType::multi_chars(&val);
                    curr_column += 1;
                }
            }
        }

        // Fail if token type hasn't been updated yet
        if t_type == Some(TokenType::Unknown) {
            println!("Error: Unknown token found '{}' , \
                at line {curr_line}, column {curr_column}.", c);
            exit(1);
        }

        tokens.push(Token {
            token:  t_type,
            value:  val,
            line:   curr_line,
            column: curr_column,
        });
        
    }

    tokens
}

// Validate identifier
fn is_identifier(s: &str) -> bool {
    if s.contains('_') || s.chars().all(char::is_alphabetic) || s.chars().all(char::is_numeric) {
        let first_c = s.chars().next().unwrap();
        // C identifiers cannot start with a number, but can contain them
        if !first_c.is_numeric() {
            return true
        }
    }
    false
}

// Check if char isn't a letter or whitespace
fn is_symbol(c: char) -> bool {
    if !c.is_alphanumeric() && !c.is_whitespace() {
        return true
    }
    false
}
