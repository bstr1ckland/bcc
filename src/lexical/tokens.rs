// bcc - Ben's C Compiler for C89 (ANSI C)
// Defining lexical token types

use std::fmt::Debug;
use std::str::FromStr;
use std::num::ParseIntError;

pub static KEYWORDS: &[&str] = &[
    // types
    "void",
    "int",
    "char",
    "float",
    "double",
    "short",
    "long",
    "signed",
    "unsigned",
    "const",

    // control flow
    "if",
    "else",
    "switch",
    "return",
    "for",
    "while",
    "break",
    "continue",
];

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // ======================
    // General Token Types
    // ======================

    // Single char tokens
    LeftBrace,              // {
    RightBrace,             // }
    LeftParenthesis,        // (
    RightParenthesis,       // )
    LeftBracket,            // [
    RightBracket,           // ]
    SemiColon,              // ;
    Colon,                  // :
    Ampersand,              // &
    Percent,                // %
    Equal,                  // =
    Dot,                    // .
    Comma,                  // ,

    // Math operations and comparisons
    Plus,                   // +
    Minus,                  // -
    Slash,                  // /
    Star,                   // *
    GreaterThan,            // >
    LessThan,               // <
    Pipe,                   // |

    // Multi-character tokens
    GreaterThanOrEqual,     // >=
    LessThanOrEqual,        // <=
    PlusPlus,               // ++
    MinusMinus,             // --
    PlusEqual,              // +=
    MinusEqual,             // -=
    SlashEqual,             // /=
    StarEqual,              // *=
    EqualEqual,             // ==
    NotEqual,               // !=

    NewLine,                // \n

    Identifier,             // int x, where x is identifier
    CharLiteral,            // 'a'
    StringLiteral,          // "hello"
    NumberLiteral,          // 3.14

    // ======================
    // Reserved Keywords
    // ======================

    // Data Types
    Void,
    Int,
    Char,
    Float,
    Double,
    Short,
    Long,
    Signed,
    Unsigned,
    Const,

    // Control Flow
    If,
    Else,
    Switch,
    Return,
    For,
    While,
    Break,
    Continue,

    // Used for TokenType initialization
    Unknown,
    
    // Value here is the library
    Include(String), // TODO: Figure out how to acutally implement this.
    // Maybe we need to store #include< , library, and > seperately?
}

impl TokenType {
    // Assign token type to each valid character.
    pub fn single_chars(c: char) -> TokenType {
        match c {
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            '[' => TokenType::LeftBracket,
            ']' => TokenType::RightBracket,
            '(' => TokenType::LeftParenthesis,
            ')' => TokenType::RightParenthesis,
            ';' => TokenType::SemiColon,
            ':' => TokenType::Colon,
            '&' => TokenType::Ampersand,
            '%' => TokenType::Percent,
            '=' => TokenType::Equal,
            '.' => TokenType::Dot,
            ',' => TokenType::Comma,
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '/' => TokenType::Slash,
            '*' => TokenType::Star,
            '>' => TokenType::GreaterThan,
            '<' => TokenType::LessThan,
            '|' => TokenType::Pipe,

            _ => TokenType::Unknown,
        }
    }

    // Assign token type to each valid multi character string.
    pub fn multi_chars(s: &str) -> TokenType {
        match s {
            ">=" => TokenType::GreaterThanOrEqual,
            "<=" => TokenType::LessThanOrEqual,
            "++" => TokenType::PlusPlus,
            "--" => TokenType::MinusMinus,
            "+=" => TokenType::PlusEqual,
            "-=" => TokenType::MinusEqual,
            "/=" => TokenType::SlashEqual,
            "*=" => TokenType::StarEqual,
            "==" => TokenType::EqualEqual,
            "!=" => TokenType::NotEqual,

            // Data types
            "void"     => TokenType::Void,
            "int"      => TokenType::Int,
            "char"     => TokenType::Char,
            "float"    => TokenType::Float,
            "double"   => TokenType::Double,
            "short"    => TokenType::Short,
            "long"     => TokenType::Long,
            "signed"   => TokenType::Signed,
            "unsigned" => TokenType::Unsigned,
            "const"    => TokenType::Const,
            
            // Control
            "if"       => TokenType::If,
            "else"     => TokenType::Else,
            "switch"   => TokenType::Switch,
            "return"   => TokenType::Return,
            "for"      => TokenType::For,
            "while"    => TokenType::While,
            "break"    => TokenType::Break,
            "continue" => TokenType::Continue,

            _ => TokenType::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub token  : TokenType,
    pub value  : String,
    pub line   : u32,
    pub column : u32,
}

impl Token {
    // TODO: WIP - Should return @s, but trimmed and unwrapped.
    pub fn convert_to_string(s: &str) -> String {
        s.to_string() // need to add more
    }

    // TODO: WIP - Converts a string s and returns a number.
    pub fn convert_to_number<T>(s: &str) -> Result<T, ParseIntError> where T: std::str::FromStr<Err = ParseIntError> {
        FromStr::from_str(s)
    }

    /// Verifies a character is a symbol.
    ///
    /// Examples:
    /// ```
    /// [!, =, &, /] are all valid symbols, and will return true.
    /// [a, 4, b, \n] are not valid symbols, and will return false.
    /// ```
    pub fn is_symbol(c: &char) -> bool {
        if !c.is_alphanumeric() && !c.is_whitespace() {
            return true
        }
        false
    }

    /// Verifies that
    ///
    /// Examples:
    /// ```
    /// [foo, _foo, foo1] are valid identifiers, and will return true.
    /// [1foo, foo-, foo$] are not valid identifiers, and will return false.
    /// ```
    pub fn is_identifier(s: &str) -> bool {
        if s.contains('_') || s.chars().all(char::is_alphabetic) || s.chars().all(char::is_numeric) {
            let first_c = s.chars().next().unwrap();
            // C identifiers cannot start with a number, but can contain them
            if !first_c.is_numeric() {
                return true
            }
        }
        false
    }

    // TODO implement later:
    //  - unwrap string, numbers, identifiers, chars,
}
