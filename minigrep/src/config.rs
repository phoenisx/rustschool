pub mod config {
    pub struct Config {
        query: String,
        filename: String,
    }

    impl Config {
        pub fn new(args: &[String]) -> Config {
            // args[1] -> String, Therefore, &args[1] -> &String...
            let query = args[1].clone();
            let filename = args[2].clone();

            Config { query, filename }
        }

        pub fn get_query(&self) -> &String {
            &self.query
        }

        pub fn get_filename(&self) -> &String {
            &self.filename
        }
    }
}
