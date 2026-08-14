

pub enum TokenKind {
    // Single char tokens
    LeftBrace,        // {
    RightBrace,       // }
    LeftParenthesis,  // (
    RightParenthesis, // )
    LeftBracket,      // [
    RightBracket,     // ]
    SemiColon,        // ;
    Colon,            // :

    // Literals

    // Keywords
    Void,
    Include,
}
