#![warn(rust_2018_idioms)]
#![forbid(elided_lifetimes_in_paths, unsafe_code)]

mod graphic;
mod tag;
pub mod tags;
mod value;

pub use graphic::Graphic;
pub use value::Value;
