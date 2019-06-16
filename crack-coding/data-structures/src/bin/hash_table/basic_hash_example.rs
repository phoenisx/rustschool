#![allow(unused)]
#![allow(dead_code)]
// Well this is a very naive and simple implementation fo Hash map in rust.
// It's just to learn Rust, via data structures...
// Simplefied becoz I will concentrate only for number and strings as keys...

// TODO: Read this and implement HashTable...
// https://github.com/contain-rs/linked-hash-map/blob/master/src/lib.rs
// https://doc.rust-lang.org/stable/book/ch19-01-unsafe-rust.html

// Following is a very naive implementation, and might be even a bad code
// in terms of memory usage and stuff, but this is the simplest form of HashTable
// that anyone can learn from, using safe code in Rust...

use std::collections::LinkedList;

static MAX_CAPACITY: u8 = 30u8;

pub trait HashBuilder {
  fn hash(&self) -> usize;
}

// This is not exactly method overriding
// But this is what is the closest way A method can be overriden for various types
// in Rust...
impl HashBuilder for usize {
  fn hash(&self) -> usize {
    *self % MAX_CAPACITY as usize
  }
}

impl HashBuilder for String {
  fn hash(&self) -> usize {
    // This algo, is not secure and not uniform...
    // Different types of Hashing:
    // - Additive Hashing
    // - DJB2
    // - Folding Hash
    // - MD5
    // - SHA-2 (Most secure of all, but less efficient)
    self.chars().into_iter().map(|c| c as usize).sum::<usize>() % MAX_CAPACITY as usize
  }
}

// This is partially copied from -> https://github.com/rust-lang/hashbrown/blob/master/src/raw/mod.rs
// To understand the advanced Rust use of generics and pointers for lazy loaded values...
// struct HashNode<T> {
//   data: NonNull<T>,
//   // TODO: add `next` property to iterate through List of Nodes...
// }

// impl<T> HashNode<T> {
//   fn new() -> Self {
//     Self {
//       data: NonNull::dangling(), // https://doc.rust-lang.org/std/ptr/struct.NonNull.html
//     }
//   }
// }

#[derive(Clone)]
struct HashNode<K, V> {
  key: K,
  val: V,
}

impl<K, V> HashNode<K, V> {
  fn new(key: K, val: V) -> Self {
    Self { key, val }
  }
}

impl<K: PartialEq, V> Eq for HashNode<K, V> {}
impl<K: PartialEq, V> PartialEq for HashNode<K, V> {
  fn eq(&self, other: &Self) -> bool {
    self.key == other.key
  }
}

pub struct HashMap<K: HashBuilder, V> {
  table: Vec<Vec<HashNode<K, V>>>, // We need to store hashes in tables
}

impl<K, V> HashMap<K, V>
where
  K: PartialEq + HashBuilder + Clone,
  V: PartialEq + Clone,
{
  pub fn new() -> Self {
    Self {
      table: vec![Vec::new(); MAX_CAPACITY as usize],
    }
  }

  pub fn put(&mut self, key: K, val: V) {
    let hash = key.hash();
    let new_node = HashNode::new(key, val);
    let node: &mut Vec<HashNode<K, V>> = self.table.get_mut(hash).unwrap(); // Since I know, it is already pre-populated, unwrap will never panic.

    let mut index = 0;
    let mut found_index: Option<usize> = None;
    // find index
    for ele in node.iter() {
      if (*ele == new_node) {
        found_index = Some(index);
        break;
      }
      index += 1;
    }
    // node.push_back(HashNode::new(key, val));
    match found_index {
      Some(i) =>
      // For simplicity, for now, will not care about similar key replacements, if the key already exists.
      // Will just push to the list, even though it is similar data.
      {
        node[i] = new_node;
      } // Should actually replace the data, instead of appending
      None => node.push(new_node),
    }
  }

  pub fn get(&self, key: K) -> Option<&V> {
    let hash = key.hash();
    let node: &Vec<HashNode<K, V>> = self.table.get(hash).unwrap();
    for ele in node.iter() {
      if (ele.key == key) {
        return Some(&ele.val);
      }
    }
    None
  }
}
