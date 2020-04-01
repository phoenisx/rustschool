// Derived or Custom Types in Rust are, Structs and Enums.
use std::cmp::Ordering;

#[derive(Debug)]
struct Simple {
    width: usize,
    height: usize,
    bit: char, // It's not a bit anymore thoug, it is of unicode size
}

// Details: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html#how-can-i-implement-partialord
impl PartialOrd for Simple {
    fn partial_cmp(&self, second: &Simple) -> Option<Ordering> {
        let area = self.width * self.height;
        let area2 = second.width * self.height;
        area.partial_cmp(&area2)
    }
}

impl PartialEq for Simple {
    fn eq(&self, second: &Self) -> bool {
        self.width == second.width && self.height == second.height
    }
}

// What's a Emtpy Struct, yeah this is unit Struct, not sure what that means
#[derive(Debug)]
struct Empty;

// A tuple struct
#[derive(Debug)]
struct Position(i32, i32);

fn main() {
    // Initializing of struct is simple, U don't have to use `new` keyword here.
    let simple = Simple {
        width: 24,
        height: 12,
        bit: '™'
    };
    let simple2 = Simple {
        width: 24,
        height: 12,
        bit: '™'
    };

    println!("Simple Struct: {:#?}", simple);
    println!("Without PartialEq I can't do Equality Comparisons: {}", simple == simple2);
    println!("Without PartialOrd or Ord, I can't do Comparisons: {}, {}", simple <= simple2, simple > simple2);

    // Empty Struct;
    let empty = Empty;
    println!("Empty Struct: {:?}", empty);

    // A tuple struct, it just helps to give tuples a name I guess.
    let position = Position(2, 3);
    let position_primitive = (2, 3); // This is same as above;
    println!("Struct Tuple: {:?}, x: {}, y: {}", position, position.0, position.1);
    println!("Primitive Tuple: {:?}, x: {}", position_primitive, position_primitive.0);
}
