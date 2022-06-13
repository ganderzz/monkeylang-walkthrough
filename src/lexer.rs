use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Lexer {
    input: String,
    /// The current position in `input`.
    position: usize,
    /// Current reading position after current `ch`.
    read_position: usize,
    ch: char,
}

impl Lexer {
    /// Creates a new instance of Lexer.
    /// Automatically reads and assigns `ch` to be the first character of the input.
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

    /// Reads the given input and converts each value into a token.
    pub fn read(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        loop {
            tokens.push(self.next_token());
            if self.is_end_of_file() {
                tokens.push(Token::new(TokenType::EOF));
                break;
            }
        }

        tokens
    }

    /// Reads a char from the input and advances the `read_position`.
    /// `ch` gets set to a null value when we read the end of the input.
    fn read_char(&mut self) {
        if self.is_end_of_file() {
            self.ch = '0';
        } else {
            self.ch = self
                .input
                .chars()
                .nth(self.read_position)
                .unwrap_or_default();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    /// Gets the next char in the input.
    fn peak_char(&self) -> char {
        if self.is_end_of_file() {
            return '0';
        }

        self.input
            .chars()
            .nth(self.read_position)
            .unwrap_or_default()
    }

    /// Checks if we are at the end of the `input` given.
    fn is_end_of_file(&self) -> bool {
        self.read_position >= self.input.len()
    }

    /// Reads the `input` for a contiguous string; returning the result.
    fn read_identifier(&mut self) -> String {
        let position = self.position;

        loop {
            if !self.ch.is_alphabetic() || self.ch.is_ascii_whitespace() {
                break;
            }

            self.read_char();
        }

        String::from(&self.input[position..self.position])
    }

    /// Reads the `input` for a contiguous integer; returning the value.
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

    /// Matches a string to a Monkey keyword.
    /// If a keyword isn't found, then the value gets assigned as an indentifer.
    fn lookup_identifier(ident: String) -> TokenType {
        match ident.as_str() {
            "fn" => TokenType::FUNCTION,
            "let" => TokenType::LET,
            "if" => TokenType::IF,
            "else" => TokenType::ELSE,
            "return" => TokenType::RETURN,
            "true" => TokenType::TRUE,
            "false" => TokenType::FALSE,
            _ => TokenType::IDENT(ident),
        }
    }

    /// Skips ahead of the `input` until we find non-whitespace characters.
    fn skip_whitespace(&mut self) {
        loop {
            if !self.ch.is_whitespace() {
                break;
            }

            self.read_char();
        }
    }

    /// Reads `input` and returns the current token.
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            '=' => {
                if self.peak_char() == '=' {
                    self.read_char();
                    return Token::new(TokenType::EQ);
                }

                Token::new(TokenType::ASSIGN)
            }
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
            '!' => {
                if self.peak_char() == '=' {
                    self.read_char();

                    return Token::new(TokenType::NOTEQ);
                }

                Token::new(TokenType::BANG)
            }
            '<' => Token::new(TokenType::LT),
            '>' => Token::new(TokenType::GT),
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
fn it_should_lex_double_tokens() {
    // Arrange
    let input = "== !=";
    let expected = vec![
        Token::new(TokenType::EQ),
        Token::new(TokenType::NOTEQ),
        Token::new(TokenType::EOF),
    ];

    // Act
    let mut lexer = Lexer::new(input.to_string());
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
    let mut lexer = Lexer::new(input.to_string());
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
    let mut lexer = Lexer::new(input.to_string());
    let tokens = lexer.read();

    // Assert
    assert_eq!(tokens, expected);
}
