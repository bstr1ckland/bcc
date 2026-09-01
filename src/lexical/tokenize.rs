// bcc - Ben's C Compiler for C89 (ANSI C)
// Tokenizing inputs

use std::process::exit;

use super::tokens::{ Token, TokenType, KEYWORDS };

pub fn tokenize(s: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new(); 
    let mut chars = s.chars().peekable();

    let mut curr_l: u32 = 1; // line
    let mut curr_c: u32 = 1; // column

    while let Some(c) = chars.next() {
        let mut t_type = Some(TokenType::Unknown);
        let mut val = "".to_string();

        if Some(c).is_none() {
            t_type = Some(TokenType::EOF);
        }
        
        else if c == '\n' {
            t_type = Some(TokenType::NewLine);
            val.push('\n');
            curr_l += 1;
            curr_c = 0;
        }

        else if c.is_whitespace() {
            curr_c += 1;
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
            curr_c += 3;
            t_type = Some(TokenType::CharLiteral);
        }

        // String literal, ex: "hello"
        else if c == '"' {
            val.push(c);
            curr_c += 1;

            while let Some(next_c) = chars.next() {
                if next_c == '"' {
                    break;
                }
                curr_c += 1;
                val.push(next_c);
            }

            t_type = Some(TokenType::StringLiteral);
        }

        // Symbols like =, > , +
        else if is_symbol(c) {
            val.push(c);
            curr_c += 1;

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
                else if *peek_c == '\n' {
                    has_next_char = false;
                    curr_l += 1;
                    curr_c = 1;
                    t_type = Some(TokenType::NewLine);
                }
                else if peek_c.is_whitespace() {
                    has_next_char = false;
                    t_type = TokenType::single_chars(c);
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
                    curr_c += 1;
                }
            }
        }

        // Number literals
        else if c.is_numeric() {
            val.push(c);
            curr_c += 1;

            while let Some(next_c) = chars.next() {
                if !next_c.is_numeric() || next_c != '.' {
                    break;
                }
                curr_c += 1;
                val.push(next_c);
            }

            if val.parse::<f64>().is_ok() {
                t_type = Some(TokenType::NumberLiteral);
            }
        }

        // TODO: Bugs here.
        // Identifiers & Keywords
        else if c.is_alphabetic() || c == '_' {
            val.push(c);
            curr_c += 1;

            while let Some(next_c) = chars.next() {
                // Character ahead of next_c
                if let Some(peek_c) = chars.peek() {
                    if peek_c.is_whitespace() || is_symbol(*peek_c) {
                        break;
                    }

                    curr_c += 1;
                    val.push(next_c);
                }
            }

            if is_identifier(&val) {
                t_type = Some(TokenType::Identifier);
            }
            if KEYWORDS.contains(&val.as_str()) {
                t_type = TokenType::multi_chars(&val);
            }
        }

        // Fail if token type hasn't been updated yet
        if t_type == Some(TokenType::Unknown) {
            println!("Error: Unknown token found '{}' , at line {curr_l} , column {curr_c}.", c);
            exit(1);
        }

        tokens.push(Token {
            token:  t_type,
            value:  val,
            line:   curr_l,
            column: curr_c,
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
