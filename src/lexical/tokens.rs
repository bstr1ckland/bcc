// bcc Ben's C Compiler for C89 (ANSI C)
// Defining lexical token types.

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
    StringLiteral(String),  // hello
    NumberLiteral(String),  // 3.14 (will be converted later)

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

    // Other
    Include,
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

            // TODO: Find out if there are more and implement them

            _ => None,
        }
    }

    // Assign token type to each valid multi character string.
    pub fn multi_chars(s: &str) -> Option<TokenType> {
        match s {
            ">=" => Some(TokenType::GreaterThanOrEqual),
            "<=" => Some(TokenType::LessThanOrEqual),
            _ => None,
        }
    }

    // Assign token type to each valid keyword string.
    pub fn keywords(keyword: &str) -> Option<TokenType> {
        match keyword {
            // Data types
            "void" => Some(TokenType::Void),

            // Control Flow
            "if" => Some(TokenType::If),

            _ => None,

        }
    }
}
