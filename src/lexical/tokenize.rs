// bcc - Ben's C Compiler for C89 (ANSI C)
// Tokenizing inputs

use std::process::exit;

use super::tokens::{ Token, TokenType, KEYWORDS };

pub fn tokenize(s: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new(); 
    let mut chars = s.chars().peekable();

    let mut curr_l: u32 = 1; // line
    let mut curr_c: u32 = 0; // column

    // Iterate through chars
    while let Some(c) = chars.next() {
        let mut t_type = Some(TokenType::Unknown);
        let mut val = "".to_string();

        // BUG HERE
        // Currently, user needs to have an extra line at end of file,
        // This token isn't even properly stored.
        if Some(c) == None {
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

        // String || Number literal, ex: "hello"
        else if c == '"' {
            val.push(c);
            while let Some(next_c) = chars.next() {
                if next_c == '"' {
                    break;
                }
                curr_c += 1;
                val.push(next_c);
            }

            t_type = Some(TokenType::StringLiteral);
        }

        // Symbols like => , +
        else if is_symbol(c) {
            // Check if next char is whitespace, 
            // if so then don't process multiple symbols.
            if let Some(next_c) = chars.next() {
                val.push(c);
                if next_c.is_whitespace() {
                    curr_c += 1;
                    t_type = TokenType::single_chars(c)
                } else {
                    curr_c += 2;
                    val.push(next_c);
                    t_type = TokenType::multi_chars(&val);
                }
            }
        }

        // Number literals
        else if c.is_numeric() {
            val.push(c);
            while let Some(next_c) = chars.next() {
                if !next_c.is_numeric() || next_c != '.' {
                    break;
                }
                curr_c += 1;
                val.push(next_c);
            }

            if s.parse::<f64>().is_ok() {
                t_type = Some(TokenType::NumberLiteral);
            }
        }

        // Indentifiers & Keywords
        else if c.is_alphabetic() || c == '_' {
            val.push(c);
            while let Some(next_c) = chars.next() {
                if next_c.is_whitespace() || !next_c.is_alphabetic() || !next_c.is_numeric() {
                    break;
                }

                curr_c += 1;
                val.push(next_c);
            }

            if is_identifier(&s) {
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
