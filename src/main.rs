#[macro_use]
mod father;

#[macro_use]
fn main() {
    crate::father::child::A::test();
}