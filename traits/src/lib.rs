use std::fmt::Display;

mod ultimate_mod;
mod override_types;
use override_types::HashBuilder;
pub use ultimate_mod::prelude::*;

pub struct Human {
  pub age: u8,
  pub name: String,
}

impl Human {
  // Public method that can be accessed from anywhere, if this module is used.
  pub fn age_category(&self) {
    if self.age > 70 {
      println!("Is Old");
    } else if self.age > 35 {
      println!("Is Mature");
    } else {
      println!("Is Young");
    }
  }

  // Should not have access from anywhere.
  fn mole_position() {
    println!("Left Bottom on Face Cheek");
  }
}

impl Shinobi for Human {
  fn chakra_type(&self) {
    println!("Has Wind type Chakra");
  }
}

pub struct HashTable<K, V> {
  key: K,
  value: V
}

impl<K: HashBuilder + Display, V: Display> HashTable<K, V> {
  pub fn new(key: K, value: V) -> HashTable<K, V> {
    HashTable {
      key,
      value
    }
  }

  pub fn printMe(&self) {
    println!("Key: {}, Hash: {}, Value: {}", self.key, self.key.build(), self.value);
  }
}
