#![allow(unused)]
#![allow(dead_code)]

struct Person {
    first: String,
    last: String,
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first && self.last == other.last
    }
}

impl Eq for Person {}

fn main() {
    let mut b1 = Person {
        first: String::from("Harry"),
        last: String::from("Potter")
    };

    let mut b2 = Person {
        first: String::from("Hermione"),
        last: String::from("Granger")
    };

    let mut b3 = Person {
        first: String::from("Harry"),
        last: String::from("Potter")
    };

    println!("b1 == b2: {:?}", b1 == b2);
    println!("b1 == b3: {:?}", b1 == b3);
}
