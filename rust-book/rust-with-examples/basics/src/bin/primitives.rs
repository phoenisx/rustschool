/**
 * Any Global function defined, always has static lifetime, unless
 * defined separately.
 */

/// Tuples can be of same type inside
fn same_type_tuples() -> (u32, u32, u32) {
    return (1u32, 32u32, 43u32);
}

/// Tuples can have different types as well
fn diff_type_tuples() -> (u32, f64, &'static str) {
    (1u32, 32.334f64, "Shub")
}

fn main() {
    // Tuples are primitive types in rust, still they require
    // debug type printer.
    println!("Same Typed Tuples: {:#?}", same_type_tuples());

    println!("Same Typed Tuples: {:?}", diff_type_tuples());

    // Yes `()` this is a type in Rust, to denote `void`.
    println!("Empty/Void Data: {:?}", ());

    // Variable Shadowing, meaning same named variable can have two different types
    // where the last found variable in the Block, should be used.
    let i_am_number_and_boolean = 12;
    println!("What am I: {}", i_am_number_and_boolean);

    {
        let i_am_number_and_boolean = true;
        println!("What am I now: {}", i_am_number_and_boolean);
    }
    let i_am_number_and_boolean = 12.445; // f64
    println!("What am I, at last: {}", i_am_number_and_boolean);
}
