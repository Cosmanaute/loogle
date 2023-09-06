use std::{io, fs, path::Path};
mod parser;

fn crawl(root: &str) -> io::Result<()> {
    let path = Path::new(root);
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let dir = entry?;
            let dir_path = dir.path();

            if dir_path.is_dir() {
                if let Some(dir_str) = dir_path.to_str() {crawl(dir_str)?;}
                else {eprintln!("Error reading directory: {:?}", dir_path);}
            }
            else {
                if dir_path.to_str().unwrap().ends_with(".html") {
                    println!("\n{}\n", dir_path.to_str().unwrap());
                    let _ = parser::parse(&dir_path.to_str().unwrap())?;
                }
            }
        }
    } else {
        eprintln!("Error reading directory: {}", root);
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let path = "."; 
    crawl(path)?;

    Ok(())
}
