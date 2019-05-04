use std::fmt::Debug;

// https://leetcode.com/explore/learn/card/linked-list/
// Also, will be using the guide https://rust-unofficial.github.io/too-many-lists/

// Following is the representation for the Data Structure
// for Singly Linked List

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

impl<T: Debug> Node<T> {
  fn new(data: T) -> Node<T> {
    Node {
      data,
      next: None
    }
  }

  // Following is required for Making `Node` struct return an
  // iterator on demand
  fn iter(&self) -> Iter<T> {
    Iter {
      // `as_ref is required to convert `Option's` data into a reference,
      // i.e., `Box will turn into `&Box`
      // So, we need to dereference &Box twice nad then return &node of that.
      // `node` here is a reference to `Box`, thus &**node is &Node
      next: self.next.as_ref().map( |node| &**node )
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

fn add<T>(head: &mut Node<T>, node: Node<T>) {
  head.next = Some(Box::new(node));
}

fn main () {
  let mut head = Node::new(24);
  add(&mut head, Node::new(12));
  add(&mut head, Node::new(14));

  println!("::Inside Loop::");
  print!("[");
  for data in head.iter() {
    print!("{:?}, ", data);
  }
  println!("]");

  println!("List: {:?}", head);
}
