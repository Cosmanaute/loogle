use std::env;
use std::process::exit;
mod parser;
mod lexer;

fn main() -> std::io::Result<()> {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    if argc == 0 {
        exit(1); 
    }

    const PATH: &str = "../documents"; 
    let mut result = parser::crawl(PATH, &argv)?;
    result.sort_by(|a, b| a.1.cmp(&b.1));
    for i in result.iter() {
        println!("{:?}", i);
    }

    Ok(())
}
