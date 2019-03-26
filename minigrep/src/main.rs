use std::env;
use std::fs;

use minigrep;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = minigrep::Config::new(&args);

    let file_data =
        fs::read_to_string(config.get_filename()).expect("Reading File Failed. Is File present??");

    println!("File Data: {}", file_data);
}
