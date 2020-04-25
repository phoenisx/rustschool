#![allow(dead_code, unused_variables)]
// Credits https://stackoverflow.com/a/41056727/2849127

// Default Rust drops the structs and inner properties in order they were defined.

/*
/**
 * Uncomment to test Auto Drop sequence
 */
struct PrintDrop(&'static str);

impl Drop for PrintDrop {
    fn drop(&mut self) {
        println!("Dropping:: {}", self.0);
    }
}

struct AutoDrop {
    foo: PrintDrop,
    bar: PrintDrop,
}

fn main() {
    let auto_drop = AutoDrop {
        foo: PrintDrop("foo"),
        bar: PrintDrop("bar"),
    };
}
*/

use std::mem::ManuallyDrop; // Helps to allow dropping instances manually, but is unsafe.

struct PrintDrop(&'static str);

impl Drop for PrintDrop {
    fn drop(&mut self) {
        println!("Dropping:: {}", self.0);
    }
}

struct ManualDrop {
    foo: ManuallyDrop<String>,
    foobar: PrintDrop,
    bar: ManuallyDrop<String>,
    bat: PrintDrop,
}

impl Drop for ManualDrop {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.bar);
            ManuallyDrop::drop(&mut self.foo);
            println!("Dropping:: bar and then foo");
        }
    }
}

fn main() {
    let manual_drop = ManualDrop {
        foo: ManuallyDrop::new("foo".into()),
        bar: ManuallyDrop::new("bar".into()),
        foobar: PrintDrop("foobar"),
        bat: PrintDrop("bat"),
    };
}
