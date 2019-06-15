use std::fmt::Debug;

/**
 * Breakdown #1:
 *  - `&[T]` is a slice reference. Thus can't modify the data
 *  - Also since `T` is generic, there is no way to `print` param value.
 *    * Naah! There is a way to print param value, using trait bounds
 *  - `&[T]`, how does it work with `Vector` can be found in Rust Book Ch-15,
 *    -- Read Deref traits for details...
 */
fn check<T>(data: &[T])
where
  T: Debug,
{
  let mut i = 0;
  for item in data.iter() {
    println!("Element[{}]: {:?}", i, item);
    i += 1;
  }
}

// This Code does not Panic anymore...
fn main() {
  let mut v = vec![1, 2, 3, 4];
  let mut s = String::from("Test FOO");
  check(&v);
  // check(&(s.into_bytes()));
  println!("Vector: {:?}", v);
  println!("String: {}", s);
}
