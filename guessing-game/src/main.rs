// https://github.com/adeschamps/lsm303/commit/15be707404f9caebde2abcb1af3c3154493e0a43
#![allow(unused_doc_comments)]
/**
 * Breakdown #1:
 *  - Above statement is called Rust Attributes, Details [here](https://doc.rust-lang.org/reference/attributes.html)
 *  - Single/Multi Line Comment[s].
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
use std::cmp::Ordering;

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

    /**
     * Breakdown #5:
     *  - Variables are created using `let` keyword.
     *  - Following is an example for Immutable Variable (i.e., cannot be changed)...
     *      -- So, secret_number = 12; // Updating immutable variable will throw compile error.
     *  - Type::Method(), i.e. rand::thread_rng() (for example), is a static method of type `rand`
     *    (called `associated function` of Type `rand`)...
     */
    let secret_number = rand::thread_rng().gen_range(1, 101);

    /**
     * Breakdown #6:
     *  - Following is an exxample of Looping in Rust, here it's an infinite loop block.
     *  - To come out of a loop block, use `break` keyword...
     *  - To skip and continue to next iteration in the loop, use `continue` keyword...
     */
    loop {
        /**
         * Breakdown #7:
         *  - Rust has primitive data types, that get stored in stack...
         *  - While the following example creates a Complex Data type, that gets stored in Heap
         *      -- Example a string - String::from("Example String"), which will also return an immutable instance.
         */
        let mut guess = String::new(); // Mutable Data (can be changed)...

        /**
         * Breakdown #8:
         *  - Take input from the user.
         *  - Store the input Read Stream to Mutable insatance of String created above.
         *      -- (&mut guess), passes a mutable reference of guess to read_line function.
         *  - Also Check for exceptions, if it does, exit from the running program.
         */
        io::stdin().read_line(&mut guess)
            .expect("User Input Save Failed...");

        /**
         * Breakdown #9:
         *  - Type Conversion.
         *  - Variable Shadowing, which a unique Rust feature, where the same variable name can be used again,
         *    in the same scope to update the variable, with same or different data type and value.
         *  - Error Handling.
         *  - Variable assignment using complex expressions. Following example demostrates assigning value to variable
         *    with a `switch/case` (`match` for Rust) condition.
         */
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number, // No Terminator used thus the expression here will be returned, i.e., number will return as value here
            Err(_) => {
                // `_` is a special param to accept all cases, i.e., becomes a default case here.
                // Or a Catch Expression in terms of try/catch block...
                println!("String toNumber parse Error");
                continue; // COntinue to skip the loop...
            }
        };

        /**
         * Breakdown #10:
         *  - Using Ordering Trait. Even I don't know what trait exactly means right now, will explain it in next chapter...
         *  - Match Conditional Statements, (Switch/Case)
         *  - `println!` macro with dynammic arguments for formatting strings.
         */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Guess Too Low"),
            Ordering::Greater => println!("Guess Too High"),
            Ordering::Equal => {
                println!("You are Awesome Dude!!!, {} is Correct Guess...", guess);
                break;
            }
        };
    }
}
