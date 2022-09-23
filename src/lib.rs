#![allow(
	clippy::tabs_in_doc_comments,
	invalid_doc_attributes,
	unused_attributes
)]
#![warn(rust_2018_idioms)]
#![forbid(unsafe_code)]

// TODO use xmlwriter crate if it ever creates a new release
#[path = "xmlwriter/src/lib.rs"]
mod xmlwriter;

mod graphic;
mod tag;
pub mod tags;
mod value;

pub use graphic::Graphic;
pub use value::Value;
