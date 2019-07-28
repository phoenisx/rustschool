use std::env;
use std::process;

use minigrep;

fn main() {
    let config = minigrep::Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        println!("Problem reading File: {}", err);
        process::exit(1);
    }
}
