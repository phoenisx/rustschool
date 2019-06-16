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
