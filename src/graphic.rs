use crate::{
	tag::Tag as _,
	tags::Svg,
	xmlwriter::{Indent, Options as XmlOptions, XmlWriter}
};
use std::ops::{Deref, DerefMut};

/// An SVG graphic.
pub struct Graphic(Svg);

impl Default for Graphic {
	fn default() -> Self {
		Self::new()
	}
}

impl Graphic {
	pub fn new() -> Self {
		let mut svg = Svg::new();
		svg.set_attr("xmlns".into(), "http://www.w3.org/2000/svg");
		Self(svg)
	}

	fn write(&self, opts: XmlOptions, pretty: bool) -> String {
		let mut writer = XmlWriter::new(opts);
		self.0.write_to(&mut writer, pretty);
		writer.end_document()
	}

	#[allow(clippy::inherent_to_string)]
	pub fn to_string(&self) -> String {
		let opts = XmlOptions {
			use_single_quote: true,
			indent: Indent::None,
			attributes_indent: Indent::None
		};
		self.write(opts, false)
	}

	pub fn to_string_pretty(&self) -> String {
		let opts = XmlOptions {
			use_single_quote: true,
			indent: Indent::Spaces(2),
			attributes_indent: Indent::None
		};
		self.write(opts, true)
	}
}

impl Deref for Graphic {
	type Target = Svg;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for Graphic {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::tags::{TagWithPresentationAttributes, Text};

	#[test]
	fn empty_svg() {
		let svg = Graphic::new();
		let expected = "<svg xmlns='http://www.w3.org/2000/svg'/>";
		assert_eq!(svg.to_string(), expected);
	}

	#[test]
	fn hello_world() {
		let mut svg = Graphic::new();
		svg.set_view_box("0 0 200 50");
		svg.push(
			Text::new()
				.with_x(100)
				.with_y(25)
				.with_text_anchor("middle")
				.with_dominant_baseline("middle")
				.append("Hello World!")
		);
		let expected = concat!(
			"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 200 50'>",
			"<text x='100' y='25' text-anchor='middle' dominant-baseline='middle'>",
			"Hello World!",
			"</text>",
			"</svg>"
		);
		assert_eq!(svg.to_string(), expected);
	}
}
