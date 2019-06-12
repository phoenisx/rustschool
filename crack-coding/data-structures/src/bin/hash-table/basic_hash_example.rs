trait DefaultHashBuilder<K> {
  fn build(key: K) -> usize;
};

// Method overriding in Rust...
impl DefaultHashBuilder

pub struct HashMap<K, V, S> {
  hash_builder: DefaultHashBuilder<K>
}
