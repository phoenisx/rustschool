#![allow(unreachable_code)]
#![allow(dead_code)]

use std::fmt::Display;

trait IsOkay {
  fn is_okay(&self) -> bool;
}

impl IsOkay for u32 {
  fn is_okay(&self) -> bool {
    *self > 10
  }
}

impl IsOkay for f32 {
  fn is_okay(&self) -> bool {
    *self > 10.0
  }
}

impl IsOkay for char {
  fn is_okay(&self) -> bool {
    *self as u8 > 65 // I guess this is how type conversion works in Rust...
  }
}

// Figured out it is better and more promising to use &str, &[T] or &T for
// function arguments, for Complex data types, unless and untill you want to be too specific and add or remove data.
// Read the following for details. Read Deref as well Ch-15 in Rust book.
// https://stackoverflow.com/questions/40006219/why-is-it-discouraged-to-accept-a-reference-to-a-string-string-vec-vec-o
//
// Read this https://doc.rust-lang.org/book/ch10-02-traits.html, for understanding traits
fn print_generic_on_compare_v1<T: PartialOrd + Copy + Display + IsOkay>(vector: &[T]) {
  // vector.push(data); // Can't be used, Param should be specific `&mut Vec<T>` to use Vec Methods...
  for &item in vector.iter() {
    if item.is_okay() {
      println!("Will work since <T> has Display Trait: {}", item);
    } // But can be used to read data...
  }
}

// This works because, i32, f32, f64 and char, they all implement From<u8>.
fn print_generic_on_compare_v2<T: PartialOrd + Copy + Display + From<u8>>(vector: &[T]) {
  // Simple and elegant way...
  for &item in vector.iter() {
    if item > T::from(10) {
      println!("Will work since <T> has Display Trait: {}", item);
    }
  }
}

// Without Copy Trait...
fn print_generic_on_compare_v3<T: PartialOrd + Display + From<u8>>(vector: &[T]) -> &T {
  let mut random_return_val = &vector[0];
  // Simple and elegant way...
  for item in vector.iter() {
    if item > &T::from(10) {
      println!("Will work since <T> has Display Trait: {}", item);
      random_return_val = &item;
    }
  }

  random_return_val
}

fn main() {
  print_generic_on_compare_v1(&vec![1, 2, 20]);
  print_generic_on_compare_v1(&vec![11.2, 2.1, 10.2]);
  print_generic_on_compare_v1(&vec!['a', 'z', 'A', 'Z']);

  print_generic_on_compare_v2(&vec![1, 2, 20]);
  print_generic_on_compare_v2(&vec![11.2, 2.1, 10.2]);
  print_generic_on_compare_v2(&vec!['a', 'z', 'A', 'Z']);

  print_generic_on_compare_v3(&vec![1, 2, 20]);
  print_generic_on_compare_v3(&vec![11.2, 2.1, 10.2]);
  print_generic_on_compare_v3(&vec!['a', 'z', 'A', 'Z']);
}
