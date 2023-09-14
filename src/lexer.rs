// making struct lexer
pub struct Lexer {
    input: String,
    position: usize,
    ch: char,
}

// making custom type TokenType
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TokenType {
    Letter,
    Number,
    Symbol,
    Space,
    Other,
    EOF,
}

//making struct token
#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub token_t: TokenType,
    pub content: char,
}

// implementing methods for lexer making it a class
impl Lexer {
    // contructor
    pub fn new(input: String) -> Self {
        Self {input, position: 0, ch: '\0'}
    }

    // function for getting the next token
    pub fn next_token(&mut self) -> Option<Token> {
        // checking if it is at the end of the file or not
        if self.position >= self.input.len() {
            return Some(Token {token_t: TokenType::EOF, content: '\0'});
        }

        // assigning self.ch with the current character if it does not return Error
        if let Some(ch) = self.input.chars().nth(self.position) {
            self.ch = ch;
        }
        else {
            return Some(Token {token_t: TokenType::EOF, content: '\0'});
        }

        // checking character type as bytes
        self.position += 1;
        let token = match self.ch {
            ' ' => Token {token_t: TokenType::Space, content: self.ch},
            _c if self.ch as u8 >= 65 && self.ch as u8 <= 90 => Token {token_t: TokenType::Letter, content: self.ch},
            _c if self.ch as u8 >= 97 && self.ch as u8 <= 122 => Token {token_t: TokenType::Letter, content: self.ch}, 
            _c if self.ch as u8 >= 48 && self.ch as u8 <= 57 => Token {token_t: TokenType::Number, content: self.ch},
            _c if self.ch as u8 >= 33 && self.ch as u8 <= 64 => Token {token_t: TokenType::Symbol, content: self.ch},
            _c if self.ch as u8 >= 91 && self.ch as u8 <= 96 => Token {token_t: TokenType::Symbol, content: self.ch},
            _c if self.ch as u8 >= 123 && self.ch as u8 <= 126 => Token {token_t: TokenType::Symbol, content: self.ch},
            _ => Token {token_t: TokenType::Other, content: self.ch}
        };
        
        return Some(token);
    }
}
