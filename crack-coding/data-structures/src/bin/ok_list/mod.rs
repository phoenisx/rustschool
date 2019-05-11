// Ok so this is a complete copy of what I have learnt from
// https://rust-unofficial.github.io/too-many-lists/second.html


// To be noted: Since any of the structs or enums don't have pub scope specifier
// they all will be treated as private members...
use std::mem;

#[derive(Debug)]
pub struct List {
  head: Link
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
  pub fn new () -> List {
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
  pub fn push (&mut self, elem: i32) {
    let node = Box::new(Node {
      elem,
      next: mem::replace(&mut self.head, None)
    });

    self.head = Some(node);
  }

  // this is lifo list, so latest head should be popped
  pub fn pop (&mut self) -> Option<i32> {
    if let Some(boxed_node) = mem::replace(&mut self.head, None) {
      self.head = boxed_node.next;
      Some(boxed_node.elem)
    } else {
      None
    }
  }
}

