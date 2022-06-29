#[macro_use]
extern crate macros;

#[test]
pub fn tokens() {
    println!("norm:   {:?}, \nnorm_w: {:?}, \nnorm_b: {:?}", eq!(norm(ONES @ 1)), eq!(norm_w(ONES @ 1)), eq!(norm_b(ONES @ 1)))
}