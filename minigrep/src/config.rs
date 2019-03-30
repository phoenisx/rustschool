use std::env;

pub struct Config {
    query: String,
    filename: String,
    is_case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // args[1] -> String, Therefore, &args[1] -> &String...
        if args.len() < 3 {
            return Err("Required arguments: [regexp] [filepath]");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
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
