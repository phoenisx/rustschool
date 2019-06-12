pub trait HashBuilder<T> {
  fn build(key: T) -> usize;
}

impl HashBuilder for usize {
  fn build(key: usize) -> usize {
    key % 10
  }
}
