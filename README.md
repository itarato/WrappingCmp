Wrapping Comparable Numeric Types
---------------------------------

Wrapping add/sub on it's own does not know how many times it wrapped and in what direction.
This lib provides type overrides for native numeric types.


Example
-------

```rust
let left = WrappedNum::new(200u8);
let mut right = WrappedNum::new(200u8);

// They both equal at this point.
assert!(left == right);

right.wrapping_add(100u8);

// So natively right is smaller.
assert_eq!(44, right.get());
assert!(left.get() > right.get());

// But wrapping-aware it's greater.
assert!(left < right);
```
