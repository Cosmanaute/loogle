use scraper::{Html, Selector};
use crate::lexer::*;
use std::fs;
use std::io::Read;
use std::path::Path;

pub fn crawl(root: &str, terms: &Vec<String>) -> std::io::Result<()> {
    let mut evaled: Vec<(String, i32)> = Vec::new();
    let path = Path::new(root);
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let dir = entry?;
            let dir_path = dir.path();

            if dir_path.is_dir() {
                if let Some(dir_str) = dir_path.to_str() {crawl(dir_str, &terms)?;}
                else {eprintln!("Error reading directory: {:?}", dir_path);}
            }
            else {
                if dir_path.to_str().unwrap().ends_with(".html") {
                    let tokens = parse(&dir_path.to_str().unwrap())?;
                    let pts = evaluate(tokens, &terms)?;
                    let result: (String, i32) = (dir_path.to_str().unwrap().to_string(), pts);
                    evaled.push(result.clone()); 
                }
            }
        }
    } else {
        eprintln!("Error reading directory: {}", root);
    }
    println!("{:?}", evaled);

    Ok(())
}

fn filter_tokens(tokens: &Vec<Token>) -> std::io::Result<Vec<String>> {
    let mut filtered: Vec<String> = Vec::new();
    let mut buffer = String::new();

    for token in tokens.iter() {
        match token.token_t {
            TokenType::Space => {
                filtered.push(buffer.to_lowercase().clone());
                println!("{}", buffer);
                buffer.clear();
            },
            TokenType::Letter => {
                buffer.push(token.content);
            },
            _ => {
                continue;
            }
        };
    }
        
    if !buffer.is_empty() {
        filtered.push(buffer.clone());
    }

    Ok(filtered)
}

pub fn evaluate(tokens: Vec<Token>, terms: &Vec<String>) -> std::io::Result<i32> {
    let arguments: Vec<String> = terms.clone();
    let filtered_tokens: Vec<String>  = filter_tokens(&tokens)?; 
    let mut pts = 0;
    
    // checking if terms appears
    for i in filtered_tokens.into_iter() {
        for j in arguments.iter() {
            if i == j.to_string() {
               pts += 1;
            }
        }  
    }

    Ok(pts)
}

pub fn parse(file: &str) -> std::io::Result<Vec<Token>> {
    let mut f = fs::File::open(&file)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let selection = Html::parse_document(&contents);
    let buffer = Selector::parse("p").unwrap();
    let buffer_2 = Selector::parse("span").unwrap();
    let mut paragraphs = String::new();

    for elem in selection.select(&buffer) {
        let tmp = elem.text().collect::<String>();
        paragraphs.push_str(&tmp);
    }

    paragraphs.push(' ');

    for elem in selection.select(&buffer_2) {
        let tmp = elem.text().collect::<String>();
        paragraphs.push_str(&tmp);
    }

    let mut tokens: Vec<Token> = Vec::new();
    let mut lexer = Lexer::new(paragraphs);
    let mut token: Token = lexer.next_token().unwrap();

    while token.token_t != TokenType::EOF {
        tokens.push(token);
        token = lexer.next_token().unwrap();
    } 

    Ok(tokens)
}
