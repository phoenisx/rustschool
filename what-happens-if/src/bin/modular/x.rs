/// This is a module x, which can access
/// x folder as it's own module.

mod y;

pub use y::Y;

#[derive(Debug)]
pub struct X {
    pub data: Y,
}
