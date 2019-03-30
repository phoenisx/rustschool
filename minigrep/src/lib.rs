// Error is a trait representing the basic expectations for error values, i.e., values of type E in Result<T, E>
use std::error::Error;
use std::fs;

// Basically I separated out Config, only to test re-exports...
mod config;
pub use config::Config;

mod search;

///
/// This method is the main cycle, which runs the logical part of the application.
/// It can throw error, but nothing is returned when it passes.
///
/// @returns Result<Ok, Err>
///
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let file_data = fs::read_to_string(config.get_filename())?; // Propagating Error to caller.
  let found = search::search(config.get_query(), &file_data);
  println!("Found: {:?}", found);
  Ok(())
}
