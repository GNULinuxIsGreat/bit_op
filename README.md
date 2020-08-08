
[![License](https://img.shields.io/badge/license-MIT-0fff0f.svg)](https://opensource.org/licenses/MIT)
[![License](https://img.shields.io/badge/license-APACHE-0fff0f.svg)](https://www.apache.org/licenses/LICENSE-2.0)
![](https://tokei.rs/b1/github/GNULinuxIsGreat/bit_op)
# Bit manipulation for rust
## Examples

### Set bit that are selected
```rust
use bit_op::BitOp;
use bit_op::bit_u8::*;
let mut x = 0b00001111u8;
x.set(B7);
assert_eq!(x, 0b10001111);
let mut y = 0u8;
y.set(B7 | B0);
assert_eq!(y, 0b10000001);
```
### Reset bit that are selected
```rust
use bit_op::BitOp;
use bit_op::bit_u8::*;
let mut x = 0b00001111u8;
x.reset(B0 | B1 | B2 | B3);
assert_eq!(x, 0);
let mut y = 0b11111111u8;
y.reset(B7 | B0);
assert_eq!(y, 0b01111110);
```
### Toggle bit that are selected
```rust
use bit_op::BitOp;
use bit_op::bit_u8::*;
let mut x = 0b00001111u8;
x.toggle(B5 | B4 | B3 | B2);
assert_eq!(x, 0b00110011);
x.toggle(B5 | B4 | B3 | B2);
assert_eq!(x, 0b00001111);
```

### Get the bits that are desired
```rust
use bit_op::BitOp;
use bit_op::bit_u8::*;
let mut x = 0b10000001u8;
assert_eq!(x.get(B7), 0b10000000);
assert_eq!(x.get(B6), 0b00000000);
assert_eq!(x.get(B0), 0b00000001);
```
## License
Licensed under either of Apache License, Version 2.0 or MIT license at your option.
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
