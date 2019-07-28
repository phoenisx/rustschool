pub trait HashBuilder {
  fn build(&self) -> usize;
}

impl HashBuilder for usize {
  fn build(&self) -> usize {
    *self % usize::max_value()
  }
}

impl HashBuilder for String {
  fn build(&self) -> usize {
    self.chars().into_iter().map(|c| c as usize).sum()
  }
}
