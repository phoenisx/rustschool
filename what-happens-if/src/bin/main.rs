// Multiple binary packages can be directly built if they are present in `src/bin` (Cargo's default binary location)
// Checkout the following for details:
//      https://stackoverflow.com/questions/36604010/how-can-i-build-multiple-binaries-with-cargo
//      https://doc.rust-lang.org/cargo/reference/manifest.html#configuring-a-target
// Also, `cargo run --bin {FILE_NAME}` is used to run specific binary...
#![allow(dead_code)]
/**
 * I might miss many details for module examples. If you find one add in here. Thanks
 *
 * Breakdown #1:
 *  - How to create Inline Modules.
 *  - Nesting Modules and accessing them using relative or absolute paths...
 *  - Any Function, DataType etc define in a module is by default private. `pub` keyword is used to make
 *      it available to outside world.
 *  - Child Modules are also required to be public scoped if it is to be used by some other module,
 *      that is not a sibling or is outside of parent module. Check Rust book for better explainations.
 */
mod test_inline_mod {
    fn _mult_by_2_normal(val: isize) -> isize {
        val * 2
    }

    pub mod scope_1 {
        pub enum MultiplyType {
            SHIFT,
            MULTIPLY,
        }

        // From this function it can be seen that why a function should be declared Public or Private.
        // Keep Functions Private to hide logic that might not be required by the outside world. Like here
        // anybody will care about multiplying a number by 2, how it's done (implementation logic) is something
        // not so important to others, if you expose your APIs. Thus, keep those stuffs private, so that those logics cannot be messed
        // by someone else...
        pub fn mult_by_2(val: isize, mul_type: MultiplyType) -> isize {
            match mul_type {
                MultiplyType::SHIFT => _mult_by_2_shift(val),
                MultiplyType::MULTIPLY => super::_mult_by_2_normal(val),
            }
        }

        fn _mult_by_2_shift(val: isize) -> isize {
            val << 1
        }

    }

    pub mod scope_2 {
        pub fn mult_by_4(val: isize) -> isize {
            crate::test_inline_mod::scope_1::mult_by_2(
                // Absolute Path for accessing a module function
                super::_mult_by_2_normal(val),
                super::scope_1::MultiplyType::SHIFT, // Relative Path for accessing a module function
            )
        }
    }
}

/**
 * Breakdown #2:
 *  - Bring Functions, Enums etc to executable scope, to access them directly, instead of typing long strings again and again..
 */
use self::test_inline_mod::scope_1::MultiplyType;
use self::test_inline_mod::scope_2; // Or import the whole module, and access inner implementations with short paths.

fn main() {
    println!(
        "1. Example Access Module: mult_by_2(34) -> {}",
        self::test_inline_mod::scope_1::mult_by_2(34, MultiplyType::SHIFT)
    );
    println!(
        "2. Example Access Module: mult_by_4(12) -> {}",
        scope_2::mult_by_4(12)
    );
}
