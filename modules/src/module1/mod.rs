// This module will be named as module1
// Can be used inside main.rs as -> `mod module1`

// re=exporting the nested_mod's modules, with shorter paths and different name;
pub use self::module1_person::person;

mod nested_mod;

pub mod module1_person {
  pub use super::nested_mod::output as person;
}
