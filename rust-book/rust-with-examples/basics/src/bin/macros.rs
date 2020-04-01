// Macro is simple...
macro_rules! i_rock {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello Aditi");
    };
}

// Macro with args
macro_rules! i_rock_always {
    ($my_var: ident) => {
        // The macro will expand into the contents of this block.
        println!("{}", $my_var);
    };
}

fn main() {
    // This call will expand into `println!("Hello");`
    i_rock!();

    let something = "be something";
    i_rock_always!(something);
    println!("Since Macro does not take ownership: {}", something);
}
