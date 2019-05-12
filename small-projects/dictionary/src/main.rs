use std::env;

mod arguments;

fn main() {
    let args = arguments::Arguments::new(env::args());
    println!("{:?}", args);
}
