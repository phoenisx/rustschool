mod s_list;
use s_list::LinkedList;

fn main() {
  let list: LinkedList<usize> = LinkedList::new();
  println!("List: {:?}", list);
}
