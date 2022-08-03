# ga-macros
A macro for rust that expands arbitrary geometric algebra expressions to an explicitly defined number/array.

## Goals
  - Compute as much information as possible at compile time
  - Make writing geometric algebra expressions look better in code

## Provided Macros

  - **eq:** expands expressions as seen above
  - **t:** returns the type that eq! expands to: [f64; len!()]
  - **len:** takes no input, expands to the length of each array
  - **dims:** takes no input, expands to the number of dimensions used in the algebra 
  - **eq_peek:** (debug) expands expressions as seen above, but wrapped in quotes so the expansion can be veiwed
  - **get_tokens:** (debug) expands to the list of tokens taken in by the procedural macro
  
## Example Expansions
3D VGA: 
```
let a = eq!(3e0 + -2e12);     // -> let a = {[0.0, 3.0, 0.0, 0.0, 0.0, 0.0, -2.0, 0.0]};
let b = eq!(2 - 6e1 + e12);   // -> let b = {[2.0, 0.0, -6.0, 0.0, 0.0, 0.0, 1.0, 0.0]};
println!("{:?}", eq!(a + b)); // -> println!("{:?}", {[a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3], a[4] + b[4], a[5] + b[5], a[6] + b[6], a[7] + b[7]]});
```
```
let f = |a: f64| eq!(#a + #a * e1);           // -> let f = |a: f64| {[(a as f64), 0.0, (a as f64), 0.0, 0.0, 0.0, 0.0, 0.0]}
let arr = vec![eq!(5e1 + 2e01 + 4e12)];       // -> let arr = vec![{[0.0, 0.0, 5.0, 0.0, 2.0, 0.0, 4.0, 0.0]}]
println!("{:?}", eq!(f(3.0) @ 1 + arr[0] @ 2)); // -> println!("{:?}", {[0.0, f(3.0)[1], f(3.0)[2], f(3.0)[3], arr[0][4], arr[0][5], arr[0][6], 0.0]})
```

## Eq! Operations (in order of execution last to first)
- **'+'** => Addition
- **'-'** => Subtraction
- **'*'** => Geometric product
- **'|'** => Inner product
- **'&'** => Outer product/Meet
- **'%'** => Join
- **'>'** => Sandwich product
- **'/'** => Division (divisor must be a real number)
- **'@'** => Grade select (selector must a real number s.t. 0 <= n <= dims!())
- **'~'** => Reverse
- **'!'** => Dual

## Inputting variables
All inputed terms are assumed to be (or return) Vec<f64> or [f64; len!()] unless prefixed with: #

Example (3D VGA):
```
let x: usize = 5;
let y = eq!(e1);

assert_eq!([0.0, 0.0, 5.0, 0.0, 0.0, 0.0, 0.0, 0.0], eq!(#x * y))
```
### Restricted terms
The following are used as functions for eq! and can't be used inside eq! or eq_peek!
- motor( or motor[
- norm( or norm[
- norm_b( or norm_b[
- norm_w( or norm_w[
- bulk( or bulk[
- weight( or weight[
