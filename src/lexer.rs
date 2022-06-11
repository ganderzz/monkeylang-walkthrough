use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '0',
        };
        lexer.read_char();

        lexer
    }

    pub fn read(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        loop {
            if self.read_position > self.input.len() {
                break;
            }

            tokens.push(self.next_token());
        }

        tokens
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;

        loop {
            if !self.ch.is_alphabetic() {
                break;
            }

            self.read_char();
        }

        String::from(&self.input[position..self.position])
    }

    fn read_digit(&mut self) -> i32 {
        let position = self.position;

        loop {
            if !self.ch.is_digit(10) {
                break;
            }

            self.read_char();
        }

        self.input[position..self.position].parse().unwrap()
    }

    fn lookup_identifier(ident: String) -> TokenType {
        match ident.as_str() {
            "fn" => TokenType::FUNCTION,
            "let" => TokenType::LET,
            _ => TokenType::IDENT(ident),
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            if !self.ch.is_whitespace() {
                break;
            }

            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            '=' => Token::new(TokenType::ASSIGN),
            ';' => Token::new(TokenType::SEMICOLON),
            '(' => Token::new(TokenType::LPAREN),
            ')' => Token::new(TokenType::RPAREN),
            '{' => Token::new(TokenType::LBRACE),
            '}' => Token::new(TokenType::RBRACE),
            ',' => Token::new(TokenType::COMMA),
            '+' => Token::new(TokenType::PLUS),
            item => {
                if item.is_alphabetic() {
                    return Token::new(Lexer::lookup_identifier(self.read_identifier()));
                }
                if item.is_digit(10) {
                    return Token::new(TokenType::INT(self.read_digit()));
                }

                Token::new(TokenType::ILLEGAL)
            }
        };

        self.read_char();

        token
    }
}

#[test]
fn it_should_read_chars() {
    let input = "let a = 5;";
    let mut lexer = Lexer::new(input.to_string());

    for char in input.chars() {
        assert_eq!(lexer.ch, char);
        lexer.read_char();
    }

    // Read one last time for the null-state
    lexer.read_char();
    assert_eq!(lexer.ch, '0');
}

#[test]
fn it_should_give_tokens() {
    let input = "let a = 5;";
    let expected = vec![
        Token::new(TokenType::LET),
        Token::new(TokenType::IDENT("a".to_string())),
        Token::new(TokenType::ASSIGN),
        Token::new(TokenType::INT(5)),
        Token::new(TokenType::SEMICOLON),
    ];
    let mut lexer = Lexer::new(input.to_string());

    for expected_token in expected {
        assert_eq!(lexer.next_token(), expected_token);
    }
}
