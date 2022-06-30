extern crate macros;
use macros::*;

#[test]
pub fn token_tests() {
    println!("tokens: {}", get_tokens!(5*"a.0" + "bc" * "d[ef]"));
}