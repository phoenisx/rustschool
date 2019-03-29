
pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str>  {
        // args[1] -> String, Therefore, &args[1] -> &String...
        if args.len() < 3 {
            return Err("Required arguments: [regexp] [filepath]");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }

    pub fn get_query(&self) -> &String {
        &self.query
    }

    pub fn get_filename(&self) -> &String {
        &self.filename
    }
}
