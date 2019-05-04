use std::fmt;
use std::mem;

// https://leetcode.com/explore/learn/card/linked-list/
// Also, will be using the guide https://rust-unofficial.github.io/too-many-lists/

// Following is the representation for the Data Structure
// for Singly Linked List
#[derive(Debug)]
struct List<T> {
  head: Option<Box<Node<T>>> // Should own the Node for head, may or may not have the Node on creation
}

// Currently this data-type can store only a single type of consistent data for one instance of list
#[derive(Debug)]
struct Node<T> {
  data: T,
  next: Option<Box<Node<T>>>
}

// Iterating in Rust can be done immutably/mutable, thus it is
// a good convention to provide a separate immutable/mutable object to iter over
// as when iterating, data can get consumed and ownership is lost, thus data gets invalidated
// Also, It is important to provide a returning Iter object with same lifetime as to
// LinkedList node, because compiler is not smart enough to figure it out.
struct Iter<'a, T> {
  // Will return a reference, instead of boxed reference, as we don't want the actual owner to
  // lose ownership.
  next: Option<&'a Node<T>>
}

impl<T: fmt::Debug> List<T> {
  fn new() -> List<T> {
    List {
      head: None
    }
  }

  // Works on the principle of LIFO, thus, insertion is an O(1) task
  // as we just create a new node, add old head to this new node.next,
  // and reassign self.head to new node...
  fn push(&mut self, data: T) {
    let node = Node {
      data,
      next: mem::replace(&mut self.head, None)
    };

    self.head = Some(Box::new(node));
  }

  // Following is required for Making `Node` struct return an
  // iterator on demand
  fn iter(&self) -> Iter<T> {
    Iter {
      // `as_ref is required to convert `Option's` data into a reference,
      // i.e., `Box will turn into `&Box`
      // So, we need to dereference &Box twice nad then return &node of that.
      // `node` here is a reference to `Box`, thus &**node is &Node
      next: self.head.as_ref().map( |node| &**node )
    }
  }
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    self.next.map(|node| {
      // store the next Node reference, so that, we
      self.next = node.next.as_ref().map( |node| &**node ); // same thing as inter()
      &node.data // return a reference to data for loop, as we don't want to change ownership
    })
  }
}

fn main () {
  let mut list = List::new();
  list.push(12);
  list.push(24);

  println!("::Inside Loop::");
  let mut iterator = list.iter();
  print!("[{:?}", iterator.next().unwrap());
  for data in iterator {
    print!(", {:?}", data);
  }
  println!("]");

  println!("List: {:?}", list);
}
