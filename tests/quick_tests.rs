extern crate ga_macros;
use ga_macros::*;

#[test]
// pub fn tester() {
//     println!("{:?}", eq!(norm(e1))); // tests labels in algebra.json
// }

// #[test]
// pub fn expo_test() {
//     println!("{:?}", eq!(2^0.5))
// }

// #[test]
// pub fn const_test() {
//     println!("{:?}", eq!(e));
// }

#[test]
pub fn my_tests() {
    println!("{:?}", eq_peek!(tensor[i] * tensor[end]));
}