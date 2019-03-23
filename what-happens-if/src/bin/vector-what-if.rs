#![allow(dead_code)]

fn change_scope(vector: &mut Vec<i32>) {
  vector.push(45);
}

fn change_scope_generic<T>(vector: &mut Vec<T>, data: T) {
  vector.push(data);
}

fn main() {
  let mut v = Vec::new();
  let _x = 23;
  v.push(24); // In Same scope, if pushing data to vector is done later somewhere in the code, then datatype is not required to be specified.
  println!("Vector: {:?}", v);

  let mut v2 = Vec::new();
  change_scope(&mut v2); // It's clear that type is still not required as function defines the type, even though scope changes for the variable...
  println!("Vector 2: {:?}", v2);

  // Breakdown #1:
  // It works even with generics. Conclusion, Vector Type is not required to be specified while creation, untill
  // and unless no data is ever been pushed in the same scope. For Eg., If I might have created Vector in Main function,
  // and somehow asynchronous it's data is pushed somewhere else, that defining datatype would be necessary...
  let mut v3 = Vec::new();
  change_scope_generic(&mut v3, 12);
  println!("Vector 2: {:?}", v3);
}
