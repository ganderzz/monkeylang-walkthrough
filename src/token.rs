#[derive(Debug, PartialEq)]
pub enum TokenType {
    #[allow(dead_code)]
    EOF,
    ILLEGAL,

    IDENT(String),
    INT(i32),

    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    FORWARDSLASH,
    LT,
    GT,

    EQ,
    NOTEQ,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub struct Token {
    pub t_type: TokenType,
}

impl Token {
    pub fn new(t: TokenType) -> Self {
        Token { t_type: t }
    }
}

#[test]
fn it_gives_a_valid_token() {
    assert_eq!(Token::new(TokenType::FUNCTION).t_type, TokenType::FUNCTION)
}
