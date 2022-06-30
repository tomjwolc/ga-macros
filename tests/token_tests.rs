extern crate macros;
use macros::*;

#[test]
pub fn token_tests() {
    println!("tokens: {:?}", eq_peek!(a.a + b[0] + c.0 + c.1));
}