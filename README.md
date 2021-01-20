# Gray code


## Info

Author: Masutani, Bansho

Email: ban-m@g.ecc.u-tokyo.ac.jp


## Content

This is a tiny library implementing Gray code. For example,

```rust
use graycode::GrayCode;
let mut gc = GrayCode::new(3).into_iter();
assert_eq!(gc.next(), Some(0b000));
assert_eq!(gc.next(), Some(0b001));
assert_eq!(gc.next(), Some(0b011));
assert_eq!(gc.next(), Some(0b010));
assert_eq!(gc.next(), Some(0b110));
assert_eq!(gc.next(), Some(0b111));
assert_eq!(gc.next(), Some(0b101));
```

`GrayCodeFlip` provides a method to obtain the location of the flipped bit in each iteration. 

```rust
let mut gc = GrayCodeFlip::new(3).into_iter();
assert_eq!(gc.next(), Some(0)); // This behavior might change in later versions.
assert_eq!(gc.next(), Some(0));
assert_eq!(gc.next(), Some(1));
assert_eq!(gc.next(), Some(0));
assert_eq!(gc.next(), Some(2));
assert_eq!(gc.next(), Some(0));
assert_eq!(gc.next(), Some(1));
assert_eq!(gc.next(), Some(0));
assert_eq!(gc.next(), None);
```
