# ga-rust-macro
> This is not working yet

A macro for rust that expands arbitrary geometric algebra expressions to an explicitly defined number/array.

Example expansion (within 3D VGA):
```
let v1 = eq!(3e0 + -2e12);
let v2 = eq!(2 - 6e1 + e12);
println!("{:?}", eq!(v1 + v2));
```
Becomes: 
```
let v1 = {[0.0, 3.0, 0.0, 0.0, 0.0, 0.0, -2.0, 0.0]};
let v2 = {[2.0, 0.0, -6.0, 0.0, 0.0, 0.0, 1.0, 0.0]};
println!("{:?}", {[v1[0] + v2[0], v1[1] + v2[1], v1[2] + v2[2], v1[3] + v2[3], v1[4] + v2[4], v1[5] + v2[5], v1[6] + v2[6], v1[7] + v2[7]]});
```

This library provides 5 macros: 
  - **eq:** expands expressions as seen above
  - **len:** takes no input, expands to the length of each array
  - **dims:** takes no input, expands to the number of dimensions used in the algebra 
  - **eq_peek:** (debug) expands expressions as seen above, but wrapped in quotes so the expansion can be veiwed
  - **get_tokens:** (debug) expands to the list of tokens taken in by the procedural macro
