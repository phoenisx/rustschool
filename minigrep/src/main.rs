use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (_expression, filename) = parse_config(&args);

    let file_data = fs::read_to_string(filename).expect("Reading File Failed. Is File present??");

    println!("File Data: {}", file_data);
}

fn parse_config(args: &[String]) -> (&String, &String) {
    let expression = &args[1]; // args[1] -> String, Therefore, &args[1] -> &String...
    let filename = &args[2];

    (expression, filename)
}
