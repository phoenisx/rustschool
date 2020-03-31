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
}
