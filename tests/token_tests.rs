extern crate macros;
use macros::*;

#[test]
pub fn token_tests() {
    println!("tokens: {}", eq_peek!(e1));
}