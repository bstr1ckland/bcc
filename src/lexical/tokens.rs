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
    NewLine,                // \n
    Identifier,             // int x, where x is identifer
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
}

impl TokenType {
    // Assign token type to each valid character.
    pub fn single_chars(c: char) -> Option<TokenType> {
        match c {
            '{' => Some(TokenType::LeftBrace),
            '}' => Some(TokenType::RightBrace),
            '[' => Some(TokenType::LeftBracket),
            ']' => Some(TokenType::RightBracket),
            '(' => Some(TokenType::LeftParenthesis),
            ')' => Some(TokenType::RightParenthesis),
            ';' => Some(TokenType::SemiColon),
            ':' => Some(TokenType::Colon),
            '&' => Some(TokenType::Ampersand),
            '%' => Some(TokenType::Percent),
            '=' => Some(TokenType::Equal),
            '.' => Some(TokenType::Dot),
            ',' => Some(TokenType::Comma),

            '+' => Some(TokenType::Plus),
            '-' => Some(TokenType::Minus),
            '/' => Some(TokenType::Slash),
            '*' => Some(TokenType::Star),
            '>' => Some(TokenType::GreaterThan),
            '<' => Some(TokenType::LessThan),
            '|' => Some(TokenType::Pipe),

            _ => None,
        }
    }

    // Assign token type to each valid multi character string.
    pub fn multi_chars(s: &str) -> Option<TokenType> {
        match s {
            ">=" => Some(TokenType::GreaterThanOrEqual),
            "<=" => Some(TokenType::LessThanOrEqual),
            "++" => Some(TokenType::PlusPlus),
            "--" => Some(TokenType::MinusMinus),
            "+=" => Some(TokenType::PlusEqual),
            "-=" => Some(TokenType::MinusEqual),
            "/=" => Some(TokenType::SlashEqual),
            "*=" => Some(TokenType::StarEqual),
            "==" => Some(TokenType::EqualEqual),

            // Data types
            "void"     => Some(TokenType::Void),
            "int"      => Some(TokenType::Int),
            "char"     => Some(TokenType::Char),
            "float"    => Some(TokenType::Float),
            "double"   => Some(TokenType::Double),
            "short"    => Some(TokenType::Short),
            "long"     => Some(TokenType::Long),
            "signed"   => Some(TokenType::Signed),
            "unsigned" => Some(TokenType::Unsigned),
            "const"    => Some(TokenType::Const),

            // Control Flow
            "if"       => Some(TokenType::If),
            "else"     => Some(TokenType::Else),
            "switch"   => Some(TokenType::Switch),
            "return"   => Some(TokenType::Return),
            "for"      => Some(TokenType::For),
            "while"    => Some(TokenType::While),
            "break"    => Some(TokenType::Break),
            "continue" => Some(TokenType::Continue),

            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub token:  Option<TokenType>,
    pub value:  String,
    pub line:   u32,
    pub column: u32,
}

impl Token {
    // Should return @s, but trimmed and unwrapped.
    pub fn convert_to_string(s: &str) -> String {
        s.to_string() // need to add more
    }

    // Convers a string s and returns a number.
    pub fn convert_to_number<T>(s: &str) -> Result<T, ParseIntError>
        where T: std::str::FromStr<Err = ParseIntError> {
        FromStr::from_str(s)
    }

    // TODO implement later:
    //  - unwrap string, numbers, identifers, chars,
}
