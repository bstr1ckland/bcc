// bcc Ben's C Compiler for C89 (ANSI C)
// Defining lexical token types.

#[derive(Debug)]
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
    DoubleQuotation,        // "
    SingleQuotation,        // '

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
}

impl TokenType {
    // Assign token type to each valid character.
    pub fn single_chars(c: &char) -> Option<TokenType> {
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
            '"' => Some(TokenType::DoubleQuotation),
            '\'' => Some(TokenType::SingleQuotation),

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

            _ => None,
        }
    }

    // Assign token type to each valid keyword string.
    pub fn keywords(keyword: &str) -> Option<TokenType> {
        match keyword {
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
    token: TokenType,
    line: i32,
    column: i32,
    // store <optional> value for things like
    // char literals, string literals, number literals..?
}
// TODO implement later:
//  - unwrap string, numbers, identifers, chars,
