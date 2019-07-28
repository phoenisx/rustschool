// https://github.com/adeschamps/lsm303/commit/15be707404f9caebde2abcb1af3c3154493e0a43
#![allow(unused_doc_comments)]

use std::io;

/**
 * Why Ownership:
 *
 * Low Level Programming interacts with Hardware Memory Directly.
 * Dyanamic Memory Management (Allocation/Deallocation) on Heap are difficult
 * to manage, when a program's codebase grows.
 * Rust Provides Ownership Rules to overcome these possible runtime memory leaks...
 *
 * Ownership Rules:
 *  - Each value in Rust has a variable that’s called its owner.
 *  - There can only be one owner at a time.
 *  - When the owner goes out of scope, the value will be dropped.
 *
 * Possible Memory issues that a user faces while development, could be:
 *  - Memory Leak: when a Memory is allocated, but developer forgets to de-allocate it.
 *  - Double Free Corruption: when there are two pointers, pointing to the same memory and tries
 *      to de-allocate the same memory twice.
 *  - Dangling Pointer (References): when a pointer still points to a memory, that either is given
 *      to someone else, or is freed.
 *
 * Ownership works only for Data Types that dynamically allocates memory from Heap...
 */

/// Breakdown #1:
///     - `///` is used in Rust to provide documentations, instead of `/** */`...
///     - "bar" is a String Literal, and stored in memory stack,
///         while `String::from("foo")`returns a String Object, which is stored in heap memory
///         thus followes all Ownership rules...
///     - Variables in Heap can be modified with more/less memory, with same pointer, if they are mutable
///         like -> msg does...
fn simple_ownership_example() {
    // If String Literal was mutable, re-assigning a new value would have provided `msg_literal` a totally new
    // stack memory to point to...
    println!("\nThis Example Demostrates Ownership which helps in removing Memory Leaks here,");
    println!("by invalidating variable when out of scope called as Resource Acquisition Is Initialization (RAII)");
    let msg_literal = "bar";
    let mut msg = String::from(msg_literal); // without `mut`, msg won't be able to modify the memory...
    println!("`msg` value: {}", msg);
    msg.push_str(" | foo");
    println!("`msg` modified value (Ownership still retained): {}", msg);
    /**
     * Breakdown #2:
     *  - Ownership moved to `msg2` from `msg`, thus after re-assigning statement variable `msg` is invalid to use
     *  - There can only be one owner at a time to a memory.
     *  - Though there can be more than one pointer to the same memory, as `msg3` is a pointer to `msg2`.
     *  - Cloning an object to other variable, to maintain ownership and creating a new memory with same data for other variable
     *      example `msg4`
     */
    let msg2 = msg; // Moves ownership to `msg2`, which helps in overcoming issues related to Double Free Corruption as well
    let msg3 = &msg2;
    let msg4 = msg2.clone();
    println!("`msg2` new variable with moved ownership from `msg` to `msg2`, `msg2`: '{}', `msg1`: is invalidated now ", msg2);
    println!("`msg3` is a pointer to `msg2`, thus no change in ownership, `msg2`: '{}', `msg3`: '{}' ", msg2, msg3);
    println!("`msg4` is a clone to `msg2`, thus no change in ownership for msg2, `msg2`: '{}', `msg4`: '{}' ", msg2, msg4);
} // Once this function completes executing, all scoped variables inside it will lose there ownership and will be `drop`ed from memory

fn takes_ownership (msg: String) {
    println!("whatever string is passed to this function will lose there ownership in parent scope: {}", msg);
}

/// This Function takes any argument immutable/mutable, and converts it to a new
/// variable that is mutable, and has it's own ownership...
fn return_ownership(mut msg: String) -> String {
    msg.push_str(" #modified");
    msg
}

fn return_mutiple_values(line: String) -> (String, usize) {
    /**
     * Breakdown #3:
     *  - We can return a tuple to provide metadata, which helps in getting the ownership back as well...
     *  - `(line, line.len())`, cannot be used directly, as line is returned, which moves the ownership and invalidates,
     *      before even using `line.len()`. Thus, do all operations before returning the variable the operations are supposed
     *      to run on...
     */
    let size = line.len();
    (line, size)
}

/**
 * Breakdown #4:
 *  - If you want to retain the ownership of the instance, in parent scope,
 *      create functions that accepts references as params, instead of actual Type.
 *  - Following Example is for immutable reference, which cannot modify the actual String data in it.
 */
fn pass_reference_to_retain_ownership (line: &String) -> usize {
    // Here `line` is a reference variable (pointer) for type String and not a String a variable.
    line.len()
}

/**
 * Breakdown #5:
 *  - To be able to modify the actual data, params should be a mutable reference to the Type.
 *  - We can directly call a method from different namespace, without importing it first, via
 *      prepending the namespace befor method call...
 */
fn mutable_pass_by_reference (line: &mut String) -> String {
    /// * std::str::replace, std is the default namespace...
    /// * `line` is not modified, while replace returns a new String instance with the replaced data...
    str::replace(line, " ", "::")
    // OR
    // line.replace(" ", "::");
}

fn modify_mutable_reference (line: &mut String) -> String {
    line.push_str(", added sugar");
    line.to_string()
}

/**
 * Breakdown #6:
 *  - Immutable references can be made as many times as we want.
 *  - Mutable references can only be created once in present scope...
 *      Why? To prevent Data Races (Race Condition), as Mutable Reference can write to memory as well
 *      compared to Immutable Reference...
 * Details - https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references
 */
fn one_or_many () {
    let line1 = String::from("foo");
    let mut line2 = String::from("bar");

    let line_imm_1 = &line1; // Immutable Reference
    let line_imm_2 = &line1;
    let line_mut_1 = &mut line2; // Mutable Reference...
    // let line_mut_2 = &mut line2; // This will fail, as only one mutable reference can be present in the scope...
    println!("imm-1: {}, imm-2: {}, mut-1: {}, mut-2: Cannot be done on same scope", line_imm_1, line_imm_2, line_mut_1);

    // But if Mutable References are passed to a function, they can be used multiple times, as the scope of usage changes
    // when function is passed by Mutable References...
    let modified = modify_mutable_reference(&mut line2);
    println!("\tOriginal Line: {},\n\tModified return for Mutable Reference Param: {}", line2, modified);
}

fn dangling_reference () -> String {
    let line = String::from("foo loves bar");

    /**
     * Breakdown #7
     *  - Returning a reference here will fail, as ownership of line invalidates once function finishes
     *      deallocating the memory. Thus, returned reference would have pointed to a non-existent memory (corrupted).
     */
    // &line
    line // To solve this, Rust allows moving the ownership to parent scope... Return the whole String Instance and move Ownership...
}

fn main() {
    loop {
        println!("\n\nChose an Option:");
        println!("1. Simple Ownership Example");
        println!("2. Lose Ownership on Function Call");
        println!("3. Return Ownership after Function Call");
        println!("4. Return Multiple values after Function Call");
        println!("5. Pass by reference (Immutable)");
        println!("6. Pass by reference (Mutable)");
        println!("7. Immutable Reference vs Mutable Reference");
        println!("8. Dangling Reference");
        // Slicing in Ownership Chapter can be looked into https://doc.rust-lang.org/book/ch04-03-slices.html
        // as the details that presents is based on above knowledge of Ownership...

        let mut option = String::new();
        io::stdin().read_line(&mut option)
            .expect("Error saving stdin");

        match option.trim() {
            "1" => simple_ownership_example(),
            "2" => {
                let msg = String::from("Owner here");
                takes_ownership(msg); // Loses Ownership;
                // `msg` Cannot be used from now on...
            },
            "3" => {
                let msg = String::from("Owner here");
                let msg = return_ownership(msg); // New Ownership;
                println!("With new ownership: {}", msg);
            },
            "4" => {
                // Destructuring is possible in Rust, as follows:
                let (line, length) = return_mutiple_values(String::from("Hello World!!!"));
                println!("Real String: {}, It's size: {}", line, length);
            },
            "5" => {
                let owner = String::from("Hello World!!");
                let mut length = pass_reference_to_retain_ownership(&owner);
                println!("Actual String (Ownership still retained after func call): {}, Returned String Size: {}", owner, length);
                length = pass_reference_to_retain_ownership(&owner);
                println!("Actual String can be passed as immutable reference as many times as we want: {}, Length: {}", owner, length);
            },
            "6" => {
                let mut owner = String::from("Hello World!!");
                let mut modified = mutable_pass_by_reference(&mut owner);
                println!("Ownership still retained after first func call): {}, Modified Return String: {}", owner, modified);
                modified = modify_mutable_reference(&mut owner);
                println!("Mutable reference as function params can be passed as many times as we want");
                println!("Problem is Original Data also get's modified!!");
                println!("\tOriginal: {},\n\tModified: {}", owner, modified);
            },
            "7" => one_or_many(),
            "8" => {
                let line = dangling_reference();
                println!("Returned String: {}", line);
            },
            _ => break // Exit
        }
    }
}
