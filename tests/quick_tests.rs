extern crate ga_macros;
use ga_macros::*;

// #[test]
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
    let z = |_: &str| [0.0, 5.0];


    println!("{:?}", eq!("complex": 2*i));
    println!("{:?}", eq!("complex": 1 + z("abcd")));
    println!("{:?}", eq_peek!("complex": abcd(efgh(38743 + g(284))[2]+-=&4ij[2nf[3 + e(7)]])));
    println!("{:?}", eq_peek!("complex": abcd(efgh+-=&4ij)(k)[lmn](o)[p][q]));
}