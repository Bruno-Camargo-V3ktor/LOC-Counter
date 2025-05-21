// Line Of Code - Counter
// Usage:
//  -f = "./src/main.rs, ./src/test.rs"
//  -d = "./src" -e = "rs, js, html"

mod config;

use config::Configuration;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let config = Configuration::new(args);

    println!("{:?}", config);
}
