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
        let file_content =
            read_file(file).unwrap_or_else(|_| panic!("Cound not read file {}", file));
        let loc = file_content
            .iter()
            .filter(|line| is_code_line(line))
            .collect::<Vec<_>>()
            .len();

        println!("-- File: `{}` contains: {} LOC", file, loc);
    }

    //println!("{:?}", config);
}

fn read_file(file_name: &str) -> Result<Vec<String>, ()> {
    if let Ok(file) = File::open(file_name) {
        let reader = BufReader::new(file);
        Ok(reader.lines().map(|l| l.unwrap_or_default()).collect())
    } else {
        Err(())
    }
}

fn is_code_line(line: &str) -> bool {
    let line = line.trim();
    if line.is_empty() || line.starts_with("//") {
        return false;
    }

    true
}
