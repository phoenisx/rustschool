use std::fmt::{Formatter, Display, Result};
use std::io::{self, Read};

pub struct Input {
    pub data: String,
}

impl Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.data)
    }
}

impl Input {
    pub fn new() -> Self {
        let mut input = Input { data: String::default() };
        // `read_to_string` requires EOF, to finish up itself...
        match io::stdin().read_to_string(&mut input.data) {
            _ => input,
        }
    }
}
