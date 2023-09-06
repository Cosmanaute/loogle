use scraper::{Html, Selector};
use std::{fs, io::Read};

pub fn parse(file: &str) -> std::io::Result<()> {
    let mut f = fs::File::open(&file)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let selection = Html::parse_document(&contents);
    let buffer = Selector::parse("p").unwrap();
    let mut paragraphs = String::new();

    for elem in selection.select(&buffer) {
        let para = elem.text().collect::<String>();
        paragraphs.push_str(&para);
    }

    let mut tokens: Vec<Token> = Vec::new();
    let mut lexer = Lexer::new(paragraphs);
    let mut token: Token = lexer.next_token().unwrap();

    while token.token_t != TokenType::EOF {
        tokens.push(token);
        println!("{:?}", token);
        token = lexer.next_token().unwrap();
    } 


    Ok(())
}

struct Lexer {
    input: String,
    position: usize,
    ch: char,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum TokenType {
    Letter,
    Number,
    Symbol,
    Space,
    Other,
    EOF,
}

#[derive(Debug, Clone, Copy)]
struct Token {
    token_t: TokenType,
    content: char,
}

impl Lexer {
    fn new(input: String) -> Self {
        Self {input, position: 0, ch: '\0'}
    }

    fn next_token(&mut self) -> Option<Token> {
        if self.position >= self.input.len() {
            return Some(Token {token_t: TokenType::EOF, content: '\0'});
        }

        if let Some(ch) = self.input.chars().nth(self.position) {
            self.ch = ch;
        }

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

