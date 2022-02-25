#[macro_use]
extern crate lazy_static;
extern crate regex;

pub mod error;
mod metadata;
mod pattern;
#[cfg(test)]
mod test;

pub use metadata::extension::FileExtension;
pub use metadata::extension::SubtitleExtension;
pub use metadata::extension::VideoExtension;
pub use metadata::Metadata;
