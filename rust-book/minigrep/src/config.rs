use std::env;

pub struct Config {
    query: String,
    filename: String,
    is_case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // This points to the command name arg...
        let query = match args.next() {
            Some(query) => query,
            None => return Err("No Query provided for search") // Since we want to trhow error, not store a value back to query here...
        };
        let filename = match args.next() {
            Some(filename) => filename,
            None => return Err("No Filename provided for search")
        };
        let is_case_sensitive = env::var("CASE_INSENSITIVE").is_err(); // Flase if ENV is present (regardless of value), else true.

        Ok(Config {
            query,
            filename,
            is_case_sensitive,
        })
    }

    pub fn get_query(&self) -> &String {
        &self.query
    }

    pub fn get_filename(&self) -> &String {
        &self.filename
    }

    pub fn is_sensitive(&self) -> &bool {
        &self.is_case_sensitive
    }
}
