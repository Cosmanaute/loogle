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

    const PATH: &str = "."; 
    parser::crawl(PATH, &argv)?;

    Ok(())
}
