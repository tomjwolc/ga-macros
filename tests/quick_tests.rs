extern crate ga_macros;
use ga_macros::*;

#[test]
pub fn tester() {
    println!("{:?}", eq!(norm(e1))); // tests labels in algebra.json
}

#[test]
pub fn expo_test() {
    println!("{:?}", eq!(2^0.5))
}