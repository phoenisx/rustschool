// This part of code, show unsafe side of Rust.
// Which when writing a sane program, is never required.
// But for various data-structures, which are built from
// scratch (even the DS inside Rust's own STD lib) uses unsafe
// code for accessing the memory and managing it, without worrying
// about safety.

// Check Rust nomicon - https://doc.rust-lang.org/nomicon/
// which is all about unsafe side of Rust.

// Here I will be using Cell and PhantomData only
// for my use cases...

// `Cell` is used for making an immutable property, to be accessed as mutable
// `PhantomData` is a no-op Type, which is used only for marking a custom
// type with lifetimes. Even I am not quite sure right now, what PhantomData
// exactly is used for.

use std::cell::Cell;
use std::fmt;
use std::marker::PhantomData;

trait Item {}

#[derive(Debug)]
struct Book<'a> {
  name: &'a str,
}

impl<'a> Item for Book<'a> {}

#[derive(Debug)]
struct Weapon<'a> {
  weapon_type: &'a str,
}

impl<'a> Item for Weapon<'a> {}

struct Employee<'a, T: Item + fmt::Debug> {
  id: usize,
  name: Cell<&'a str>,
  owned_items: *const T,  // raw-pointer, which right now
  marker: PhantomData<T>, // This says, Employee owns `T`, and will destroy T when it gets destroyed
}

impl<'a, T> fmt::Debug for Employee<'a, T>
where
  T: Item + fmt::Debug,
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    unsafe {
      write!(
        f,
        "{}, {}, {:?}, {:?}",
        self.id,
        self.name.get(),
        self.owned_items,
        *self.owned_items // This requires unsafe block, as raw pointers can be null
                          // But right now I am sure of the data's existence, so it's fine here...
      )
    }
  }
}

fn main() {
  let b1 = Book {
    name: "Alice in Wonderland",
  };

  let x = Employee {
    id: 1,
    name: Cell::new("Subroto"),
    owned_items: &b1,
    marker: PhantomData,
  };

  // Cell can manipulate immutable data...
  x.name.set("Aditi");

  println!("Emp: {:?}", x);
}
