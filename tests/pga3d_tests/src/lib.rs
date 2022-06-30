use macros::*;

#[test]
pub fn simple_numbers() {
    assert_eq!([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], eq!(0));
    assert_eq!([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0], eq!(e0123));
}

#[test]
pub fn simple_expressions() {
    assert_eq!([0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 8.0, 0.0, 0.0, 0.0], eq!(3e1 + 4e3 * 2e01));
}

#[test]
pub fn linear_combinations() {
    assert_eq!([0.0, 1.0, 1.0, -1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], eq!(e0 + e1 - e2 + e3));
    assert_eq!([-3.0, -1.0, 3.0, 0.0, -3.0, 2.3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0], eq!(3.0e1 + -3e3 + 2.3e01 - 2 + -1 - e0 + e0123));
}

#[test]
pub fn tokens() {
    println!("tokens: {}", get_tokens!(5*"a.0" + "bc" * "d[ef]"));
}