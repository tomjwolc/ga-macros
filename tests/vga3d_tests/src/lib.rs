#[macro_use]
use macros::*;

#[test]
pub fn simple_numbers() {
    assert_eq!([3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], eq!(3));
    assert_eq!([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], eq!(0));
    assert_eq!([-3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], eq!(-3));
    assert_eq!([-3.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], eq!(-3.1));
}

#[test]
pub fn implicit_multiplication() {
    assert_eq!([0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0], eq!(3e1));
    assert_eq!([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.0], eq!(3e012));
    assert_eq!([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], eq!(0e0));
    assert_eq!([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], eq!(0e012));
    assert_eq!([0.0, 0.0, -3.0, 0.0, 0.0, 0.0, 0.0, 0.0], eq!(-3e1));
    assert_eq!([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -3.0], eq!(-3e012));
}

#[test]
pub fn linear_combinations() {
    assert_eq!([1.0, -2.1, 3.0, -4.0, 0.0, 0.0, -3.0, 0.0], eq!(3e1 + -4e2 - 1 - 3e12 + -2.1e0 + 2));
}

pub struct A { a: [f64; len!()] }

#[test]
pub fn weird_inputs() {
    let b: Vec<[f64; len!()]> = vec![eq!(e2)];
    let c = (eq!(e01), eq!(12));

    assert_eq!([0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0], eq!(a.a + b[0] + c.0 + c.1));
}
