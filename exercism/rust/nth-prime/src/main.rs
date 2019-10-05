use nth_prime::nth;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let value: String = args.get(1).unwrap().to_string();
    println!("Prime: {}", nth(value.parse::<u32>().unwrap()));
}
