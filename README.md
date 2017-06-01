[![Build Status](https://travis-ci.org/sdleffler/empty-box-rs.svg?branch=master)](https://travis-ci.org/sdleffler/empty-box-rs)
[![Docs Status](https://docs.rs/empty-box/badge.svg)](https://docs.rs/empty-box)
[![On crates.io](https://img.shields.io/crates/v/empty-box.svg)](https://crates.io/crates/empty-box)

# `EmptyBox`, a way to safely move values in and out of `Box`s without reallocations

`EmptyBox` is similar to a statically checked `Box<Option<T>>`:

```rust
use empty_box::EmptyBox;

// A box with a string!
let boxed = Box::new("Hello!".to_string());

// Oh no, we don't like that string.
let (string, empty) = EmptyBox::take(boxed);

// Let's make an objectively superior string, and put it into the original
// box.
let superior = "Objectively superior string!".to_string();

// Now we have our superior string in the box!
let boxed = empty.put(superior); 

assert_eq!("Hello!", string);
assert_eq!("Objectively superior string!", &*boxed);
```

Creating an `EmptyBox` from a `Box` and then putting a `T` back into the
`EmptyBox` will avoid allocating a new `Box`, instead reusing whatever old
`Box` the `T` was `EmptyBox::take`n from.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
