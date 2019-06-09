mod module1;

use module1::person::Person;

fn main() {
    let p1 = Person::new("Subroto", "Biswas");

    println!("Person: {}", p1.full_name());
}
