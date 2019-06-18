use reverse_string::*;

fn main() {
  let name = reverse("uüu");
  let name2 = reverse("子猫");
  let name3 = reverse("Jürgen ǅemal");
  println!("{}", name);
  println!("{}", name2);
  println!("{}", name3);
}
