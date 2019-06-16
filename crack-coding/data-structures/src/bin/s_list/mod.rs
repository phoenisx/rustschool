#![allow(unused)]
#![allow(dead_code)]
// This implementation is mine, from scratch, learning from pluralsight
// course - https://app.pluralsight.com/library/courses/ads-part1

#[derive(Debug)]
struct Node<T> {
  val: T,
  next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
  fn new(val: T) -> Self {
    Self { val, next: None }
  }
}

#[derive(Debug)]
pub struct LinkedList<T> {
  head: Option<Node<T>>,
}

impl<T> LinkedList<T> {
  pub fn new() -> Self {
    Self { head: None }
  }
}
