use scraper::{Html, Selector};
use std::io::Result;
use crate::lexer::*;
use std::fs;
use std::io::Read;
use std::path::Path;

pub fn crawl(root: &str, terms: &Vec<String>) -> Result<Vec<(String, i32)>> {
    let mut evaled: Vec<(String, i32)> = Vec::new(); // creating new empty vector with tuple
    let path = Path::new(root);
    if path.is_dir() {
        // reading directories
        for entry in fs::read_dir(path)? {
            let dir = entry?;
            let dir_path = dir.path();

            if dir_path.is_dir() {
                // using recursion if path = directory
                if let Some(dir_str) = dir_path.to_str() {crawl(dir_str, &terms)?;}
                else {eprintln!("Error reading directory: {:?}", dir_path);}
            }
            else {
                // checking if file is .html file
                if dir_path.to_str().unwrap().ends_with(".html") {
                    let tokens = parse(&dir_path.to_str().unwrap())?;
                    // evluating tokens with the terms and giving points
                    let pts = evaluate(tokens, &terms)?;
                    let result: (String, i32) = (dir_path.to_str().unwrap().to_string(), pts);
                    evaled.push(result.clone()); 
                }
            }
        }
    } else {
        eprintln!("Error reading directory: {}", root);
    }

    Ok(evaled)
}

fn filter_tokens(tokens: &Vec<Token>) -> Result<Vec<String>> {
    let mut filtered: Vec<String> = Vec::new();
    let mut buffer = String::new();

    for token in tokens.iter() {
        match token.token_t {
            TokenType::Space => {
                filtered.push(buffer.to_lowercase().clone());
                buffer.clear();
            },
            TokenType::Symbol => {continue;},
            _ => {
                buffer.push(token.content);
            }
        };
    }
        
    if !buffer.is_empty() {
        filtered.push(buffer.clone());
    }

    Ok(filtered)
}

pub fn evaluate(tokens: Vec<Token>, terms: &Vec<String>) -> Result<i32> {
    let arguments: Vec<String> = terms.clone();
    let filtered: Vec<String>  = filter_tokens(&tokens)?; 
    let mut pts = 0;
    
    // checking if terms appears
    for i in filtered.clone().into_iter() {
        for j in arguments.iter() {
            if i == j.to_string() {
               pts += 1;
            }
        }  
    }

    // checking if terms appears after each other
    for i in 1..=arguments.len() {
        for j in arguments.windows(i) {
            for k in filtered.windows(i) {
                if j == k {
                    let factor: i32 = match i {
                        1 => 0,
                        _ => 1,
                    };
                    pts += i as i32 * factor;
                }
            }
        }
    }
    
    Ok(pts)
}

// parsing file using scraper library
pub fn parse(file: &str) -> Result<Vec<Token>> {
    let mut f = fs::File::open(&file)?;
    let mut contents = String::new(); // creating new empty string
    f.read_to_string(&mut contents)?; // reading to string

    let selection = Html::parse_document(&contents);
    let buffer = Selector::parse("p").unwrap();
    let buffer_2 = Selector::parse("span").unwrap();
    let mut paragraphs = String::new(); // creating new empty string

    // iterating through the text and adding it ot the paragraphs-string
    for elem in selection.select(&buffer) {
        let tmp = elem.text().collect::<String>();
        paragraphs.push_str(&tmp);
    }

    paragraphs.push(' ');

    // iterating through the text and adding it ot the paragraphs-string
    for elem in selection.select(&buffer_2) {
        let tmp = elem.text().collect::<String>();
        paragraphs.push_str(&tmp);
    }

    let mut tokens: Vec<Token> = Vec::new(); // creating new empty vector(list) with type Token
    let mut lexer = Lexer::new(paragraphs); // creating new varivble with Lexer class in lexer.rs
    let mut token: Token = lexer.next_token().unwrap(); // starting token

    while token.token_t != TokenType::EOF { // looping and checking if the token is EOF (end of file)
        tokens.push(token);
        token = lexer.next_token().unwrap();
    } 

    Ok(tokens)
}
