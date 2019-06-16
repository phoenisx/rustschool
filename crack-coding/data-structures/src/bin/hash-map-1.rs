#![allow(unused)]
#![allow(dead_code)]

mod hash_table;
use hash_table::HashMap;

fn main() {
  let mut map = HashMap::new();

  map.put(String::from("foo"), 12);
  map.put(String::from("bar"), 24);
  map.put(String::from("foo"), 36); // Replaces the old value

  println!("Foo: {}", map.get(String::from("foo")).unwrap());
  println!("Bar: {}", map.get(String::from("bar")).unwrap());
}
