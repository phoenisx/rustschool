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
    // Why the Type is `&'static str`, instead of `&str`, Check this https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision
    // And according to this following should be `&'a str`, with struct also having a lifetime of it's own.
    // For now as the app is just for example purpose, it's fine to use static lifetime...
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

/**
 * Breakdown #5:
 *  - Implementing a Method (function bound to an instance) for Struct
 *  - Type of self is Color due to this method being inside the impl Color context
 *  - Methods can take ownership of self, borrow self immutably (as done below),
 *      or borrow self mutably, just as they can any other parameter.
 *  - Also, impl blocks can have one or more methods in it defined
 */
impl Color {
    /// @param {u8} percent: represent amount to be lighten from color (0 - 100)
    fn red(&self) -> u8 {
        self.0
    }

    fn green(&self) -> u8 {
        self.1
    }

    fn blue(&self) -> u8 {
        self.2
    }
}

fn main() {
    loop {
        let mut choice = String::new();
        println!("Make a choice:");
        println!("\t1. Usual Struct Creation");
        println!("\t2. Struct Creation using (field init shorthand), similar to ES6 Object Property Shorthand");
        println!("\t3. Is inner member of struct immutable as well?");
        println!("\t4. From other user instance");
        println!("\t5. Instantiate Color Tuple");
        println!("\t6. Color Struct Method example");
        println!("\t7. Test Color Impls after method call statement is already done, with associated methods");

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
                println!("=== User1 created: {:#?}", user);
                println!("=== User2 created from User1: {:#?}\n", user2);
            },
            "5" => {
                let color = Color(122, 255, 150);
                println!("Struct Tuple example: {:?}", color);
                println!("Access a single value from tuple struct: {}\n", color.2);
            },
            "6" => {
                let color = Color(100, 100, 100).darken(20);
                println!("Darken Color using Instance Method: {:#?}", color);
                println!("Red: {:#?}", color.red());
                println!("Green: {:#?}", color.green());
                println!("Blue: {:#?}\n", color.blue());
            },
            "7" => println!("Lighten Color using Struct Associated Method: {:#?}\n", Color::lighten(Color(100, 100, 100), 20)),
            _ => break
        }
    }
}

/**
 * Breakdown #5:
 *  - This Breakdown is to check if method implementaion after the method is already called workd in Rust or not.
 *      -- And it works...
 *  - We can have separate impl blocks as well...
 *  - This Breakdown also shows Associated methods to Struct, or we can say, static methods...
 */
impl Color {
    /**
     * Breakdown #6:
     *  - One thing is clear from the following function. A Mutable Owner can be converted to an Immutable Owner while
     *      it's getting moved.
     */
    /// @param {u8} percent: represent amount to be lighten from color (0 - 100)
    fn darken(mut self, percent: u8) -> Color {
        self.0 -= percent;
        self.1 -= percent;
        self.2 -= percent;
        self
    }

    /// This Will Consume the passed color's ownership as well...
    fn lighten(color: Color, percent: u8) -> Color {
        Color(
            color.0 + percent,
            color.1 + percent,
            color.2 + percent
        )
    }
}
