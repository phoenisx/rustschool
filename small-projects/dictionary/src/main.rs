use std::env;

mod arguments;
mod config;

fn main() {
    let args = arguments::Arguments::new(env::args());
    println!("{:?}", args);

    let config = config::Config::new();
    println!("{:?}", config);
}
