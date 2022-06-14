use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    /// The current position in `input`.
    position: usize,
    /// Current reading position after current `ch`.
    read_position: usize,
    current_character: Option<char>,
}

impl<'a> Lexer<'a> {
    /// Creates a new instance of Lexer.
    /// Automatically reads and assigns `ch` to be the first character of the input.
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            current_character: None,
        };
        lexer.read_char();

        lexer
    }

    /// Reads the given input and converts each value into a token.
    pub fn read(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        loop {
            let token = self.next_token();

            if token.is_none() {
                tokens.push(Token::new(TokenType::EOF));
                break;
            }

            tokens.push(token.unwrap());
        }

        tokens
    }

    /// Reads a char from the input and advances the `read_position`.
    /// `ch` gets set to a null value when we read the end of the input.
    fn read_char(&mut self) {
        if self.is_end_of_file() {
            self.current_character = None;
        } else {
            self.current_character = self.input.chars().nth(self.read_position);
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    /// Gets the next char in the input.
    fn peak_char(&self) -> Option<char> {
        if self.is_end_of_file() {
            return None;
        }

        self.input.chars().nth(self.read_position)
    }

    /// Checks if we are at the end of the `input` given.
    fn is_end_of_file(&self) -> bool {
        self.read_position > self.input.len()
    }

    /// Reads the `input` for a contiguous string; returning the result.
    fn read_identifier(&mut self) -> &'a str {
        let position = self.position;

        loop {
            if self.is_end_of_file() {
                break;
            }

            if !self.current_character.unwrap().is_alphabetic()
                || self.current_character.unwrap().is_ascii_whitespace()
            {
                break;
            }

            self.read_char();
        }

        &self.input[position..self.position]
    }

    /// Reads the `input` for a contiguous integer; returning the value.
    fn read_digit(&mut self) -> i32 {
        let position = self.position;

        loop {
            if !self.current_character.unwrap().is_digit(10) {
                break;
            }

            self.read_char();
        }

        self.input[position..self.position].parse().unwrap()
    }

    /// Matches a string to a Monkey keyword.
    /// If a keyword isn't found, then the value gets assigned as an indentifer.
    fn lookup_identifier(ident: &str) -> TokenType {
        match ident {
            "fn" => TokenType::FUNCTION,
            "let" => TokenType::LET,
            "if" => TokenType::IF,
            "else" => TokenType::ELSE,
            "return" => TokenType::RETURN,
            "true" => TokenType::TRUE,
            "false" => TokenType::FALSE,
            _ => TokenType::IDENT(String::from(ident)),
        }
    }

    /// Skips ahead of the `input` until we find non-whitespace characters.
    fn skip_whitespace(&mut self) {
        loop {
            if self.current_character.is_none()
                || !self.current_character.unwrap().is_ascii_whitespace()
            {
                return;
            }

            self.read_char();
        }
    }

    /// Reads `input` and returns the current token.
    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if self.current_character.is_none() {
            return None;
        }

        let token = match self
            .current_character
            .expect("Current character is not known.")
        {
            '=' => match self.peak_char() {
                Some(x) => {
                    if x == '=' {
                        self.read_char();

                        Token::new(TokenType::EQ)
                    } else {
                        Token::new(TokenType::ASSIGN)
                    }
                }
                None => Token::new(TokenType::ILLEGAL),
            },
            ';' => Token::new(TokenType::SEMICOLON),
            '(' => Token::new(TokenType::LPAREN),
            ')' => Token::new(TokenType::RPAREN),
            '{' => Token::new(TokenType::LBRACE),
            '}' => Token::new(TokenType::RBRACE),
            ',' => Token::new(TokenType::COMMA),
            '+' => Token::new(TokenType::PLUS),
            '-' => Token::new(TokenType::MINUS),
            '*' => Token::new(TokenType::ASTERISK),
            '/' => Token::new(TokenType::FORWARDSLASH),
            '!' => match self.peak_char() {
                Some(x) => {
                    if x == '=' {
                        self.read_char();

                        Token::new(TokenType::NOTEQ)
                    } else {
                        Token::new(TokenType::BANG)
                    }
                }
                None => Token::new(TokenType::ILLEGAL),
            },
            '<' => Token::new(TokenType::LT),
            '>' => Token::new(TokenType::GT),
            item => {
                if item.is_alphabetic() {
                    Token::new(Lexer::lookup_identifier(self.read_identifier()))
                } else if item.is_digit(10) {
                    Token::new(TokenType::INT(self.read_digit()))
                } else {
                    Token::new(TokenType::ILLEGAL)
                }
            }
        };

        self.read_char();

        Some(token)
    }
}

#[test]
fn it_should_read_chars() {
    let input = "let a = 5;";
    let mut lexer = Lexer::new(input);

    for char in input.chars() {
        assert_eq!(lexer.current_character.unwrap(), char);
        lexer.read_char();
    }
}

#[test]
fn it_should_lex_double_tokens() {
    // Arrange
    let input = "== !=";
    let expected = vec![
        Token::new(TokenType::EQ),
        Token::new(TokenType::NOTEQ),
        Token::new(TokenType::EOF),
    ];

    // Act
    let mut lexer = Lexer::new(input);
    let tokens = lexer.read();

    // Assert
    assert_eq!(tokens, expected);
}

#[test]
fn it_should_lex_keywords_tokens() {
    // Arrange
    let input = "fn let true false if else return";
    let expected = vec![
        Token::new(TokenType::FUNCTION),
        Token::new(TokenType::LET),
        Token::new(TokenType::TRUE),
        Token::new(TokenType::FALSE),
        Token::new(TokenType::IF),
        Token::new(TokenType::ELSE),
        Token::new(TokenType::RETURN),
        Token::new(TokenType::EOF),
    ];

    // Act
    let mut lexer = Lexer::new(input);
    let tokens = lexer.read();

    // Assert
    assert_eq!(tokens, expected);
}

#[test]
fn it_should_lex_single_tokens() {
    // Arrange
    let input = "=+-*/!<>,;(){}";
    let expected = vec![
        Token::new(TokenType::ASSIGN),
        Token::new(TokenType::PLUS),
        Token::new(TokenType::MINUS),
        Token::new(TokenType::ASTERISK),
        Token::new(TokenType::FORWARDSLASH),
        Token::new(TokenType::BANG),
        Token::new(TokenType::LT),
        Token::new(TokenType::GT),
        Token::new(TokenType::COMMA),
        Token::new(TokenType::SEMICOLON),
        Token::new(TokenType::LPAREN),
        Token::new(TokenType::RPAREN),
        Token::new(TokenType::LBRACE),
        Token::new(TokenType::RBRACE),
        Token::new(TokenType::EOF),
    ];

    // Act
    let mut lexer = Lexer::new(input);
    let tokens = lexer.read();

    // Assert
    assert_eq!(tokens, expected);
}
