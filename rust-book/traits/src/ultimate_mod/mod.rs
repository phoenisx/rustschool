// https://stackoverflow.com/questions/26435102/in-rust-what-is-the-purpose-of-a-mod-rs-file
// https://doc.rust-lang.org/rust-by-example/mod/split.html
pub mod prelude;

pub trait Shinobi {
  fn chakra_type(&self);
}
