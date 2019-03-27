// Basically I separated out Config, only to test re-exports...
use std::fs;

mod config;
pub use config::config::Config;

pub fn run(config: Config) {
	let file_data =
        fs::read_to_string(config.get_filename()).expect("Reading File Failed. Is File present??");

    println!("File Data: {}", file_data);	
}
