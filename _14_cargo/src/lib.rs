//! # Chapter 14  _14_cargo
//!
//! `_14_cargo` is a collection of utilities to make performing
//! certain calculations more convenient.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = _14_cargo::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
pub mod art;
