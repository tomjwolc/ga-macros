extern crate macros;
use macros::*;

#[test]
pub fn tester() {
    let a = 5;
    let b = |x: usize| 2 * x;
    println!("{:?}", eq!(#a * (e1 + e12) + #b(4)));
}