// Well this is a very naive and simple implementation fo Hash map in rust.
// It's just to learn Rust, via data structures...
// Simplefied becoz I will concentrate only for number and strings as keys...
use core::ptr::NonNull;

pub trait HashBuilder {
  fn hash(&self) -> usize;
}

// This is not exactly method overriding
// But this is what is the closest way A method can be overriden for various types
// in Rust...
impl HashBuilder for usize {
  fn hash(&self) -> usize {
    *self % usize::max_value()
  }
}

impl HashBuilder for String {
  fn hash(&self) -> usize {
    self.chars().into_iter().map(|c| c as usize).sum()
  }
}

// This is partially copied from -> https://github.com/rust-lang/hashbrown/blob/master/src/raw/mod.rs
// To understand the advanced Rust use of generics and pointers for lazy loaded values...
struct HashNode<T> {
  data: NonNull<T>,
  // TODO: add `next` property to iterate through List of Nodes...
}

impl<T> HashNode<T> {
  fn new() -> Self {
    Self {
      data: NonNull::dangling(), // https://doc.rust-lang.org/std/ptr/struct.NonNull.html
    }
  }
}

pub struct HashMap<K: HashBuilder, V> {
  table: HashNode<(K, V)>,
}

impl<K, V> HashMap<K, V>
where
  K: HashBuilder,
{
  pub fn new() -> Self {
    Self {
      table: HashNode::new(),
    }
  }
}
