#![allow(unused)]
#![allow(dead_code)]
#![allow(non_snake_case)]
// Ok so this is a complete copy of what I have learnt from
// https://rust-unofficial.github.io/too-many-lists/second.html

// To be noted: Since any of the structs or enums don't have pub scope specifier
// they all will be treated as private members...

/**
 * Breakdown #2:
 *
 * - If I somehow want to store a generic that can be of any type, it is wise
 *   to create an enum/struct with a generic member, this way I have to push struct instance
 *   inside the list, where struct instance could be of any random type.
 */

#[derive(Debug)]
pub struct List<T> {
  head: Link<T>,
}

// Better way to shorten the re-used types, using aliases...
// Think of Link as a special pointer, which stores the node location
// and also owns it.
type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
  elem: T,
  next: Link<T>,
}

impl<T> List<T> {
  pub fn new() -> Self {
    List { head: None }
  }

  /**
   * Breakdown #1:
   *
   * - `mem:replace` is an unsafe code, remember that, but it can be treated as safe rust code, since it's tested.
   * -  Up here, I am replacing the previous head with None, and dissasociating it
   *    from self.head's ownership, so that we can mutate self.head to some other
   *    Option value (if needed), which is not possible due to borrow checker...
   */
  pub fn push(&mut self, elem: T) {
    self.head = Some(Box::new(Node {
      elem,
      next: self.head.take(),
    }));
  }

  // this is lifo list, so latest head should be popped
  pub fn pop(&mut self) -> Option<T> {
    self.head.take().map(|node| {
      self.head = node.next;
      node.elem
    })
  }

  // This will preserve the ownership of node.
  pub fn peek(&self) -> Option<&T> {
    self.head.as_ref().map(|node| &node.elem)
  }
}

/**
 * It is a good practise in rust to implement all three types of iterators
 * for custom collection data structures...
 *
 * - IntoIter - to get ownership on collection elements, and consume them
 * - Iter - to get immutable reference on collection items, without consuming them
 * - IterMut - to get mutable reference on collection items, without consuming them
 */

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    // We just need to pass ownership by popping head
    // This way the whole list will be consumed after loop iteration finishes.
    self.0.pop()
  }
}

pub struct Iter<'a, T> {
  next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    self.next.map(|node| {
      self.next = node.next.as_ref().map(|node| &**node);
      &node.elem
    })
  }
}

impl<T> List<T> {
  // passing `self`, instead of `&self` or `&mut self` shifts the ownership to Iterator
  pub fn into_iter(self) -> IntoIter<T> {
    IntoIter(self)
  }

  // passing head boxed node, so that we can loop through it's next property, till it becomes `None`
  pub fn iter(&self) -> Iter<T> {
    Iter {
      next: self.head.as_ref().map(|node| &**node),
    }
  }

  // TODO: Create a method that converts List to an Array, using it's iterators
}

/**
 * Note to self: In Tutorials, Drop trait is specifically implemented, to handle Stack overflow dues to
 * recursion function calls. So, even if the Drop trait is implemented properly for all our data types,
 * it should be noted, at some point, when List size grows, dropping each item in list, would pile up the
 * function stack and thus lead to stack overflow in the end. So we need to implement our own Drop trait
 */

impl<T> Drop for List<T> {
  fn drop(&mut self) {
    // Set everything to None, so that we don't get a function stack on drop, for recursive drops
    // instead this will be just one function call, dropping all Box<Node>, by triggering `drop` by replacing
    // Link to none, which will make previous link an invalid memory and should be removed.
    while let Some(mut node) = self.head.take() {
      self.head = node.next.take();
    }
  }
}

// All my tests goes here...
#[test]
fn consumeIterators() {
  let mut list = List::new();
  list.push(1);
  list.push(2);
  list.push(3);

  let mut iter = list.into_iter();
  assert_eq!(iter.next(), Some(3)); // Since it's a LIFO structure
  assert_eq!(iter.next(), Some(2)); // Since it's a LIFO structure
  assert_eq!(iter.next(), Some(1)); // Since it's a LIFO structure
  assert_eq!(iter.next(), None);
}

#[test]
fn immuteIteratorNonConsummable() {
  let mut list = List::new();
  list.push(1);
  list.push(2);
  list.push(3);

  let mut iter = list.iter();
  assert_eq!(iter.next(), Some(&3)); // Since it's a LIFO structure
  assert_eq!(iter.next(), Some(&2)); // Since it's a LIFO structure
  assert_eq!(iter.next(), Some(&1)); // Since it's a LIFO structure
  assert_eq!(iter.next(), None);
}
