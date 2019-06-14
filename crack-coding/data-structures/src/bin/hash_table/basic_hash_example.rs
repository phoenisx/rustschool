// Well this is a very naive and simple implementation fo Hash map in rust.
// It's just to learn Rust, via data structures...
// Simplefied becoz I will concentrate only for number and strings as keys...

// TODO: Read this and implement HashTable...
// https://github.com/contain-rs/linked-hash-map/blob/master/src/lib.rs
// https://doc.rust-lang.org/stable/book/ch19-01-unsafe-rust.html

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

struct HashNode<K, V> {
  key: K,
  val: V,
  next: Option<Box<HashNode<K, V>>>
}

impl<K, V> HashNode<K, V> {
  fn new(key: K, val: V) -> Self {
    Self {
      key,
      val,
      next: None
    }
  }
}

pub struct HashMap<K: HashBuilder, V> {
  table: Vec<HashNode<K, V>>, // We need to store hashes in tables
}

impl<K, V> HashMap<K, V>
where
  K: HashBuilder,
{
  pub fn new() -> Self {
    Self {
      table: Vec::with_capacity(MAX_CAPACITY as usize)
    }
  }

  pub fn put(&mut self, key: K, val: V) {
    let hash = key.hash();
    // let node = &mut self.table[hash];
    match node
  }
}
