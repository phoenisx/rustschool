pub struct Person {
  first_name: String,
  last_name: String,
}

impl Person {
  pub fn new(first_name: &str, last_name: &str) -> Person {
    Person {
      first_name: String::from(first_name),
      last_name: String::from(last_name),
    }
  }

  pub fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }
}
