fn main() {
    // Declare a variable binding
    let out_scope;

    {
        // Initialize the binding
        out_scope = 4;

        // Shadow the outer scoped variable.
        let out_scope = 12;
        println!("Inner Scoped, shadowed variable: {}", out_scope);
    }

    println!("Outer Scope, initialized in separate inner block: {}", out_scope);
}
