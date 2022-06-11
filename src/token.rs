#[derive(Debug, PartialEq)]
pub enum TokenType {
    #[allow(dead_code)]
    EOF,
    ILLEGAL,

    IDENT(String),
    INT(i32),

    ASSIGN,
    PLUS,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub struct Token {
    t_type: TokenType,
    line_number: i32,
    file_name: String,
}

impl Token {
    pub fn new(t: TokenType) -> Self {
        Token {
            t_type: t,
            line_number: 0,
            file_name: String::from(""),
        }
    }
}

#[test]
fn it_gives_a_valid_token() {
    assert_eq!(Token::new(TokenType::FUNCTION).t_type, TokenType::FUNCTION)
}
