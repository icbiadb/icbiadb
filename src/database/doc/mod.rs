//! A document database implementation
//!
//! # Example
//!
//! ```
//! // You can have rust code between fences inside the comments
//! // If you pass --test to `rustdoc`, it will even test it for you!
//! use doc::Person;
//! let person = Person::new("name");
//! ```

pub mod parser;
pub mod types;

#[derive(Default)]
pub struct DocDb {}
