mod ok_list;

fn main() {
  let mut list = ok_list::List::new();
  let mut string_list = ok_list::List::new();

  list.push(12);
  list.push(23);
  list.push(42);

  string_list.push("Subroto");
  string_list.push("And");
  string_list.push("Aditi");

  println!("{:?}", list);
  println!("{:?}", list.pop());
  println!("{:?}", list);

  println!("{:?}", string_list);
  println!("{:?}", string_list.peek());
}
