# display_with

[![Crates.io](https://img.shields.io/crates/v/display_with)](https://crates.io/crates/display_with)
[![Downloads](https://img.shields.io/crates/d/display_with.svg)](https://crates.io/crates/display_with)
[![Documentation](https://docs.rs/display_with/badge.svg)](https://docs.rs/display_with)
[![License](https://img.shields.io/crates/l/display_with)](https://crates.io/crates/display_with)
[![Dependency Status](https://deps.rs/repo/github/JohnScience/display_with/status.svg)](https://deps.rs/repo/github/JohnScience/display_with)

Return opaque impl Display and/or impl Debug types via display_with and debug_with.

## Installation

```console
cargo add display_with
```

## Usage

```rust
use display_with::{display_with, debug_with};

let display = display_with(|f| write!(f, "Hello, world!"));
assert_eq!(format!("{display}"), "Hello, world!");

let debug = debug_with(|f| write!(f, "Hello, world!"));
assert_eq!(format!("{debug:?}"), "Hello, world!");
```

This can be combined with the `format_args!` macro to use the opaque types with the `write!` and `writeln!` macros.

```rust
use core::fmt::Write;
use display_with::{display_with, debug_with};
fn main() -> std::fmt::Result {
    let display = display_with(|f| write!(f, "Hello, world!"));
    let mut s = String::new();
    // Unlike `s.push_str(&format!("{display}"))`, this doesn't require an extra allocation.
    write!(&mut s, "{}", format_args!("{display}"))?;
    Ok(())
}
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
