use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[2];
    println!("In file {}", filename);

    let file_data = fs::read_to_string(filename).expect("Reading File Failed. Is File present??");

    println!("File Data: {}", file_data);
}
