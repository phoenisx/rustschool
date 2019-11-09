/// Smart Pointers. There are many, but here we will be using just:
/// 1. Box<T> - Cannot have more than once reference to a data.
/// 2. Rc<T> - Used mainly for keeping multiple immutable references
/// 3. RefCell<T> - Like `Box<T>`, but can be used for mutability with immutable data
///
/// DO READ - https://doc.rust-lang.org/book/ch15-06-reference-cycles.html

use std::rc::Rc;

struct Tv {
    name: String
}

struct Man {
    tv: Rc<Tv>
}

struct Box {
    ball: Box<String>
}

fn main() {
    let x = Tv {
        name: String::from("tv1"),
    };

    let counter = Rc::new(x);

    let man1 = Man {
        // Rc::clone is used for increasing Rc counter
        tv: Rc::clone(&counter)
    };

    let man2 = Man {
        tv: Rc::clone(&counter)
    };

    println!("M1: {:?}, M2: {:?}", man1.tv.name, man2.tv.name)
}
