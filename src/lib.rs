#![no_std]

//! A simple crate that provides a way to create instances of opaque types that implement
//! [`fmt::Display`] and [`fmt::Debug`] by calling a provided formatting closure.
//!
//! # Examples
//!
//! ```
//! use display_with::{display_with, debug_with};
//!
//! let display = display_with(|f| write!(f, "Hello, world!"));
//! assert_eq!(format!("{display}"), "Hello, world!");
//!
//! let debug = debug_with(|f| write!(f, "Hello, world!"));
//! assert_eq!(format!("{debug:?}"), "Hello, world!");
//! ```
//!
//! This can be combined with the `format_args!` macro to use the opaque types with the `write!` and
//! `writeln!` macros.
//!
//! ```
//! use core::fmt::Write;
//! use display_with::{display_with, debug_with};
//!
//! fn main() -> std::fmt::Result {
//!     let display = display_with(|f| write!(f, "Hello, world!"));
//!     let mut s = String::new();
//!     // Unlike `s.push_str(&format!("{display}"))`, this doesn't require an extra allocation.
//!     write!(&mut s, "{}", format_args!("{display}"))?;
//!     Ok(())
//! }
//! ```
//!
//!
//! Credit: <https://internals.rust-lang.org/t/format-args-with-long-lifetimes/19494/2>.

use core::fmt;

/// Creates an instance of an opaque type that implements [`fmt::Display`] by calling the provided
/// formatting closure.
///
/// # Examples
///
/// ```
/// use display_with::display_with;
///
/// let display = display_with(|f| write!(f, "Hello, world!"));
/// assert_eq!(format!("{display}"), "Hello, world!");
/// ```
pub fn display_with(f: impl Fn(&mut fmt::Formatter) -> fmt::Result) -> impl fmt::Display {
    struct DisplayWith<F>(F);

    impl<F> fmt::Display for DisplayWith<F>
    where
        F: Fn(&mut fmt::Formatter) -> fmt::Result,
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.0(f)
        }
    }

    DisplayWith(f)
}

/// Creates an instance of an opaque type that implements [`fmt::Debug`] by calling the provided
/// formatting closure.
///
/// # Examples
///
/// ```
/// use display_with::debug_with;
///
/// let debug = debug_with(|f| write!(f, "Hello, world!"));
/// assert_eq!(format!("{debug:?}"), "Hello, world!");
/// ```
pub fn debug_with(f: impl Fn(&mut fmt::Formatter) -> fmt::Result) -> impl fmt::Debug {
    struct DebugWith<F>(F);

    impl<F> fmt::Debug for DebugWith<F>
    where
        F: Fn(&mut fmt::Formatter) -> fmt::Result,
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.0(f)
        }
    }

    DebugWith(f)
}
