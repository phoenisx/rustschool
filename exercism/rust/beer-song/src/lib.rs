pub fn map_to_verse(n: i32) -> (String, String, String, String) {
  match n {
    0 => (
      String::from("No more bottles"),
      String::from("no more bottles"),
      String::from("Go to the store and buy some more"),
      String::from("99 bottles"),
    ),
    1 => (
      String::from("1 bottle"),
      String::from("1 bottle"),
      String::from("Take it down and pass it around"),
      String::from("no more bottles"),
    ),
    _ => (
      format!("{} bottles", n),
      format!("{} bottles", n),
      String::from("Take one down and pass it around"),
      format!("{} bottle{}", n - 1, if n - 1 == 1 { "" } else { "s" }),
    ),
  }
}

pub fn verse(n: i32) -> String {
  let nth_statements = map_to_verse(n);
  format!(
    "{} of beer on the wall, {} of beer.\n{}, {} of beer on the wall.\n",
    nth_statements.0, nth_statements.1, nth_statements.2, nth_statements.3
  )
}

pub fn sing(start: i32, end: i32) -> String {
  let mut counter = start;
  let mut output = String::from("");
  while counter >= end {
    output.push_str(&verse(counter));
    if counter != end {
      output.push_str("\n");
    }
    counter -= 1;
  }
  return output;
}
