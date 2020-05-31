mod x;

use x::{X, Y};

/// This example demonstrates how we can modularize our code
/// using same names for module and module folder to access
/// each other.
fn main() {
    let x = X {
        data: Y::Works
    };

    println!(":: {:#?} ::", x);
}
