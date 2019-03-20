/**
 * Breakdown #1:
 */
// This is Single Line Comment
/*
    This is a Multi-line comment
    To Compile/Run this program - use `cargo run`
*/

/**
 * Breakdown #2:
 *  - This is how we import/require different modules/libraries (or in Rust terms Crates).
 */
use std::io;

/**
 * Breakdown #3:
 *  - `rand` is a crate in Rust, which needs to be separately installed
 *  - Installing a crate requires modification of `Cargo.toml` file, with desired:
 *      - [CrateName] = "VERSION NUMBER"
 *      - Then running `cargo build` in terminal.
 */
use rand::Rng;

fn main() {
    /**
     * Breakdown #4:
     *  - println!() / print!() are `macro`s, not a function (functions in Rust are without `!`).
     *  - String Literals are passed using double quotes, like -> "String"
     *  - Statements in Rust should always end with terminator, which is `;` for Rust, else it will be
     *    considered as an `expression` in Rust.
     */
    print!(".......... ");
    print!("Guess a Number");
    println!(" ..........");
}
