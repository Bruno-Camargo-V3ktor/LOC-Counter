// Line Of Code - Counter
// Usage:
//  -f = "./src/main.rs, ./src/test.rs"
//  -d = "./src" -e = "rs, js, html"

mod config;

use config::Configuration;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let config = Configuration::new(args);

    for file in &config.file_names {
        let file_content = read_file(file).expect(&format!("Cound not read file {}", file));
        println!("File: {} contains: {}", file, file_content.len());
    }

    //println!("{:?}", config);
}

fn read_file(file_name: &str) -> Result<Vec<String>, ()> {
    if let Ok(file) = File::open(file_name) {
        let reader = BufReader::new(file);
        Ok(reader.lines().map(|l| l.unwrap_or(String::new())).collect())
    } else {
        Err(())
    }
}
