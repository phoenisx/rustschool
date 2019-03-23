fn get_slice(string: &String) -> &str {
  // Returns an instance of slice, i.e., Box<str> Smart reference
  // Thus they cannot be modified.
  &string[..5]
}

// This Code Panics...
fn main() {
  let mut string = String::from("Hello World");
  let mut str_5 = get_slice(&string);
  println!("{}", str_5.get("He").unwrap());
}
