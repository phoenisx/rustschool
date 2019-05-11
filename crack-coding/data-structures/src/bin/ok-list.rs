mod ok_list;

fn main () {
  let mut list = ok_list::List::new();

  list.push(12);
  list.push(23);
  list.push(42);

  println!("{:?}", list);
  println!("{:?}", list.pop());
  println!("{:?}", list);
}
