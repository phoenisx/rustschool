// Merging Chapter 5 and 6 together, as they are common in other laguages as well

/**
 * Breakdown #1
 *  - Struct Definition
 *  - `&'static str` is used for defining string literal data type.
 *      TODO: find out what &' is used for...
 *  - use attribute #[derive(Debug)] for User to be printable.
 *      -- https://stackoverflow.com/questions/30253422/how-to-print-structs-and-arrays
 *      -- But this might be not the best way. TODO: Will update when I will know the details...
 */
#[derive(Clone, Debug)] // https://doc.rust-lang.org/rust-by-example/trait/clone.html
struct User {
    name: String,
    email: &'static str, // https://www.reddit.com/r/rust/comments/2o8r94/why_is_a_string_literal_not_always_of_type_str/
    age: u8,
    active: bool
}

fn create_user() -> User {
    // Create a new user and return it. No `new` keyword required...
    User {
        name: String::from("foo"),
        email: "bar",
        age: 24,
        active: true
    }
}

fn create_user_with_shorthand(name: String, email: &'static str, age: u8) -> User {
    User {
        name, email, age,
        active: false
    }
}

/**
 * Breakdown #2
 *  - Immutability concept for Structs work similar to any other variable assignment in Rust.
 *  - If a variable is marked as `mut`, then all it's member variables are also mutable...
 */
fn test_struct_members_immutability() {
    let mut user = create_user_with_shorthand(String::from("Dumb"), "foo", 12);
    user.name = String::from("Dumber");
    println!("=== USER Modified: {:?}\n", user);
}

/**
 * Breakdown #3:
 *  - `user`.[property_name] is acceptable to access `user` instance members. This can also be used
 *      to create other user instance from existing user.
 *  - using struct update syntax (similar to JS Object Spread, but with varius catches to keep in mind).
 *      -- This example is using Object Spread.
 *      -- Catch 1: Overrides only those member, which are not explicitly modified/created.
 *  - Check this `rustc --explain E0507`, references doesn't work for update syntax
 *  - Normal pass by value works, thus to maintain the ownership, we need to send a clone of the struct instance instead.
 */
fn create_from(user: User) -> User {
    User {
        email: "foo@bar.com",
        ..user
    }
}

/**
 * Breakdown #4:
 *  - Defining a Tuple Struct.
 *  - This type of struct can be useful for similar list of data, that means something.
 *      -- Like Color(u8, u8, u8);
 */
#[derive(Debug)]
struct Color(u8, u8, u8); // (Red, Green, Blue)

fn main() {
    let mut choice = String::new();
    loop {
        println!("Make a choice:");
        println!("\t1. Usual Struct Creation");
        println!("\t2. Struct Creation using (field init shorthand), similar to ES6 Object Property Shorthand");
        println!("\t3. Is inner member of struct immutable as well?");
        println!("\t4. From other user instance");
        println!("\t5. Instantiate Color Tuple");

        std::io::stdin().read_line(&mut choice)
            .expect("Input Save failed...");
        match choice.trim() {
            "1" => {
                let user = create_user();
                println!("=== USER created: {:?}\n", user);
            },
            "2" => {
                let user = create_user_with_shorthand(String::from("Subroto"), "test@mail.com", 24);
                println!("=== USER created: {:?}\n", user);
            },
            "3" => test_struct_members_immutability(),
            "4" => {
                let user = create_user_with_shorthand(String::from("Subroto"), "test@mail.com", 24);
                // Needed to use Clone trait, as simply passing would move the ownership
                // While passing a reference throws error for `cannot move out of borrowed content`.
                let user2 = create_from(user.clone());
                println!("=== User1 created: {:?}", user);
                println!("=== User2 created from User1: {:?}\n", user2);
            },
            "5" => {
                let color = Color(122, 255, 150);
                println!("Struct Tuple example: {:?}", color);
                println!("Access a single value from tuple struct: {}\n", color.2);
            },
            _ => break
        }
    }
}
