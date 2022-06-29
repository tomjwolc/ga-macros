#[macro_use]
extern crate macros;

#[test]
pub fn tokens() {
    let v1 = eq!(3e0 + -2e12);
    let v2 = eq!(2 - 6e1 + e12);
    println!("{:?}", eq!(v1 + v2));

    println!("let v1 = {};", eq_peek!(3e0 + -2e12));
    println!("let v2 = {};", eq_peek!(2 - 6e1 + e12));
    println!("println!(\"{{:?}}\", {:?})", eq_peek!(v1 + v2));
}