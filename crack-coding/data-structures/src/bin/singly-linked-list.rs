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

fn main () {
  let tail = Node {
    data: 12,
    next: None
  };

  let head = Node {
    data: 24,
    next: Some(Box::new(tail)) // tail gets invalid, after changing the ownership...
  };

  println!("List: {:?}", head);
}
