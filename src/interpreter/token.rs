#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    EOF,
    PROGRAM,
    NEWLINE,
    NUMBER,
    IDENT,
    STRING,
    SEMICOLON,
    BLOCK,
    ILLEGAL,

    // Keywords
    LET,
    PRINT,
    END,
    IF,
    THEN,
    WHILE,
    ELSEIF,
    ELSE,

    // Operators
    EQ,
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    EQEQ,
    NOTEQ,
    LT,
    LTEQ,
    GT,
    GTEQ,
}

impl TokenType {
    pub fn get_keyword_token(text: &str) -> Option<TokenType> {
        let upper_text = text.to_uppercase();
        match upper_text.as_str() {
            "LET"    => Some(TokenType::LET),
            "PRINT"  => Some(TokenType::PRINT),
            "END"    => Some(TokenType::END),
            "IF"     => Some(TokenType::IF),
            "THEN"   => Some(TokenType::THEN),
            "WHILE"  => Some(TokenType::WHILE),
            "ELSEIF" => Some(TokenType::ELSEIF),
            "ELSE"   => Some(TokenType::ELSE),
            _        => None
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    token_type: TokenType,
    token_text: String,
}

impl Token {
    pub fn new(token_type: TokenType, token_text: String) -> Token {
        Token {
            token_type: token_type,
            token_text: token_text
        }
    }

    pub fn get_token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn get_token_text(&self) -> &str {
        &self.token_text
    }
}
