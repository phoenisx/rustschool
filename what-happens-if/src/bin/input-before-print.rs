use std::io;
// Write trait needs to be imported to use flush and other Write specific methods...
use std::io::Write;

fn main() {
  let mut data = String::new();

  // eprint!("\nIntensity: "); // Either print to stderror, which flushes all the content everytime. OR
  // Use print!(), but it's required to flush the buffer to stdout, as it's cpu intensive...
  // Check this - https://github.com/rust-lang/rust/issues/23818
  // And this - https://stackoverflow.com/questions/15042849/what-does-flushing-the-buffer-mean & https://www.geeksforgeeks.org/buffer-flush-means-c/
  // for details
  print!("\nTest: ");
  io::stdout().flush().expect("I don't care...");

  io::stdin().read_line(&mut data).expect("Never Mind!!");

  print!("Test 2: {}, New Input: ", data.trim()); // `data` contains the newline char as well...
  io::stdout().flush().expect("I don't care...");

  io::stdin()
    .read_line(&mut data)
    .expect("Never Mind, Just an Error :P!!");
}
