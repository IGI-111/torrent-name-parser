#[macro_use]
extern crate lazy_static;
extern crate regex;

pub mod error;
mod metadata;
mod pattern;
#[cfg(test)]
mod test;

pub use metadata::Metadata;
