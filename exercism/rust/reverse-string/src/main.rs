use reverse_string::*;

fn main() {
  let name = reverse("uüu");
  let name2 = reverse("子猫");
  println!("{}", name);
  println!("{}", name2);
}
