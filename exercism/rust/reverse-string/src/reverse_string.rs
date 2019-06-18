#![allow(dead_code)]
extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

// This function will only work for ASCII chars
// and also will solve the problem in O(n)
// Not sure for now, if there is any better solution
// Can make the loop to run for n/2, but that is still O(n) complexity
pub fn reverse(original: &str) -> String {
  let size = original.len();
  let mut reversed = String::with_capacity(size);
  let original_bytes = original.as_bytes();

  for index in 1..=size {
    reversed.push(original_bytes[size - index] as char);
  }

  return reversed;
}

pub fn simple_reverse(original: &str) -> String {
  return original.chars().rev().collect::<String>();
}

pub fn grapheme_reverse(original: &str) -> String {
  let original_uchars = UnicodeSegmentation::graphemes(original, true).collect::<Vec<&str>>();
  let size = original_uchars.len();
  let mut reversed = String::with_capacity(size);

  for index in 1..=size {
    reversed.push_str(original_uchars[size - index]);
  }

  return reversed;
}
