use std::env;

mod arguments;
mod config;

fn main() {
    let args = arguments::Arguments::new(env::args());
    match args {
        Ok(args) => run(args),
        Err(e) => println!("{}", e.kind().as_str()),
    }
}

// Will run the console command, if proper number of args are passed
fn run(args: arguments::Arguments) {
    let config = config::Config::new();
}
