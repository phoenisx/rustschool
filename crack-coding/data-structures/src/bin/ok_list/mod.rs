// Ok so this is a complete copy of what I have learnt from
// https://rust-unofficial.github.io/too-many-lists/second.html

// To be noted: Since any of the structs or enums don't have pub scope specifier
// they all will be treated as private members...

#[derive(Debug)]
pub struct List {
  head: Link,
}

// Better way to shorten the re-used types, using aliases...
// Think of Link as a special pointer, which stores the node location
// and also owns it.
type Link = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
  elem: i32,
  next: Link,
}

impl List {
  pub fn new() -> List {
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
  pub fn push(&mut self, elem: i32) {
    self.head = Some(Box::new(Node {
      elem,
      next: self.head.take(),
    }));
  }

  // this is lifo list, so latest head should be popped
  pub fn pop(&mut self) -> Option<i32> {
    self.head.take().map(|node| {
      self.head = node.next;
      node.elem
    })
  }
}

/**
 * Note to self: In Tutorials, Drop trait is specifically implemented, to handle Stack overflow dues to
 * recursion function calls. So, even if the Drop trait is implemented properly for all our data types,
 * it should be noted, at some point, when List size grows, dropping each item in list, would pile up the
 * function stack and thus lead to stack overflow in the end. So we need to implement our own Drop trait
 */

impl Drop for List {
  fn drop(&mut self) {
    // Set everything to None, so that we don't get a function stack on drop, for recursive drops
    // instead this will be just one function call, dropping all Box<Node>, by triggering `drop` by replacing
    // Link to none, which will make previous link an invalid memory and should be removed.
    while let Some(mut node) = self.head.take() {
      self.head = node.next.take();
    }
  }
}
