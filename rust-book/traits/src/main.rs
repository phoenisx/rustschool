use traits::Human;
use traits::Shinobi;
use traits::HashTable;

fn main() {
    let x = Human {
        age: 50,
        name: String::from("Fudal Lord"),
    };
    x.age_category();

    // I don't know HOW/WHY, Shinobi Trait is required to be in scope,\
    // to access this method, I guess if methods are implemented with a trait
    // that trait is needed to be present in scope, else compilation fails...
    x.chakra_type();

    let _map = HashTable::new(String::from("subroto"), "biswas");
    _map.printMe();
}
