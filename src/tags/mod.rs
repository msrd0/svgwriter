// @generated

use crate::{tag::Tag, value::Value};
use indexmap::IndexMap;
use std::fmt::{self, Debug, Display, Formatter, Write as _};
use xmlwriter::XmlWriter;

mod common_attrs;
pub use common_attrs::prelude::*;

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum AAttrs {
	Class,
	ExternalResourcesRequired,
	Style,
	Target,
	Transform,
	XlinkActuate,
	XlinkHref,
	XlinkShow,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	GlobalEventAttributes(common_attrs::GlobalEventAttributes),
	GraphicalEventAttributes(common_attrs::GraphicalEventAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
	XLinkAttributes(common_attrs::XLinkAttributes),
}

impl From<common_attrs::ConditionalProcessingAttributes> for AAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for AAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::GlobalEventAttributes> for AAttrs {
	fn from(attr: common_attrs::GlobalEventAttributes) -> Self {
		Self::GlobalEventAttributes(attr)
	}
}

impl From<common_attrs::GraphicalEventAttributes> for AAttrs {
	fn from(attr: common_attrs::GraphicalEventAttributes) -> Self {
		Self::GraphicalEventAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for AAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl From<common_attrs::XLinkAttributes> for AAttrs {
	fn from(attr: common_attrs::XLinkAttributes) -> Self {
		Self::XLinkAttributes(attr)
	}
}

impl AAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Style => "style",
			Self::Target => "target",
			Self::Transform => "transform",
			Self::XlinkActuate => "xlink:actuate",
			Self::XlinkHref => "xlink:href",
			Self::XlinkShow => "xlink:show",
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::GlobalEventAttributes(attr) => attr.as_str(),
			Self::GraphicalEventAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
			Self::XLinkAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for AAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for AAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod a_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::A {}
	impl Content for super::AltGlyphDef {}
	impl Content for super::Animate {}
	impl Content for super::AnimateColor {}
	impl Content for super::AnimateMotion {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Circle {}
	impl Content for super::ClipPath {}
	impl Content for super::ColorProfile {}
	impl Content for super::Cursor {}
	impl Content for super::Defs {}
	impl Content for super::Desc {}
	impl Content for super::Discard {}
	impl Content for super::Ellipse {}
	impl Content for super::Filter {}
	impl Content for super::Font {}
	impl Content for super::FontFace {}
	impl Content for super::ForeignObject {}
	impl Content for super::Group {}
	impl Content for super::Image {}
	impl Content for super::Line {}
	impl Content for super::LinearGradient {}
	impl Content for super::Marker {}
	impl Content for super::Mask {}
	impl Content for super::Metadata {}
	impl Content for super::Mpath {}
	impl Content for super::Path {}
	impl Content for super::Pattern {}
	impl Content for super::Polygon {}
	impl Content for super::Polyline {}
	impl Content for super::RadialGradient {}
	impl Content for super::Rect {}
	impl Content for super::Script {}
	impl Content for super::Set {}
	impl Content for super::Stop {}
	impl Content for super::Style {}
	impl Content for super::Svg {}
	impl Content for super::Switch {}
	impl Content for super::Symbol {}
	impl Content for super::Text {}
	impl Content for super::Title {}
	impl Content for super::Use {}
	impl Content for super::View {}
}

#[doc = "The [`<a>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("a.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<a>`](A)\n",
	"- [`<altGlyphDef>`](AltGlyphDef)\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateColor>`](AnimateColor)\n",
	"- [`<animateMotion>`](AnimateMotion)\n",
	"- [`<animateTransform>`](AnimateTransform)\n",
	"- [`<circle>`](Circle)\n",
	"- [`<clipPath>`](ClipPath)\n",
	"- [`<color-profile>`](ColorProfile)\n",
	"- [`<cursor>`](Cursor)\n",
	"- [`<defs>`](Defs)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<discard>`](Discard)\n",
	"- [`<ellipse>`](Ellipse)\n",
	"- [`<filter>`](Filter)\n",
	"- [`<font>`](Font)\n",
	"- [`<font-face>`](FontFace)\n",
	"- [`<foreignObject>`](ForeignObject)\n",
	"- [`<g>`](Group)\n",
	"- [`<image>`](Image)\n",
	"- [`<line>`](Line)\n",
	"- [`<linearGradient>`](LinearGradient)\n",
	"- [`<marker>`](Marker)\n",
	"- [`<mask>`](Mask)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<mpath>`](Mpath)\n",
	"- [`<path>`](Path)\n",
	"- [`<pattern>`](Pattern)\n",
	"- [`<polygon>`](Polygon)\n",
	"- [`<polyline>`](Polyline)\n",
	"- [`<radialGradient>`](RadialGradient)\n",
	"- [`<rect>`](Rect)\n",
	"- [`<script>`](Script)\n",
	"- [`<set>`](Set)\n",
	"- [`<stop>`](Stop)\n",
	"- [`<style>`](Style)\n",
	"- [`<svg>`](Svg)\n",
	"- [`<switch>`](Switch)\n",
	"- [`<symbol>`](Symbol)\n",
	"- [`<text>`](Text)\n",
	"- [`<title>`](Title)\n",
	"- [`<use>`](Use)\n",
	"- [`<view>`](View)\n"
)]
#[doc = "\n\n [`<a>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/a"]
#[derive(Debug)]
pub struct A {
	attrs: IndexMap<AAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn a_private::Content>>,
}

impl Default for A {
	fn default() -> Self {
		Self::new()
	}
}

impl A {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: a_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: a_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: AAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: AAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(AAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(AAttrs::ExternalResourcesRequired)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(AAttrs::Style)
	}

	/// Set the `target` attribute.
	pub fn with_target<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AAttrs::Target, value);
		self
	}

	/// Set the `target` attribute.
	pub fn set_target<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AAttrs::Target, value);
	}

	/// Get the `target` attribute.
	pub fn target(&self) -> Option<&dyn Value> {
		self.get_attr(AAttrs::Target)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AAttrs::Transform, value);
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AAttrs::Transform, value);
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&dyn Value> {
		self.get_attr(AAttrs::Transform)
	}

	/// Set the `xlink:actuate` attribute.
	pub fn with_xlink_actuate<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AAttrs::XlinkActuate, value);
		self
	}

	/// Set the `xlink:actuate` attribute.
	pub fn set_xlink_actuate<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AAttrs::XlinkActuate, value);
	}

	/// Get the `xlink:actuate` attribute.
	pub fn xlink_actuate(&self) -> Option<&dyn Value> {
		self.get_attr(AAttrs::XlinkActuate)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AAttrs::XlinkHref, value);
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AAttrs::XlinkHref, value);
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&dyn Value> {
		self.get_attr(AAttrs::XlinkHref)
	}

	/// Set the `xlink:show` attribute.
	pub fn with_xlink_show<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AAttrs::XlinkShow, value);
		self
	}

	/// Set the `xlink:show` attribute.
	pub fn set_xlink_show<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AAttrs::XlinkShow, value);
	}

	/// Get the `xlink:show` attribute.
	pub fn xlink_show(&self) -> Option<&dyn Value> {
		self.get_attr(AAttrs::XlinkShow)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for A {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(AAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for A {}

impl common_attrs::CoreAttributesSetter for A {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(AAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for A {}

impl common_attrs::GlobalEventAttributesSetter for A {
	fn set_attr<V>(&mut self, attr: common_attrs::GlobalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GlobalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(AAttrs::from(attr))
	}
}

impl TagWithGlobalEventAttributes for A {}

impl common_attrs::GraphicalEventAttributesSetter for A {
	fn set_attr<V>(&mut self, attr: common_attrs::GraphicalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(AAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for A {}

impl common_attrs::PresentationAttributesSetter for A {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(AAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for A {}

impl common_attrs::XLinkAttributesSetter for A {
	fn set_attr<V>(&mut self, attr: common_attrs::XLinkAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&dyn Value> {
		self.get_attr(AAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for A {}

impl Tag for A {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("a");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum AltGlyphAttrs {
	Class,
	Dx,
	Dy,
	ExternalResourcesRequired,
	Format,
	GlyphRef,
	Rotate,
	Style,
	X,
	XlinkHref,
	Y,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	GlobalEventAttributes(common_attrs::GlobalEventAttributes),
	GraphicalEventAttributes(common_attrs::GraphicalEventAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
	XLinkAttributes(common_attrs::XLinkAttributes),
}

impl From<common_attrs::ConditionalProcessingAttributes> for AltGlyphAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for AltGlyphAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::GlobalEventAttributes> for AltGlyphAttrs {
	fn from(attr: common_attrs::GlobalEventAttributes) -> Self {
		Self::GlobalEventAttributes(attr)
	}
}

impl From<common_attrs::GraphicalEventAttributes> for AltGlyphAttrs {
	fn from(attr: common_attrs::GraphicalEventAttributes) -> Self {
		Self::GraphicalEventAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for AltGlyphAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl From<common_attrs::XLinkAttributes> for AltGlyphAttrs {
	fn from(attr: common_attrs::XLinkAttributes) -> Self {
		Self::XLinkAttributes(attr)
	}
}

impl AltGlyphAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::Dx => "dx",
			Self::Dy => "dy",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Format => "format",
			Self::GlyphRef => "glyphRef",
			Self::Rotate => "rotate",
			Self::Style => "style",
			Self::X => "x",
			Self::XlinkHref => "xlink:href",
			Self::Y => "y",
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::GlobalEventAttributes(attr) => attr.as_str(),
			Self::GraphicalEventAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
			Self::XLinkAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for AltGlyphAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for AltGlyphAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[doc = "The [`<altGlyph>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("altGlyph.md")]
#[doc = "\n\n [`<altGlyph>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/altGlyph"]
#[derive(Debug)]
pub struct AltGlyph {
	attrs: IndexMap<AltGlyphAttrs, Box<dyn Value>>,
	content: String,
}

impl Default for AltGlyph {
	fn default() -> Self {
		Self::new()
	}
}

impl AltGlyph {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: String::new()
		}
	}

	pub fn push<T: Display>(&mut self, content: T) {
		write!(self.content, "{content}").unwrap();
	}

	pub fn append<T: Display>(mut self, content: T) -> Self {
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: AltGlyphAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: AltGlyphAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(AltGlyphAttrs::Class)
	}

	/// Set the `dx` attribute.
	pub fn with_dx<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphAttrs::Dx, value);
		self
	}

	/// Set the `dx` attribute.
	pub fn set_dx<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphAttrs::Dx, value);
	}

	/// Get the `dx` attribute.
	pub fn dx(&self) -> Option<&dyn Value> {
		self.get_attr(AltGlyphAttrs::Dx)
	}

	/// Set the `dy` attribute.
	pub fn with_dy<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphAttrs::Dy, value);
		self
	}

	/// Set the `dy` attribute.
	pub fn set_dy<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphAttrs::Dy, value);
	}

	/// Get the `dy` attribute.
	pub fn dy(&self) -> Option<&dyn Value> {
		self.get_attr(AltGlyphAttrs::Dy)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(AltGlyphAttrs::ExternalResourcesRequired)
	}

	/// Set the `format` attribute.
	pub fn with_format<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphAttrs::Format, value);
		self
	}

	/// Set the `format` attribute.
	pub fn set_format<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphAttrs::Format, value);
	}

	/// Get the `format` attribute.
	pub fn format(&self) -> Option<&dyn Value> {
		self.get_attr(AltGlyphAttrs::Format)
	}

	/// Set the `glyphRef` attribute.
	pub fn with_glyph_ref<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphAttrs::GlyphRef, value);
		self
	}

	/// Set the `glyphRef` attribute.
	pub fn set_glyph_ref<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphAttrs::GlyphRef, value);
	}

	/// Get the `glyphRef` attribute.
	pub fn glyph_ref(&self) -> Option<&dyn Value> {
		self.get_attr(AltGlyphAttrs::GlyphRef)
	}

	/// Set the `rotate` attribute.
	pub fn with_rotate<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphAttrs::Rotate, value);
		self
	}

	/// Set the `rotate` attribute.
	pub fn set_rotate<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphAttrs::Rotate, value);
	}

	/// Get the `rotate` attribute.
	pub fn rotate(&self) -> Option<&dyn Value> {
		self.get_attr(AltGlyphAttrs::Rotate)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(AltGlyphAttrs::Style)
	}

	/// Set the `x` attribute.
	pub fn with_x<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphAttrs::X, value);
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphAttrs::X, value);
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&dyn Value> {
		self.get_attr(AltGlyphAttrs::X)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphAttrs::XlinkHref, value);
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphAttrs::XlinkHref, value);
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&dyn Value> {
		self.get_attr(AltGlyphAttrs::XlinkHref)
	}

	/// Set the `y` attribute.
	pub fn with_y<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphAttrs::Y, value);
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphAttrs::Y, value);
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&dyn Value> {
		self.get_attr(AltGlyphAttrs::Y)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for AltGlyph {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(AltGlyphAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for AltGlyph {}

impl common_attrs::CoreAttributesSetter for AltGlyph {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(AltGlyphAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for AltGlyph {}

impl common_attrs::GlobalEventAttributesSetter for AltGlyph {
	fn set_attr<V>(&mut self, attr: common_attrs::GlobalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GlobalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(AltGlyphAttrs::from(attr))
	}
}

impl TagWithGlobalEventAttributes for AltGlyph {}

impl common_attrs::GraphicalEventAttributesSetter for AltGlyph {
	fn set_attr<V>(&mut self, attr: common_attrs::GraphicalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(AltGlyphAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for AltGlyph {}

impl common_attrs::PresentationAttributesSetter for AltGlyph {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(AltGlyphAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for AltGlyph {}

impl common_attrs::XLinkAttributesSetter for AltGlyph {
	fn set_attr<V>(&mut self, attr: common_attrs::XLinkAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&dyn Value> {
		self.get_attr(AltGlyphAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for AltGlyph {}

impl Tag for AltGlyph {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("altGlyph");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		w.write_cdata_text(&self.content);
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum AltGlyphDefAttrs {
	CoreAttributes(common_attrs::CoreAttributes),
}

impl From<common_attrs::CoreAttributes> for AltGlyphDefAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl AltGlyphDefAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::CoreAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for AltGlyphDefAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for AltGlyphDefAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[doc = "The [`<altGlyphDef>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("altGlyphDef.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n"
)]
#[doc = "\n\n [`<altGlyphDef>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/altGlyphDef"]
#[derive(Debug)]
pub struct AltGlyphDef {
	attrs: IndexMap<AltGlyphDefAttrs, Box<dyn Value>>,
}

impl Default for AltGlyphDef {
	fn default() -> Self {
		Self::new()
	}
}

impl AltGlyphDef {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
		}
	}

	fn set_attr<V>(&mut self, attr: AltGlyphDefAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: AltGlyphDefAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}
}

impl common_attrs::CoreAttributesSetter for AltGlyphDef {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphDefAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(AltGlyphDefAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for AltGlyphDef {}

impl Tag for AltGlyphDef {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("altGlyphDef");
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum AltGlyphItemAttrs {
	CoreAttributes(common_attrs::CoreAttributes),
}

impl From<common_attrs::CoreAttributes> for AltGlyphItemAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl AltGlyphItemAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::CoreAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for AltGlyphItemAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for AltGlyphItemAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[doc = "The [`<altGlyphItem>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("altGlyphItem.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n"
)]
#[doc = "\n\n [`<altGlyphItem>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/altGlyphItem"]
#[derive(Debug)]
pub struct AltGlyphItem {
	attrs: IndexMap<AltGlyphItemAttrs, Box<dyn Value>>,
}

impl Default for AltGlyphItem {
	fn default() -> Self {
		Self::new()
	}
}

impl AltGlyphItem {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
		}
	}

	fn set_attr<V>(&mut self, attr: AltGlyphItemAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: AltGlyphItemAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}
}

impl common_attrs::CoreAttributesSetter for AltGlyphItem {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AltGlyphItemAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(AltGlyphItemAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for AltGlyphItem {}

impl Tag for AltGlyphItem {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("altGlyphItem");
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum AnimateAttrs {
	AttributeName,
	AttributeType,
	Dur,
	ExternalResourcesRequired,
	From,
	RepeatCount,
	To,
	AnimationAdditionAttributes(common_attrs::AnimationAdditionAttributes),
	AnimationAttributeTargetAttributes(common_attrs::AnimationAttributeTargetAttributes),
	AnimationEventAttributes(common_attrs::AnimationEventAttributes),
	AnimationTimingAttributes(common_attrs::AnimationTimingAttributes),
	AnimationValueAttributes(common_attrs::AnimationValueAttributes),
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	XLinkAttributes(common_attrs::XLinkAttributes),
}

impl From<common_attrs::AnimationAdditionAttributes> for AnimateAttrs {
	fn from(attr: common_attrs::AnimationAdditionAttributes) -> Self {
		Self::AnimationAdditionAttributes(attr)
	}
}

impl From<common_attrs::AnimationAttributeTargetAttributes> for AnimateAttrs {
	fn from(attr: common_attrs::AnimationAttributeTargetAttributes) -> Self {
		Self::AnimationAttributeTargetAttributes(attr)
	}
}

impl From<common_attrs::AnimationEventAttributes> for AnimateAttrs {
	fn from(attr: common_attrs::AnimationEventAttributes) -> Self {
		Self::AnimationEventAttributes(attr)
	}
}

impl From<common_attrs::AnimationTimingAttributes> for AnimateAttrs {
	fn from(attr: common_attrs::AnimationTimingAttributes) -> Self {
		Self::AnimationTimingAttributes(attr)
	}
}

impl From<common_attrs::AnimationValueAttributes> for AnimateAttrs {
	fn from(attr: common_attrs::AnimationValueAttributes) -> Self {
		Self::AnimationValueAttributes(attr)
	}
}

impl From<common_attrs::ConditionalProcessingAttributes> for AnimateAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for AnimateAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::XLinkAttributes> for AnimateAttrs {
	fn from(attr: common_attrs::XLinkAttributes) -> Self {
		Self::XLinkAttributes(attr)
	}
}

impl AnimateAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::AttributeName => "attributeName",
			Self::AttributeType => "attributeType",
			Self::Dur => "dur",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::From => "from",
			Self::RepeatCount => "repeatCount",
			Self::To => "to",
			Self::AnimationAdditionAttributes(attr) => attr.as_str(),
			Self::AnimationAttributeTargetAttributes(attr) => attr.as_str(),
			Self::AnimationEventAttributes(attr) => attr.as_str(),
			Self::AnimationTimingAttributes(attr) => attr.as_str(),
			Self::AnimationValueAttributes(attr) => attr.as_str(),
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::XLinkAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for AnimateAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for AnimateAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod animate_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Desc {}
	impl Content for super::Metadata {}
	impl Content for super::Title {}
}

#[doc = "The [`<animate>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("animate.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<desc>`](Desc)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<title>`](Title)\n"
)]
#[doc = "\n\n [`<animate>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/animate"]
#[derive(Debug)]
pub struct Animate {
	attrs: IndexMap<AnimateAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn animate_private::Content>>,
}

impl Default for Animate {
	fn default() -> Self {
		Self::new()
	}
}

impl Animate {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: animate_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: animate_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: AnimateAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: AnimateAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `attributeName` attribute.
	pub fn with_attribute_name<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimateAttrs::AttributeName, value);
		self
	}

	/// Set the `attributeName` attribute.
	pub fn set_attribute_name<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateAttrs::AttributeName, value);
	}

	/// Get the `attributeName` attribute.
	pub fn attribute_name(&self) -> Option<&dyn Value> {
		self.get_attr(AnimateAttrs::AttributeName)
	}

	/// Set the `attributeType` attribute.
	pub fn with_attribute_type<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimateAttrs::AttributeType, value);
		self
	}

	/// Set the `attributeType` attribute.
	pub fn set_attribute_type<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateAttrs::AttributeType, value);
	}

	/// Get the `attributeType` attribute.
	pub fn attribute_type(&self) -> Option<&dyn Value> {
		self.get_attr(AnimateAttrs::AttributeType)
	}

	/// Set the `dur` attribute.
	pub fn with_dur<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimateAttrs::Dur, value);
		self
	}

	/// Set the `dur` attribute.
	pub fn set_dur<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateAttrs::Dur, value);
	}

	/// Get the `dur` attribute.
	pub fn dur(&self) -> Option<&dyn Value> {
		self.get_attr(AnimateAttrs::Dur)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimateAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(AnimateAttrs::ExternalResourcesRequired)
	}

	/// Set the `from` attribute.
	pub fn with_from<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimateAttrs::From, value);
		self
	}

	/// Set the `from` attribute.
	pub fn set_from<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateAttrs::From, value);
	}

	/// Get the `from` attribute.
	pub fn from(&self) -> Option<&dyn Value> {
		self.get_attr(AnimateAttrs::From)
	}

	/// Set the `repeatCount` attribute.
	pub fn with_repeat_count<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimateAttrs::RepeatCount, value);
		self
	}

	/// Set the `repeatCount` attribute.
	pub fn set_repeat_count<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateAttrs::RepeatCount, value);
	}

	/// Get the `repeatCount` attribute.
	pub fn repeat_count(&self) -> Option<&dyn Value> {
		self.get_attr(AnimateAttrs::RepeatCount)
	}

	/// Set the `to` attribute.
	pub fn with_to<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimateAttrs::To, value);
		self
	}

	/// Set the `to` attribute.
	pub fn set_to<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateAttrs::To, value);
	}

	/// Get the `to` attribute.
	pub fn to(&self) -> Option<&dyn Value> {
		self.get_attr(AnimateAttrs::To)
	}
}

impl common_attrs::AnimationAdditionAttributesSetter for Animate {
	fn set_attr<V>(&mut self, attr: common_attrs::AnimationAdditionAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationAdditionAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateAttrs::from(attr))
	}
}

impl TagWithAnimationAdditionAttributes for Animate {}

impl common_attrs::AnimationAttributeTargetAttributesSetter for Animate {
	fn set_attr<V>(&mut self, attr: common_attrs::AnimationAttributeTargetAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationAttributeTargetAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateAttrs::from(attr))
	}
}

impl TagWithAnimationAttributeTargetAttributes for Animate {}

impl common_attrs::AnimationEventAttributesSetter for Animate {
	fn set_attr<V>(&mut self, attr: common_attrs::AnimationEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationEventAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateAttrs::from(attr))
	}
}

impl TagWithAnimationEventAttributes for Animate {}

impl common_attrs::AnimationTimingAttributesSetter for Animate {
	fn set_attr<V>(&mut self, attr: common_attrs::AnimationTimingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationTimingAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateAttrs::from(attr))
	}
}

impl TagWithAnimationTimingAttributes for Animate {}

impl common_attrs::AnimationValueAttributesSetter for Animate {
	fn set_attr<V>(&mut self, attr: common_attrs::AnimationValueAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationValueAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateAttrs::from(attr))
	}
}

impl TagWithAnimationValueAttributes for Animate {}

impl common_attrs::ConditionalProcessingAttributesSetter for Animate {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Animate {}

impl common_attrs::CoreAttributesSetter for Animate {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Animate {}

impl common_attrs::XLinkAttributesSetter for Animate {
	fn set_attr<V>(&mut self, attr: common_attrs::XLinkAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for Animate {}

impl Tag for Animate {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("animate");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum AnimateColorAttrs {
	By,
	ExternalResourcesRequired,
	From,
	To,
	AnimationAdditionAttributes(common_attrs::AnimationAdditionAttributes),
	AnimationAttributeTargetAttributes(common_attrs::AnimationAttributeTargetAttributes),
	AnimationEventAttributes(common_attrs::AnimationEventAttributes),
	AnimationTimingAttributes(common_attrs::AnimationTimingAttributes),
	AnimationValueAttributes(common_attrs::AnimationValueAttributes),
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	XLinkAttributes(common_attrs::XLinkAttributes),
}

impl From<common_attrs::AnimationAdditionAttributes> for AnimateColorAttrs {
	fn from(attr: common_attrs::AnimationAdditionAttributes) -> Self {
		Self::AnimationAdditionAttributes(attr)
	}
}

impl From<common_attrs::AnimationAttributeTargetAttributes> for AnimateColorAttrs {
	fn from(attr: common_attrs::AnimationAttributeTargetAttributes) -> Self {
		Self::AnimationAttributeTargetAttributes(attr)
	}
}

impl From<common_attrs::AnimationEventAttributes> for AnimateColorAttrs {
	fn from(attr: common_attrs::AnimationEventAttributes) -> Self {
		Self::AnimationEventAttributes(attr)
	}
}

impl From<common_attrs::AnimationTimingAttributes> for AnimateColorAttrs {
	fn from(attr: common_attrs::AnimationTimingAttributes) -> Self {
		Self::AnimationTimingAttributes(attr)
	}
}

impl From<common_attrs::AnimationValueAttributes> for AnimateColorAttrs {
	fn from(attr: common_attrs::AnimationValueAttributes) -> Self {
		Self::AnimationValueAttributes(attr)
	}
}

impl From<common_attrs::ConditionalProcessingAttributes> for AnimateColorAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for AnimateColorAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::XLinkAttributes> for AnimateColorAttrs {
	fn from(attr: common_attrs::XLinkAttributes) -> Self {
		Self::XLinkAttributes(attr)
	}
}

impl AnimateColorAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::By => "by",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::From => "from",
			Self::To => "to",
			Self::AnimationAdditionAttributes(attr) => attr.as_str(),
			Self::AnimationAttributeTargetAttributes(attr) => attr.as_str(),
			Self::AnimationEventAttributes(attr) => attr.as_str(),
			Self::AnimationTimingAttributes(attr) => attr.as_str(),
			Self::AnimationValueAttributes(attr) => attr.as_str(),
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::XLinkAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for AnimateColorAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for AnimateColorAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod animate_color_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Desc {}
	impl Content for super::Metadata {}
	impl Content for super::Title {}
}

#[doc = "The [`<animateColor>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("animateColor.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<desc>`](Desc)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<title>`](Title)\n"
)]
#[doc = "\n\n [`<animateColor>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/animateColor"]
#[derive(Debug)]
pub struct AnimateColor {
	attrs: IndexMap<AnimateColorAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn animate_color_private::Content>>,
}

impl Default for AnimateColor {
	fn default() -> Self {
		Self::new()
	}
}

impl AnimateColor {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: animate_color_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: animate_color_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: AnimateColorAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: AnimateColorAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `by` attribute.
	pub fn with_by<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimateColorAttrs::By, value);
		self
	}

	/// Set the `by` attribute.
	pub fn set_by<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateColorAttrs::By, value);
	}

	/// Get the `by` attribute.
	pub fn by(&self) -> Option<&dyn Value> {
		self.get_attr(AnimateColorAttrs::By)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimateColorAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateColorAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(AnimateColorAttrs::ExternalResourcesRequired)
	}

	/// Set the `from` attribute.
	pub fn with_from<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimateColorAttrs::From, value);
		self
	}

	/// Set the `from` attribute.
	pub fn set_from<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateColorAttrs::From, value);
	}

	/// Get the `from` attribute.
	pub fn from(&self) -> Option<&dyn Value> {
		self.get_attr(AnimateColorAttrs::From)
	}

	/// Set the `to` attribute.
	pub fn with_to<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimateColorAttrs::To, value);
		self
	}

	/// Set the `to` attribute.
	pub fn set_to<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateColorAttrs::To, value);
	}

	/// Get the `to` attribute.
	pub fn to(&self) -> Option<&dyn Value> {
		self.get_attr(AnimateColorAttrs::To)
	}
}

impl common_attrs::AnimationAdditionAttributesSetter for AnimateColor {
	fn set_attr<V>(&mut self, attr: common_attrs::AnimationAdditionAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateColorAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationAdditionAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateColorAttrs::from(attr))
	}
}

impl TagWithAnimationAdditionAttributes for AnimateColor {}

impl common_attrs::AnimationAttributeTargetAttributesSetter for AnimateColor {
	fn set_attr<V>(&mut self, attr: common_attrs::AnimationAttributeTargetAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateColorAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationAttributeTargetAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateColorAttrs::from(attr))
	}
}

impl TagWithAnimationAttributeTargetAttributes for AnimateColor {}

impl common_attrs::AnimationEventAttributesSetter for AnimateColor {
	fn set_attr<V>(&mut self, attr: common_attrs::AnimationEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateColorAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationEventAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateColorAttrs::from(attr))
	}
}

impl TagWithAnimationEventAttributes for AnimateColor {}

impl common_attrs::AnimationTimingAttributesSetter for AnimateColor {
	fn set_attr<V>(&mut self, attr: common_attrs::AnimationTimingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateColorAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationTimingAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateColorAttrs::from(attr))
	}
}

impl TagWithAnimationTimingAttributes for AnimateColor {}

impl common_attrs::AnimationValueAttributesSetter for AnimateColor {
	fn set_attr<V>(&mut self, attr: common_attrs::AnimationValueAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateColorAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationValueAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateColorAttrs::from(attr))
	}
}

impl TagWithAnimationValueAttributes for AnimateColor {}

impl common_attrs::ConditionalProcessingAttributesSetter for AnimateColor {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateColorAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateColorAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for AnimateColor {}

impl common_attrs::CoreAttributesSetter for AnimateColor {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateColorAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateColorAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for AnimateColor {}

impl common_attrs::XLinkAttributesSetter for AnimateColor {
	fn set_attr<V>(&mut self, attr: common_attrs::XLinkAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateColorAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateColorAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for AnimateColor {}

impl Tag for AnimateColor {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("animateColor");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum AnimateMotionAttrs {
	CalcMode,
	ExternalResourcesRequired,
	KeyPoints,
	Origin,
	Path,
	Rotate,
	AnimationAdditionAttributes(common_attrs::AnimationAdditionAttributes),
	AnimationEventAttributes(common_attrs::AnimationEventAttributes),
	AnimationTimingAttributes(common_attrs::AnimationTimingAttributes),
	AnimationValueAttributes(common_attrs::AnimationValueAttributes),
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	XLinkAttributes(common_attrs::XLinkAttributes),
}

impl From<common_attrs::AnimationAdditionAttributes> for AnimateMotionAttrs {
	fn from(attr: common_attrs::AnimationAdditionAttributes) -> Self {
		Self::AnimationAdditionAttributes(attr)
	}
}

impl From<common_attrs::AnimationEventAttributes> for AnimateMotionAttrs {
	fn from(attr: common_attrs::AnimationEventAttributes) -> Self {
		Self::AnimationEventAttributes(attr)
	}
}

impl From<common_attrs::AnimationTimingAttributes> for AnimateMotionAttrs {
	fn from(attr: common_attrs::AnimationTimingAttributes) -> Self {
		Self::AnimationTimingAttributes(attr)
	}
}

impl From<common_attrs::AnimationValueAttributes> for AnimateMotionAttrs {
	fn from(attr: common_attrs::AnimationValueAttributes) -> Self {
		Self::AnimationValueAttributes(attr)
	}
}

impl From<common_attrs::ConditionalProcessingAttributes> for AnimateMotionAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for AnimateMotionAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::XLinkAttributes> for AnimateMotionAttrs {
	fn from(attr: common_attrs::XLinkAttributes) -> Self {
		Self::XLinkAttributes(attr)
	}
}

impl AnimateMotionAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::CalcMode => "calcMode",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::KeyPoints => "keyPoints",
			Self::Origin => "origin",
			Self::Path => "path",
			Self::Rotate => "rotate",
			Self::AnimationAdditionAttributes(attr) => attr.as_str(),
			Self::AnimationEventAttributes(attr) => attr.as_str(),
			Self::AnimationTimingAttributes(attr) => attr.as_str(),
			Self::AnimationValueAttributes(attr) => attr.as_str(),
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::XLinkAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for AnimateMotionAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for AnimateMotionAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod animate_motion_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Desc {}
	impl Content for super::Metadata {}
	impl Content for super::Mpath {}
	impl Content for super::Title {}
}

#[doc = "The [`<animateMotion>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("animateMotion.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<desc>`](Desc)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<mpath>`](Mpath)\n",
	"- [`<title>`](Title)\n"
)]
#[doc = "\n\n [`<animateMotion>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/animateMotion"]
#[derive(Debug)]
pub struct AnimateMotion {
	attrs: IndexMap<AnimateMotionAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn animate_motion_private::Content>>,
}

impl Default for AnimateMotion {
	fn default() -> Self {
		Self::new()
	}
}

impl AnimateMotion {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: animate_motion_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: animate_motion_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: AnimateMotionAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: AnimateMotionAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `calcMode` attribute.
	pub fn with_calc_mode<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimateMotionAttrs::CalcMode, value);
		self
	}

	/// Set the `calcMode` attribute.
	pub fn set_calc_mode<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateMotionAttrs::CalcMode, value);
	}

	/// Get the `calcMode` attribute.
	pub fn calc_mode(&self) -> Option<&dyn Value> {
		self.get_attr(AnimateMotionAttrs::CalcMode)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimateMotionAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateMotionAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(AnimateMotionAttrs::ExternalResourcesRequired)
	}

	/// Set the `keyPoints` attribute.
	pub fn with_key_points<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimateMotionAttrs::KeyPoints, value);
		self
	}

	/// Set the `keyPoints` attribute.
	pub fn set_key_points<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateMotionAttrs::KeyPoints, value);
	}

	/// Get the `keyPoints` attribute.
	pub fn key_points(&self) -> Option<&dyn Value> {
		self.get_attr(AnimateMotionAttrs::KeyPoints)
	}

	/// Set the `origin` attribute.
	pub fn with_origin<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimateMotionAttrs::Origin, value);
		self
	}

	/// Set the `origin` attribute.
	pub fn set_origin<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateMotionAttrs::Origin, value);
	}

	/// Get the `origin` attribute.
	pub fn origin(&self) -> Option<&dyn Value> {
		self.get_attr(AnimateMotionAttrs::Origin)
	}

	/// Set the `path` attribute.
	pub fn with_path<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimateMotionAttrs::Path, value);
		self
	}

	/// Set the `path` attribute.
	pub fn set_path<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateMotionAttrs::Path, value);
	}

	/// Get the `path` attribute.
	pub fn path(&self) -> Option<&dyn Value> {
		self.get_attr(AnimateMotionAttrs::Path)
	}

	/// Set the `rotate` attribute.
	pub fn with_rotate<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimateMotionAttrs::Rotate, value);
		self
	}

	/// Set the `rotate` attribute.
	pub fn set_rotate<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateMotionAttrs::Rotate, value);
	}

	/// Get the `rotate` attribute.
	pub fn rotate(&self) -> Option<&dyn Value> {
		self.get_attr(AnimateMotionAttrs::Rotate)
	}
}

impl common_attrs::AnimationAdditionAttributesSetter for AnimateMotion {
	fn set_attr<V>(&mut self, attr: common_attrs::AnimationAdditionAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateMotionAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationAdditionAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateMotionAttrs::from(attr))
	}
}

impl TagWithAnimationAdditionAttributes for AnimateMotion {}

impl common_attrs::AnimationEventAttributesSetter for AnimateMotion {
	fn set_attr<V>(&mut self, attr: common_attrs::AnimationEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateMotionAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationEventAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateMotionAttrs::from(attr))
	}
}

impl TagWithAnimationEventAttributes for AnimateMotion {}

impl common_attrs::AnimationTimingAttributesSetter for AnimateMotion {
	fn set_attr<V>(&mut self, attr: common_attrs::AnimationTimingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateMotionAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationTimingAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateMotionAttrs::from(attr))
	}
}

impl TagWithAnimationTimingAttributes for AnimateMotion {}

impl common_attrs::AnimationValueAttributesSetter for AnimateMotion {
	fn set_attr<V>(&mut self, attr: common_attrs::AnimationValueAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateMotionAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationValueAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateMotionAttrs::from(attr))
	}
}

impl TagWithAnimationValueAttributes for AnimateMotion {}

impl common_attrs::ConditionalProcessingAttributesSetter for AnimateMotion {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateMotionAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateMotionAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for AnimateMotion {}

impl common_attrs::CoreAttributesSetter for AnimateMotion {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateMotionAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateMotionAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for AnimateMotion {}

impl common_attrs::XLinkAttributesSetter for AnimateMotion {
	fn set_attr<V>(&mut self, attr: common_attrs::XLinkAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateMotionAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateMotionAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for AnimateMotion {}

impl Tag for AnimateMotion {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("animateMotion");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum AnimateTransformAttrs {
	By,
	ExternalResourcesRequired,
	From,
	To,
	Type,
	AnimationAdditionAttributes(common_attrs::AnimationAdditionAttributes),
	AnimationAttributeTargetAttributes(common_attrs::AnimationAttributeTargetAttributes),
	AnimationEventAttributes(common_attrs::AnimationEventAttributes),
	AnimationTimingAttributes(common_attrs::AnimationTimingAttributes),
	AnimationValueAttributes(common_attrs::AnimationValueAttributes),
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	XLinkAttributes(common_attrs::XLinkAttributes),
}

impl From<common_attrs::AnimationAdditionAttributes> for AnimateTransformAttrs {
	fn from(attr: common_attrs::AnimationAdditionAttributes) -> Self {
		Self::AnimationAdditionAttributes(attr)
	}
}

impl From<common_attrs::AnimationAttributeTargetAttributes> for AnimateTransformAttrs {
	fn from(attr: common_attrs::AnimationAttributeTargetAttributes) -> Self {
		Self::AnimationAttributeTargetAttributes(attr)
	}
}

impl From<common_attrs::AnimationEventAttributes> for AnimateTransformAttrs {
	fn from(attr: common_attrs::AnimationEventAttributes) -> Self {
		Self::AnimationEventAttributes(attr)
	}
}

impl From<common_attrs::AnimationTimingAttributes> for AnimateTransformAttrs {
	fn from(attr: common_attrs::AnimationTimingAttributes) -> Self {
		Self::AnimationTimingAttributes(attr)
	}
}

impl From<common_attrs::AnimationValueAttributes> for AnimateTransformAttrs {
	fn from(attr: common_attrs::AnimationValueAttributes) -> Self {
		Self::AnimationValueAttributes(attr)
	}
}

impl From<common_attrs::ConditionalProcessingAttributes> for AnimateTransformAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for AnimateTransformAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::XLinkAttributes> for AnimateTransformAttrs {
	fn from(attr: common_attrs::XLinkAttributes) -> Self {
		Self::XLinkAttributes(attr)
	}
}

impl AnimateTransformAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::By => "by",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::From => "from",
			Self::To => "to",
			Self::Type => "type",
			Self::AnimationAdditionAttributes(attr) => attr.as_str(),
			Self::AnimationAttributeTargetAttributes(attr) => attr.as_str(),
			Self::AnimationEventAttributes(attr) => attr.as_str(),
			Self::AnimationTimingAttributes(attr) => attr.as_str(),
			Self::AnimationValueAttributes(attr) => attr.as_str(),
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::XLinkAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for AnimateTransformAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for AnimateTransformAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod animate_transform_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Desc {}
	impl Content for super::Metadata {}
	impl Content for super::Title {}
}

#[doc = "The [`<animateTransform>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("animateTransform.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<desc>`](Desc)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<title>`](Title)\n"
)]
#[doc = "\n\n [`<animateTransform>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/animateTransform"]
#[derive(Debug)]
pub struct AnimateTransform {
	attrs: IndexMap<AnimateTransformAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn animate_transform_private::Content>>,
}

impl Default for AnimateTransform {
	fn default() -> Self {
		Self::new()
	}
}

impl AnimateTransform {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: animate_transform_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: animate_transform_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: AnimateTransformAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: AnimateTransformAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `by` attribute.
	pub fn with_by<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimateTransformAttrs::By, value);
		self
	}

	/// Set the `by` attribute.
	pub fn set_by<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateTransformAttrs::By, value);
	}

	/// Get the `by` attribute.
	pub fn by(&self) -> Option<&dyn Value> {
		self.get_attr(AnimateTransformAttrs::By)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimateTransformAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateTransformAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(AnimateTransformAttrs::ExternalResourcesRequired)
	}

	/// Set the `from` attribute.
	pub fn with_from<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimateTransformAttrs::From, value);
		self
	}

	/// Set the `from` attribute.
	pub fn set_from<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateTransformAttrs::From, value);
	}

	/// Get the `from` attribute.
	pub fn from(&self) -> Option<&dyn Value> {
		self.get_attr(AnimateTransformAttrs::From)
	}

	/// Set the `to` attribute.
	pub fn with_to<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimateTransformAttrs::To, value);
		self
	}

	/// Set the `to` attribute.
	pub fn set_to<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateTransformAttrs::To, value);
	}

	/// Get the `to` attribute.
	pub fn to(&self) -> Option<&dyn Value> {
		self.get_attr(AnimateTransformAttrs::To)
	}

	/// Set the `type` attribute.
	pub fn with_ty<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimateTransformAttrs::Type, value);
		self
	}

	/// Set the `type` attribute.
	pub fn set_ty<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateTransformAttrs::Type, value);
	}

	/// Get the `type` attribute.
	pub fn ty(&self) -> Option<&dyn Value> {
		self.get_attr(AnimateTransformAttrs::Type)
	}
}

impl common_attrs::AnimationAdditionAttributesSetter for AnimateTransform {
	fn set_attr<V>(&mut self, attr: common_attrs::AnimationAdditionAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateTransformAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationAdditionAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateTransformAttrs::from(attr))
	}
}

impl TagWithAnimationAdditionAttributes for AnimateTransform {}

impl common_attrs::AnimationAttributeTargetAttributesSetter for AnimateTransform {
	fn set_attr<V>(&mut self, attr: common_attrs::AnimationAttributeTargetAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateTransformAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationAttributeTargetAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateTransformAttrs::from(attr))
	}
}

impl TagWithAnimationAttributeTargetAttributes for AnimateTransform {}

impl common_attrs::AnimationEventAttributesSetter for AnimateTransform {
	fn set_attr<V>(&mut self, attr: common_attrs::AnimationEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateTransformAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationEventAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateTransformAttrs::from(attr))
	}
}

impl TagWithAnimationEventAttributes for AnimateTransform {}

impl common_attrs::AnimationTimingAttributesSetter for AnimateTransform {
	fn set_attr<V>(&mut self, attr: common_attrs::AnimationTimingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateTransformAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationTimingAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateTransformAttrs::from(attr))
	}
}

impl TagWithAnimationTimingAttributes for AnimateTransform {}

impl common_attrs::AnimationValueAttributesSetter for AnimateTransform {
	fn set_attr<V>(&mut self, attr: common_attrs::AnimationValueAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateTransformAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationValueAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateTransformAttrs::from(attr))
	}
}

impl TagWithAnimationValueAttributes for AnimateTransform {}

impl common_attrs::ConditionalProcessingAttributesSetter for AnimateTransform {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateTransformAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateTransformAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for AnimateTransform {}

impl common_attrs::CoreAttributesSetter for AnimateTransform {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateTransformAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateTransformAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for AnimateTransform {}

impl common_attrs::XLinkAttributesSetter for AnimateTransform {
	fn set_attr<V>(&mut self, attr: common_attrs::XLinkAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimateTransformAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&dyn Value> {
		self.get_attr(AnimateTransformAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for AnimateTransform {}

impl Tag for AnimateTransform {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("animateTransform");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum CircleAttrs {
	Class,
	Cx,
	Cy,
	ExternalResourcesRequired,
	R,
	Style,
	Transform,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	GlobalEventAttributes(common_attrs::GlobalEventAttributes),
	GraphicalEventAttributes(common_attrs::GraphicalEventAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::ConditionalProcessingAttributes> for CircleAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for CircleAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::GlobalEventAttributes> for CircleAttrs {
	fn from(attr: common_attrs::GlobalEventAttributes) -> Self {
		Self::GlobalEventAttributes(attr)
	}
}

impl From<common_attrs::GraphicalEventAttributes> for CircleAttrs {
	fn from(attr: common_attrs::GraphicalEventAttributes) -> Self {
		Self::GraphicalEventAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for CircleAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl CircleAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::Cx => "cx",
			Self::Cy => "cy",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::R => "r",
			Self::Style => "style",
			Self::Transform => "transform",
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::GlobalEventAttributes(attr) => attr.as_str(),
			Self::GraphicalEventAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for CircleAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for CircleAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod circle_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::AnimateMotion {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Desc {}
	impl Content for super::Discard {}
	impl Content for super::Metadata {}
	impl Content for super::Mpath {}
	impl Content for super::Set {}
	impl Content for super::Title {}
}

#[doc = "The [`<circle>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("circle.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateMotion>`](AnimateMotion)\n",
	"- [`<animateTransform>`](AnimateTransform)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<discard>`](Discard)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<mpath>`](Mpath)\n",
	"- [`<set>`](Set)\n",
	"- [`<title>`](Title)\n"
)]
#[doc = "\n\n [`<circle>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/circle"]
#[derive(Debug)]
pub struct Circle {
	attrs: IndexMap<CircleAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn circle_private::Content>>,
}

impl Default for Circle {
	fn default() -> Self {
		Self::new()
	}
}

impl Circle {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: circle_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: circle_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: CircleAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: CircleAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(CircleAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(CircleAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(CircleAttrs::Class)
	}

	/// Set the `cx` attribute.
	pub fn with_cx<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(CircleAttrs::Cx, value);
		self
	}

	/// Set the `cx` attribute.
	pub fn set_cx<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(CircleAttrs::Cx, value);
	}

	/// Get the `cx` attribute.
	pub fn cx(&self) -> Option<&dyn Value> {
		self.get_attr(CircleAttrs::Cx)
	}

	/// Set the `cy` attribute.
	pub fn with_cy<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(CircleAttrs::Cy, value);
		self
	}

	/// Set the `cy` attribute.
	pub fn set_cy<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(CircleAttrs::Cy, value);
	}

	/// Get the `cy` attribute.
	pub fn cy(&self) -> Option<&dyn Value> {
		self.get_attr(CircleAttrs::Cy)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(CircleAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(CircleAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(CircleAttrs::ExternalResourcesRequired)
	}

	/// Set the `r` attribute.
	pub fn with_r<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(CircleAttrs::R, value);
		self
	}

	/// Set the `r` attribute.
	pub fn set_r<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(CircleAttrs::R, value);
	}

	/// Get the `r` attribute.
	pub fn r(&self) -> Option<&dyn Value> {
		self.get_attr(CircleAttrs::R)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(CircleAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(CircleAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(CircleAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(CircleAttrs::Transform, value);
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(CircleAttrs::Transform, value);
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&dyn Value> {
		self.get_attr(CircleAttrs::Transform)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Circle {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(CircleAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(CircleAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Circle {}

impl common_attrs::CoreAttributesSetter for Circle {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(CircleAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(CircleAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Circle {}

impl common_attrs::GlobalEventAttributesSetter for Circle {
	fn set_attr<V>(&mut self, attr: common_attrs::GlobalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(CircleAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GlobalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(CircleAttrs::from(attr))
	}
}

impl TagWithGlobalEventAttributes for Circle {}

impl common_attrs::GraphicalEventAttributesSetter for Circle {
	fn set_attr<V>(&mut self, attr: common_attrs::GraphicalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(CircleAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(CircleAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Circle {}

impl common_attrs::PresentationAttributesSetter for Circle {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(CircleAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(CircleAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Circle {}

impl Tag for Circle {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("circle");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum ClipPathAttrs {
	Class,
	ClipPathUnits,
	ExternalResourcesRequired,
	Style,
	Transform,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::ConditionalProcessingAttributes> for ClipPathAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for ClipPathAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for ClipPathAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl ClipPathAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ClipPathUnits => "clipPathUnits",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Style => "style",
			Self::Transform => "transform",
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for ClipPathAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for ClipPathAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod clip_path_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::AnimateColor {}
	impl Content for super::AnimateMotion {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Circle {}
	impl Content for super::Desc {}
	impl Content for super::Discard {}
	impl Content for super::Ellipse {}
	impl Content for super::Line {}
	impl Content for super::Metadata {}
	impl Content for super::Mpath {}
	impl Content for super::Path {}
	impl Content for super::Polygon {}
	impl Content for super::Polyline {}
	impl Content for super::Rect {}
	impl Content for super::Set {}
	impl Content for super::Text {}
	impl Content for super::Title {}
	impl Content for super::Use {}
}

#[doc = "The [`<clipPath>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("clipPath.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateColor>`](AnimateColor)\n",
	"- [`<animateMotion>`](AnimateMotion)\n",
	"- [`<animateTransform>`](AnimateTransform)\n",
	"- [`<circle>`](Circle)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<discard>`](Discard)\n",
	"- [`<ellipse>`](Ellipse)\n",
	"- [`<line>`](Line)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<mpath>`](Mpath)\n",
	"- [`<path>`](Path)\n",
	"- [`<polygon>`](Polygon)\n",
	"- [`<polyline>`](Polyline)\n",
	"- [`<rect>`](Rect)\n",
	"- [`<set>`](Set)\n",
	"- [`<text>`](Text)\n",
	"- [`<title>`](Title)\n",
	"- [`<use>`](Use)\n"
)]
#[doc = "\n\n [`<clipPath>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/clipPath"]
#[derive(Debug)]
pub struct ClipPath {
	attrs: IndexMap<ClipPathAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn clip_path_private::Content>>,
}

impl Default for ClipPath {
	fn default() -> Self {
		Self::new()
	}
}

impl ClipPath {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: clip_path_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: clip_path_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: ClipPathAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: ClipPathAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ClipPathAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ClipPathAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(ClipPathAttrs::Class)
	}

	/// Set the `clipPathUnits` attribute.
	pub fn with_clip_path_units<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ClipPathAttrs::ClipPathUnits, value);
		self
	}

	/// Set the `clipPathUnits` attribute.
	pub fn set_clip_path_units<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ClipPathAttrs::ClipPathUnits, value);
	}

	/// Get the `clipPathUnits` attribute.
	pub fn clip_path_units(&self) -> Option<&dyn Value> {
		self.get_attr(ClipPathAttrs::ClipPathUnits)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ClipPathAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ClipPathAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(ClipPathAttrs::ExternalResourcesRequired)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ClipPathAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ClipPathAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(ClipPathAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ClipPathAttrs::Transform, value);
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ClipPathAttrs::Transform, value);
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&dyn Value> {
		self.get_attr(ClipPathAttrs::Transform)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for ClipPath {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ClipPathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(ClipPathAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for ClipPath {}

impl common_attrs::CoreAttributesSetter for ClipPath {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ClipPathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(ClipPathAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for ClipPath {}

impl common_attrs::PresentationAttributesSetter for ClipPath {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ClipPathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(ClipPathAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for ClipPath {}

impl Tag for ClipPath {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("clipPath");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum ColorProfileAttrs {
	Local,
	Name,
	RenderingIntent,
	XlinkHref,
	CoreAttributes(common_attrs::CoreAttributes),
	XLinkAttributes(common_attrs::XLinkAttributes),
}

impl From<common_attrs::CoreAttributes> for ColorProfileAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::XLinkAttributes> for ColorProfileAttrs {
	fn from(attr: common_attrs::XLinkAttributes) -> Self {
		Self::XLinkAttributes(attr)
	}
}

impl ColorProfileAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Local => "local",
			Self::Name => "name",
			Self::RenderingIntent => "rendering-intent",
			Self::XlinkHref => "xlink:href",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::XLinkAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for ColorProfileAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for ColorProfileAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod color_profile_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Desc {}
	impl Content for super::Metadata {}
	impl Content for super::Title {}
}

#[doc = "The [`<color-profile>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("color-profile.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<desc>`](Desc)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<title>`](Title)\n"
)]
#[doc = "\n\n [`<color-profile>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/color-profile"]
#[derive(Debug)]
pub struct ColorProfile {
	attrs: IndexMap<ColorProfileAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn color_profile_private::Content>>,
}

impl Default for ColorProfile {
	fn default() -> Self {
		Self::new()
	}
}

impl ColorProfile {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: color_profile_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: color_profile_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: ColorProfileAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: ColorProfileAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `local` attribute.
	pub fn with_local<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ColorProfileAttrs::Local, value);
		self
	}

	/// Set the `local` attribute.
	pub fn set_local<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ColorProfileAttrs::Local, value);
	}

	/// Get the `local` attribute.
	pub fn local(&self) -> Option<&dyn Value> {
		self.get_attr(ColorProfileAttrs::Local)
	}

	/// Set the `name` attribute.
	pub fn with_name<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ColorProfileAttrs::Name, value);
		self
	}

	/// Set the `name` attribute.
	pub fn set_name<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ColorProfileAttrs::Name, value);
	}

	/// Get the `name` attribute.
	pub fn name(&self) -> Option<&dyn Value> {
		self.get_attr(ColorProfileAttrs::Name)
	}

	/// Set the `rendering-intent` attribute.
	pub fn with_rendering_intent<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ColorProfileAttrs::RenderingIntent, value);
		self
	}

	/// Set the `rendering-intent` attribute.
	pub fn set_rendering_intent<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ColorProfileAttrs::RenderingIntent, value);
	}

	/// Get the `rendering-intent` attribute.
	pub fn rendering_intent(&self) -> Option<&dyn Value> {
		self.get_attr(ColorProfileAttrs::RenderingIntent)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ColorProfileAttrs::XlinkHref, value);
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ColorProfileAttrs::XlinkHref, value);
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&dyn Value> {
		self.get_attr(ColorProfileAttrs::XlinkHref)
	}
}

impl common_attrs::CoreAttributesSetter for ColorProfile {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ColorProfileAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(ColorProfileAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for ColorProfile {}

impl common_attrs::XLinkAttributesSetter for ColorProfile {
	fn set_attr<V>(&mut self, attr: common_attrs::XLinkAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ColorProfileAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&dyn Value> {
		self.get_attr(ColorProfileAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for ColorProfile {}

impl Tag for ColorProfile {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("color-profile");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum CursorAttrs {
	ExternalResourcesRequired,
	X,
	XlinkHref,
	Y,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	XLinkAttributes(common_attrs::XLinkAttributes),
}

impl From<common_attrs::ConditionalProcessingAttributes> for CursorAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for CursorAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::XLinkAttributes> for CursorAttrs {
	fn from(attr: common_attrs::XLinkAttributes) -> Self {
		Self::XLinkAttributes(attr)
	}
}

impl CursorAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::X => "x",
			Self::XlinkHref => "xlink:href",
			Self::Y => "y",
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::XLinkAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for CursorAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for CursorAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod cursor_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Desc {}
	impl Content for super::Metadata {}
	impl Content for super::Title {}
}

#[doc = "The [`<cursor>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("cursor.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<desc>`](Desc)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<title>`](Title)\n"
)]
#[doc = "\n\n [`<cursor>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/cursor"]
#[derive(Debug)]
pub struct Cursor {
	attrs: IndexMap<CursorAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn cursor_private::Content>>,
}

impl Default for Cursor {
	fn default() -> Self {
		Self::new()
	}
}

impl Cursor {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: cursor_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: cursor_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: CursorAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: CursorAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(CursorAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(CursorAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(CursorAttrs::ExternalResourcesRequired)
	}

	/// Set the `x` attribute.
	pub fn with_x<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(CursorAttrs::X, value);
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(CursorAttrs::X, value);
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&dyn Value> {
		self.get_attr(CursorAttrs::X)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(CursorAttrs::XlinkHref, value);
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(CursorAttrs::XlinkHref, value);
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&dyn Value> {
		self.get_attr(CursorAttrs::XlinkHref)
	}

	/// Set the `y` attribute.
	pub fn with_y<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(CursorAttrs::Y, value);
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(CursorAttrs::Y, value);
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&dyn Value> {
		self.get_attr(CursorAttrs::Y)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Cursor {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(CursorAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(CursorAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Cursor {}

impl common_attrs::CoreAttributesSetter for Cursor {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(CursorAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(CursorAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Cursor {}

impl common_attrs::XLinkAttributesSetter for Cursor {
	fn set_attr<V>(&mut self, attr: common_attrs::XLinkAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(CursorAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&dyn Value> {
		self.get_attr(CursorAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for Cursor {}

impl Tag for Cursor {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("cursor");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum DefsAttrs {
	Class,
	ExternalResourcesRequired,
	Style,
	Transform,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	GlobalEventAttributes(common_attrs::GlobalEventAttributes),
	GraphicalEventAttributes(common_attrs::GraphicalEventAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::ConditionalProcessingAttributes> for DefsAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for DefsAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::GlobalEventAttributes> for DefsAttrs {
	fn from(attr: common_attrs::GlobalEventAttributes) -> Self {
		Self::GlobalEventAttributes(attr)
	}
}

impl From<common_attrs::GraphicalEventAttributes> for DefsAttrs {
	fn from(attr: common_attrs::GraphicalEventAttributes) -> Self {
		Self::GraphicalEventAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for DefsAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl DefsAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Style => "style",
			Self::Transform => "transform",
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::GlobalEventAttributes(attr) => attr.as_str(),
			Self::GraphicalEventAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for DefsAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for DefsAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod defs_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::A {}
	impl Content for super::AltGlyphDef {}
	impl Content for super::Animate {}
	impl Content for super::AnimateColor {}
	impl Content for super::AnimateMotion {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Circle {}
	impl Content for super::ClipPath {}
	impl Content for super::ColorProfile {}
	impl Content for super::Cursor {}
	impl Content for super::Defs {}
	impl Content for super::Desc {}
	impl Content for super::Discard {}
	impl Content for super::Ellipse {}
	impl Content for super::Filter {}
	impl Content for super::Font {}
	impl Content for super::FontFace {}
	impl Content for super::ForeignObject {}
	impl Content for super::Group {}
	impl Content for super::Image {}
	impl Content for super::Line {}
	impl Content for super::LinearGradient {}
	impl Content for super::Marker {}
	impl Content for super::Mask {}
	impl Content for super::Metadata {}
	impl Content for super::Mpath {}
	impl Content for super::Path {}
	impl Content for super::Pattern {}
	impl Content for super::Polygon {}
	impl Content for super::Polyline {}
	impl Content for super::RadialGradient {}
	impl Content for super::Rect {}
	impl Content for super::Script {}
	impl Content for super::Set {}
	impl Content for super::Stop {}
	impl Content for super::Style {}
	impl Content for super::Svg {}
	impl Content for super::Switch {}
	impl Content for super::Symbol {}
	impl Content for super::Text {}
	impl Content for super::Title {}
	impl Content for super::Use {}
	impl Content for super::View {}
}

#[doc = "The [`<defs>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("defs.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<a>`](A)\n",
	"- [`<altGlyphDef>`](AltGlyphDef)\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateColor>`](AnimateColor)\n",
	"- [`<animateMotion>`](AnimateMotion)\n",
	"- [`<animateTransform>`](AnimateTransform)\n",
	"- [`<circle>`](Circle)\n",
	"- [`<clipPath>`](ClipPath)\n",
	"- [`<color-profile>`](ColorProfile)\n",
	"- [`<cursor>`](Cursor)\n",
	"- [`<defs>`](Defs)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<discard>`](Discard)\n",
	"- [`<ellipse>`](Ellipse)\n",
	"- [`<filter>`](Filter)\n",
	"- [`<font>`](Font)\n",
	"- [`<font-face>`](FontFace)\n",
	"- [`<foreignObject>`](ForeignObject)\n",
	"- [`<g>`](Group)\n",
	"- [`<image>`](Image)\n",
	"- [`<line>`](Line)\n",
	"- [`<linearGradient>`](LinearGradient)\n",
	"- [`<marker>`](Marker)\n",
	"- [`<mask>`](Mask)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<mpath>`](Mpath)\n",
	"- [`<path>`](Path)\n",
	"- [`<pattern>`](Pattern)\n",
	"- [`<polygon>`](Polygon)\n",
	"- [`<polyline>`](Polyline)\n",
	"- [`<radialGradient>`](RadialGradient)\n",
	"- [`<rect>`](Rect)\n",
	"- [`<script>`](Script)\n",
	"- [`<set>`](Set)\n",
	"- [`<stop>`](Stop)\n",
	"- [`<style>`](Style)\n",
	"- [`<svg>`](Svg)\n",
	"- [`<switch>`](Switch)\n",
	"- [`<symbol>`](Symbol)\n",
	"- [`<text>`](Text)\n",
	"- [`<title>`](Title)\n",
	"- [`<use>`](Use)\n",
	"- [`<view>`](View)\n"
)]
#[doc = "\n\n [`<defs>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/defs"]
#[derive(Debug)]
pub struct Defs {
	attrs: IndexMap<DefsAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn defs_private::Content>>,
}

impl Default for Defs {
	fn default() -> Self {
		Self::new()
	}
}

impl Defs {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: defs_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: defs_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: DefsAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: DefsAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(DefsAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(DefsAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(DefsAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(DefsAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(DefsAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(DefsAttrs::ExternalResourcesRequired)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(DefsAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(DefsAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(DefsAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(DefsAttrs::Transform, value);
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(DefsAttrs::Transform, value);
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&dyn Value> {
		self.get_attr(DefsAttrs::Transform)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Defs {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(DefsAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(DefsAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Defs {}

impl common_attrs::CoreAttributesSetter for Defs {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(DefsAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(DefsAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Defs {}

impl common_attrs::GlobalEventAttributesSetter for Defs {
	fn set_attr<V>(&mut self, attr: common_attrs::GlobalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(DefsAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GlobalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(DefsAttrs::from(attr))
	}
}

impl TagWithGlobalEventAttributes for Defs {}

impl common_attrs::GraphicalEventAttributesSetter for Defs {
	fn set_attr<V>(&mut self, attr: common_attrs::GraphicalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(DefsAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(DefsAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Defs {}

impl common_attrs::PresentationAttributesSetter for Defs {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(DefsAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(DefsAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Defs {}

impl Tag for Defs {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("defs");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum DescAttrs {
	Class,
	Style,
	CoreAttributes(common_attrs::CoreAttributes),
}

impl From<common_attrs::CoreAttributes> for DescAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl DescAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::Style => "style",
			Self::CoreAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for DescAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for DescAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[doc = "The [`<desc>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("desc.md")]
#[doc = "\n\n [`<desc>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/desc"]
#[derive(Debug)]
pub struct Desc {
	attrs: IndexMap<DescAttrs, Box<dyn Value>>,
	content: String,
}

impl Default for Desc {
	fn default() -> Self {
		Self::new()
	}
}

impl Desc {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: String::new()
		}
	}

	pub fn push<T: Display>(&mut self, content: T) {
		write!(self.content, "{content}").unwrap();
	}

	pub fn append<T: Display>(mut self, content: T) -> Self {
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: DescAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: DescAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(DescAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(DescAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(DescAttrs::Class)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(DescAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(DescAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(DescAttrs::Style)
	}
}

impl common_attrs::CoreAttributesSetter for Desc {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(DescAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(DescAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Desc {}

impl Tag for Desc {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("desc");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		w.write_cdata_text(&self.content);
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum DiscardAttrs {
	Begin,
	Href,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
}

impl From<common_attrs::ConditionalProcessingAttributes> for DiscardAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for DiscardAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl DiscardAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Begin => "begin",
			Self::Href => "href",
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for DiscardAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for DiscardAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod discard_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Desc {}
	impl Content for super::Metadata {}
	impl Content for super::Script {}
	impl Content for super::Title {}
}

#[doc = "The [`<discard>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("discard.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<desc>`](Desc)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<script>`](Script)\n",
	"- [`<title>`](Title)\n"
)]
#[doc = "\n\n [`<discard>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/discard"]
#[derive(Debug)]
pub struct Discard {
	attrs: IndexMap<DiscardAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn discard_private::Content>>,
}

impl Default for Discard {
	fn default() -> Self {
		Self::new()
	}
}

impl Discard {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: discard_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: discard_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: DiscardAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: DiscardAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `begin` attribute.
	pub fn with_begin<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(DiscardAttrs::Begin, value);
		self
	}

	/// Set the `begin` attribute.
	pub fn set_begin<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(DiscardAttrs::Begin, value);
	}

	/// Get the `begin` attribute.
	pub fn begin(&self) -> Option<&dyn Value> {
		self.get_attr(DiscardAttrs::Begin)
	}

	/// Set the `href` attribute.
	pub fn with_href<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(DiscardAttrs::Href, value);
		self
	}

	/// Set the `href` attribute.
	pub fn set_href<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(DiscardAttrs::Href, value);
	}

	/// Get the `href` attribute.
	pub fn href(&self) -> Option<&dyn Value> {
		self.get_attr(DiscardAttrs::Href)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Discard {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(DiscardAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(DiscardAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Discard {}

impl common_attrs::CoreAttributesSetter for Discard {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(DiscardAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(DiscardAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Discard {}

impl Tag for Discard {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("discard");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum EllipseAttrs {
	Class,
	Cx,
	Cy,
	ExternalResourcesRequired,
	Rx,
	Ry,
	Style,
	Transform,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	GlobalEventAttributes(common_attrs::GlobalEventAttributes),
	GraphicalEventAttributes(common_attrs::GraphicalEventAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::ConditionalProcessingAttributes> for EllipseAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for EllipseAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::GlobalEventAttributes> for EllipseAttrs {
	fn from(attr: common_attrs::GlobalEventAttributes) -> Self {
		Self::GlobalEventAttributes(attr)
	}
}

impl From<common_attrs::GraphicalEventAttributes> for EllipseAttrs {
	fn from(attr: common_attrs::GraphicalEventAttributes) -> Self {
		Self::GraphicalEventAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for EllipseAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl EllipseAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::Cx => "cx",
			Self::Cy => "cy",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Rx => "rx",
			Self::Ry => "ry",
			Self::Style => "style",
			Self::Transform => "transform",
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::GlobalEventAttributes(attr) => attr.as_str(),
			Self::GraphicalEventAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for EllipseAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for EllipseAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod ellipse_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::AnimateMotion {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Desc {}
	impl Content for super::Discard {}
	impl Content for super::Metadata {}
	impl Content for super::Mpath {}
	impl Content for super::Set {}
	impl Content for super::Title {}
}

#[doc = "The [`<ellipse>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("ellipse.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateMotion>`](AnimateMotion)\n",
	"- [`<animateTransform>`](AnimateTransform)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<discard>`](Discard)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<mpath>`](Mpath)\n",
	"- [`<set>`](Set)\n",
	"- [`<title>`](Title)\n"
)]
#[doc = "\n\n [`<ellipse>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/ellipse"]
#[derive(Debug)]
pub struct Ellipse {
	attrs: IndexMap<EllipseAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn ellipse_private::Content>>,
}

impl Default for Ellipse {
	fn default() -> Self {
		Self::new()
	}
}

impl Ellipse {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: ellipse_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: ellipse_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: EllipseAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: EllipseAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(EllipseAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(EllipseAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(EllipseAttrs::Class)
	}

	/// Set the `cx` attribute.
	pub fn with_cx<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(EllipseAttrs::Cx, value);
		self
	}

	/// Set the `cx` attribute.
	pub fn set_cx<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(EllipseAttrs::Cx, value);
	}

	/// Get the `cx` attribute.
	pub fn cx(&self) -> Option<&dyn Value> {
		self.get_attr(EllipseAttrs::Cx)
	}

	/// Set the `cy` attribute.
	pub fn with_cy<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(EllipseAttrs::Cy, value);
		self
	}

	/// Set the `cy` attribute.
	pub fn set_cy<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(EllipseAttrs::Cy, value);
	}

	/// Get the `cy` attribute.
	pub fn cy(&self) -> Option<&dyn Value> {
		self.get_attr(EllipseAttrs::Cy)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(EllipseAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(EllipseAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(EllipseAttrs::ExternalResourcesRequired)
	}

	/// Set the `rx` attribute.
	pub fn with_rx<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(EllipseAttrs::Rx, value);
		self
	}

	/// Set the `rx` attribute.
	pub fn set_rx<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(EllipseAttrs::Rx, value);
	}

	/// Get the `rx` attribute.
	pub fn rx(&self) -> Option<&dyn Value> {
		self.get_attr(EllipseAttrs::Rx)
	}

	/// Set the `ry` attribute.
	pub fn with_ry<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(EllipseAttrs::Ry, value);
		self
	}

	/// Set the `ry` attribute.
	pub fn set_ry<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(EllipseAttrs::Ry, value);
	}

	/// Get the `ry` attribute.
	pub fn ry(&self) -> Option<&dyn Value> {
		self.get_attr(EllipseAttrs::Ry)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(EllipseAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(EllipseAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(EllipseAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(EllipseAttrs::Transform, value);
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(EllipseAttrs::Transform, value);
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&dyn Value> {
		self.get_attr(EllipseAttrs::Transform)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Ellipse {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(EllipseAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(EllipseAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Ellipse {}

impl common_attrs::CoreAttributesSetter for Ellipse {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(EllipseAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(EllipseAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Ellipse {}

impl common_attrs::GlobalEventAttributesSetter for Ellipse {
	fn set_attr<V>(&mut self, attr: common_attrs::GlobalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(EllipseAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GlobalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(EllipseAttrs::from(attr))
	}
}

impl TagWithGlobalEventAttributes for Ellipse {}

impl common_attrs::GraphicalEventAttributesSetter for Ellipse {
	fn set_attr<V>(&mut self, attr: common_attrs::GraphicalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(EllipseAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(EllipseAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Ellipse {}

impl common_attrs::PresentationAttributesSetter for Ellipse {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(EllipseAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(EllipseAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Ellipse {}

impl Tag for Ellipse {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("ellipse");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeBlendAttrs {
	Class,
	In,
	In2,
	Mode,
	Style,
	CoreAttributes(common_attrs::CoreAttributes),
	FilterAttributes(common_attrs::FilterAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::CoreAttributes> for FeBlendAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::FilterAttributes> for FeBlendAttrs {
	fn from(attr: common_attrs::FilterAttributes) -> Self {
		Self::FilterAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for FeBlendAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl FeBlendAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::In => "in",
			Self::In2 => "in2",
			Self::Mode => "mode",
			Self::Style => "style",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::FilterAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FeBlendAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FeBlendAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod fe_blend_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feBlend>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feBlend.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<set>`](Set)\n"
)]
#[doc = "\n\n [`<feBlend>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feBlend"]
#[derive(Debug)]
pub struct FeBlend {
	attrs: IndexMap<FeBlendAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn fe_blend_private::Content>>,
}

impl Default for FeBlend {
	fn default() -> Self {
		Self::new()
	}
}

impl FeBlend {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: fe_blend_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: fe_blend_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: FeBlendAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FeBlendAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeBlendAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeBlendAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(FeBlendAttrs::Class)
	}

	/// Set the `in` attribute.
	pub fn with_in1<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeBlendAttrs::In, value);
		self
	}

	/// Set the `in` attribute.
	pub fn set_in1<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeBlendAttrs::In, value);
	}

	/// Get the `in` attribute.
	pub fn in1(&self) -> Option<&dyn Value> {
		self.get_attr(FeBlendAttrs::In)
	}

	/// Set the `in2` attribute.
	pub fn with_in2<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeBlendAttrs::In2, value);
		self
	}

	/// Set the `in2` attribute.
	pub fn set_in2<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeBlendAttrs::In2, value);
	}

	/// Get the `in2` attribute.
	pub fn in2(&self) -> Option<&dyn Value> {
		self.get_attr(FeBlendAttrs::In2)
	}

	/// Set the `mode` attribute.
	pub fn with_mode<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeBlendAttrs::Mode, value);
		self
	}

	/// Set the `mode` attribute.
	pub fn set_mode<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeBlendAttrs::Mode, value);
	}

	/// Get the `mode` attribute.
	pub fn mode(&self) -> Option<&dyn Value> {
		self.get_attr(FeBlendAttrs::Mode)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeBlendAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeBlendAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(FeBlendAttrs::Style)
	}
}

impl common_attrs::CoreAttributesSetter for FeBlend {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeBlendAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FeBlendAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeBlend {}

impl common_attrs::FilterAttributesSetter for FeBlend {
	fn set_attr<V>(&mut self, attr: common_attrs::FilterAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeBlendAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&dyn Value> {
		self.get_attr(FeBlendAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeBlend {}

impl common_attrs::PresentationAttributesSetter for FeBlend {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeBlendAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(FeBlendAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeBlend {}

impl Tag for FeBlend {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("feBlend");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeColorMatrixAttrs {
	Class,
	In,
	Style,
	Type,
	Values,
	CoreAttributes(common_attrs::CoreAttributes),
	FilterAttributes(common_attrs::FilterAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::CoreAttributes> for FeColorMatrixAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::FilterAttributes> for FeColorMatrixAttrs {
	fn from(attr: common_attrs::FilterAttributes) -> Self {
		Self::FilterAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for FeColorMatrixAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl FeColorMatrixAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::In => "in",
			Self::Style => "style",
			Self::Type => "type",
			Self::Values => "values",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::FilterAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FeColorMatrixAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FeColorMatrixAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod fe_color_matrix_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feColorMatrix>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feColorMatrix.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<set>`](Set)\n"
)]
#[doc = "\n\n [`<feColorMatrix>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feColorMatrix"]
#[derive(Debug)]
pub struct FeColorMatrix {
	attrs: IndexMap<FeColorMatrixAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn fe_color_matrix_private::Content>>,
}

impl Default for FeColorMatrix {
	fn default() -> Self {
		Self::new()
	}
}

impl FeColorMatrix {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: fe_color_matrix_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: fe_color_matrix_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: FeColorMatrixAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FeColorMatrixAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeColorMatrixAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeColorMatrixAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(FeColorMatrixAttrs::Class)
	}

	/// Set the `in` attribute.
	pub fn with_in1<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeColorMatrixAttrs::In, value);
		self
	}

	/// Set the `in` attribute.
	pub fn set_in1<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeColorMatrixAttrs::In, value);
	}

	/// Get the `in` attribute.
	pub fn in1(&self) -> Option<&dyn Value> {
		self.get_attr(FeColorMatrixAttrs::In)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeColorMatrixAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeColorMatrixAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(FeColorMatrixAttrs::Style)
	}

	/// Set the `type` attribute.
	pub fn with_ty<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeColorMatrixAttrs::Type, value);
		self
	}

	/// Set the `type` attribute.
	pub fn set_ty<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeColorMatrixAttrs::Type, value);
	}

	/// Get the `type` attribute.
	pub fn ty(&self) -> Option<&dyn Value> {
		self.get_attr(FeColorMatrixAttrs::Type)
	}

	/// Set the `values` attribute.
	pub fn with_values<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeColorMatrixAttrs::Values, value);
		self
	}

	/// Set the `values` attribute.
	pub fn set_values<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeColorMatrixAttrs::Values, value);
	}

	/// Get the `values` attribute.
	pub fn values(&self) -> Option<&dyn Value> {
		self.get_attr(FeColorMatrixAttrs::Values)
	}
}

impl common_attrs::CoreAttributesSetter for FeColorMatrix {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeColorMatrixAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FeColorMatrixAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeColorMatrix {}

impl common_attrs::FilterAttributesSetter for FeColorMatrix {
	fn set_attr<V>(&mut self, attr: common_attrs::FilterAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeColorMatrixAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&dyn Value> {
		self.get_attr(FeColorMatrixAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeColorMatrix {}

impl common_attrs::PresentationAttributesSetter for FeColorMatrix {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeColorMatrixAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(FeColorMatrixAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeColorMatrix {}

impl Tag for FeColorMatrix {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("feColorMatrix");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeComponentTransferAttrs {
	Class,
	In,
	Style,
	CoreAttributes(common_attrs::CoreAttributes),
	FilterAttributes(common_attrs::FilterAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::CoreAttributes> for FeComponentTransferAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::FilterAttributes> for FeComponentTransferAttrs {
	fn from(attr: common_attrs::FilterAttributes) -> Self {
		Self::FilterAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for FeComponentTransferAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl FeComponentTransferAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::In => "in",
			Self::Style => "style",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::FilterAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FeComponentTransferAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FeComponentTransferAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod fe_component_transfer_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::FeFuncA {}
	impl Content for super::FeFuncB {}
	impl Content for super::FeFuncG {}
	impl Content for super::FeFuncR {}
}

#[doc = "The [`<feComponentTransfer>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feComponentTransfer.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<feFuncA>`](FeFuncA)\n",
	"- [`<feFuncB>`](FeFuncB)\n",
	"- [`<feFuncG>`](FeFuncG)\n",
	"- [`<feFuncR>`](FeFuncR)\n"
)]
#[doc = "\n\n [`<feComponentTransfer>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feComponentTransfer"]
#[derive(Debug)]
pub struct FeComponentTransfer {
	attrs: IndexMap<FeComponentTransferAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn fe_component_transfer_private::Content>>,
}

impl Default for FeComponentTransfer {
	fn default() -> Self {
		Self::new()
	}
}

impl FeComponentTransfer {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: fe_component_transfer_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: fe_component_transfer_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: FeComponentTransferAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FeComponentTransferAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeComponentTransferAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeComponentTransferAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(FeComponentTransferAttrs::Class)
	}

	/// Set the `in` attribute.
	pub fn with_in1<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeComponentTransferAttrs::In, value);
		self
	}

	/// Set the `in` attribute.
	pub fn set_in1<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeComponentTransferAttrs::In, value);
	}

	/// Get the `in` attribute.
	pub fn in1(&self) -> Option<&dyn Value> {
		self.get_attr(FeComponentTransferAttrs::In)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeComponentTransferAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeComponentTransferAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(FeComponentTransferAttrs::Style)
	}
}

impl common_attrs::CoreAttributesSetter for FeComponentTransfer {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeComponentTransferAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FeComponentTransferAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeComponentTransfer {}

impl common_attrs::FilterAttributesSetter for FeComponentTransfer {
	fn set_attr<V>(&mut self, attr: common_attrs::FilterAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeComponentTransferAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&dyn Value> {
		self.get_attr(FeComponentTransferAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeComponentTransfer {}

impl common_attrs::PresentationAttributesSetter for FeComponentTransfer {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeComponentTransferAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(FeComponentTransferAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeComponentTransfer {}

impl Tag for FeComponentTransfer {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("feComponentTransfer");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeCompositeAttrs {
	Class,
	In,
	In2,
	K1,
	K2,
	K3,
	K4,
	Operator,
	Style,
	CoreAttributes(common_attrs::CoreAttributes),
	FilterAttributes(common_attrs::FilterAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::CoreAttributes> for FeCompositeAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::FilterAttributes> for FeCompositeAttrs {
	fn from(attr: common_attrs::FilterAttributes) -> Self {
		Self::FilterAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for FeCompositeAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl FeCompositeAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::In => "in",
			Self::In2 => "in2",
			Self::K1 => "k1",
			Self::K2 => "k2",
			Self::K3 => "k3",
			Self::K4 => "k4",
			Self::Operator => "operator",
			Self::Style => "style",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::FilterAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FeCompositeAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FeCompositeAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod fe_composite_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feComposite>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feComposite.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<set>`](Set)\n"
)]
#[doc = "\n\n [`<feComposite>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feComposite"]
#[derive(Debug)]
pub struct FeComposite {
	attrs: IndexMap<FeCompositeAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn fe_composite_private::Content>>,
}

impl Default for FeComposite {
	fn default() -> Self {
		Self::new()
	}
}

impl FeComposite {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: fe_composite_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: fe_composite_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: FeCompositeAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FeCompositeAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeCompositeAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeCompositeAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(FeCompositeAttrs::Class)
	}

	/// Set the `in` attribute.
	pub fn with_in1<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeCompositeAttrs::In, value);
		self
	}

	/// Set the `in` attribute.
	pub fn set_in1<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeCompositeAttrs::In, value);
	}

	/// Get the `in` attribute.
	pub fn in1(&self) -> Option<&dyn Value> {
		self.get_attr(FeCompositeAttrs::In)
	}

	/// Set the `in2` attribute.
	pub fn with_in2<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeCompositeAttrs::In2, value);
		self
	}

	/// Set the `in2` attribute.
	pub fn set_in2<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeCompositeAttrs::In2, value);
	}

	/// Get the `in2` attribute.
	pub fn in2(&self) -> Option<&dyn Value> {
		self.get_attr(FeCompositeAttrs::In2)
	}

	/// Set the `k1` attribute.
	pub fn with_k1<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeCompositeAttrs::K1, value);
		self
	}

	/// Set the `k1` attribute.
	pub fn set_k1<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeCompositeAttrs::K1, value);
	}

	/// Get the `k1` attribute.
	pub fn k1(&self) -> Option<&dyn Value> {
		self.get_attr(FeCompositeAttrs::K1)
	}

	/// Set the `k2` attribute.
	pub fn with_k2<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeCompositeAttrs::K2, value);
		self
	}

	/// Set the `k2` attribute.
	pub fn set_k2<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeCompositeAttrs::K2, value);
	}

	/// Get the `k2` attribute.
	pub fn k2(&self) -> Option<&dyn Value> {
		self.get_attr(FeCompositeAttrs::K2)
	}

	/// Set the `k3` attribute.
	pub fn with_k3<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeCompositeAttrs::K3, value);
		self
	}

	/// Set the `k3` attribute.
	pub fn set_k3<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeCompositeAttrs::K3, value);
	}

	/// Get the `k3` attribute.
	pub fn k3(&self) -> Option<&dyn Value> {
		self.get_attr(FeCompositeAttrs::K3)
	}

	/// Set the `k4` attribute.
	pub fn with_k4<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeCompositeAttrs::K4, value);
		self
	}

	/// Set the `k4` attribute.
	pub fn set_k4<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeCompositeAttrs::K4, value);
	}

	/// Get the `k4` attribute.
	pub fn k4(&self) -> Option<&dyn Value> {
		self.get_attr(FeCompositeAttrs::K4)
	}

	/// Set the `operator` attribute.
	pub fn with_operator<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeCompositeAttrs::Operator, value);
		self
	}

	/// Set the `operator` attribute.
	pub fn set_operator<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeCompositeAttrs::Operator, value);
	}

	/// Get the `operator` attribute.
	pub fn operator(&self) -> Option<&dyn Value> {
		self.get_attr(FeCompositeAttrs::Operator)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeCompositeAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeCompositeAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(FeCompositeAttrs::Style)
	}
}

impl common_attrs::CoreAttributesSetter for FeComposite {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeCompositeAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FeCompositeAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeComposite {}

impl common_attrs::FilterAttributesSetter for FeComposite {
	fn set_attr<V>(&mut self, attr: common_attrs::FilterAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeCompositeAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&dyn Value> {
		self.get_attr(FeCompositeAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeComposite {}

impl common_attrs::PresentationAttributesSetter for FeComposite {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeCompositeAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(FeCompositeAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeComposite {}

impl Tag for FeComposite {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("feComposite");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeConvolveMatrixAttrs {
	Bias,
	Class,
	Divisor,
	EdgeMode,
	In,
	KernelMatrix,
	KernelUnitLength,
	Order,
	PreserveAlpha,
	Style,
	TargetX,
	TargetY,
	CoreAttributes(common_attrs::CoreAttributes),
	FilterAttributes(common_attrs::FilterAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::CoreAttributes> for FeConvolveMatrixAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::FilterAttributes> for FeConvolveMatrixAttrs {
	fn from(attr: common_attrs::FilterAttributes) -> Self {
		Self::FilterAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for FeConvolveMatrixAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl FeConvolveMatrixAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Bias => "bias",
			Self::Class => "class",
			Self::Divisor => "divisor",
			Self::EdgeMode => "edgeMode",
			Self::In => "in",
			Self::KernelMatrix => "kernelMatrix",
			Self::KernelUnitLength => "kernelUnitLength",
			Self::Order => "order",
			Self::PreserveAlpha => "preserveAlpha",
			Self::Style => "style",
			Self::TargetX => "targetX",
			Self::TargetY => "targetY",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::FilterAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FeConvolveMatrixAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FeConvolveMatrixAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod fe_convolve_matrix_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feConvolveMatrix>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feConvolveMatrix.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<set>`](Set)\n"
)]
#[doc = "\n\n [`<feConvolveMatrix>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feConvolveMatrix"]
#[derive(Debug)]
pub struct FeConvolveMatrix {
	attrs: IndexMap<FeConvolveMatrixAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn fe_convolve_matrix_private::Content>>,
}

impl Default for FeConvolveMatrix {
	fn default() -> Self {
		Self::new()
	}
}

impl FeConvolveMatrix {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: fe_convolve_matrix_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: fe_convolve_matrix_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: FeConvolveMatrixAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FeConvolveMatrixAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `bias` attribute.
	pub fn with_bias<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeConvolveMatrixAttrs::Bias, value);
		self
	}

	/// Set the `bias` attribute.
	pub fn set_bias<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeConvolveMatrixAttrs::Bias, value);
	}

	/// Get the `bias` attribute.
	pub fn bias(&self) -> Option<&dyn Value> {
		self.get_attr(FeConvolveMatrixAttrs::Bias)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeConvolveMatrixAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeConvolveMatrixAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(FeConvolveMatrixAttrs::Class)
	}

	/// Set the `divisor` attribute.
	pub fn with_divisor<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeConvolveMatrixAttrs::Divisor, value);
		self
	}

	/// Set the `divisor` attribute.
	pub fn set_divisor<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeConvolveMatrixAttrs::Divisor, value);
	}

	/// Get the `divisor` attribute.
	pub fn divisor(&self) -> Option<&dyn Value> {
		self.get_attr(FeConvolveMatrixAttrs::Divisor)
	}

	/// Set the `edgeMode` attribute.
	pub fn with_edge_mode<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeConvolveMatrixAttrs::EdgeMode, value);
		self
	}

	/// Set the `edgeMode` attribute.
	pub fn set_edge_mode<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeConvolveMatrixAttrs::EdgeMode, value);
	}

	/// Get the `edgeMode` attribute.
	pub fn edge_mode(&self) -> Option<&dyn Value> {
		self.get_attr(FeConvolveMatrixAttrs::EdgeMode)
	}

	/// Set the `in` attribute.
	pub fn with_in1<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeConvolveMatrixAttrs::In, value);
		self
	}

	/// Set the `in` attribute.
	pub fn set_in1<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeConvolveMatrixAttrs::In, value);
	}

	/// Get the `in` attribute.
	pub fn in1(&self) -> Option<&dyn Value> {
		self.get_attr(FeConvolveMatrixAttrs::In)
	}

	/// Set the `kernelMatrix` attribute.
	pub fn with_kernel_matrix<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeConvolveMatrixAttrs::KernelMatrix, value);
		self
	}

	/// Set the `kernelMatrix` attribute.
	pub fn set_kernel_matrix<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeConvolveMatrixAttrs::KernelMatrix, value);
	}

	/// Get the `kernelMatrix` attribute.
	pub fn kernel_matrix(&self) -> Option<&dyn Value> {
		self.get_attr(FeConvolveMatrixAttrs::KernelMatrix)
	}

	/// Set the `kernelUnitLength` attribute.
	pub fn with_kernel_unit_length<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeConvolveMatrixAttrs::KernelUnitLength, value);
		self
	}

	/// Set the `kernelUnitLength` attribute.
	pub fn set_kernel_unit_length<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeConvolveMatrixAttrs::KernelUnitLength, value);
	}

	/// Get the `kernelUnitLength` attribute.
	pub fn kernel_unit_length(&self) -> Option<&dyn Value> {
		self.get_attr(FeConvolveMatrixAttrs::KernelUnitLength)
	}

	/// Set the `order` attribute.
	pub fn with_order<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeConvolveMatrixAttrs::Order, value);
		self
	}

	/// Set the `order` attribute.
	pub fn set_order<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeConvolveMatrixAttrs::Order, value);
	}

	/// Get the `order` attribute.
	pub fn order(&self) -> Option<&dyn Value> {
		self.get_attr(FeConvolveMatrixAttrs::Order)
	}

	/// Set the `preserveAlpha` attribute.
	pub fn with_preserve_alpha<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeConvolveMatrixAttrs::PreserveAlpha, value);
		self
	}

	/// Set the `preserveAlpha` attribute.
	pub fn set_preserve_alpha<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeConvolveMatrixAttrs::PreserveAlpha, value);
	}

	/// Get the `preserveAlpha` attribute.
	pub fn preserve_alpha(&self) -> Option<&dyn Value> {
		self.get_attr(FeConvolveMatrixAttrs::PreserveAlpha)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeConvolveMatrixAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeConvolveMatrixAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(FeConvolveMatrixAttrs::Style)
	}

	/// Set the `targetX` attribute.
	pub fn with_target_x<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeConvolveMatrixAttrs::TargetX, value);
		self
	}

	/// Set the `targetX` attribute.
	pub fn set_target_x<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeConvolveMatrixAttrs::TargetX, value);
	}

	/// Get the `targetX` attribute.
	pub fn target_x(&self) -> Option<&dyn Value> {
		self.get_attr(FeConvolveMatrixAttrs::TargetX)
	}

	/// Set the `targetY` attribute.
	pub fn with_target_y<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeConvolveMatrixAttrs::TargetY, value);
		self
	}

	/// Set the `targetY` attribute.
	pub fn set_target_y<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeConvolveMatrixAttrs::TargetY, value);
	}

	/// Get the `targetY` attribute.
	pub fn target_y(&self) -> Option<&dyn Value> {
		self.get_attr(FeConvolveMatrixAttrs::TargetY)
	}
}

impl common_attrs::CoreAttributesSetter for FeConvolveMatrix {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeConvolveMatrixAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FeConvolveMatrixAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeConvolveMatrix {}

impl common_attrs::FilterAttributesSetter for FeConvolveMatrix {
	fn set_attr<V>(&mut self, attr: common_attrs::FilterAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeConvolveMatrixAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&dyn Value> {
		self.get_attr(FeConvolveMatrixAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeConvolveMatrix {}

impl common_attrs::PresentationAttributesSetter for FeConvolveMatrix {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeConvolveMatrixAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(FeConvolveMatrixAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeConvolveMatrix {}

impl Tag for FeConvolveMatrix {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("feConvolveMatrix");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeDiffuseLightingAttrs {
	Class,
	DiffuseConstant,
	In,
	KernelUnitLength,
	Style,
	SurfaceScale,
	CoreAttributes(common_attrs::CoreAttributes),
	FilterAttributes(common_attrs::FilterAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::CoreAttributes> for FeDiffuseLightingAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::FilterAttributes> for FeDiffuseLightingAttrs {
	fn from(attr: common_attrs::FilterAttributes) -> Self {
		Self::FilterAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for FeDiffuseLightingAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl FeDiffuseLightingAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::DiffuseConstant => "diffuseConstant",
			Self::In => "in",
			Self::KernelUnitLength => "kernelUnitLength",
			Self::Style => "style",
			Self::SurfaceScale => "surfaceScale",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::FilterAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FeDiffuseLightingAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FeDiffuseLightingAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[doc = "The [`<feDiffuseLighting>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feDiffuseLighting.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n"
)]
#[doc = "\n\n [`<feDiffuseLighting>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feDiffuseLighting"]
#[derive(Debug)]
pub struct FeDiffuseLighting {
	attrs: IndexMap<FeDiffuseLightingAttrs, Box<dyn Value>>,
}

impl Default for FeDiffuseLighting {
	fn default() -> Self {
		Self::new()
	}
}

impl FeDiffuseLighting {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
		}
	}

	fn set_attr<V>(&mut self, attr: FeDiffuseLightingAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FeDiffuseLightingAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeDiffuseLightingAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDiffuseLightingAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(FeDiffuseLightingAttrs::Class)
	}

	/// Set the `diffuseConstant` attribute.
	pub fn with_diffuse_constant<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeDiffuseLightingAttrs::DiffuseConstant, value);
		self
	}

	/// Set the `diffuseConstant` attribute.
	pub fn set_diffuse_constant<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDiffuseLightingAttrs::DiffuseConstant, value);
	}

	/// Get the `diffuseConstant` attribute.
	pub fn diffuse_constant(&self) -> Option<&dyn Value> {
		self.get_attr(FeDiffuseLightingAttrs::DiffuseConstant)
	}

	/// Set the `in` attribute.
	pub fn with_in1<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeDiffuseLightingAttrs::In, value);
		self
	}

	/// Set the `in` attribute.
	pub fn set_in1<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDiffuseLightingAttrs::In, value);
	}

	/// Get the `in` attribute.
	pub fn in1(&self) -> Option<&dyn Value> {
		self.get_attr(FeDiffuseLightingAttrs::In)
	}

	/// Set the `kernelUnitLength` attribute.
	pub fn with_kernel_unit_length<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeDiffuseLightingAttrs::KernelUnitLength, value);
		self
	}

	/// Set the `kernelUnitLength` attribute.
	pub fn set_kernel_unit_length<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDiffuseLightingAttrs::KernelUnitLength, value);
	}

	/// Get the `kernelUnitLength` attribute.
	pub fn kernel_unit_length(&self) -> Option<&dyn Value> {
		self.get_attr(FeDiffuseLightingAttrs::KernelUnitLength)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeDiffuseLightingAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDiffuseLightingAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(FeDiffuseLightingAttrs::Style)
	}

	/// Set the `surfaceScale` attribute.
	pub fn with_surface_scale<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeDiffuseLightingAttrs::SurfaceScale, value);
		self
	}

	/// Set the `surfaceScale` attribute.
	pub fn set_surface_scale<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDiffuseLightingAttrs::SurfaceScale, value);
	}

	/// Get the `surfaceScale` attribute.
	pub fn surface_scale(&self) -> Option<&dyn Value> {
		self.get_attr(FeDiffuseLightingAttrs::SurfaceScale)
	}
}

impl common_attrs::CoreAttributesSetter for FeDiffuseLighting {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDiffuseLightingAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FeDiffuseLightingAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeDiffuseLighting {}

impl common_attrs::FilterAttributesSetter for FeDiffuseLighting {
	fn set_attr<V>(&mut self, attr: common_attrs::FilterAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDiffuseLightingAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&dyn Value> {
		self.get_attr(FeDiffuseLightingAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeDiffuseLighting {}

impl common_attrs::PresentationAttributesSetter for FeDiffuseLighting {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDiffuseLightingAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(FeDiffuseLightingAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeDiffuseLighting {}

impl Tag for FeDiffuseLighting {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("feDiffuseLighting");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeDisplacementMapAttrs {
	Class,
	In,
	In2,
	Scale,
	Style,
	XChannelSelector,
	YChannelSelector,
	CoreAttributes(common_attrs::CoreAttributes),
	FilterAttributes(common_attrs::FilterAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::CoreAttributes> for FeDisplacementMapAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::FilterAttributes> for FeDisplacementMapAttrs {
	fn from(attr: common_attrs::FilterAttributes) -> Self {
		Self::FilterAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for FeDisplacementMapAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl FeDisplacementMapAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::In => "in",
			Self::In2 => "in2",
			Self::Scale => "scale",
			Self::Style => "style",
			Self::XChannelSelector => "xChannelSelector",
			Self::YChannelSelector => "yChannelSelector",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::FilterAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FeDisplacementMapAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FeDisplacementMapAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod fe_displacement_map_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feDisplacementMap>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feDisplacementMap.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<set>`](Set)\n"
)]
#[doc = "\n\n [`<feDisplacementMap>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feDisplacementMap"]
#[derive(Debug)]
pub struct FeDisplacementMap {
	attrs: IndexMap<FeDisplacementMapAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn fe_displacement_map_private::Content>>,
}

impl Default for FeDisplacementMap {
	fn default() -> Self {
		Self::new()
	}
}

impl FeDisplacementMap {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: fe_displacement_map_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: fe_displacement_map_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: FeDisplacementMapAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FeDisplacementMapAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeDisplacementMapAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDisplacementMapAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(FeDisplacementMapAttrs::Class)
	}

	/// Set the `in` attribute.
	pub fn with_in1<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeDisplacementMapAttrs::In, value);
		self
	}

	/// Set the `in` attribute.
	pub fn set_in1<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDisplacementMapAttrs::In, value);
	}

	/// Get the `in` attribute.
	pub fn in1(&self) -> Option<&dyn Value> {
		self.get_attr(FeDisplacementMapAttrs::In)
	}

	/// Set the `in2` attribute.
	pub fn with_in2<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeDisplacementMapAttrs::In2, value);
		self
	}

	/// Set the `in2` attribute.
	pub fn set_in2<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDisplacementMapAttrs::In2, value);
	}

	/// Get the `in2` attribute.
	pub fn in2(&self) -> Option<&dyn Value> {
		self.get_attr(FeDisplacementMapAttrs::In2)
	}

	/// Set the `scale` attribute.
	pub fn with_scale<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeDisplacementMapAttrs::Scale, value);
		self
	}

	/// Set the `scale` attribute.
	pub fn set_scale<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDisplacementMapAttrs::Scale, value);
	}

	/// Get the `scale` attribute.
	pub fn scale(&self) -> Option<&dyn Value> {
		self.get_attr(FeDisplacementMapAttrs::Scale)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeDisplacementMapAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDisplacementMapAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(FeDisplacementMapAttrs::Style)
	}

	/// Set the `xChannelSelector` attribute.
	pub fn with_x_channel_selector<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeDisplacementMapAttrs::XChannelSelector, value);
		self
	}

	/// Set the `xChannelSelector` attribute.
	pub fn set_x_channel_selector<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDisplacementMapAttrs::XChannelSelector, value);
	}

	/// Get the `xChannelSelector` attribute.
	pub fn x_channel_selector(&self) -> Option<&dyn Value> {
		self.get_attr(FeDisplacementMapAttrs::XChannelSelector)
	}

	/// Set the `yChannelSelector` attribute.
	pub fn with_y_channel_selector<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeDisplacementMapAttrs::YChannelSelector, value);
		self
	}

	/// Set the `yChannelSelector` attribute.
	pub fn set_y_channel_selector<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDisplacementMapAttrs::YChannelSelector, value);
	}

	/// Get the `yChannelSelector` attribute.
	pub fn y_channel_selector(&self) -> Option<&dyn Value> {
		self.get_attr(FeDisplacementMapAttrs::YChannelSelector)
	}
}

impl common_attrs::CoreAttributesSetter for FeDisplacementMap {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDisplacementMapAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FeDisplacementMapAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeDisplacementMap {}

impl common_attrs::FilterAttributesSetter for FeDisplacementMap {
	fn set_attr<V>(&mut self, attr: common_attrs::FilterAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDisplacementMapAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&dyn Value> {
		self.get_attr(FeDisplacementMapAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeDisplacementMap {}

impl common_attrs::PresentationAttributesSetter for FeDisplacementMap {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDisplacementMapAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(FeDisplacementMapAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeDisplacementMap {}

impl Tag for FeDisplacementMap {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("feDisplacementMap");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeDistantLightAttrs {
	Azimuth,
	Elevation,
	CoreAttributes(common_attrs::CoreAttributes),
}

impl From<common_attrs::CoreAttributes> for FeDistantLightAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl FeDistantLightAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Azimuth => "azimuth",
			Self::Elevation => "elevation",
			Self::CoreAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FeDistantLightAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FeDistantLightAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod fe_distant_light_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feDistantLight>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feDistantLight.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<set>`](Set)\n"
)]
#[doc = "\n\n [`<feDistantLight>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feDistantLight"]
#[derive(Debug)]
pub struct FeDistantLight {
	attrs: IndexMap<FeDistantLightAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn fe_distant_light_private::Content>>,
}

impl Default for FeDistantLight {
	fn default() -> Self {
		Self::new()
	}
}

impl FeDistantLight {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: fe_distant_light_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: fe_distant_light_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: FeDistantLightAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FeDistantLightAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `azimuth` attribute.
	pub fn with_azimuth<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeDistantLightAttrs::Azimuth, value);
		self
	}

	/// Set the `azimuth` attribute.
	pub fn set_azimuth<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDistantLightAttrs::Azimuth, value);
	}

	/// Get the `azimuth` attribute.
	pub fn azimuth(&self) -> Option<&dyn Value> {
		self.get_attr(FeDistantLightAttrs::Azimuth)
	}

	/// Set the `elevation` attribute.
	pub fn with_elevation<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeDistantLightAttrs::Elevation, value);
		self
	}

	/// Set the `elevation` attribute.
	pub fn set_elevation<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDistantLightAttrs::Elevation, value);
	}

	/// Get the `elevation` attribute.
	pub fn elevation(&self) -> Option<&dyn Value> {
		self.get_attr(FeDistantLightAttrs::Elevation)
	}
}

impl common_attrs::CoreAttributesSetter for FeDistantLight {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDistantLightAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FeDistantLightAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeDistantLight {}

impl Tag for FeDistantLight {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("feDistantLight");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeDropShadowAttrs {
	Class,
	Dx,
	Dy,
	In,
	StdDeviation,
	Style,
	CoreAttributes(common_attrs::CoreAttributes),
	FilterAttributes(common_attrs::FilterAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::CoreAttributes> for FeDropShadowAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::FilterAttributes> for FeDropShadowAttrs {
	fn from(attr: common_attrs::FilterAttributes) -> Self {
		Self::FilterAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for FeDropShadowAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl FeDropShadowAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::Dx => "dx",
			Self::Dy => "dy",
			Self::In => "in",
			Self::StdDeviation => "stdDeviation",
			Self::Style => "style",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::FilterAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FeDropShadowAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FeDropShadowAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod fe_drop_shadow_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Script {}
	impl Content for super::Set {}
}

#[doc = "The [`<feDropShadow>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feDropShadow.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<script>`](Script)\n",
	"- [`<set>`](Set)\n"
)]
#[doc = "\n\n [`<feDropShadow>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feDropShadow"]
#[derive(Debug)]
pub struct FeDropShadow {
	attrs: IndexMap<FeDropShadowAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn fe_drop_shadow_private::Content>>,
}

impl Default for FeDropShadow {
	fn default() -> Self {
		Self::new()
	}
}

impl FeDropShadow {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: fe_drop_shadow_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: fe_drop_shadow_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: FeDropShadowAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FeDropShadowAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeDropShadowAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDropShadowAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(FeDropShadowAttrs::Class)
	}

	/// Set the `dx` attribute.
	pub fn with_dx<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeDropShadowAttrs::Dx, value);
		self
	}

	/// Set the `dx` attribute.
	pub fn set_dx<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDropShadowAttrs::Dx, value);
	}

	/// Get the `dx` attribute.
	pub fn dx(&self) -> Option<&dyn Value> {
		self.get_attr(FeDropShadowAttrs::Dx)
	}

	/// Set the `dy` attribute.
	pub fn with_dy<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeDropShadowAttrs::Dy, value);
		self
	}

	/// Set the `dy` attribute.
	pub fn set_dy<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDropShadowAttrs::Dy, value);
	}

	/// Get the `dy` attribute.
	pub fn dy(&self) -> Option<&dyn Value> {
		self.get_attr(FeDropShadowAttrs::Dy)
	}

	/// Set the `in` attribute.
	pub fn with_in1<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeDropShadowAttrs::In, value);
		self
	}

	/// Set the `in` attribute.
	pub fn set_in1<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDropShadowAttrs::In, value);
	}

	/// Get the `in` attribute.
	pub fn in1(&self) -> Option<&dyn Value> {
		self.get_attr(FeDropShadowAttrs::In)
	}

	/// Set the `stdDeviation` attribute.
	pub fn with_std_deviation<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeDropShadowAttrs::StdDeviation, value);
		self
	}

	/// Set the `stdDeviation` attribute.
	pub fn set_std_deviation<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDropShadowAttrs::StdDeviation, value);
	}

	/// Get the `stdDeviation` attribute.
	pub fn std_deviation(&self) -> Option<&dyn Value> {
		self.get_attr(FeDropShadowAttrs::StdDeviation)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeDropShadowAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDropShadowAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(FeDropShadowAttrs::Style)
	}
}

impl common_attrs::CoreAttributesSetter for FeDropShadow {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDropShadowAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FeDropShadowAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeDropShadow {}

impl common_attrs::FilterAttributesSetter for FeDropShadow {
	fn set_attr<V>(&mut self, attr: common_attrs::FilterAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDropShadowAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&dyn Value> {
		self.get_attr(FeDropShadowAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeDropShadow {}

impl common_attrs::PresentationAttributesSetter for FeDropShadow {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeDropShadowAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(FeDropShadowAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeDropShadow {}

impl Tag for FeDropShadow {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("feDropShadow");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeFloodAttrs {
	Class,
	FloodColor,
	FloodOpacity,
	Style,
	CoreAttributes(common_attrs::CoreAttributes),
	FilterAttributes(common_attrs::FilterAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::CoreAttributes> for FeFloodAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::FilterAttributes> for FeFloodAttrs {
	fn from(attr: common_attrs::FilterAttributes) -> Self {
		Self::FilterAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for FeFloodAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl FeFloodAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::FloodColor => "flood-color",
			Self::FloodOpacity => "flood-opacity",
			Self::Style => "style",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::FilterAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FeFloodAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FeFloodAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod fe_flood_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::AnimateColor {}
	impl Content for super::Set {}
}

#[doc = "The [`<feFlood>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feFlood.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateColor>`](AnimateColor)\n",
	"- [`<set>`](Set)\n"
)]
#[doc = "\n\n [`<feFlood>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feFlood"]
#[derive(Debug)]
pub struct FeFlood {
	attrs: IndexMap<FeFloodAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn fe_flood_private::Content>>,
}

impl Default for FeFlood {
	fn default() -> Self {
		Self::new()
	}
}

impl FeFlood {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: fe_flood_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: fe_flood_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: FeFloodAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FeFloodAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeFloodAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeFloodAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(FeFloodAttrs::Class)
	}

	/// Set the `flood-color` attribute.
	pub fn with_flood_color<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeFloodAttrs::FloodColor, value);
		self
	}

	/// Set the `flood-color` attribute.
	pub fn set_flood_color<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeFloodAttrs::FloodColor, value);
	}

	/// Get the `flood-color` attribute.
	pub fn flood_color(&self) -> Option<&dyn Value> {
		self.get_attr(FeFloodAttrs::FloodColor)
	}

	/// Set the `flood-opacity` attribute.
	pub fn with_flood_opacity<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeFloodAttrs::FloodOpacity, value);
		self
	}

	/// Set the `flood-opacity` attribute.
	pub fn set_flood_opacity<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeFloodAttrs::FloodOpacity, value);
	}

	/// Get the `flood-opacity` attribute.
	pub fn flood_opacity(&self) -> Option<&dyn Value> {
		self.get_attr(FeFloodAttrs::FloodOpacity)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeFloodAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeFloodAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(FeFloodAttrs::Style)
	}
}

impl common_attrs::CoreAttributesSetter for FeFlood {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeFloodAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FeFloodAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeFlood {}

impl common_attrs::FilterAttributesSetter for FeFlood {
	fn set_attr<V>(&mut self, attr: common_attrs::FilterAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeFloodAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&dyn Value> {
		self.get_attr(FeFloodAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeFlood {}

impl common_attrs::PresentationAttributesSetter for FeFlood {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeFloodAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(FeFloodAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeFlood {}

impl Tag for FeFlood {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("feFlood");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeFuncAAttrs {
	CoreAttributes(common_attrs::CoreAttributes),
	TransferFunctionAttributes(common_attrs::TransferFunctionAttributes),
}

impl From<common_attrs::CoreAttributes> for FeFuncAAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::TransferFunctionAttributes> for FeFuncAAttrs {
	fn from(attr: common_attrs::TransferFunctionAttributes) -> Self {
		Self::TransferFunctionAttributes(attr)
	}
}

impl FeFuncAAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::TransferFunctionAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FeFuncAAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FeFuncAAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod fe_func_a_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feFuncA>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feFuncA.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<set>`](Set)\n"
)]
#[doc = "\n\n [`<feFuncA>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feFuncA"]
#[derive(Debug)]
pub struct FeFuncA {
	attrs: IndexMap<FeFuncAAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn fe_func_a_private::Content>>,
}

impl Default for FeFuncA {
	fn default() -> Self {
		Self::new()
	}
}

impl FeFuncA {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: fe_func_a_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: fe_func_a_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: FeFuncAAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FeFuncAAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}
}

impl common_attrs::CoreAttributesSetter for FeFuncA {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeFuncAAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FeFuncAAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeFuncA {}

impl common_attrs::TransferFunctionAttributesSetter for FeFuncA {
	fn set_attr<V>(&mut self, attr: common_attrs::TransferFunctionAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeFuncAAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::TransferFunctionAttributes) -> Option<&dyn Value> {
		self.get_attr(FeFuncAAttrs::from(attr))
	}
}

impl TagWithTransferFunctionAttributes for FeFuncA {}

impl Tag for FeFuncA {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("feFuncA");
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeFuncBAttrs {
	CoreAttributes(common_attrs::CoreAttributes),
	TransferFunctionAttributes(common_attrs::TransferFunctionAttributes),
}

impl From<common_attrs::CoreAttributes> for FeFuncBAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::TransferFunctionAttributes> for FeFuncBAttrs {
	fn from(attr: common_attrs::TransferFunctionAttributes) -> Self {
		Self::TransferFunctionAttributes(attr)
	}
}

impl FeFuncBAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::TransferFunctionAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FeFuncBAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FeFuncBAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod fe_func_b_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feFuncB>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feFuncB.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<set>`](Set)\n"
)]
#[doc = "\n\n [`<feFuncB>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feFuncB"]
#[derive(Debug)]
pub struct FeFuncB {
	attrs: IndexMap<FeFuncBAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn fe_func_b_private::Content>>,
}

impl Default for FeFuncB {
	fn default() -> Self {
		Self::new()
	}
}

impl FeFuncB {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: fe_func_b_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: fe_func_b_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: FeFuncBAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FeFuncBAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}
}

impl common_attrs::CoreAttributesSetter for FeFuncB {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeFuncBAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FeFuncBAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeFuncB {}

impl common_attrs::TransferFunctionAttributesSetter for FeFuncB {
	fn set_attr<V>(&mut self, attr: common_attrs::TransferFunctionAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeFuncBAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::TransferFunctionAttributes) -> Option<&dyn Value> {
		self.get_attr(FeFuncBAttrs::from(attr))
	}
}

impl TagWithTransferFunctionAttributes for FeFuncB {}

impl Tag for FeFuncB {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("feFuncB");
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeFuncGAttrs {
	CoreAttributes(common_attrs::CoreAttributes),
	TransferFunctionAttributes(common_attrs::TransferFunctionAttributes),
}

impl From<common_attrs::CoreAttributes> for FeFuncGAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::TransferFunctionAttributes> for FeFuncGAttrs {
	fn from(attr: common_attrs::TransferFunctionAttributes) -> Self {
		Self::TransferFunctionAttributes(attr)
	}
}

impl FeFuncGAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::TransferFunctionAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FeFuncGAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FeFuncGAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod fe_func_g_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feFuncG>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feFuncG.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<set>`](Set)\n"
)]
#[doc = "\n\n [`<feFuncG>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feFuncG"]
#[derive(Debug)]
pub struct FeFuncG {
	attrs: IndexMap<FeFuncGAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn fe_func_g_private::Content>>,
}

impl Default for FeFuncG {
	fn default() -> Self {
		Self::new()
	}
}

impl FeFuncG {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: fe_func_g_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: fe_func_g_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: FeFuncGAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FeFuncGAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}
}

impl common_attrs::CoreAttributesSetter for FeFuncG {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeFuncGAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FeFuncGAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeFuncG {}

impl common_attrs::TransferFunctionAttributesSetter for FeFuncG {
	fn set_attr<V>(&mut self, attr: common_attrs::TransferFunctionAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeFuncGAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::TransferFunctionAttributes) -> Option<&dyn Value> {
		self.get_attr(FeFuncGAttrs::from(attr))
	}
}

impl TagWithTransferFunctionAttributes for FeFuncG {}

impl Tag for FeFuncG {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("feFuncG");
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeFuncRAttrs {
	CoreAttributes(common_attrs::CoreAttributes),
	TransferFunctionAttributes(common_attrs::TransferFunctionAttributes),
}

impl From<common_attrs::CoreAttributes> for FeFuncRAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::TransferFunctionAttributes> for FeFuncRAttrs {
	fn from(attr: common_attrs::TransferFunctionAttributes) -> Self {
		Self::TransferFunctionAttributes(attr)
	}
}

impl FeFuncRAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::TransferFunctionAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FeFuncRAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FeFuncRAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod fe_func_r_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feFuncR>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feFuncR.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<set>`](Set)\n"
)]
#[doc = "\n\n [`<feFuncR>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feFuncR"]
#[derive(Debug)]
pub struct FeFuncR {
	attrs: IndexMap<FeFuncRAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn fe_func_r_private::Content>>,
}

impl Default for FeFuncR {
	fn default() -> Self {
		Self::new()
	}
}

impl FeFuncR {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: fe_func_r_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: fe_func_r_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: FeFuncRAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FeFuncRAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}
}

impl common_attrs::CoreAttributesSetter for FeFuncR {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeFuncRAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FeFuncRAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeFuncR {}

impl common_attrs::TransferFunctionAttributesSetter for FeFuncR {
	fn set_attr<V>(&mut self, attr: common_attrs::TransferFunctionAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeFuncRAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::TransferFunctionAttributes) -> Option<&dyn Value> {
		self.get_attr(FeFuncRAttrs::from(attr))
	}
}

impl TagWithTransferFunctionAttributes for FeFuncR {}

impl Tag for FeFuncR {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("feFuncR");
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeGaussianBlurAttrs {
	Class,
	In,
	StdDeviation,
	Style,
	CoreAttributes(common_attrs::CoreAttributes),
	FilterAttributes(common_attrs::FilterAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::CoreAttributes> for FeGaussianBlurAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::FilterAttributes> for FeGaussianBlurAttrs {
	fn from(attr: common_attrs::FilterAttributes) -> Self {
		Self::FilterAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for FeGaussianBlurAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl FeGaussianBlurAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::In => "in",
			Self::StdDeviation => "stdDeviation",
			Self::Style => "style",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::FilterAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FeGaussianBlurAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FeGaussianBlurAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod fe_gaussian_blur_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feGaussianBlur>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feGaussianBlur.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<set>`](Set)\n"
)]
#[doc = "\n\n [`<feGaussianBlur>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feGaussianBlur"]
#[derive(Debug)]
pub struct FeGaussianBlur {
	attrs: IndexMap<FeGaussianBlurAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn fe_gaussian_blur_private::Content>>,
}

impl Default for FeGaussianBlur {
	fn default() -> Self {
		Self::new()
	}
}

impl FeGaussianBlur {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: fe_gaussian_blur_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: fe_gaussian_blur_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: FeGaussianBlurAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FeGaussianBlurAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeGaussianBlurAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeGaussianBlurAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(FeGaussianBlurAttrs::Class)
	}

	/// Set the `in` attribute.
	pub fn with_in1<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeGaussianBlurAttrs::In, value);
		self
	}

	/// Set the `in` attribute.
	pub fn set_in1<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeGaussianBlurAttrs::In, value);
	}

	/// Get the `in` attribute.
	pub fn in1(&self) -> Option<&dyn Value> {
		self.get_attr(FeGaussianBlurAttrs::In)
	}

	/// Set the `stdDeviation` attribute.
	pub fn with_std_deviation<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeGaussianBlurAttrs::StdDeviation, value);
		self
	}

	/// Set the `stdDeviation` attribute.
	pub fn set_std_deviation<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeGaussianBlurAttrs::StdDeviation, value);
	}

	/// Get the `stdDeviation` attribute.
	pub fn std_deviation(&self) -> Option<&dyn Value> {
		self.get_attr(FeGaussianBlurAttrs::StdDeviation)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeGaussianBlurAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeGaussianBlurAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(FeGaussianBlurAttrs::Style)
	}
}

impl common_attrs::CoreAttributesSetter for FeGaussianBlur {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeGaussianBlurAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FeGaussianBlurAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeGaussianBlur {}

impl common_attrs::FilterAttributesSetter for FeGaussianBlur {
	fn set_attr<V>(&mut self, attr: common_attrs::FilterAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeGaussianBlurAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&dyn Value> {
		self.get_attr(FeGaussianBlurAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeGaussianBlur {}

impl common_attrs::PresentationAttributesSetter for FeGaussianBlur {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeGaussianBlurAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(FeGaussianBlurAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeGaussianBlur {}

impl Tag for FeGaussianBlur {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("feGaussianBlur");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeImageAttrs {
	Class,
	ExternalResourcesRequired,
	PreserveAspectRatio,
	Style,
	XlinkHref,
	CoreAttributes(common_attrs::CoreAttributes),
	FilterAttributes(common_attrs::FilterAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
	XLinkAttributes(common_attrs::XLinkAttributes),
}

impl From<common_attrs::CoreAttributes> for FeImageAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::FilterAttributes> for FeImageAttrs {
	fn from(attr: common_attrs::FilterAttributes) -> Self {
		Self::FilterAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for FeImageAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl From<common_attrs::XLinkAttributes> for FeImageAttrs {
	fn from(attr: common_attrs::XLinkAttributes) -> Self {
		Self::XLinkAttributes(attr)
	}
}

impl FeImageAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::PreserveAspectRatio => "preserveAspectRatio",
			Self::Style => "style",
			Self::XlinkHref => "xlink:href",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::FilterAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
			Self::XLinkAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FeImageAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FeImageAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod fe_image_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Set {}
}

#[doc = "The [`<feImage>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feImage.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateTransform>`](AnimateTransform)\n",
	"- [`<set>`](Set)\n"
)]
#[doc = "\n\n [`<feImage>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feImage"]
#[derive(Debug)]
pub struct FeImage {
	attrs: IndexMap<FeImageAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn fe_image_private::Content>>,
}

impl Default for FeImage {
	fn default() -> Self {
		Self::new()
	}
}

impl FeImage {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: fe_image_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: fe_image_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: FeImageAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FeImageAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeImageAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeImageAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(FeImageAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeImageAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeImageAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(FeImageAttrs::ExternalResourcesRequired)
	}

	/// Set the `preserveAspectRatio` attribute.
	pub fn with_preserve_aspect_ratio<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeImageAttrs::PreserveAspectRatio, value);
		self
	}

	/// Set the `preserveAspectRatio` attribute.
	pub fn set_preserve_aspect_ratio<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeImageAttrs::PreserveAspectRatio, value);
	}

	/// Get the `preserveAspectRatio` attribute.
	pub fn preserve_aspect_ratio(&self) -> Option<&dyn Value> {
		self.get_attr(FeImageAttrs::PreserveAspectRatio)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeImageAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeImageAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(FeImageAttrs::Style)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeImageAttrs::XlinkHref, value);
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeImageAttrs::XlinkHref, value);
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&dyn Value> {
		self.get_attr(FeImageAttrs::XlinkHref)
	}
}

impl common_attrs::CoreAttributesSetter for FeImage {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeImageAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FeImageAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeImage {}

impl common_attrs::FilterAttributesSetter for FeImage {
	fn set_attr<V>(&mut self, attr: common_attrs::FilterAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeImageAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&dyn Value> {
		self.get_attr(FeImageAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeImage {}

impl common_attrs::PresentationAttributesSetter for FeImage {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeImageAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(FeImageAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeImage {}

impl common_attrs::XLinkAttributesSetter for FeImage {
	fn set_attr<V>(&mut self, attr: common_attrs::XLinkAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeImageAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&dyn Value> {
		self.get_attr(FeImageAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for FeImage {}

impl Tag for FeImage {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("feImage");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeMergeAttrs {
	Class,
	Style,
	CoreAttributes(common_attrs::CoreAttributes),
	FilterAttributes(common_attrs::FilterAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::CoreAttributes> for FeMergeAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::FilterAttributes> for FeMergeAttrs {
	fn from(attr: common_attrs::FilterAttributes) -> Self {
		Self::FilterAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for FeMergeAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl FeMergeAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::Style => "style",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::FilterAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FeMergeAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FeMergeAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod fe_merge_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::FeMergeNode {}
}

#[doc = "The [`<feMerge>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feMerge.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<feMergeNode>`](FeMergeNode)\n"
)]
#[doc = "\n\n [`<feMerge>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feMerge"]
#[derive(Debug)]
pub struct FeMerge {
	attrs: IndexMap<FeMergeAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn fe_merge_private::Content>>,
}

impl Default for FeMerge {
	fn default() -> Self {
		Self::new()
	}
}

impl FeMerge {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: fe_merge_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: fe_merge_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: FeMergeAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FeMergeAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeMergeAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeMergeAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(FeMergeAttrs::Class)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeMergeAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeMergeAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(FeMergeAttrs::Style)
	}
}

impl common_attrs::CoreAttributesSetter for FeMerge {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeMergeAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FeMergeAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeMerge {}

impl common_attrs::FilterAttributesSetter for FeMerge {
	fn set_attr<V>(&mut self, attr: common_attrs::FilterAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeMergeAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&dyn Value> {
		self.get_attr(FeMergeAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeMerge {}

impl common_attrs::PresentationAttributesSetter for FeMerge {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeMergeAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(FeMergeAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeMerge {}

impl Tag for FeMerge {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("feMerge");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeMergeNodeAttrs {
	In,
	CoreAttributes(common_attrs::CoreAttributes),
}

impl From<common_attrs::CoreAttributes> for FeMergeNodeAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl FeMergeNodeAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::In => "in",
			Self::CoreAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FeMergeNodeAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FeMergeNodeAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod fe_merge_node_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feMergeNode>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feMergeNode.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<set>`](Set)\n"
)]
#[doc = "\n\n [`<feMergeNode>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feMergeNode"]
#[derive(Debug)]
pub struct FeMergeNode {
	attrs: IndexMap<FeMergeNodeAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn fe_merge_node_private::Content>>,
}

impl Default for FeMergeNode {
	fn default() -> Self {
		Self::new()
	}
}

impl FeMergeNode {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: fe_merge_node_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: fe_merge_node_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: FeMergeNodeAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FeMergeNodeAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `in` attribute.
	pub fn with_in1<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeMergeNodeAttrs::In, value);
		self
	}

	/// Set the `in` attribute.
	pub fn set_in1<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeMergeNodeAttrs::In, value);
	}

	/// Get the `in` attribute.
	pub fn in1(&self) -> Option<&dyn Value> {
		self.get_attr(FeMergeNodeAttrs::In)
	}
}

impl common_attrs::CoreAttributesSetter for FeMergeNode {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeMergeNodeAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FeMergeNodeAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeMergeNode {}

impl Tag for FeMergeNode {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("feMergeNode");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeMorphologyAttrs {
	Class,
	In,
	Operator,
	Radius,
	Style,
	CoreAttributes(common_attrs::CoreAttributes),
	FilterAttributes(common_attrs::FilterAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::CoreAttributes> for FeMorphologyAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::FilterAttributes> for FeMorphologyAttrs {
	fn from(attr: common_attrs::FilterAttributes) -> Self {
		Self::FilterAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for FeMorphologyAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl FeMorphologyAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::In => "in",
			Self::Operator => "operator",
			Self::Radius => "radius",
			Self::Style => "style",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::FilterAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FeMorphologyAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FeMorphologyAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod fe_morphology_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feMorphology>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feMorphology.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<set>`](Set)\n"
)]
#[doc = "\n\n [`<feMorphology>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feMorphology"]
#[derive(Debug)]
pub struct FeMorphology {
	attrs: IndexMap<FeMorphologyAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn fe_morphology_private::Content>>,
}

impl Default for FeMorphology {
	fn default() -> Self {
		Self::new()
	}
}

impl FeMorphology {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: fe_morphology_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: fe_morphology_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: FeMorphologyAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FeMorphologyAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeMorphologyAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeMorphologyAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(FeMorphologyAttrs::Class)
	}

	/// Set the `in` attribute.
	pub fn with_in1<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeMorphologyAttrs::In, value);
		self
	}

	/// Set the `in` attribute.
	pub fn set_in1<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeMorphologyAttrs::In, value);
	}

	/// Get the `in` attribute.
	pub fn in1(&self) -> Option<&dyn Value> {
		self.get_attr(FeMorphologyAttrs::In)
	}

	/// Set the `operator` attribute.
	pub fn with_operator<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeMorphologyAttrs::Operator, value);
		self
	}

	/// Set the `operator` attribute.
	pub fn set_operator<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeMorphologyAttrs::Operator, value);
	}

	/// Get the `operator` attribute.
	pub fn operator(&self) -> Option<&dyn Value> {
		self.get_attr(FeMorphologyAttrs::Operator)
	}

	/// Set the `radius` attribute.
	pub fn with_radius<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeMorphologyAttrs::Radius, value);
		self
	}

	/// Set the `radius` attribute.
	pub fn set_radius<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeMorphologyAttrs::Radius, value);
	}

	/// Get the `radius` attribute.
	pub fn radius(&self) -> Option<&dyn Value> {
		self.get_attr(FeMorphologyAttrs::Radius)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeMorphologyAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeMorphologyAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(FeMorphologyAttrs::Style)
	}
}

impl common_attrs::CoreAttributesSetter for FeMorphology {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeMorphologyAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FeMorphologyAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeMorphology {}

impl common_attrs::FilterAttributesSetter for FeMorphology {
	fn set_attr<V>(&mut self, attr: common_attrs::FilterAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeMorphologyAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&dyn Value> {
		self.get_attr(FeMorphologyAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeMorphology {}

impl common_attrs::PresentationAttributesSetter for FeMorphology {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeMorphologyAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(FeMorphologyAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeMorphology {}

impl Tag for FeMorphology {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("feMorphology");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeOffsetAttrs {
	Class,
	Dx,
	Dy,
	In,
	Style,
	CoreAttributes(common_attrs::CoreAttributes),
	FilterAttributes(common_attrs::FilterAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::CoreAttributes> for FeOffsetAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::FilterAttributes> for FeOffsetAttrs {
	fn from(attr: common_attrs::FilterAttributes) -> Self {
		Self::FilterAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for FeOffsetAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl FeOffsetAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::Dx => "dx",
			Self::Dy => "dy",
			Self::In => "in",
			Self::Style => "style",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::FilterAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FeOffsetAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FeOffsetAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod fe_offset_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feOffset>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feOffset.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<set>`](Set)\n"
)]
#[doc = "\n\n [`<feOffset>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feOffset"]
#[derive(Debug)]
pub struct FeOffset {
	attrs: IndexMap<FeOffsetAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn fe_offset_private::Content>>,
}

impl Default for FeOffset {
	fn default() -> Self {
		Self::new()
	}
}

impl FeOffset {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: fe_offset_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: fe_offset_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: FeOffsetAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FeOffsetAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeOffsetAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeOffsetAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(FeOffsetAttrs::Class)
	}

	/// Set the `dx` attribute.
	pub fn with_dx<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeOffsetAttrs::Dx, value);
		self
	}

	/// Set the `dx` attribute.
	pub fn set_dx<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeOffsetAttrs::Dx, value);
	}

	/// Get the `dx` attribute.
	pub fn dx(&self) -> Option<&dyn Value> {
		self.get_attr(FeOffsetAttrs::Dx)
	}

	/// Set the `dy` attribute.
	pub fn with_dy<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeOffsetAttrs::Dy, value);
		self
	}

	/// Set the `dy` attribute.
	pub fn set_dy<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeOffsetAttrs::Dy, value);
	}

	/// Get the `dy` attribute.
	pub fn dy(&self) -> Option<&dyn Value> {
		self.get_attr(FeOffsetAttrs::Dy)
	}

	/// Set the `in` attribute.
	pub fn with_in1<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeOffsetAttrs::In, value);
		self
	}

	/// Set the `in` attribute.
	pub fn set_in1<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeOffsetAttrs::In, value);
	}

	/// Get the `in` attribute.
	pub fn in1(&self) -> Option<&dyn Value> {
		self.get_attr(FeOffsetAttrs::In)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeOffsetAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeOffsetAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(FeOffsetAttrs::Style)
	}
}

impl common_attrs::CoreAttributesSetter for FeOffset {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeOffsetAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FeOffsetAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeOffset {}

impl common_attrs::FilterAttributesSetter for FeOffset {
	fn set_attr<V>(&mut self, attr: common_attrs::FilterAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeOffsetAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&dyn Value> {
		self.get_attr(FeOffsetAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeOffset {}

impl common_attrs::PresentationAttributesSetter for FeOffset {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeOffsetAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(FeOffsetAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeOffset {}

impl Tag for FeOffset {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("feOffset");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FePointLightAttrs {
	X,
	Y,
	Z,
	CoreAttributes(common_attrs::CoreAttributes),
}

impl From<common_attrs::CoreAttributes> for FePointLightAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl FePointLightAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::X => "x",
			Self::Y => "y",
			Self::Z => "z",
			Self::CoreAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FePointLightAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FePointLightAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod fe_point_light_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<fePointLight>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("fePointLight.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<set>`](Set)\n"
)]
#[doc = "\n\n [`<fePointLight>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/fePointLight"]
#[derive(Debug)]
pub struct FePointLight {
	attrs: IndexMap<FePointLightAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn fe_point_light_private::Content>>,
}

impl Default for FePointLight {
	fn default() -> Self {
		Self::new()
	}
}

impl FePointLight {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: fe_point_light_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: fe_point_light_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: FePointLightAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FePointLightAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `x` attribute.
	pub fn with_x<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FePointLightAttrs::X, value);
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FePointLightAttrs::X, value);
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&dyn Value> {
		self.get_attr(FePointLightAttrs::X)
	}

	/// Set the `y` attribute.
	pub fn with_y<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FePointLightAttrs::Y, value);
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FePointLightAttrs::Y, value);
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&dyn Value> {
		self.get_attr(FePointLightAttrs::Y)
	}

	/// Set the `z` attribute.
	pub fn with_z<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FePointLightAttrs::Z, value);
		self
	}

	/// Set the `z` attribute.
	pub fn set_z<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FePointLightAttrs::Z, value);
	}

	/// Get the `z` attribute.
	pub fn z(&self) -> Option<&dyn Value> {
		self.get_attr(FePointLightAttrs::Z)
	}
}

impl common_attrs::CoreAttributesSetter for FePointLight {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FePointLightAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FePointLightAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FePointLight {}

impl Tag for FePointLight {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("fePointLight");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeSpecularLightingAttrs {
	Class,
	In,
	KernelUnitLength,
	SpecularConstant,
	SpecularExponent,
	Style,
	SurfaceScale,
	CoreAttributes(common_attrs::CoreAttributes),
	FilterAttributes(common_attrs::FilterAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::CoreAttributes> for FeSpecularLightingAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::FilterAttributes> for FeSpecularLightingAttrs {
	fn from(attr: common_attrs::FilterAttributes) -> Self {
		Self::FilterAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for FeSpecularLightingAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl FeSpecularLightingAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::In => "in",
			Self::KernelUnitLength => "kernelUnitLength",
			Self::SpecularConstant => "specularConstant",
			Self::SpecularExponent => "specularExponent",
			Self::Style => "style",
			Self::SurfaceScale => "surfaceScale",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::FilterAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FeSpecularLightingAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FeSpecularLightingAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[doc = "The [`<feSpecularLighting>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feSpecularLighting.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n"
)]
#[doc = "\n\n [`<feSpecularLighting>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feSpecularLighting"]
#[derive(Debug)]
pub struct FeSpecularLighting {
	attrs: IndexMap<FeSpecularLightingAttrs, Box<dyn Value>>,
}

impl Default for FeSpecularLighting {
	fn default() -> Self {
		Self::new()
	}
}

impl FeSpecularLighting {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
		}
	}

	fn set_attr<V>(&mut self, attr: FeSpecularLightingAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FeSpecularLightingAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeSpecularLightingAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeSpecularLightingAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(FeSpecularLightingAttrs::Class)
	}

	/// Set the `in` attribute.
	pub fn with_in1<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeSpecularLightingAttrs::In, value);
		self
	}

	/// Set the `in` attribute.
	pub fn set_in1<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeSpecularLightingAttrs::In, value);
	}

	/// Get the `in` attribute.
	pub fn in1(&self) -> Option<&dyn Value> {
		self.get_attr(FeSpecularLightingAttrs::In)
	}

	/// Set the `kernelUnitLength` attribute.
	pub fn with_kernel_unit_length<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeSpecularLightingAttrs::KernelUnitLength, value);
		self
	}

	/// Set the `kernelUnitLength` attribute.
	pub fn set_kernel_unit_length<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeSpecularLightingAttrs::KernelUnitLength, value);
	}

	/// Get the `kernelUnitLength` attribute.
	pub fn kernel_unit_length(&self) -> Option<&dyn Value> {
		self.get_attr(FeSpecularLightingAttrs::KernelUnitLength)
	}

	/// Set the `specularConstant` attribute.
	pub fn with_specular_constant<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeSpecularLightingAttrs::SpecularConstant, value);
		self
	}

	/// Set the `specularConstant` attribute.
	pub fn set_specular_constant<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeSpecularLightingAttrs::SpecularConstant, value);
	}

	/// Get the `specularConstant` attribute.
	pub fn specular_constant(&self) -> Option<&dyn Value> {
		self.get_attr(FeSpecularLightingAttrs::SpecularConstant)
	}

	/// Set the `specularExponent` attribute.
	pub fn with_specular_exponent<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeSpecularLightingAttrs::SpecularExponent, value);
		self
	}

	/// Set the `specularExponent` attribute.
	pub fn set_specular_exponent<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeSpecularLightingAttrs::SpecularExponent, value);
	}

	/// Get the `specularExponent` attribute.
	pub fn specular_exponent(&self) -> Option<&dyn Value> {
		self.get_attr(FeSpecularLightingAttrs::SpecularExponent)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeSpecularLightingAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeSpecularLightingAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(FeSpecularLightingAttrs::Style)
	}

	/// Set the `surfaceScale` attribute.
	pub fn with_surface_scale<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeSpecularLightingAttrs::SurfaceScale, value);
		self
	}

	/// Set the `surfaceScale` attribute.
	pub fn set_surface_scale<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeSpecularLightingAttrs::SurfaceScale, value);
	}

	/// Get the `surfaceScale` attribute.
	pub fn surface_scale(&self) -> Option<&dyn Value> {
		self.get_attr(FeSpecularLightingAttrs::SurfaceScale)
	}
}

impl common_attrs::CoreAttributesSetter for FeSpecularLighting {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeSpecularLightingAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FeSpecularLightingAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeSpecularLighting {}

impl common_attrs::FilterAttributesSetter for FeSpecularLighting {
	fn set_attr<V>(&mut self, attr: common_attrs::FilterAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeSpecularLightingAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&dyn Value> {
		self.get_attr(FeSpecularLightingAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeSpecularLighting {}

impl common_attrs::PresentationAttributesSetter for FeSpecularLighting {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeSpecularLightingAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(FeSpecularLightingAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeSpecularLighting {}

impl Tag for FeSpecularLighting {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("feSpecularLighting");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeSpotLightAttrs {
	LimitingConeAngle,
	PointsAtX,
	PointsAtY,
	PointsAtZ,
	SpecularExponent,
	X,
	Y,
	Z,
	CoreAttributes(common_attrs::CoreAttributes),
}

impl From<common_attrs::CoreAttributes> for FeSpotLightAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl FeSpotLightAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::LimitingConeAngle => "limitingConeAngle",
			Self::PointsAtX => "pointsAtX",
			Self::PointsAtY => "pointsAtY",
			Self::PointsAtZ => "pointsAtZ",
			Self::SpecularExponent => "specularExponent",
			Self::X => "x",
			Self::Y => "y",
			Self::Z => "z",
			Self::CoreAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FeSpotLightAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FeSpotLightAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod fe_spot_light_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feSpotLight>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feSpotLight.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<set>`](Set)\n"
)]
#[doc = "\n\n [`<feSpotLight>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feSpotLight"]
#[derive(Debug)]
pub struct FeSpotLight {
	attrs: IndexMap<FeSpotLightAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn fe_spot_light_private::Content>>,
}

impl Default for FeSpotLight {
	fn default() -> Self {
		Self::new()
	}
}

impl FeSpotLight {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: fe_spot_light_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: fe_spot_light_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: FeSpotLightAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FeSpotLightAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `limitingConeAngle` attribute.
	pub fn with_limiting_cone_angle<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeSpotLightAttrs::LimitingConeAngle, value);
		self
	}

	/// Set the `limitingConeAngle` attribute.
	pub fn set_limiting_cone_angle<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeSpotLightAttrs::LimitingConeAngle, value);
	}

	/// Get the `limitingConeAngle` attribute.
	pub fn limiting_cone_angle(&self) -> Option<&dyn Value> {
		self.get_attr(FeSpotLightAttrs::LimitingConeAngle)
	}

	/// Set the `pointsAtX` attribute.
	pub fn with_points_at_x<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeSpotLightAttrs::PointsAtX, value);
		self
	}

	/// Set the `pointsAtX` attribute.
	pub fn set_points_at_x<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeSpotLightAttrs::PointsAtX, value);
	}

	/// Get the `pointsAtX` attribute.
	pub fn points_at_x(&self) -> Option<&dyn Value> {
		self.get_attr(FeSpotLightAttrs::PointsAtX)
	}

	/// Set the `pointsAtY` attribute.
	pub fn with_points_at_y<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeSpotLightAttrs::PointsAtY, value);
		self
	}

	/// Set the `pointsAtY` attribute.
	pub fn set_points_at_y<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeSpotLightAttrs::PointsAtY, value);
	}

	/// Get the `pointsAtY` attribute.
	pub fn points_at_y(&self) -> Option<&dyn Value> {
		self.get_attr(FeSpotLightAttrs::PointsAtY)
	}

	/// Set the `pointsAtZ` attribute.
	pub fn with_points_at_z<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeSpotLightAttrs::PointsAtZ, value);
		self
	}

	/// Set the `pointsAtZ` attribute.
	pub fn set_points_at_z<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeSpotLightAttrs::PointsAtZ, value);
	}

	/// Get the `pointsAtZ` attribute.
	pub fn points_at_z(&self) -> Option<&dyn Value> {
		self.get_attr(FeSpotLightAttrs::PointsAtZ)
	}

	/// Set the `specularExponent` attribute.
	pub fn with_specular_exponent<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeSpotLightAttrs::SpecularExponent, value);
		self
	}

	/// Set the `specularExponent` attribute.
	pub fn set_specular_exponent<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeSpotLightAttrs::SpecularExponent, value);
	}

	/// Get the `specularExponent` attribute.
	pub fn specular_exponent(&self) -> Option<&dyn Value> {
		self.get_attr(FeSpotLightAttrs::SpecularExponent)
	}

	/// Set the `x` attribute.
	pub fn with_x<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeSpotLightAttrs::X, value);
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeSpotLightAttrs::X, value);
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&dyn Value> {
		self.get_attr(FeSpotLightAttrs::X)
	}

	/// Set the `y` attribute.
	pub fn with_y<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeSpotLightAttrs::Y, value);
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeSpotLightAttrs::Y, value);
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&dyn Value> {
		self.get_attr(FeSpotLightAttrs::Y)
	}

	/// Set the `z` attribute.
	pub fn with_z<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeSpotLightAttrs::Z, value);
		self
	}

	/// Set the `z` attribute.
	pub fn set_z<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeSpotLightAttrs::Z, value);
	}

	/// Get the `z` attribute.
	pub fn z(&self) -> Option<&dyn Value> {
		self.get_attr(FeSpotLightAttrs::Z)
	}
}

impl common_attrs::CoreAttributesSetter for FeSpotLight {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeSpotLightAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FeSpotLightAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeSpotLight {}

impl Tag for FeSpotLight {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("feSpotLight");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeTileAttrs {
	Class,
	In,
	Style,
	CoreAttributes(common_attrs::CoreAttributes),
	FilterAttributes(common_attrs::FilterAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::CoreAttributes> for FeTileAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::FilterAttributes> for FeTileAttrs {
	fn from(attr: common_attrs::FilterAttributes) -> Self {
		Self::FilterAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for FeTileAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl FeTileAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::In => "in",
			Self::Style => "style",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::FilterAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FeTileAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FeTileAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod fe_tile_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feTile>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feTile.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<set>`](Set)\n"
)]
#[doc = "\n\n [`<feTile>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feTile"]
#[derive(Debug)]
pub struct FeTile {
	attrs: IndexMap<FeTileAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn fe_tile_private::Content>>,
}

impl Default for FeTile {
	fn default() -> Self {
		Self::new()
	}
}

impl FeTile {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: fe_tile_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: fe_tile_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: FeTileAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FeTileAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeTileAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeTileAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(FeTileAttrs::Class)
	}

	/// Set the `in` attribute.
	pub fn with_in1<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeTileAttrs::In, value);
		self
	}

	/// Set the `in` attribute.
	pub fn set_in1<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeTileAttrs::In, value);
	}

	/// Get the `in` attribute.
	pub fn in1(&self) -> Option<&dyn Value> {
		self.get_attr(FeTileAttrs::In)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeTileAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeTileAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(FeTileAttrs::Style)
	}
}

impl common_attrs::CoreAttributesSetter for FeTile {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeTileAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FeTileAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeTile {}

impl common_attrs::FilterAttributesSetter for FeTile {
	fn set_attr<V>(&mut self, attr: common_attrs::FilterAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeTileAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&dyn Value> {
		self.get_attr(FeTileAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeTile {}

impl common_attrs::PresentationAttributesSetter for FeTile {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeTileAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(FeTileAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeTile {}

impl Tag for FeTile {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("feTile");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeTurbulenceAttrs {
	BaseFrequency,
	Class,
	NumOctaves,
	Seed,
	StitchTiles,
	Style,
	Type,
	CoreAttributes(common_attrs::CoreAttributes),
	FilterAttributes(common_attrs::FilterAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::CoreAttributes> for FeTurbulenceAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::FilterAttributes> for FeTurbulenceAttrs {
	fn from(attr: common_attrs::FilterAttributes) -> Self {
		Self::FilterAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for FeTurbulenceAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl FeTurbulenceAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::BaseFrequency => "baseFrequency",
			Self::Class => "class",
			Self::NumOctaves => "numOctaves",
			Self::Seed => "seed",
			Self::StitchTiles => "stitchTiles",
			Self::Style => "style",
			Self::Type => "type",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::FilterAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FeTurbulenceAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FeTurbulenceAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod fe_turbulence_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feTurbulence>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feTurbulence.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<set>`](Set)\n"
)]
#[doc = "\n\n [`<feTurbulence>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feTurbulence"]
#[derive(Debug)]
pub struct FeTurbulence {
	attrs: IndexMap<FeTurbulenceAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn fe_turbulence_private::Content>>,
}

impl Default for FeTurbulence {
	fn default() -> Self {
		Self::new()
	}
}

impl FeTurbulence {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: fe_turbulence_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: fe_turbulence_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: FeTurbulenceAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FeTurbulenceAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `baseFrequency` attribute.
	pub fn with_base_frequency<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeTurbulenceAttrs::BaseFrequency, value);
		self
	}

	/// Set the `baseFrequency` attribute.
	pub fn set_base_frequency<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeTurbulenceAttrs::BaseFrequency, value);
	}

	/// Get the `baseFrequency` attribute.
	pub fn base_frequency(&self) -> Option<&dyn Value> {
		self.get_attr(FeTurbulenceAttrs::BaseFrequency)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeTurbulenceAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeTurbulenceAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(FeTurbulenceAttrs::Class)
	}

	/// Set the `numOctaves` attribute.
	pub fn with_num_octaves<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeTurbulenceAttrs::NumOctaves, value);
		self
	}

	/// Set the `numOctaves` attribute.
	pub fn set_num_octaves<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeTurbulenceAttrs::NumOctaves, value);
	}

	/// Get the `numOctaves` attribute.
	pub fn num_octaves(&self) -> Option<&dyn Value> {
		self.get_attr(FeTurbulenceAttrs::NumOctaves)
	}

	/// Set the `seed` attribute.
	pub fn with_seed<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeTurbulenceAttrs::Seed, value);
		self
	}

	/// Set the `seed` attribute.
	pub fn set_seed<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeTurbulenceAttrs::Seed, value);
	}

	/// Get the `seed` attribute.
	pub fn seed(&self) -> Option<&dyn Value> {
		self.get_attr(FeTurbulenceAttrs::Seed)
	}

	/// Set the `stitchTiles` attribute.
	pub fn with_stitch_tiles<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeTurbulenceAttrs::StitchTiles, value);
		self
	}

	/// Set the `stitchTiles` attribute.
	pub fn set_stitch_tiles<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeTurbulenceAttrs::StitchTiles, value);
	}

	/// Get the `stitchTiles` attribute.
	pub fn stitch_tiles(&self) -> Option<&dyn Value> {
		self.get_attr(FeTurbulenceAttrs::StitchTiles)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeTurbulenceAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeTurbulenceAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(FeTurbulenceAttrs::Style)
	}

	/// Set the `type` attribute.
	pub fn with_ty<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FeTurbulenceAttrs::Type, value);
		self
	}

	/// Set the `type` attribute.
	pub fn set_ty<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeTurbulenceAttrs::Type, value);
	}

	/// Get the `type` attribute.
	pub fn ty(&self) -> Option<&dyn Value> {
		self.get_attr(FeTurbulenceAttrs::Type)
	}
}

impl common_attrs::CoreAttributesSetter for FeTurbulence {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeTurbulenceAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FeTurbulenceAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeTurbulence {}

impl common_attrs::FilterAttributesSetter for FeTurbulence {
	fn set_attr<V>(&mut self, attr: common_attrs::FilterAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeTurbulenceAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&dyn Value> {
		self.get_attr(FeTurbulenceAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeTurbulence {}

impl common_attrs::PresentationAttributesSetter for FeTurbulence {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FeTurbulenceAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(FeTurbulenceAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeTurbulence {}

impl Tag for FeTurbulence {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("feTurbulence");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FilterAttrs {
	Class,
	ExternalResourcesRequired,
	FilterRes,
	FilterUnits,
	Height,
	PrimitiveUnits,
	Style,
	Width,
	X,
	XlinkHref,
	Y,
	CoreAttributes(common_attrs::CoreAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
	XLinkAttributes(common_attrs::XLinkAttributes),
}

impl From<common_attrs::CoreAttributes> for FilterAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for FilterAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl From<common_attrs::XLinkAttributes> for FilterAttrs {
	fn from(attr: common_attrs::XLinkAttributes) -> Self {
		Self::XLinkAttributes(attr)
	}
}

impl FilterAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::FilterRes => "filterRes",
			Self::FilterUnits => "filterUnits",
			Self::Height => "height",
			Self::PrimitiveUnits => "primitiveUnits",
			Self::Style => "style",
			Self::Width => "width",
			Self::X => "x",
			Self::XlinkHref => "xlink:href",
			Self::Y => "y",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
			Self::XLinkAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FilterAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FilterAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod filter_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Desc {}
	impl Content for super::FeBlend {}
	impl Content for super::FeColorMatrix {}
	impl Content for super::FeComponentTransfer {}
	impl Content for super::FeComposite {}
	impl Content for super::FeConvolveMatrix {}
	impl Content for super::FeDiffuseLighting {}
	impl Content for super::FeDisplacementMap {}
	impl Content for super::FeDropShadow {}
	impl Content for super::FeFlood {}
	impl Content for super::FeGaussianBlur {}
	impl Content for super::FeImage {}
	impl Content for super::FeMerge {}
	impl Content for super::FeMorphology {}
	impl Content for super::FeOffset {}
	impl Content for super::FeSpecularLighting {}
	impl Content for super::FeTile {}
	impl Content for super::FeTurbulence {}
	impl Content for super::Metadata {}
	impl Content for super::Set {}
	impl Content for super::Title {}
}

#[doc = "The [`<filter>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("filter.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<feBlend>`](FeBlend)\n",
	"- [`<feColorMatrix>`](FeColorMatrix)\n",
	"- [`<feComponentTransfer>`](FeComponentTransfer)\n",
	"- [`<feComposite>`](FeComposite)\n",
	"- [`<feConvolveMatrix>`](FeConvolveMatrix)\n",
	"- [`<feDiffuseLighting>`](FeDiffuseLighting)\n",
	"- [`<feDisplacementMap>`](FeDisplacementMap)\n",
	"- [`<feDropShadow>`](FeDropShadow)\n",
	"- [`<feFlood>`](FeFlood)\n",
	"- [`<feGaussianBlur>`](FeGaussianBlur)\n",
	"- [`<feImage>`](FeImage)\n",
	"- [`<feMerge>`](FeMerge)\n",
	"- [`<feMorphology>`](FeMorphology)\n",
	"- [`<feOffset>`](FeOffset)\n",
	"- [`<feSpecularLighting>`](FeSpecularLighting)\n",
	"- [`<feTile>`](FeTile)\n",
	"- [`<feTurbulence>`](FeTurbulence)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<set>`](Set)\n",
	"- [`<title>`](Title)\n"
)]
#[doc = "\n\n [`<filter>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/filter"]
#[derive(Debug)]
pub struct Filter {
	attrs: IndexMap<FilterAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn filter_private::Content>>,
}

impl Default for Filter {
	fn default() -> Self {
		Self::new()
	}
}

impl Filter {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: filter_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: filter_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: FilterAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FilterAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(FilterAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(FilterAttrs::ExternalResourcesRequired)
	}

	/// Set the `filterRes` attribute.
	pub fn with_filter_res<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttrs::FilterRes, value);
		self
	}

	/// Set the `filterRes` attribute.
	pub fn set_filter_res<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttrs::FilterRes, value);
	}

	/// Get the `filterRes` attribute.
	pub fn filter_res(&self) -> Option<&dyn Value> {
		self.get_attr(FilterAttrs::FilterRes)
	}

	/// Set the `filterUnits` attribute.
	pub fn with_filter_units<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttrs::FilterUnits, value);
		self
	}

	/// Set the `filterUnits` attribute.
	pub fn set_filter_units<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttrs::FilterUnits, value);
	}

	/// Get the `filterUnits` attribute.
	pub fn filter_units(&self) -> Option<&dyn Value> {
		self.get_attr(FilterAttrs::FilterUnits)
	}

	/// Set the `height` attribute.
	pub fn with_height<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttrs::Height, value);
		self
	}

	/// Set the `height` attribute.
	pub fn set_height<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttrs::Height, value);
	}

	/// Get the `height` attribute.
	pub fn height(&self) -> Option<&dyn Value> {
		self.get_attr(FilterAttrs::Height)
	}

	/// Set the `primitiveUnits` attribute.
	pub fn with_primitive_units<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttrs::PrimitiveUnits, value);
		self
	}

	/// Set the `primitiveUnits` attribute.
	pub fn set_primitive_units<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttrs::PrimitiveUnits, value);
	}

	/// Get the `primitiveUnits` attribute.
	pub fn primitive_units(&self) -> Option<&dyn Value> {
		self.get_attr(FilterAttrs::PrimitiveUnits)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(FilterAttrs::Style)
	}

	/// Set the `width` attribute.
	pub fn with_width<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttrs::Width, value);
		self
	}

	/// Set the `width` attribute.
	pub fn set_width<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttrs::Width, value);
	}

	/// Get the `width` attribute.
	pub fn width(&self) -> Option<&dyn Value> {
		self.get_attr(FilterAttrs::Width)
	}

	/// Set the `x` attribute.
	pub fn with_x<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttrs::X, value);
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttrs::X, value);
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&dyn Value> {
		self.get_attr(FilterAttrs::X)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttrs::XlinkHref, value);
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttrs::XlinkHref, value);
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&dyn Value> {
		self.get_attr(FilterAttrs::XlinkHref)
	}

	/// Set the `y` attribute.
	pub fn with_y<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttrs::Y, value);
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttrs::Y, value);
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&dyn Value> {
		self.get_attr(FilterAttrs::Y)
	}
}

impl common_attrs::CoreAttributesSetter for Filter {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FilterAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Filter {}

impl common_attrs::PresentationAttributesSetter for Filter {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(FilterAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Filter {}

impl common_attrs::XLinkAttributesSetter for Filter {
	fn set_attr<V>(&mut self, attr: common_attrs::XLinkAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&dyn Value> {
		self.get_attr(FilterAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for Filter {}

impl Tag for Filter {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("filter");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FontAttrs {
	Class,
	ExternalResourcesRequired,
	HorizAdvX,
	HorizOriginX,
	HorizOriginY,
	Style,
	VertAdvY,
	VertOriginX,
	VertOriginY,
	CoreAttributes(common_attrs::CoreAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::CoreAttributes> for FontAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for FontAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl FontAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::HorizAdvX => "horiz-adv-x",
			Self::HorizOriginX => "horiz-origin-x",
			Self::HorizOriginY => "horiz-origin-y",
			Self::Style => "style",
			Self::VertAdvY => "vert-adv-y",
			Self::VertOriginX => "vert-origin-x",
			Self::VertOriginY => "vert-origin-y",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FontAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FontAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod font_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Desc {}
	impl Content for super::FontFace {}
	impl Content for super::Glyph {}
	impl Content for super::Hkern {}
	impl Content for super::Metadata {}
	impl Content for super::MissingGlyph {}
	impl Content for super::Title {}
	impl Content for super::Vkern {}
}

#[doc = "The [`<font>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("font.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<desc>`](Desc)\n",
	"- [`<font-face>`](FontFace)\n",
	"- [`<glyph>`](Glyph)\n",
	"- [`<hkern>`](Hkern)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<missing-glyph>`](MissingGlyph)\n",
	"- [`<title>`](Title)\n",
	"- [`<vkern>`](Vkern)\n"
)]
#[doc = "\n\n [`<font>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/font"]
#[derive(Debug)]
pub struct Font {
	attrs: IndexMap<FontAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn font_private::Content>>,
}

impl Default for Font {
	fn default() -> Self {
		Self::new()
	}
}

impl Font {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: font_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: font_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: FontAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FontAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(FontAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(FontAttrs::ExternalResourcesRequired)
	}

	/// Set the `horiz-adv-x` attribute.
	pub fn with_horiz_adv_x<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontAttrs::HorizAdvX, value);
		self
	}

	/// Set the `horiz-adv-x` attribute.
	pub fn set_horiz_adv_x<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontAttrs::HorizAdvX, value);
	}

	/// Get the `horiz-adv-x` attribute.
	pub fn horiz_adv_x(&self) -> Option<&dyn Value> {
		self.get_attr(FontAttrs::HorizAdvX)
	}

	/// Set the `horiz-origin-x` attribute.
	pub fn with_horiz_origin_x<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontAttrs::HorizOriginX, value);
		self
	}

	/// Set the `horiz-origin-x` attribute.
	pub fn set_horiz_origin_x<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontAttrs::HorizOriginX, value);
	}

	/// Get the `horiz-origin-x` attribute.
	pub fn horiz_origin_x(&self) -> Option<&dyn Value> {
		self.get_attr(FontAttrs::HorizOriginX)
	}

	/// Set the `horiz-origin-y` attribute.
	pub fn with_horiz_origin_y<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontAttrs::HorizOriginY, value);
		self
	}

	/// Set the `horiz-origin-y` attribute.
	pub fn set_horiz_origin_y<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontAttrs::HorizOriginY, value);
	}

	/// Get the `horiz-origin-y` attribute.
	pub fn horiz_origin_y(&self) -> Option<&dyn Value> {
		self.get_attr(FontAttrs::HorizOriginY)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(FontAttrs::Style)
	}

	/// Set the `vert-adv-y` attribute.
	pub fn with_vert_adv_y<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontAttrs::VertAdvY, value);
		self
	}

	/// Set the `vert-adv-y` attribute.
	pub fn set_vert_adv_y<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontAttrs::VertAdvY, value);
	}

	/// Get the `vert-adv-y` attribute.
	pub fn vert_adv_y(&self) -> Option<&dyn Value> {
		self.get_attr(FontAttrs::VertAdvY)
	}

	/// Set the `vert-origin-x` attribute.
	pub fn with_vert_origin_x<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontAttrs::VertOriginX, value);
		self
	}

	/// Set the `vert-origin-x` attribute.
	pub fn set_vert_origin_x<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontAttrs::VertOriginX, value);
	}

	/// Get the `vert-origin-x` attribute.
	pub fn vert_origin_x(&self) -> Option<&dyn Value> {
		self.get_attr(FontAttrs::VertOriginX)
	}

	/// Set the `vert-origin-y` attribute.
	pub fn with_vert_origin_y<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontAttrs::VertOriginY, value);
		self
	}

	/// Set the `vert-origin-y` attribute.
	pub fn set_vert_origin_y<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontAttrs::VertOriginY, value);
	}

	/// Get the `vert-origin-y` attribute.
	pub fn vert_origin_y(&self) -> Option<&dyn Value> {
		self.get_attr(FontAttrs::VertOriginY)
	}
}

impl common_attrs::CoreAttributesSetter for Font {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FontAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Font {}

impl common_attrs::PresentationAttributesSetter for Font {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(FontAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Font {}

impl Tag for Font {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("font");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FontFaceAttrs {
	AccentHeight,
	Alphabetic,
	Ascent,
	Bbox,
	CapHeight,
	Descent,
	FontFamily,
	FontSize,
	FontStretch,
	FontStyle,
	FontVariant,
	FontWeight,
	Hanging,
	Ideographic,
	Mathematical,
	OverlinePosition,
	OverlineThickness,
	Panose1,
	Slope,
	Stemh,
	Stemv,
	StrikethroughPosition,
	StrikethroughThickness,
	UnderlinePosition,
	UnderlineThickness,
	UnicodeRange,
	UnitsPerEm,
	VAlphabetic,
	VHanging,
	VIdeographic,
	VMathematical,
	Widths,
	XHeight,
	CoreAttributes(common_attrs::CoreAttributes),
}

impl From<common_attrs::CoreAttributes> for FontFaceAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl FontFaceAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::AccentHeight => "accent-height",
			Self::Alphabetic => "alphabetic",
			Self::Ascent => "ascent",
			Self::Bbox => "bbox",
			Self::CapHeight => "cap-height",
			Self::Descent => "descent",
			Self::FontFamily => "font-family",
			Self::FontSize => "font-size",
			Self::FontStretch => "font-stretch",
			Self::FontStyle => "font-style",
			Self::FontVariant => "font-variant",
			Self::FontWeight => "font-weight",
			Self::Hanging => "hanging",
			Self::Ideographic => "ideographic",
			Self::Mathematical => "mathematical",
			Self::OverlinePosition => "overline-position",
			Self::OverlineThickness => "overline-thickness",
			Self::Panose1 => "panose-1",
			Self::Slope => "slope",
			Self::Stemh => "stemh",
			Self::Stemv => "stemv",
			Self::StrikethroughPosition => "strikethrough-position",
			Self::StrikethroughThickness => "strikethrough-thickness",
			Self::UnderlinePosition => "underline-position",
			Self::UnderlineThickness => "underline-thickness",
			Self::UnicodeRange => "unicode-range",
			Self::UnitsPerEm => "units-per-em",
			Self::VAlphabetic => "v-alphabetic",
			Self::VHanging => "v-hanging",
			Self::VIdeographic => "v-ideographic",
			Self::VMathematical => "v-mathematical",
			Self::Widths => "widths",
			Self::XHeight => "x-height",
			Self::CoreAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FontFaceAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FontFaceAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[doc = "The [`<font-face>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("font-face.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n"
)]
#[doc = "\n\n [`<font-face>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/font-face"]
#[derive(Debug)]
pub struct FontFace {
	attrs: IndexMap<FontFaceAttrs, Box<dyn Value>>,
}

impl Default for FontFace {
	fn default() -> Self {
		Self::new()
	}
}

impl FontFace {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
		}
	}

	fn set_attr<V>(&mut self, attr: FontFaceAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FontFaceAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `accent-height` attribute.
	pub fn with_accent_height<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::AccentHeight, value);
		self
	}

	/// Set the `accent-height` attribute.
	pub fn set_accent_height<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::AccentHeight, value);
	}

	/// Get the `accent-height` attribute.
	pub fn accent_height(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::AccentHeight)
	}

	/// Set the `alphabetic` attribute.
	pub fn with_alphabetic<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::Alphabetic, value);
		self
	}

	/// Set the `alphabetic` attribute.
	pub fn set_alphabetic<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::Alphabetic, value);
	}

	/// Get the `alphabetic` attribute.
	pub fn alphabetic(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::Alphabetic)
	}

	/// Set the `ascent` attribute.
	pub fn with_ascent<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::Ascent, value);
		self
	}

	/// Set the `ascent` attribute.
	pub fn set_ascent<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::Ascent, value);
	}

	/// Get the `ascent` attribute.
	pub fn ascent(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::Ascent)
	}

	/// Set the `bbox` attribute.
	pub fn with_bbox<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::Bbox, value);
		self
	}

	/// Set the `bbox` attribute.
	pub fn set_bbox<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::Bbox, value);
	}

	/// Get the `bbox` attribute.
	pub fn bbox(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::Bbox)
	}

	/// Set the `cap-height` attribute.
	pub fn with_cap_height<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::CapHeight, value);
		self
	}

	/// Set the `cap-height` attribute.
	pub fn set_cap_height<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::CapHeight, value);
	}

	/// Get the `cap-height` attribute.
	pub fn cap_height(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::CapHeight)
	}

	/// Set the `descent` attribute.
	pub fn with_descent<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::Descent, value);
		self
	}

	/// Set the `descent` attribute.
	pub fn set_descent<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::Descent, value);
	}

	/// Get the `descent` attribute.
	pub fn descent(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::Descent)
	}

	/// Set the `font-family` attribute.
	pub fn with_font_family<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::FontFamily, value);
		self
	}

	/// Set the `font-family` attribute.
	pub fn set_font_family<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::FontFamily, value);
	}

	/// Get the `font-family` attribute.
	pub fn font_family(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::FontFamily)
	}

	/// Set the `font-size` attribute.
	pub fn with_font_size<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::FontSize, value);
		self
	}

	/// Set the `font-size` attribute.
	pub fn set_font_size<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::FontSize, value);
	}

	/// Get the `font-size` attribute.
	pub fn font_size(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::FontSize)
	}

	/// Set the `font-stretch` attribute.
	pub fn with_font_stretch<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::FontStretch, value);
		self
	}

	/// Set the `font-stretch` attribute.
	pub fn set_font_stretch<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::FontStretch, value);
	}

	/// Get the `font-stretch` attribute.
	pub fn font_stretch(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::FontStretch)
	}

	/// Set the `font-style` attribute.
	pub fn with_font_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::FontStyle, value);
		self
	}

	/// Set the `font-style` attribute.
	pub fn set_font_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::FontStyle, value);
	}

	/// Get the `font-style` attribute.
	pub fn font_style(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::FontStyle)
	}

	/// Set the `font-variant` attribute.
	pub fn with_font_variant<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::FontVariant, value);
		self
	}

	/// Set the `font-variant` attribute.
	pub fn set_font_variant<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::FontVariant, value);
	}

	/// Get the `font-variant` attribute.
	pub fn font_variant(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::FontVariant)
	}

	/// Set the `font-weight` attribute.
	pub fn with_font_weight<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::FontWeight, value);
		self
	}

	/// Set the `font-weight` attribute.
	pub fn set_font_weight<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::FontWeight, value);
	}

	/// Get the `font-weight` attribute.
	pub fn font_weight(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::FontWeight)
	}

	/// Set the `hanging` attribute.
	pub fn with_hanging<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::Hanging, value);
		self
	}

	/// Set the `hanging` attribute.
	pub fn set_hanging<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::Hanging, value);
	}

	/// Get the `hanging` attribute.
	pub fn hanging(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::Hanging)
	}

	/// Set the `ideographic` attribute.
	pub fn with_ideographic<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::Ideographic, value);
		self
	}

	/// Set the `ideographic` attribute.
	pub fn set_ideographic<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::Ideographic, value);
	}

	/// Get the `ideographic` attribute.
	pub fn ideographic(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::Ideographic)
	}

	/// Set the `mathematical` attribute.
	pub fn with_mathematical<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::Mathematical, value);
		self
	}

	/// Set the `mathematical` attribute.
	pub fn set_mathematical<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::Mathematical, value);
	}

	/// Get the `mathematical` attribute.
	pub fn mathematical(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::Mathematical)
	}

	/// Set the `overline-position` attribute.
	pub fn with_overline_position<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::OverlinePosition, value);
		self
	}

	/// Set the `overline-position` attribute.
	pub fn set_overline_position<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::OverlinePosition, value);
	}

	/// Get the `overline-position` attribute.
	pub fn overline_position(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::OverlinePosition)
	}

	/// Set the `overline-thickness` attribute.
	pub fn with_overline_thickness<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::OverlineThickness, value);
		self
	}

	/// Set the `overline-thickness` attribute.
	pub fn set_overline_thickness<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::OverlineThickness, value);
	}

	/// Get the `overline-thickness` attribute.
	pub fn overline_thickness(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::OverlineThickness)
	}

	/// Set the `panose-1` attribute.
	pub fn with_panose_1<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::Panose1, value);
		self
	}

	/// Set the `panose-1` attribute.
	pub fn set_panose_1<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::Panose1, value);
	}

	/// Get the `panose-1` attribute.
	pub fn panose_1(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::Panose1)
	}

	/// Set the `slope` attribute.
	pub fn with_slope<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::Slope, value);
		self
	}

	/// Set the `slope` attribute.
	pub fn set_slope<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::Slope, value);
	}

	/// Get the `slope` attribute.
	pub fn slope(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::Slope)
	}

	/// Set the `stemh` attribute.
	pub fn with_stemh<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::Stemh, value);
		self
	}

	/// Set the `stemh` attribute.
	pub fn set_stemh<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::Stemh, value);
	}

	/// Get the `stemh` attribute.
	pub fn stemh(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::Stemh)
	}

	/// Set the `stemv` attribute.
	pub fn with_stemv<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::Stemv, value);
		self
	}

	/// Set the `stemv` attribute.
	pub fn set_stemv<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::Stemv, value);
	}

	/// Get the `stemv` attribute.
	pub fn stemv(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::Stemv)
	}

	/// Set the `strikethrough-position` attribute.
	pub fn with_strikethrough_position<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::StrikethroughPosition, value);
		self
	}

	/// Set the `strikethrough-position` attribute.
	pub fn set_strikethrough_position<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::StrikethroughPosition, value);
	}

	/// Get the `strikethrough-position` attribute.
	pub fn strikethrough_position(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::StrikethroughPosition)
	}

	/// Set the `strikethrough-thickness` attribute.
	pub fn with_strikethrough_thickness<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::StrikethroughThickness, value);
		self
	}

	/// Set the `strikethrough-thickness` attribute.
	pub fn set_strikethrough_thickness<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::StrikethroughThickness, value);
	}

	/// Get the `strikethrough-thickness` attribute.
	pub fn strikethrough_thickness(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::StrikethroughThickness)
	}

	/// Set the `underline-position` attribute.
	pub fn with_underline_position<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::UnderlinePosition, value);
		self
	}

	/// Set the `underline-position` attribute.
	pub fn set_underline_position<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::UnderlinePosition, value);
	}

	/// Get the `underline-position` attribute.
	pub fn underline_position(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::UnderlinePosition)
	}

	/// Set the `underline-thickness` attribute.
	pub fn with_underline_thickness<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::UnderlineThickness, value);
		self
	}

	/// Set the `underline-thickness` attribute.
	pub fn set_underline_thickness<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::UnderlineThickness, value);
	}

	/// Get the `underline-thickness` attribute.
	pub fn underline_thickness(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::UnderlineThickness)
	}

	/// Set the `unicode-range` attribute.
	pub fn with_unicode_range<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::UnicodeRange, value);
		self
	}

	/// Set the `unicode-range` attribute.
	pub fn set_unicode_range<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::UnicodeRange, value);
	}

	/// Get the `unicode-range` attribute.
	pub fn unicode_range(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::UnicodeRange)
	}

	/// Set the `units-per-em` attribute.
	pub fn with_units_per_em<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::UnitsPerEm, value);
		self
	}

	/// Set the `units-per-em` attribute.
	pub fn set_units_per_em<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::UnitsPerEm, value);
	}

	/// Get the `units-per-em` attribute.
	pub fn units_per_em(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::UnitsPerEm)
	}

	/// Set the `v-alphabetic` attribute.
	pub fn with_v_alphabetic<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::VAlphabetic, value);
		self
	}

	/// Set the `v-alphabetic` attribute.
	pub fn set_v_alphabetic<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::VAlphabetic, value);
	}

	/// Get the `v-alphabetic` attribute.
	pub fn v_alphabetic(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::VAlphabetic)
	}

	/// Set the `v-hanging` attribute.
	pub fn with_v_hanging<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::VHanging, value);
		self
	}

	/// Set the `v-hanging` attribute.
	pub fn set_v_hanging<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::VHanging, value);
	}

	/// Get the `v-hanging` attribute.
	pub fn v_hanging(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::VHanging)
	}

	/// Set the `v-ideographic` attribute.
	pub fn with_v_ideographic<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::VIdeographic, value);
		self
	}

	/// Set the `v-ideographic` attribute.
	pub fn set_v_ideographic<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::VIdeographic, value);
	}

	/// Get the `v-ideographic` attribute.
	pub fn v_ideographic(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::VIdeographic)
	}

	/// Set the `v-mathematical` attribute.
	pub fn with_v_mathematical<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::VMathematical, value);
		self
	}

	/// Set the `v-mathematical` attribute.
	pub fn set_v_mathematical<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::VMathematical, value);
	}

	/// Get the `v-mathematical` attribute.
	pub fn v_mathematical(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::VMathematical)
	}

	/// Set the `widths` attribute.
	pub fn with_widths<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::Widths, value);
		self
	}

	/// Set the `widths` attribute.
	pub fn set_widths<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::Widths, value);
	}

	/// Get the `widths` attribute.
	pub fn widths(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::Widths)
	}

	/// Set the `x-height` attribute.
	pub fn with_x_height<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::XHeight, value);
		self
	}

	/// Set the `x-height` attribute.
	pub fn set_x_height<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::XHeight, value);
	}

	/// Get the `x-height` attribute.
	pub fn x_height(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::XHeight)
	}
}

impl common_attrs::CoreAttributesSetter for FontFace {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FontFaceAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FontFace {}

impl Tag for FontFace {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("font-face");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FontFaceFormatAttrs {
	String,
	CoreAttributes(common_attrs::CoreAttributes),
}

impl From<common_attrs::CoreAttributes> for FontFaceFormatAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl FontFaceFormatAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::String => "string",
			Self::CoreAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FontFaceFormatAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FontFaceFormatAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[doc = "The [`<font-face-format>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("font-face-format.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n"
)]
#[doc = "\n\n [`<font-face-format>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/font-face-format"]
#[derive(Debug)]
pub struct FontFaceFormat {
	attrs: IndexMap<FontFaceFormatAttrs, Box<dyn Value>>,
}

impl Default for FontFaceFormat {
	fn default() -> Self {
		Self::new()
	}
}

impl FontFaceFormat {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
		}
	}

	fn set_attr<V>(&mut self, attr: FontFaceFormatAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FontFaceFormatAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `string` attribute.
	pub fn with_string<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceFormatAttrs::String, value);
		self
	}

	/// Set the `string` attribute.
	pub fn set_string<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceFormatAttrs::String, value);
	}

	/// Get the `string` attribute.
	pub fn string(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceFormatAttrs::String)
	}
}

impl common_attrs::CoreAttributesSetter for FontFaceFormat {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceFormatAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FontFaceFormatAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FontFaceFormat {}

impl Tag for FontFaceFormat {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("font-face-format");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FontFaceNameAttrs {
	Name,
	CoreAttributes(common_attrs::CoreAttributes),
}

impl From<common_attrs::CoreAttributes> for FontFaceNameAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl FontFaceNameAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Name => "name",
			Self::CoreAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FontFaceNameAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FontFaceNameAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[doc = "The [`<font-face-name>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("font-face-name.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n"
)]
#[doc = "\n\n [`<font-face-name>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/font-face-name"]
#[derive(Debug)]
pub struct FontFaceName {
	attrs: IndexMap<FontFaceNameAttrs, Box<dyn Value>>,
}

impl Default for FontFaceName {
	fn default() -> Self {
		Self::new()
	}
}

impl FontFaceName {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
		}
	}

	fn set_attr<V>(&mut self, attr: FontFaceNameAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FontFaceNameAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `name` attribute.
	pub fn with_name<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceNameAttrs::Name, value);
		self
	}

	/// Set the `name` attribute.
	pub fn set_name<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceNameAttrs::Name, value);
	}

	/// Get the `name` attribute.
	pub fn name(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceNameAttrs::Name)
	}
}

impl common_attrs::CoreAttributesSetter for FontFaceName {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceNameAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FontFaceNameAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FontFaceName {}

impl Tag for FontFaceName {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("font-face-name");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FontFaceSrcAttrs {
	CoreAttributes(common_attrs::CoreAttributes),
}

impl From<common_attrs::CoreAttributes> for FontFaceSrcAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl FontFaceSrcAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::CoreAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FontFaceSrcAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FontFaceSrcAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod font_face_src_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::FontFaceName {}
	impl Content for super::FontFaceUri {}
}

#[doc = "The [`<font-face-src>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("font-face-src.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<font-face-name>`](FontFaceName)\n",
	"- [`<font-face-uri>`](FontFaceUri)\n"
)]
#[doc = "\n\n [`<font-face-src>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/font-face-src"]
#[derive(Debug)]
pub struct FontFaceSrc {
	attrs: IndexMap<FontFaceSrcAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn font_face_src_private::Content>>,
}

impl Default for FontFaceSrc {
	fn default() -> Self {
		Self::new()
	}
}

impl FontFaceSrc {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: font_face_src_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: font_face_src_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: FontFaceSrcAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FontFaceSrcAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}
}

impl common_attrs::CoreAttributesSetter for FontFaceSrc {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceSrcAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FontFaceSrcAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FontFaceSrc {}

impl Tag for FontFaceSrc {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("font-face-src");
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FontFaceUriAttrs {
	XlinkHref,
	CoreAttributes(common_attrs::CoreAttributes),
	XLinkAttributes(common_attrs::XLinkAttributes),
}

impl From<common_attrs::CoreAttributes> for FontFaceUriAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::XLinkAttributes> for FontFaceUriAttrs {
	fn from(attr: common_attrs::XLinkAttributes) -> Self {
		Self::XLinkAttributes(attr)
	}
}

impl FontFaceUriAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::XlinkHref => "xlink:href",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::XLinkAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for FontFaceUriAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FontFaceUriAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod font_face_uri_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::FontFaceFormat {}
}

#[doc = "The [`<font-face-uri>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("font-face-uri.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<font-face-format>`](FontFaceFormat)\n"
)]
#[doc = "\n\n [`<font-face-uri>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/font-face-uri"]
#[derive(Debug)]
pub struct FontFaceUri {
	attrs: IndexMap<FontFaceUriAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn font_face_uri_private::Content>>,
}

impl Default for FontFaceUri {
	fn default() -> Self {
		Self::new()
	}
}

impl FontFaceUri {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: font_face_uri_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: font_face_uri_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: FontFaceUriAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: FontFaceUriAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceUriAttrs::XlinkHref, value);
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceUriAttrs::XlinkHref, value);
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&dyn Value> {
		self.get_attr(FontFaceUriAttrs::XlinkHref)
	}
}

impl common_attrs::CoreAttributesSetter for FontFaceUri {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceUriAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(FontFaceUriAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FontFaceUri {}

impl common_attrs::XLinkAttributesSetter for FontFaceUri {
	fn set_attr<V>(&mut self, attr: common_attrs::XLinkAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FontFaceUriAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&dyn Value> {
		self.get_attr(FontFaceUriAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for FontFaceUri {}

impl Tag for FontFaceUri {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("font-face-uri");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum ForeignObjectAttrs {
	Class,
	ExternalResourcesRequired,
	Height,
	Style,
	Transform,
	Width,
	X,
	Y,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	GlobalEventAttributes(common_attrs::GlobalEventAttributes),
	GraphicalEventAttributes(common_attrs::GraphicalEventAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::ConditionalProcessingAttributes> for ForeignObjectAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for ForeignObjectAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::GlobalEventAttributes> for ForeignObjectAttrs {
	fn from(attr: common_attrs::GlobalEventAttributes) -> Self {
		Self::GlobalEventAttributes(attr)
	}
}

impl From<common_attrs::GraphicalEventAttributes> for ForeignObjectAttrs {
	fn from(attr: common_attrs::GraphicalEventAttributes) -> Self {
		Self::GraphicalEventAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for ForeignObjectAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl ForeignObjectAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Height => "height",
			Self::Style => "style",
			Self::Transform => "transform",
			Self::Width => "width",
			Self::X => "x",
			Self::Y => "y",
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::GlobalEventAttributes(attr) => attr.as_str(),
			Self::GraphicalEventAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for ForeignObjectAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for ForeignObjectAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[doc = "The [`<foreignObject>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("foreignObject.md")]
#[doc = "\n\n [`<foreignObject>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/foreignObject"]
#[derive(Debug)]
pub struct ForeignObject {
	attrs: IndexMap<ForeignObjectAttrs, Box<dyn Value>>,
	content: String,
}

impl Default for ForeignObject {
	fn default() -> Self {
		Self::new()
	}
}

impl ForeignObject {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: String::new()
		}
	}

	pub fn push<T: Display>(&mut self, content: T) {
		write!(self.content, "{content}").unwrap();
	}

	pub fn append<T: Display>(mut self, content: T) -> Self {
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: ForeignObjectAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: ForeignObjectAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ForeignObjectAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ForeignObjectAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(ForeignObjectAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ForeignObjectAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ForeignObjectAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(ForeignObjectAttrs::ExternalResourcesRequired)
	}

	/// Set the `height` attribute.
	pub fn with_height<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ForeignObjectAttrs::Height, value);
		self
	}

	/// Set the `height` attribute.
	pub fn set_height<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ForeignObjectAttrs::Height, value);
	}

	/// Get the `height` attribute.
	pub fn height(&self) -> Option<&dyn Value> {
		self.get_attr(ForeignObjectAttrs::Height)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ForeignObjectAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ForeignObjectAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(ForeignObjectAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ForeignObjectAttrs::Transform, value);
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ForeignObjectAttrs::Transform, value);
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&dyn Value> {
		self.get_attr(ForeignObjectAttrs::Transform)
	}

	/// Set the `width` attribute.
	pub fn with_width<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ForeignObjectAttrs::Width, value);
		self
	}

	/// Set the `width` attribute.
	pub fn set_width<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ForeignObjectAttrs::Width, value);
	}

	/// Get the `width` attribute.
	pub fn width(&self) -> Option<&dyn Value> {
		self.get_attr(ForeignObjectAttrs::Width)
	}

	/// Set the `x` attribute.
	pub fn with_x<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ForeignObjectAttrs::X, value);
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ForeignObjectAttrs::X, value);
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&dyn Value> {
		self.get_attr(ForeignObjectAttrs::X)
	}

	/// Set the `y` attribute.
	pub fn with_y<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ForeignObjectAttrs::Y, value);
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ForeignObjectAttrs::Y, value);
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&dyn Value> {
		self.get_attr(ForeignObjectAttrs::Y)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for ForeignObject {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ForeignObjectAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(ForeignObjectAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for ForeignObject {}

impl common_attrs::CoreAttributesSetter for ForeignObject {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ForeignObjectAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(ForeignObjectAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for ForeignObject {}

impl common_attrs::GlobalEventAttributesSetter for ForeignObject {
	fn set_attr<V>(&mut self, attr: common_attrs::GlobalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ForeignObjectAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GlobalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(ForeignObjectAttrs::from(attr))
	}
}

impl TagWithGlobalEventAttributes for ForeignObject {}

impl common_attrs::GraphicalEventAttributesSetter for ForeignObject {
	fn set_attr<V>(&mut self, attr: common_attrs::GraphicalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ForeignObjectAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(ForeignObjectAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for ForeignObject {}

impl common_attrs::PresentationAttributesSetter for ForeignObject {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ForeignObjectAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(ForeignObjectAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for ForeignObject {}

impl Tag for ForeignObject {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("foreignObject");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		w.write_cdata_text(&self.content);
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum GroupAttrs {
	Class,
	ExternalResourcesRequired,
	Style,
	Transform,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	GlobalEventAttributes(common_attrs::GlobalEventAttributes),
	GraphicalEventAttributes(common_attrs::GraphicalEventAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::ConditionalProcessingAttributes> for GroupAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for GroupAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::GlobalEventAttributes> for GroupAttrs {
	fn from(attr: common_attrs::GlobalEventAttributes) -> Self {
		Self::GlobalEventAttributes(attr)
	}
}

impl From<common_attrs::GraphicalEventAttributes> for GroupAttrs {
	fn from(attr: common_attrs::GraphicalEventAttributes) -> Self {
		Self::GraphicalEventAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for GroupAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl GroupAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Style => "style",
			Self::Transform => "transform",
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::GlobalEventAttributes(attr) => attr.as_str(),
			Self::GraphicalEventAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for GroupAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for GroupAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod g_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::A {}
	impl Content for super::AltGlyphDef {}
	impl Content for super::Animate {}
	impl Content for super::AnimateColor {}
	impl Content for super::AnimateMotion {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Circle {}
	impl Content for super::ClipPath {}
	impl Content for super::ColorProfile {}
	impl Content for super::Cursor {}
	impl Content for super::Defs {}
	impl Content for super::Desc {}
	impl Content for super::Discard {}
	impl Content for super::Ellipse {}
	impl Content for super::Filter {}
	impl Content for super::Font {}
	impl Content for super::FontFace {}
	impl Content for super::ForeignObject {}
	impl Content for super::Group {}
	impl Content for super::Image {}
	impl Content for super::Line {}
	impl Content for super::LinearGradient {}
	impl Content for super::Marker {}
	impl Content for super::Mask {}
	impl Content for super::Metadata {}
	impl Content for super::Mpath {}
	impl Content for super::Path {}
	impl Content for super::Pattern {}
	impl Content for super::Polygon {}
	impl Content for super::Polyline {}
	impl Content for super::RadialGradient {}
	impl Content for super::Rect {}
	impl Content for super::Script {}
	impl Content for super::Set {}
	impl Content for super::Stop {}
	impl Content for super::Style {}
	impl Content for super::Svg {}
	impl Content for super::Switch {}
	impl Content for super::Symbol {}
	impl Content for super::Text {}
	impl Content for super::Title {}
	impl Content for super::Use {}
	impl Content for super::View {}
}

#[doc = "The [`<g>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("g.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<a>`](A)\n",
	"- [`<altGlyphDef>`](AltGlyphDef)\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateColor>`](AnimateColor)\n",
	"- [`<animateMotion>`](AnimateMotion)\n",
	"- [`<animateTransform>`](AnimateTransform)\n",
	"- [`<circle>`](Circle)\n",
	"- [`<clipPath>`](ClipPath)\n",
	"- [`<color-profile>`](ColorProfile)\n",
	"- [`<cursor>`](Cursor)\n",
	"- [`<defs>`](Defs)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<discard>`](Discard)\n",
	"- [`<ellipse>`](Ellipse)\n",
	"- [`<filter>`](Filter)\n",
	"- [`<font>`](Font)\n",
	"- [`<font-face>`](FontFace)\n",
	"- [`<foreignObject>`](ForeignObject)\n",
	"- [`<g>`](Group)\n",
	"- [`<image>`](Image)\n",
	"- [`<line>`](Line)\n",
	"- [`<linearGradient>`](LinearGradient)\n",
	"- [`<marker>`](Marker)\n",
	"- [`<mask>`](Mask)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<mpath>`](Mpath)\n",
	"- [`<path>`](Path)\n",
	"- [`<pattern>`](Pattern)\n",
	"- [`<polygon>`](Polygon)\n",
	"- [`<polyline>`](Polyline)\n",
	"- [`<radialGradient>`](RadialGradient)\n",
	"- [`<rect>`](Rect)\n",
	"- [`<script>`](Script)\n",
	"- [`<set>`](Set)\n",
	"- [`<stop>`](Stop)\n",
	"- [`<style>`](Style)\n",
	"- [`<svg>`](Svg)\n",
	"- [`<switch>`](Switch)\n",
	"- [`<symbol>`](Symbol)\n",
	"- [`<text>`](Text)\n",
	"- [`<title>`](Title)\n",
	"- [`<use>`](Use)\n",
	"- [`<view>`](View)\n"
)]
#[doc = "\n\n [`<g>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/g"]
#[derive(Debug)]
pub struct Group {
	attrs: IndexMap<GroupAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn g_private::Content>>,
}

impl Default for Group {
	fn default() -> Self {
		Self::new()
	}
}

impl Group {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: g_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: g_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: GroupAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: GroupAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GroupAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GroupAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(GroupAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GroupAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GroupAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(GroupAttrs::ExternalResourcesRequired)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GroupAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GroupAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(GroupAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GroupAttrs::Transform, value);
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GroupAttrs::Transform, value);
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&dyn Value> {
		self.get_attr(GroupAttrs::Transform)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Group {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GroupAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(GroupAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Group {}

impl common_attrs::CoreAttributesSetter for Group {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GroupAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(GroupAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Group {}

impl common_attrs::GlobalEventAttributesSetter for Group {
	fn set_attr<V>(&mut self, attr: common_attrs::GlobalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GroupAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GlobalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(GroupAttrs::from(attr))
	}
}

impl TagWithGlobalEventAttributes for Group {}

impl common_attrs::GraphicalEventAttributesSetter for Group {
	fn set_attr<V>(&mut self, attr: common_attrs::GraphicalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GroupAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(GroupAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Group {}

impl common_attrs::PresentationAttributesSetter for Group {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GroupAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(GroupAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Group {}

impl Tag for Group {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("g");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum GlyphAttrs {
	ArabicForm,
	Class,
	D,
	GlyphName,
	HorizAdvX,
	Lang,
	Orientation,
	Style,
	Unicode,
	VertAdvY,
	VertOriginX,
	VertOriginY,
	CoreAttributes(common_attrs::CoreAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::CoreAttributes> for GlyphAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for GlyphAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl GlyphAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::ArabicForm => "arabic-form",
			Self::Class => "class",
			Self::D => "d",
			Self::GlyphName => "glyph-name",
			Self::HorizAdvX => "horiz-adv-x",
			Self::Lang => "lang",
			Self::Orientation => "orientation",
			Self::Style => "style",
			Self::Unicode => "unicode",
			Self::VertAdvY => "vert-adv-y",
			Self::VertOriginX => "vert-origin-x",
			Self::VertOriginY => "vert-origin-y",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for GlyphAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for GlyphAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod glyph_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::A {}
	impl Content for super::AltGlyphDef {}
	impl Content for super::Animate {}
	impl Content for super::AnimateColor {}
	impl Content for super::AnimateMotion {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Circle {}
	impl Content for super::ClipPath {}
	impl Content for super::ColorProfile {}
	impl Content for super::Cursor {}
	impl Content for super::Defs {}
	impl Content for super::Desc {}
	impl Content for super::Discard {}
	impl Content for super::Ellipse {}
	impl Content for super::Filter {}
	impl Content for super::Font {}
	impl Content for super::FontFace {}
	impl Content for super::ForeignObject {}
	impl Content for super::Group {}
	impl Content for super::Image {}
	impl Content for super::Line {}
	impl Content for super::LinearGradient {}
	impl Content for super::Marker {}
	impl Content for super::Mask {}
	impl Content for super::Metadata {}
	impl Content for super::Mpath {}
	impl Content for super::Path {}
	impl Content for super::Pattern {}
	impl Content for super::Polygon {}
	impl Content for super::Polyline {}
	impl Content for super::RadialGradient {}
	impl Content for super::Rect {}
	impl Content for super::Script {}
	impl Content for super::Set {}
	impl Content for super::Stop {}
	impl Content for super::Style {}
	impl Content for super::Svg {}
	impl Content for super::Switch {}
	impl Content for super::Symbol {}
	impl Content for super::Text {}
	impl Content for super::Title {}
	impl Content for super::Use {}
	impl Content for super::View {}
}

#[doc = "The [`<glyph>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("glyph.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<a>`](A)\n",
	"- [`<altGlyphDef>`](AltGlyphDef)\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateColor>`](AnimateColor)\n",
	"- [`<animateMotion>`](AnimateMotion)\n",
	"- [`<animateTransform>`](AnimateTransform)\n",
	"- [`<circle>`](Circle)\n",
	"- [`<clipPath>`](ClipPath)\n",
	"- [`<color-profile>`](ColorProfile)\n",
	"- [`<cursor>`](Cursor)\n",
	"- [`<defs>`](Defs)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<discard>`](Discard)\n",
	"- [`<ellipse>`](Ellipse)\n",
	"- [`<filter>`](Filter)\n",
	"- [`<font>`](Font)\n",
	"- [`<font-face>`](FontFace)\n",
	"- [`<foreignObject>`](ForeignObject)\n",
	"- [`<g>`](Group)\n",
	"- [`<image>`](Image)\n",
	"- [`<line>`](Line)\n",
	"- [`<linearGradient>`](LinearGradient)\n",
	"- [`<marker>`](Marker)\n",
	"- [`<mask>`](Mask)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<mpath>`](Mpath)\n",
	"- [`<path>`](Path)\n",
	"- [`<pattern>`](Pattern)\n",
	"- [`<polygon>`](Polygon)\n",
	"- [`<polyline>`](Polyline)\n",
	"- [`<radialGradient>`](RadialGradient)\n",
	"- [`<rect>`](Rect)\n",
	"- [`<script>`](Script)\n",
	"- [`<set>`](Set)\n",
	"- [`<stop>`](Stop)\n",
	"- [`<style>`](Style)\n",
	"- [`<svg>`](Svg)\n",
	"- [`<switch>`](Switch)\n",
	"- [`<symbol>`](Symbol)\n",
	"- [`<text>`](Text)\n",
	"- [`<title>`](Title)\n",
	"- [`<use>`](Use)\n",
	"- [`<view>`](View)\n"
)]
#[doc = "\n\n [`<glyph>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/glyph"]
#[derive(Debug)]
pub struct Glyph {
	attrs: IndexMap<GlyphAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn glyph_private::Content>>,
}

impl Default for Glyph {
	fn default() -> Self {
		Self::new()
	}
}

impl Glyph {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: glyph_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: glyph_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: GlyphAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: GlyphAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `arabic-form` attribute.
	pub fn with_arabic_form<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlyphAttrs::ArabicForm, value);
		self
	}

	/// Set the `arabic-form` attribute.
	pub fn set_arabic_form<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlyphAttrs::ArabicForm, value);
	}

	/// Get the `arabic-form` attribute.
	pub fn arabic_form(&self) -> Option<&dyn Value> {
		self.get_attr(GlyphAttrs::ArabicForm)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlyphAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlyphAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(GlyphAttrs::Class)
	}

	/// Set the `d` attribute.
	pub fn with_d<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlyphAttrs::D, value);
		self
	}

	/// Set the `d` attribute.
	pub fn set_d<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlyphAttrs::D, value);
	}

	/// Get the `d` attribute.
	pub fn d(&self) -> Option<&dyn Value> {
		self.get_attr(GlyphAttrs::D)
	}

	/// Set the `glyph-name` attribute.
	pub fn with_glyph_name<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlyphAttrs::GlyphName, value);
		self
	}

	/// Set the `glyph-name` attribute.
	pub fn set_glyph_name<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlyphAttrs::GlyphName, value);
	}

	/// Get the `glyph-name` attribute.
	pub fn glyph_name(&self) -> Option<&dyn Value> {
		self.get_attr(GlyphAttrs::GlyphName)
	}

	/// Set the `horiz-adv-x` attribute.
	pub fn with_horiz_adv_x<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlyphAttrs::HorizAdvX, value);
		self
	}

	/// Set the `horiz-adv-x` attribute.
	pub fn set_horiz_adv_x<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlyphAttrs::HorizAdvX, value);
	}

	/// Get the `horiz-adv-x` attribute.
	pub fn horiz_adv_x(&self) -> Option<&dyn Value> {
		self.get_attr(GlyphAttrs::HorizAdvX)
	}

	/// Set the `lang` attribute.
	pub fn with_lang<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlyphAttrs::Lang, value);
		self
	}

	/// Set the `lang` attribute.
	pub fn set_lang<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlyphAttrs::Lang, value);
	}

	/// Get the `lang` attribute.
	pub fn lang(&self) -> Option<&dyn Value> {
		self.get_attr(GlyphAttrs::Lang)
	}

	/// Set the `orientation` attribute.
	pub fn with_orientation<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlyphAttrs::Orientation, value);
		self
	}

	/// Set the `orientation` attribute.
	pub fn set_orientation<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlyphAttrs::Orientation, value);
	}

	/// Get the `orientation` attribute.
	pub fn orientation(&self) -> Option<&dyn Value> {
		self.get_attr(GlyphAttrs::Orientation)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlyphAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlyphAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(GlyphAttrs::Style)
	}

	/// Set the `unicode` attribute.
	pub fn with_unicode<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlyphAttrs::Unicode, value);
		self
	}

	/// Set the `unicode` attribute.
	pub fn set_unicode<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlyphAttrs::Unicode, value);
	}

	/// Get the `unicode` attribute.
	pub fn unicode(&self) -> Option<&dyn Value> {
		self.get_attr(GlyphAttrs::Unicode)
	}

	/// Set the `vert-adv-y` attribute.
	pub fn with_vert_adv_y<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlyphAttrs::VertAdvY, value);
		self
	}

	/// Set the `vert-adv-y` attribute.
	pub fn set_vert_adv_y<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlyphAttrs::VertAdvY, value);
	}

	/// Get the `vert-adv-y` attribute.
	pub fn vert_adv_y(&self) -> Option<&dyn Value> {
		self.get_attr(GlyphAttrs::VertAdvY)
	}

	/// Set the `vert-origin-x` attribute.
	pub fn with_vert_origin_x<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlyphAttrs::VertOriginX, value);
		self
	}

	/// Set the `vert-origin-x` attribute.
	pub fn set_vert_origin_x<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlyphAttrs::VertOriginX, value);
	}

	/// Get the `vert-origin-x` attribute.
	pub fn vert_origin_x(&self) -> Option<&dyn Value> {
		self.get_attr(GlyphAttrs::VertOriginX)
	}

	/// Set the `vert-origin-y` attribute.
	pub fn with_vert_origin_y<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlyphAttrs::VertOriginY, value);
		self
	}

	/// Set the `vert-origin-y` attribute.
	pub fn set_vert_origin_y<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlyphAttrs::VertOriginY, value);
	}

	/// Get the `vert-origin-y` attribute.
	pub fn vert_origin_y(&self) -> Option<&dyn Value> {
		self.get_attr(GlyphAttrs::VertOriginY)
	}
}

impl common_attrs::CoreAttributesSetter for Glyph {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlyphAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(GlyphAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Glyph {}

impl common_attrs::PresentationAttributesSetter for Glyph {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlyphAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(GlyphAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Glyph {}

impl Tag for Glyph {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("glyph");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum GlyphRefAttrs {
	Class,
	Dx,
	Dy,
	Format,
	GlyphRef,
	Style,
	X,
	XlinkHref,
	Y,
	CoreAttributes(common_attrs::CoreAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
	XLinkAttributes(common_attrs::XLinkAttributes),
}

impl From<common_attrs::CoreAttributes> for GlyphRefAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for GlyphRefAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl From<common_attrs::XLinkAttributes> for GlyphRefAttrs {
	fn from(attr: common_attrs::XLinkAttributes) -> Self {
		Self::XLinkAttributes(attr)
	}
}

impl GlyphRefAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::Dx => "dx",
			Self::Dy => "dy",
			Self::Format => "format",
			Self::GlyphRef => "glyphRef",
			Self::Style => "style",
			Self::X => "x",
			Self::XlinkHref => "xlink:href",
			Self::Y => "y",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
			Self::XLinkAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for GlyphRefAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for GlyphRefAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[doc = "The [`<glyphRef>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("glyphRef.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n"
)]
#[doc = "\n\n [`<glyphRef>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/glyphRef"]
#[derive(Debug)]
pub struct GlyphRef {
	attrs: IndexMap<GlyphRefAttrs, Box<dyn Value>>,
}

impl Default for GlyphRef {
	fn default() -> Self {
		Self::new()
	}
}

impl GlyphRef {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
		}
	}

	fn set_attr<V>(&mut self, attr: GlyphRefAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: GlyphRefAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlyphRefAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlyphRefAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(GlyphRefAttrs::Class)
	}

	/// Set the `dx` attribute.
	pub fn with_dx<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlyphRefAttrs::Dx, value);
		self
	}

	/// Set the `dx` attribute.
	pub fn set_dx<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlyphRefAttrs::Dx, value);
	}

	/// Get the `dx` attribute.
	pub fn dx(&self) -> Option<&dyn Value> {
		self.get_attr(GlyphRefAttrs::Dx)
	}

	/// Set the `dy` attribute.
	pub fn with_dy<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlyphRefAttrs::Dy, value);
		self
	}

	/// Set the `dy` attribute.
	pub fn set_dy<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlyphRefAttrs::Dy, value);
	}

	/// Get the `dy` attribute.
	pub fn dy(&self) -> Option<&dyn Value> {
		self.get_attr(GlyphRefAttrs::Dy)
	}

	/// Set the `format` attribute.
	pub fn with_format<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlyphRefAttrs::Format, value);
		self
	}

	/// Set the `format` attribute.
	pub fn set_format<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlyphRefAttrs::Format, value);
	}

	/// Get the `format` attribute.
	pub fn format(&self) -> Option<&dyn Value> {
		self.get_attr(GlyphRefAttrs::Format)
	}

	/// Set the `glyphRef` attribute.
	pub fn with_glyph_ref<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlyphRefAttrs::GlyphRef, value);
		self
	}

	/// Set the `glyphRef` attribute.
	pub fn set_glyph_ref<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlyphRefAttrs::GlyphRef, value);
	}

	/// Get the `glyphRef` attribute.
	pub fn glyph_ref(&self) -> Option<&dyn Value> {
		self.get_attr(GlyphRefAttrs::GlyphRef)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlyphRefAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlyphRefAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(GlyphRefAttrs::Style)
	}

	/// Set the `x` attribute.
	pub fn with_x<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlyphRefAttrs::X, value);
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlyphRefAttrs::X, value);
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&dyn Value> {
		self.get_attr(GlyphRefAttrs::X)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlyphRefAttrs::XlinkHref, value);
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlyphRefAttrs::XlinkHref, value);
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&dyn Value> {
		self.get_attr(GlyphRefAttrs::XlinkHref)
	}

	/// Set the `y` attribute.
	pub fn with_y<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlyphRefAttrs::Y, value);
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlyphRefAttrs::Y, value);
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&dyn Value> {
		self.get_attr(GlyphRefAttrs::Y)
	}
}

impl common_attrs::CoreAttributesSetter for GlyphRef {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlyphRefAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(GlyphRefAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for GlyphRef {}

impl common_attrs::PresentationAttributesSetter for GlyphRef {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlyphRefAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(GlyphRefAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for GlyphRef {}

impl common_attrs::XLinkAttributesSetter for GlyphRef {
	fn set_attr<V>(&mut self, attr: common_attrs::XLinkAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlyphRefAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&dyn Value> {
		self.get_attr(GlyphRefAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for GlyphRef {}

impl Tag for GlyphRef {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("glyphRef");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum HatchAttrs {
	HatchContentUnits,
	HatchUnits,
	Href,
	Pitch,
	Rotate,
	Transform,
	X,
	Y,
	CoreAttributes(common_attrs::CoreAttributes),
	GlobalEventAttributes(common_attrs::GlobalEventAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
	StyleAttributes(common_attrs::StyleAttributes),
}

impl From<common_attrs::CoreAttributes> for HatchAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::GlobalEventAttributes> for HatchAttrs {
	fn from(attr: common_attrs::GlobalEventAttributes) -> Self {
		Self::GlobalEventAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for HatchAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl From<common_attrs::StyleAttributes> for HatchAttrs {
	fn from(attr: common_attrs::StyleAttributes) -> Self {
		Self::StyleAttributes(attr)
	}
}

impl HatchAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::HatchContentUnits => "hatchContentUnits",
			Self::HatchUnits => "hatchUnits",
			Self::Href => "href",
			Self::Pitch => "pitch",
			Self::Rotate => "rotate",
			Self::Transform => "transform",
			Self::X => "x",
			Self::Y => "y",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::GlobalEventAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
			Self::StyleAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for HatchAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for HatchAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod hatch_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::AnimateMotion {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Desc {}
	impl Content for super::Discard {}
	impl Content for super::Hatchpath {}
	impl Content for super::Metadata {}
	impl Content for super::Mpath {}
	impl Content for super::Script {}
	impl Content for super::Set {}
	impl Content for super::Style {}
	impl Content for super::Title {}
}

#[doc = "The [`<hatch>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("hatch.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateMotion>`](AnimateMotion)\n",
	"- [`<animateTransform>`](AnimateTransform)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<discard>`](Discard)\n",
	"- [`<hatchpath>`](Hatchpath)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<mpath>`](Mpath)\n",
	"- [`<script>`](Script)\n",
	"- [`<set>`](Set)\n",
	"- [`<style>`](Style)\n",
	"- [`<title>`](Title)\n"
)]
#[doc = "\n\n [`<hatch>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/hatch"]
#[derive(Debug)]
pub struct Hatch {
	attrs: IndexMap<HatchAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn hatch_private::Content>>,
}

impl Default for Hatch {
	fn default() -> Self {
		Self::new()
	}
}

impl Hatch {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: hatch_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: hatch_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: HatchAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: HatchAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `hatchContentUnits` attribute.
	pub fn with_hatch_content_units<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(HatchAttrs::HatchContentUnits, value);
		self
	}

	/// Set the `hatchContentUnits` attribute.
	pub fn set_hatch_content_units<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(HatchAttrs::HatchContentUnits, value);
	}

	/// Get the `hatchContentUnits` attribute.
	pub fn hatch_content_units(&self) -> Option<&dyn Value> {
		self.get_attr(HatchAttrs::HatchContentUnits)
	}

	/// Set the `hatchUnits` attribute.
	pub fn with_hatch_units<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(HatchAttrs::HatchUnits, value);
		self
	}

	/// Set the `hatchUnits` attribute.
	pub fn set_hatch_units<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(HatchAttrs::HatchUnits, value);
	}

	/// Get the `hatchUnits` attribute.
	pub fn hatch_units(&self) -> Option<&dyn Value> {
		self.get_attr(HatchAttrs::HatchUnits)
	}

	/// Set the `href` attribute.
	pub fn with_href<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(HatchAttrs::Href, value);
		self
	}

	/// Set the `href` attribute.
	pub fn set_href<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(HatchAttrs::Href, value);
	}

	/// Get the `href` attribute.
	pub fn href(&self) -> Option<&dyn Value> {
		self.get_attr(HatchAttrs::Href)
	}

	/// Set the `pitch` attribute.
	pub fn with_pitch<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(HatchAttrs::Pitch, value);
		self
	}

	/// Set the `pitch` attribute.
	pub fn set_pitch<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(HatchAttrs::Pitch, value);
	}

	/// Get the `pitch` attribute.
	pub fn pitch(&self) -> Option<&dyn Value> {
		self.get_attr(HatchAttrs::Pitch)
	}

	/// Set the `rotate` attribute.
	pub fn with_rotate<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(HatchAttrs::Rotate, value);
		self
	}

	/// Set the `rotate` attribute.
	pub fn set_rotate<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(HatchAttrs::Rotate, value);
	}

	/// Get the `rotate` attribute.
	pub fn rotate(&self) -> Option<&dyn Value> {
		self.get_attr(HatchAttrs::Rotate)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(HatchAttrs::Transform, value);
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(HatchAttrs::Transform, value);
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&dyn Value> {
		self.get_attr(HatchAttrs::Transform)
	}

	/// Set the `x` attribute.
	pub fn with_x<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(HatchAttrs::X, value);
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(HatchAttrs::X, value);
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&dyn Value> {
		self.get_attr(HatchAttrs::X)
	}

	/// Set the `y` attribute.
	pub fn with_y<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(HatchAttrs::Y, value);
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(HatchAttrs::Y, value);
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&dyn Value> {
		self.get_attr(HatchAttrs::Y)
	}
}

impl common_attrs::CoreAttributesSetter for Hatch {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(HatchAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(HatchAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Hatch {}

impl common_attrs::GlobalEventAttributesSetter for Hatch {
	fn set_attr<V>(&mut self, attr: common_attrs::GlobalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(HatchAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GlobalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(HatchAttrs::from(attr))
	}
}

impl TagWithGlobalEventAttributes for Hatch {}

impl common_attrs::PresentationAttributesSetter for Hatch {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(HatchAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(HatchAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Hatch {}

impl common_attrs::StyleAttributesSetter for Hatch {
	fn set_attr<V>(&mut self, attr: common_attrs::StyleAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(HatchAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::StyleAttributes) -> Option<&dyn Value> {
		self.get_attr(HatchAttrs::from(attr))
	}
}

impl TagWithStyleAttributes for Hatch {}

impl Tag for Hatch {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("hatch");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum HatchpathAttrs {
	D,
	Offset,
	CoreAttributes(common_attrs::CoreAttributes),
	GlobalEventAttributes(common_attrs::GlobalEventAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
	StyleAttributes(common_attrs::StyleAttributes),
}

impl From<common_attrs::CoreAttributes> for HatchpathAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::GlobalEventAttributes> for HatchpathAttrs {
	fn from(attr: common_attrs::GlobalEventAttributes) -> Self {
		Self::GlobalEventAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for HatchpathAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl From<common_attrs::StyleAttributes> for HatchpathAttrs {
	fn from(attr: common_attrs::StyleAttributes) -> Self {
		Self::StyleAttributes(attr)
	}
}

impl HatchpathAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::D => "d",
			Self::Offset => "offset",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::GlobalEventAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
			Self::StyleAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for HatchpathAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for HatchpathAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod hatchpath_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::AnimateMotion {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Desc {}
	impl Content for super::Discard {}
	impl Content for super::Metadata {}
	impl Content for super::Mpath {}
	impl Content for super::Script {}
	impl Content for super::Set {}
	impl Content for super::Style {}
	impl Content for super::Title {}
}

#[doc = "The [`<hatchpath>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("hatchpath.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateMotion>`](AnimateMotion)\n",
	"- [`<animateTransform>`](AnimateTransform)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<discard>`](Discard)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<mpath>`](Mpath)\n",
	"- [`<script>`](Script)\n",
	"- [`<set>`](Set)\n",
	"- [`<style>`](Style)\n",
	"- [`<title>`](Title)\n"
)]
#[doc = "\n\n [`<hatchpath>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/hatchpath"]
#[derive(Debug)]
pub struct Hatchpath {
	attrs: IndexMap<HatchpathAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn hatchpath_private::Content>>,
}

impl Default for Hatchpath {
	fn default() -> Self {
		Self::new()
	}
}

impl Hatchpath {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: hatchpath_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: hatchpath_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: HatchpathAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: HatchpathAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `d` attribute.
	pub fn with_d<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(HatchpathAttrs::D, value);
		self
	}

	/// Set the `d` attribute.
	pub fn set_d<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(HatchpathAttrs::D, value);
	}

	/// Get the `d` attribute.
	pub fn d(&self) -> Option<&dyn Value> {
		self.get_attr(HatchpathAttrs::D)
	}

	/// Set the `offset` attribute.
	pub fn with_offset<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(HatchpathAttrs::Offset, value);
		self
	}

	/// Set the `offset` attribute.
	pub fn set_offset<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(HatchpathAttrs::Offset, value);
	}

	/// Get the `offset` attribute.
	pub fn offset(&self) -> Option<&dyn Value> {
		self.get_attr(HatchpathAttrs::Offset)
	}
}

impl common_attrs::CoreAttributesSetter for Hatchpath {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(HatchpathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(HatchpathAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Hatchpath {}

impl common_attrs::GlobalEventAttributesSetter for Hatchpath {
	fn set_attr<V>(&mut self, attr: common_attrs::GlobalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(HatchpathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GlobalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(HatchpathAttrs::from(attr))
	}
}

impl TagWithGlobalEventAttributes for Hatchpath {}

impl common_attrs::PresentationAttributesSetter for Hatchpath {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(HatchpathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(HatchpathAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Hatchpath {}

impl common_attrs::StyleAttributesSetter for Hatchpath {
	fn set_attr<V>(&mut self, attr: common_attrs::StyleAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(HatchpathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::StyleAttributes) -> Option<&dyn Value> {
		self.get_attr(HatchpathAttrs::from(attr))
	}
}

impl TagWithStyleAttributes for Hatchpath {}

impl Tag for Hatchpath {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("hatchpath");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum HkernAttrs {
	G1,
	G2,
	K,
	U1,
	U2,
	CoreAttributes(common_attrs::CoreAttributes),
}

impl From<common_attrs::CoreAttributes> for HkernAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl HkernAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::G1 => "g1",
			Self::G2 => "g2",
			Self::K => "k",
			Self::U1 => "u1",
			Self::U2 => "u2",
			Self::CoreAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for HkernAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for HkernAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[doc = "The [`<hkern>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("hkern.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n"
)]
#[doc = "\n\n [`<hkern>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/hkern"]
#[derive(Debug)]
pub struct Hkern {
	attrs: IndexMap<HkernAttrs, Box<dyn Value>>,
}

impl Default for Hkern {
	fn default() -> Self {
		Self::new()
	}
}

impl Hkern {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
		}
	}

	fn set_attr<V>(&mut self, attr: HkernAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: HkernAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `g1` attribute.
	pub fn with_g1<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(HkernAttrs::G1, value);
		self
	}

	/// Set the `g1` attribute.
	pub fn set_g1<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(HkernAttrs::G1, value);
	}

	/// Get the `g1` attribute.
	pub fn g1(&self) -> Option<&dyn Value> {
		self.get_attr(HkernAttrs::G1)
	}

	/// Set the `g2` attribute.
	pub fn with_g2<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(HkernAttrs::G2, value);
		self
	}

	/// Set the `g2` attribute.
	pub fn set_g2<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(HkernAttrs::G2, value);
	}

	/// Get the `g2` attribute.
	pub fn g2(&self) -> Option<&dyn Value> {
		self.get_attr(HkernAttrs::G2)
	}

	/// Set the `k` attribute.
	pub fn with_k<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(HkernAttrs::K, value);
		self
	}

	/// Set the `k` attribute.
	pub fn set_k<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(HkernAttrs::K, value);
	}

	/// Get the `k` attribute.
	pub fn k(&self) -> Option<&dyn Value> {
		self.get_attr(HkernAttrs::K)
	}

	/// Set the `u1` attribute.
	pub fn with_u1<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(HkernAttrs::U1, value);
		self
	}

	/// Set the `u1` attribute.
	pub fn set_u1<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(HkernAttrs::U1, value);
	}

	/// Get the `u1` attribute.
	pub fn u1(&self) -> Option<&dyn Value> {
		self.get_attr(HkernAttrs::U1)
	}

	/// Set the `u2` attribute.
	pub fn with_u2<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(HkernAttrs::U2, value);
		self
	}

	/// Set the `u2` attribute.
	pub fn set_u2<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(HkernAttrs::U2, value);
	}

	/// Get the `u2` attribute.
	pub fn u2(&self) -> Option<&dyn Value> {
		self.get_attr(HkernAttrs::U2)
	}
}

impl common_attrs::CoreAttributesSetter for Hkern {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(HkernAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(HkernAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Hkern {}

impl Tag for Hkern {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("hkern");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum ImageAttrs {
	Class,
	ExternalResourcesRequired,
	Height,
	PreserveAspectRatio,
	Style,
	Transform,
	Width,
	X,
	XlinkHref,
	Y,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	GlobalEventAttributes(common_attrs::GlobalEventAttributes),
	GraphicalEventAttributes(common_attrs::GraphicalEventAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
	XLinkAttributes(common_attrs::XLinkAttributes),
}

impl From<common_attrs::ConditionalProcessingAttributes> for ImageAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for ImageAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::GlobalEventAttributes> for ImageAttrs {
	fn from(attr: common_attrs::GlobalEventAttributes) -> Self {
		Self::GlobalEventAttributes(attr)
	}
}

impl From<common_attrs::GraphicalEventAttributes> for ImageAttrs {
	fn from(attr: common_attrs::GraphicalEventAttributes) -> Self {
		Self::GraphicalEventAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for ImageAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl From<common_attrs::XLinkAttributes> for ImageAttrs {
	fn from(attr: common_attrs::XLinkAttributes) -> Self {
		Self::XLinkAttributes(attr)
	}
}

impl ImageAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Height => "height",
			Self::PreserveAspectRatio => "preserveAspectRatio",
			Self::Style => "style",
			Self::Transform => "transform",
			Self::Width => "width",
			Self::X => "x",
			Self::XlinkHref => "xlink:href",
			Self::Y => "y",
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::GlobalEventAttributes(attr) => attr.as_str(),
			Self::GraphicalEventAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
			Self::XLinkAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for ImageAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for ImageAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod image_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::AnimateMotion {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Desc {}
	impl Content for super::Discard {}
	impl Content for super::Metadata {}
	impl Content for super::Mpath {}
	impl Content for super::Set {}
	impl Content for super::Title {}
}

#[doc = "The [`<image>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("image.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateMotion>`](AnimateMotion)\n",
	"- [`<animateTransform>`](AnimateTransform)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<discard>`](Discard)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<mpath>`](Mpath)\n",
	"- [`<set>`](Set)\n",
	"- [`<title>`](Title)\n"
)]
#[doc = "\n\n [`<image>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/image"]
#[derive(Debug)]
pub struct Image {
	attrs: IndexMap<ImageAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn image_private::Content>>,
}

impl Default for Image {
	fn default() -> Self {
		Self::new()
	}
}

impl Image {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: image_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: image_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: ImageAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: ImageAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ImageAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ImageAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(ImageAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ImageAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ImageAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(ImageAttrs::ExternalResourcesRequired)
	}

	/// Set the `height` attribute.
	pub fn with_height<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ImageAttrs::Height, value);
		self
	}

	/// Set the `height` attribute.
	pub fn set_height<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ImageAttrs::Height, value);
	}

	/// Get the `height` attribute.
	pub fn height(&self) -> Option<&dyn Value> {
		self.get_attr(ImageAttrs::Height)
	}

	/// Set the `preserveAspectRatio` attribute.
	pub fn with_preserve_aspect_ratio<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ImageAttrs::PreserveAspectRatio, value);
		self
	}

	/// Set the `preserveAspectRatio` attribute.
	pub fn set_preserve_aspect_ratio<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ImageAttrs::PreserveAspectRatio, value);
	}

	/// Get the `preserveAspectRatio` attribute.
	pub fn preserve_aspect_ratio(&self) -> Option<&dyn Value> {
		self.get_attr(ImageAttrs::PreserveAspectRatio)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ImageAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ImageAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(ImageAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ImageAttrs::Transform, value);
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ImageAttrs::Transform, value);
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&dyn Value> {
		self.get_attr(ImageAttrs::Transform)
	}

	/// Set the `width` attribute.
	pub fn with_width<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ImageAttrs::Width, value);
		self
	}

	/// Set the `width` attribute.
	pub fn set_width<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ImageAttrs::Width, value);
	}

	/// Get the `width` attribute.
	pub fn width(&self) -> Option<&dyn Value> {
		self.get_attr(ImageAttrs::Width)
	}

	/// Set the `x` attribute.
	pub fn with_x<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ImageAttrs::X, value);
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ImageAttrs::X, value);
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&dyn Value> {
		self.get_attr(ImageAttrs::X)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ImageAttrs::XlinkHref, value);
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ImageAttrs::XlinkHref, value);
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&dyn Value> {
		self.get_attr(ImageAttrs::XlinkHref)
	}

	/// Set the `y` attribute.
	pub fn with_y<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ImageAttrs::Y, value);
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ImageAttrs::Y, value);
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&dyn Value> {
		self.get_attr(ImageAttrs::Y)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Image {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ImageAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(ImageAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Image {}

impl common_attrs::CoreAttributesSetter for Image {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ImageAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(ImageAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Image {}

impl common_attrs::GlobalEventAttributesSetter for Image {
	fn set_attr<V>(&mut self, attr: common_attrs::GlobalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ImageAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GlobalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(ImageAttrs::from(attr))
	}
}

impl TagWithGlobalEventAttributes for Image {}

impl common_attrs::GraphicalEventAttributesSetter for Image {
	fn set_attr<V>(&mut self, attr: common_attrs::GraphicalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ImageAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(ImageAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Image {}

impl common_attrs::PresentationAttributesSetter for Image {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ImageAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(ImageAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Image {}

impl common_attrs::XLinkAttributesSetter for Image {
	fn set_attr<V>(&mut self, attr: common_attrs::XLinkAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ImageAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&dyn Value> {
		self.get_attr(ImageAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for Image {}

impl Tag for Image {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("image");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum LineAttrs {
	Class,
	ExternalResourcesRequired,
	Style,
	Transform,
	X1,
	X2,
	Y1,
	Y2,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	GlobalEventAttributes(common_attrs::GlobalEventAttributes),
	GraphicalEventAttributes(common_attrs::GraphicalEventAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::ConditionalProcessingAttributes> for LineAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for LineAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::GlobalEventAttributes> for LineAttrs {
	fn from(attr: common_attrs::GlobalEventAttributes) -> Self {
		Self::GlobalEventAttributes(attr)
	}
}

impl From<common_attrs::GraphicalEventAttributes> for LineAttrs {
	fn from(attr: common_attrs::GraphicalEventAttributes) -> Self {
		Self::GraphicalEventAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for LineAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl LineAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Style => "style",
			Self::Transform => "transform",
			Self::X1 => "x1",
			Self::X2 => "x2",
			Self::Y1 => "y1",
			Self::Y2 => "y2",
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::GlobalEventAttributes(attr) => attr.as_str(),
			Self::GraphicalEventAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for LineAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for LineAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod line_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::AnimateMotion {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Desc {}
	impl Content for super::Discard {}
	impl Content for super::Metadata {}
	impl Content for super::Mpath {}
	impl Content for super::Set {}
	impl Content for super::Title {}
}

#[doc = "The [`<line>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("line.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateMotion>`](AnimateMotion)\n",
	"- [`<animateTransform>`](AnimateTransform)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<discard>`](Discard)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<mpath>`](Mpath)\n",
	"- [`<set>`](Set)\n",
	"- [`<title>`](Title)\n"
)]
#[doc = "\n\n [`<line>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/line"]
#[derive(Debug)]
pub struct Line {
	attrs: IndexMap<LineAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn line_private::Content>>,
}

impl Default for Line {
	fn default() -> Self {
		Self::new()
	}
}

impl Line {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: line_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: line_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: LineAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: LineAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(LineAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(LineAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(LineAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(LineAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(LineAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(LineAttrs::ExternalResourcesRequired)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(LineAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(LineAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(LineAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(LineAttrs::Transform, value);
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(LineAttrs::Transform, value);
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&dyn Value> {
		self.get_attr(LineAttrs::Transform)
	}

	/// Set the `x1` attribute.
	pub fn with_x1<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(LineAttrs::X1, value);
		self
	}

	/// Set the `x1` attribute.
	pub fn set_x1<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(LineAttrs::X1, value);
	}

	/// Get the `x1` attribute.
	pub fn x1(&self) -> Option<&dyn Value> {
		self.get_attr(LineAttrs::X1)
	}

	/// Set the `x2` attribute.
	pub fn with_x2<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(LineAttrs::X2, value);
		self
	}

	/// Set the `x2` attribute.
	pub fn set_x2<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(LineAttrs::X2, value);
	}

	/// Get the `x2` attribute.
	pub fn x2(&self) -> Option<&dyn Value> {
		self.get_attr(LineAttrs::X2)
	}

	/// Set the `y1` attribute.
	pub fn with_y1<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(LineAttrs::Y1, value);
		self
	}

	/// Set the `y1` attribute.
	pub fn set_y1<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(LineAttrs::Y1, value);
	}

	/// Get the `y1` attribute.
	pub fn y1(&self) -> Option<&dyn Value> {
		self.get_attr(LineAttrs::Y1)
	}

	/// Set the `y2` attribute.
	pub fn with_y2<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(LineAttrs::Y2, value);
		self
	}

	/// Set the `y2` attribute.
	pub fn set_y2<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(LineAttrs::Y2, value);
	}

	/// Get the `y2` attribute.
	pub fn y2(&self) -> Option<&dyn Value> {
		self.get_attr(LineAttrs::Y2)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Line {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(LineAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(LineAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Line {}

impl common_attrs::CoreAttributesSetter for Line {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(LineAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(LineAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Line {}

impl common_attrs::GlobalEventAttributesSetter for Line {
	fn set_attr<V>(&mut self, attr: common_attrs::GlobalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(LineAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GlobalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(LineAttrs::from(attr))
	}
}

impl TagWithGlobalEventAttributes for Line {}

impl common_attrs::GraphicalEventAttributesSetter for Line {
	fn set_attr<V>(&mut self, attr: common_attrs::GraphicalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(LineAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(LineAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Line {}

impl common_attrs::PresentationAttributesSetter for Line {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(LineAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(LineAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Line {}

impl Tag for Line {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("line");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum LinearGradientAttrs {
	Class,
	ExternalResourcesRequired,
	GradientTransform,
	GradientUnits,
	SpreadMethod,
	Style,
	X1,
	X2,
	XlinkHref,
	Y1,
	Y2,
	CoreAttributes(common_attrs::CoreAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
	XLinkAttributes(common_attrs::XLinkAttributes),
}

impl From<common_attrs::CoreAttributes> for LinearGradientAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for LinearGradientAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl From<common_attrs::XLinkAttributes> for LinearGradientAttrs {
	fn from(attr: common_attrs::XLinkAttributes) -> Self {
		Self::XLinkAttributes(attr)
	}
}

impl LinearGradientAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::GradientTransform => "gradientTransform",
			Self::GradientUnits => "gradientUnits",
			Self::SpreadMethod => "spreadMethod",
			Self::Style => "style",
			Self::X1 => "x1",
			Self::X2 => "x2",
			Self::XlinkHref => "xlink:href",
			Self::Y1 => "y1",
			Self::Y2 => "y2",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
			Self::XLinkAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for LinearGradientAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for LinearGradientAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod linear_gradient_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Desc {}
	impl Content for super::Metadata {}
	impl Content for super::Set {}
	impl Content for super::Stop {}
	impl Content for super::Title {}
}

#[doc = "The [`<linearGradient>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("linearGradient.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateTransform>`](AnimateTransform)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<set>`](Set)\n",
	"- [`<stop>`](Stop)\n",
	"- [`<title>`](Title)\n"
)]
#[doc = "\n\n [`<linearGradient>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/linearGradient"]
#[derive(Debug)]
pub struct LinearGradient {
	attrs: IndexMap<LinearGradientAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn linear_gradient_private::Content>>,
}

impl Default for LinearGradient {
	fn default() -> Self {
		Self::new()
	}
}

impl LinearGradient {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: linear_gradient_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: linear_gradient_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: LinearGradientAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: LinearGradientAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(LinearGradientAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(LinearGradientAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(LinearGradientAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(LinearGradientAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(LinearGradientAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(LinearGradientAttrs::ExternalResourcesRequired)
	}

	/// Set the `gradientTransform` attribute.
	pub fn with_gradient_transform<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(LinearGradientAttrs::GradientTransform, value);
		self
	}

	/// Set the `gradientTransform` attribute.
	pub fn set_gradient_transform<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(LinearGradientAttrs::GradientTransform, value);
	}

	/// Get the `gradientTransform` attribute.
	pub fn gradient_transform(&self) -> Option<&dyn Value> {
		self.get_attr(LinearGradientAttrs::GradientTransform)
	}

	/// Set the `gradientUnits` attribute.
	pub fn with_gradient_units<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(LinearGradientAttrs::GradientUnits, value);
		self
	}

	/// Set the `gradientUnits` attribute.
	pub fn set_gradient_units<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(LinearGradientAttrs::GradientUnits, value);
	}

	/// Get the `gradientUnits` attribute.
	pub fn gradient_units(&self) -> Option<&dyn Value> {
		self.get_attr(LinearGradientAttrs::GradientUnits)
	}

	/// Set the `spreadMethod` attribute.
	pub fn with_spread_method<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(LinearGradientAttrs::SpreadMethod, value);
		self
	}

	/// Set the `spreadMethod` attribute.
	pub fn set_spread_method<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(LinearGradientAttrs::SpreadMethod, value);
	}

	/// Get the `spreadMethod` attribute.
	pub fn spread_method(&self) -> Option<&dyn Value> {
		self.get_attr(LinearGradientAttrs::SpreadMethod)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(LinearGradientAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(LinearGradientAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(LinearGradientAttrs::Style)
	}

	/// Set the `x1` attribute.
	pub fn with_x1<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(LinearGradientAttrs::X1, value);
		self
	}

	/// Set the `x1` attribute.
	pub fn set_x1<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(LinearGradientAttrs::X1, value);
	}

	/// Get the `x1` attribute.
	pub fn x1(&self) -> Option<&dyn Value> {
		self.get_attr(LinearGradientAttrs::X1)
	}

	/// Set the `x2` attribute.
	pub fn with_x2<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(LinearGradientAttrs::X2, value);
		self
	}

	/// Set the `x2` attribute.
	pub fn set_x2<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(LinearGradientAttrs::X2, value);
	}

	/// Get the `x2` attribute.
	pub fn x2(&self) -> Option<&dyn Value> {
		self.get_attr(LinearGradientAttrs::X2)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(LinearGradientAttrs::XlinkHref, value);
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(LinearGradientAttrs::XlinkHref, value);
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&dyn Value> {
		self.get_attr(LinearGradientAttrs::XlinkHref)
	}

	/// Set the `y1` attribute.
	pub fn with_y1<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(LinearGradientAttrs::Y1, value);
		self
	}

	/// Set the `y1` attribute.
	pub fn set_y1<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(LinearGradientAttrs::Y1, value);
	}

	/// Get the `y1` attribute.
	pub fn y1(&self) -> Option<&dyn Value> {
		self.get_attr(LinearGradientAttrs::Y1)
	}

	/// Set the `y2` attribute.
	pub fn with_y2<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(LinearGradientAttrs::Y2, value);
		self
	}

	/// Set the `y2` attribute.
	pub fn set_y2<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(LinearGradientAttrs::Y2, value);
	}

	/// Get the `y2` attribute.
	pub fn y2(&self) -> Option<&dyn Value> {
		self.get_attr(LinearGradientAttrs::Y2)
	}
}

impl common_attrs::CoreAttributesSetter for LinearGradient {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(LinearGradientAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(LinearGradientAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for LinearGradient {}

impl common_attrs::PresentationAttributesSetter for LinearGradient {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(LinearGradientAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(LinearGradientAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for LinearGradient {}

impl common_attrs::XLinkAttributesSetter for LinearGradient {
	fn set_attr<V>(&mut self, attr: common_attrs::XLinkAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(LinearGradientAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&dyn Value> {
		self.get_attr(LinearGradientAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for LinearGradient {}

impl Tag for LinearGradient {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("linearGradient");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum MarkerAttrs {
	Class,
	ExternalResourcesRequired,
	MarkerHeight,
	MarkerUnits,
	MarkerWidth,
	Orient,
	PreserveAspectRatio,
	RefX,
	RefY,
	Style,
	Transform,
	ViewBox,
	CoreAttributes(common_attrs::CoreAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::CoreAttributes> for MarkerAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for MarkerAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl MarkerAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::MarkerHeight => "markerHeight",
			Self::MarkerUnits => "markerUnits",
			Self::MarkerWidth => "markerWidth",
			Self::Orient => "orient",
			Self::PreserveAspectRatio => "preserveAspectRatio",
			Self::RefX => "refX",
			Self::RefY => "refY",
			Self::Style => "style",
			Self::Transform => "transform",
			Self::ViewBox => "viewBox",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for MarkerAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for MarkerAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod marker_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::A {}
	impl Content for super::AltGlyphDef {}
	impl Content for super::Animate {}
	impl Content for super::AnimateColor {}
	impl Content for super::AnimateMotion {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Circle {}
	impl Content for super::ClipPath {}
	impl Content for super::ColorProfile {}
	impl Content for super::Cursor {}
	impl Content for super::Defs {}
	impl Content for super::Desc {}
	impl Content for super::Discard {}
	impl Content for super::Ellipse {}
	impl Content for super::Filter {}
	impl Content for super::Font {}
	impl Content for super::FontFace {}
	impl Content for super::ForeignObject {}
	impl Content for super::Group {}
	impl Content for super::Image {}
	impl Content for super::Line {}
	impl Content for super::LinearGradient {}
	impl Content for super::Marker {}
	impl Content for super::Mask {}
	impl Content for super::Metadata {}
	impl Content for super::Mpath {}
	impl Content for super::Path {}
	impl Content for super::Pattern {}
	impl Content for super::Polygon {}
	impl Content for super::Polyline {}
	impl Content for super::RadialGradient {}
	impl Content for super::Rect {}
	impl Content for super::Script {}
	impl Content for super::Set {}
	impl Content for super::Stop {}
	impl Content for super::Style {}
	impl Content for super::Svg {}
	impl Content for super::Switch {}
	impl Content for super::Symbol {}
	impl Content for super::Text {}
	impl Content for super::Title {}
	impl Content for super::Use {}
	impl Content for super::View {}
}

#[doc = "The [`<marker>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("marker.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<a>`](A)\n",
	"- [`<altGlyphDef>`](AltGlyphDef)\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateColor>`](AnimateColor)\n",
	"- [`<animateMotion>`](AnimateMotion)\n",
	"- [`<animateTransform>`](AnimateTransform)\n",
	"- [`<circle>`](Circle)\n",
	"- [`<clipPath>`](ClipPath)\n",
	"- [`<color-profile>`](ColorProfile)\n",
	"- [`<cursor>`](Cursor)\n",
	"- [`<defs>`](Defs)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<discard>`](Discard)\n",
	"- [`<ellipse>`](Ellipse)\n",
	"- [`<filter>`](Filter)\n",
	"- [`<font>`](Font)\n",
	"- [`<font-face>`](FontFace)\n",
	"- [`<foreignObject>`](ForeignObject)\n",
	"- [`<g>`](Group)\n",
	"- [`<image>`](Image)\n",
	"- [`<line>`](Line)\n",
	"- [`<linearGradient>`](LinearGradient)\n",
	"- [`<marker>`](Marker)\n",
	"- [`<mask>`](Mask)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<mpath>`](Mpath)\n",
	"- [`<path>`](Path)\n",
	"- [`<pattern>`](Pattern)\n",
	"- [`<polygon>`](Polygon)\n",
	"- [`<polyline>`](Polyline)\n",
	"- [`<radialGradient>`](RadialGradient)\n",
	"- [`<rect>`](Rect)\n",
	"- [`<script>`](Script)\n",
	"- [`<set>`](Set)\n",
	"- [`<stop>`](Stop)\n",
	"- [`<style>`](Style)\n",
	"- [`<svg>`](Svg)\n",
	"- [`<switch>`](Switch)\n",
	"- [`<symbol>`](Symbol)\n",
	"- [`<text>`](Text)\n",
	"- [`<title>`](Title)\n",
	"- [`<use>`](Use)\n",
	"- [`<view>`](View)\n"
)]
#[doc = "\n\n [`<marker>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/marker"]
#[derive(Debug)]
pub struct Marker {
	attrs: IndexMap<MarkerAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn marker_private::Content>>,
}

impl Default for Marker {
	fn default() -> Self {
		Self::new()
	}
}

impl Marker {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: marker_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: marker_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: MarkerAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: MarkerAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MarkerAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MarkerAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(MarkerAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MarkerAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MarkerAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(MarkerAttrs::ExternalResourcesRequired)
	}

	/// Set the `markerHeight` attribute.
	pub fn with_marker_height<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MarkerAttrs::MarkerHeight, value);
		self
	}

	/// Set the `markerHeight` attribute.
	pub fn set_marker_height<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MarkerAttrs::MarkerHeight, value);
	}

	/// Get the `markerHeight` attribute.
	pub fn marker_height(&self) -> Option<&dyn Value> {
		self.get_attr(MarkerAttrs::MarkerHeight)
	}

	/// Set the `markerUnits` attribute.
	pub fn with_marker_units<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MarkerAttrs::MarkerUnits, value);
		self
	}

	/// Set the `markerUnits` attribute.
	pub fn set_marker_units<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MarkerAttrs::MarkerUnits, value);
	}

	/// Get the `markerUnits` attribute.
	pub fn marker_units(&self) -> Option<&dyn Value> {
		self.get_attr(MarkerAttrs::MarkerUnits)
	}

	/// Set the `markerWidth` attribute.
	pub fn with_marker_width<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MarkerAttrs::MarkerWidth, value);
		self
	}

	/// Set the `markerWidth` attribute.
	pub fn set_marker_width<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MarkerAttrs::MarkerWidth, value);
	}

	/// Get the `markerWidth` attribute.
	pub fn marker_width(&self) -> Option<&dyn Value> {
		self.get_attr(MarkerAttrs::MarkerWidth)
	}

	/// Set the `orient` attribute.
	pub fn with_orient<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MarkerAttrs::Orient, value);
		self
	}

	/// Set the `orient` attribute.
	pub fn set_orient<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MarkerAttrs::Orient, value);
	}

	/// Get the `orient` attribute.
	pub fn orient(&self) -> Option<&dyn Value> {
		self.get_attr(MarkerAttrs::Orient)
	}

	/// Set the `preserveAspectRatio` attribute.
	pub fn with_preserve_aspect_ratio<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MarkerAttrs::PreserveAspectRatio, value);
		self
	}

	/// Set the `preserveAspectRatio` attribute.
	pub fn set_preserve_aspect_ratio<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MarkerAttrs::PreserveAspectRatio, value);
	}

	/// Get the `preserveAspectRatio` attribute.
	pub fn preserve_aspect_ratio(&self) -> Option<&dyn Value> {
		self.get_attr(MarkerAttrs::PreserveAspectRatio)
	}

	/// Set the `refX` attribute.
	pub fn with_ref_x<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MarkerAttrs::RefX, value);
		self
	}

	/// Set the `refX` attribute.
	pub fn set_ref_x<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MarkerAttrs::RefX, value);
	}

	/// Get the `refX` attribute.
	pub fn ref_x(&self) -> Option<&dyn Value> {
		self.get_attr(MarkerAttrs::RefX)
	}

	/// Set the `refY` attribute.
	pub fn with_ref_y<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MarkerAttrs::RefY, value);
		self
	}

	/// Set the `refY` attribute.
	pub fn set_ref_y<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MarkerAttrs::RefY, value);
	}

	/// Get the `refY` attribute.
	pub fn ref_y(&self) -> Option<&dyn Value> {
		self.get_attr(MarkerAttrs::RefY)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MarkerAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MarkerAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(MarkerAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MarkerAttrs::Transform, value);
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MarkerAttrs::Transform, value);
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&dyn Value> {
		self.get_attr(MarkerAttrs::Transform)
	}

	/// Set the `viewBox` attribute.
	pub fn with_view_box<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MarkerAttrs::ViewBox, value);
		self
	}

	/// Set the `viewBox` attribute.
	pub fn set_view_box<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MarkerAttrs::ViewBox, value);
	}

	/// Get the `viewBox` attribute.
	pub fn view_box(&self) -> Option<&dyn Value> {
		self.get_attr(MarkerAttrs::ViewBox)
	}
}

impl common_attrs::CoreAttributesSetter for Marker {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MarkerAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(MarkerAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Marker {}

impl common_attrs::PresentationAttributesSetter for Marker {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MarkerAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(MarkerAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Marker {}

impl Tag for Marker {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("marker");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum MaskAttrs {
	Class,
	ExternalResourcesRequired,
	Height,
	MaskContentUnits,
	MaskUnits,
	Style,
	Width,
	X,
	Y,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::ConditionalProcessingAttributes> for MaskAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for MaskAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for MaskAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl MaskAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Height => "height",
			Self::MaskContentUnits => "maskContentUnits",
			Self::MaskUnits => "maskUnits",
			Self::Style => "style",
			Self::Width => "width",
			Self::X => "x",
			Self::Y => "y",
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for MaskAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for MaskAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod mask_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::A {}
	impl Content for super::AltGlyphDef {}
	impl Content for super::Animate {}
	impl Content for super::AnimateColor {}
	impl Content for super::AnimateMotion {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Circle {}
	impl Content for super::ClipPath {}
	impl Content for super::ColorProfile {}
	impl Content for super::Cursor {}
	impl Content for super::Defs {}
	impl Content for super::Desc {}
	impl Content for super::Discard {}
	impl Content for super::Ellipse {}
	impl Content for super::Filter {}
	impl Content for super::Font {}
	impl Content for super::FontFace {}
	impl Content for super::ForeignObject {}
	impl Content for super::Group {}
	impl Content for super::Image {}
	impl Content for super::Line {}
	impl Content for super::LinearGradient {}
	impl Content for super::Marker {}
	impl Content for super::Mask {}
	impl Content for super::Metadata {}
	impl Content for super::Mpath {}
	impl Content for super::Path {}
	impl Content for super::Pattern {}
	impl Content for super::Polygon {}
	impl Content for super::Polyline {}
	impl Content for super::RadialGradient {}
	impl Content for super::Rect {}
	impl Content for super::Script {}
	impl Content for super::Set {}
	impl Content for super::Stop {}
	impl Content for super::Style {}
	impl Content for super::Svg {}
	impl Content for super::Switch {}
	impl Content for super::Symbol {}
	impl Content for super::Text {}
	impl Content for super::Title {}
	impl Content for super::Use {}
	impl Content for super::View {}
}

#[doc = "The [`<mask>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("mask.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<a>`](A)\n",
	"- [`<altGlyphDef>`](AltGlyphDef)\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateColor>`](AnimateColor)\n",
	"- [`<animateMotion>`](AnimateMotion)\n",
	"- [`<animateTransform>`](AnimateTransform)\n",
	"- [`<circle>`](Circle)\n",
	"- [`<clipPath>`](ClipPath)\n",
	"- [`<color-profile>`](ColorProfile)\n",
	"- [`<cursor>`](Cursor)\n",
	"- [`<defs>`](Defs)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<discard>`](Discard)\n",
	"- [`<ellipse>`](Ellipse)\n",
	"- [`<filter>`](Filter)\n",
	"- [`<font>`](Font)\n",
	"- [`<font-face>`](FontFace)\n",
	"- [`<foreignObject>`](ForeignObject)\n",
	"- [`<g>`](Group)\n",
	"- [`<image>`](Image)\n",
	"- [`<line>`](Line)\n",
	"- [`<linearGradient>`](LinearGradient)\n",
	"- [`<marker>`](Marker)\n",
	"- [`<mask>`](Mask)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<mpath>`](Mpath)\n",
	"- [`<path>`](Path)\n",
	"- [`<pattern>`](Pattern)\n",
	"- [`<polygon>`](Polygon)\n",
	"- [`<polyline>`](Polyline)\n",
	"- [`<radialGradient>`](RadialGradient)\n",
	"- [`<rect>`](Rect)\n",
	"- [`<script>`](Script)\n",
	"- [`<set>`](Set)\n",
	"- [`<stop>`](Stop)\n",
	"- [`<style>`](Style)\n",
	"- [`<svg>`](Svg)\n",
	"- [`<switch>`](Switch)\n",
	"- [`<symbol>`](Symbol)\n",
	"- [`<text>`](Text)\n",
	"- [`<title>`](Title)\n",
	"- [`<use>`](Use)\n",
	"- [`<view>`](View)\n"
)]
#[doc = "\n\n [`<mask>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/mask"]
#[derive(Debug)]
pub struct Mask {
	attrs: IndexMap<MaskAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn mask_private::Content>>,
}

impl Default for Mask {
	fn default() -> Self {
		Self::new()
	}
}

impl Mask {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: mask_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: mask_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: MaskAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: MaskAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MaskAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MaskAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(MaskAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MaskAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MaskAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(MaskAttrs::ExternalResourcesRequired)
	}

	/// Set the `height` attribute.
	pub fn with_height<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MaskAttrs::Height, value);
		self
	}

	/// Set the `height` attribute.
	pub fn set_height<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MaskAttrs::Height, value);
	}

	/// Get the `height` attribute.
	pub fn height(&self) -> Option<&dyn Value> {
		self.get_attr(MaskAttrs::Height)
	}

	/// Set the `maskContentUnits` attribute.
	pub fn with_mask_content_units<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MaskAttrs::MaskContentUnits, value);
		self
	}

	/// Set the `maskContentUnits` attribute.
	pub fn set_mask_content_units<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MaskAttrs::MaskContentUnits, value);
	}

	/// Get the `maskContentUnits` attribute.
	pub fn mask_content_units(&self) -> Option<&dyn Value> {
		self.get_attr(MaskAttrs::MaskContentUnits)
	}

	/// Set the `maskUnits` attribute.
	pub fn with_mask_units<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MaskAttrs::MaskUnits, value);
		self
	}

	/// Set the `maskUnits` attribute.
	pub fn set_mask_units<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MaskAttrs::MaskUnits, value);
	}

	/// Get the `maskUnits` attribute.
	pub fn mask_units(&self) -> Option<&dyn Value> {
		self.get_attr(MaskAttrs::MaskUnits)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MaskAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MaskAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(MaskAttrs::Style)
	}

	/// Set the `width` attribute.
	pub fn with_width<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MaskAttrs::Width, value);
		self
	}

	/// Set the `width` attribute.
	pub fn set_width<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MaskAttrs::Width, value);
	}

	/// Get the `width` attribute.
	pub fn width(&self) -> Option<&dyn Value> {
		self.get_attr(MaskAttrs::Width)
	}

	/// Set the `x` attribute.
	pub fn with_x<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MaskAttrs::X, value);
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MaskAttrs::X, value);
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&dyn Value> {
		self.get_attr(MaskAttrs::X)
	}

	/// Set the `y` attribute.
	pub fn with_y<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MaskAttrs::Y, value);
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MaskAttrs::Y, value);
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&dyn Value> {
		self.get_attr(MaskAttrs::Y)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Mask {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MaskAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(MaskAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Mask {}

impl common_attrs::CoreAttributesSetter for Mask {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MaskAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(MaskAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Mask {}

impl common_attrs::PresentationAttributesSetter for Mask {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MaskAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(MaskAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Mask {}

impl Tag for Mask {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("mask");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum MetadataAttrs {
	CoreAttributes(common_attrs::CoreAttributes),
}

impl From<common_attrs::CoreAttributes> for MetadataAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl MetadataAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::CoreAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for MetadataAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for MetadataAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[doc = "The [`<metadata>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("metadata.md")]
#[doc = "\n\n [`<metadata>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/metadata"]
#[derive(Debug)]
pub struct Metadata {
	attrs: IndexMap<MetadataAttrs, Box<dyn Value>>,
	content: String,
}

impl Default for Metadata {
	fn default() -> Self {
		Self::new()
	}
}

impl Metadata {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: String::new()
		}
	}

	pub fn push<T: Display>(&mut self, content: T) {
		write!(self.content, "{content}").unwrap();
	}

	pub fn append<T: Display>(mut self, content: T) -> Self {
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: MetadataAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: MetadataAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}
}

impl common_attrs::CoreAttributesSetter for Metadata {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MetadataAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(MetadataAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Metadata {}

impl Tag for Metadata {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("metadata");
		w.write_cdata_text(&self.content);
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum MissingGlyphAttrs {
	Class,
	D,
	HorizAdvX,
	Style,
	VertAdvY,
	VertOriginX,
	VertOriginY,
	CoreAttributes(common_attrs::CoreAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::CoreAttributes> for MissingGlyphAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for MissingGlyphAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl MissingGlyphAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::D => "d",
			Self::HorizAdvX => "horiz-adv-x",
			Self::Style => "style",
			Self::VertAdvY => "vert-adv-y",
			Self::VertOriginX => "vert-origin-x",
			Self::VertOriginY => "vert-origin-y",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for MissingGlyphAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for MissingGlyphAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod missing_glyph_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::A {}
	impl Content for super::AltGlyphDef {}
	impl Content for super::Animate {}
	impl Content for super::AnimateColor {}
	impl Content for super::AnimateMotion {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Circle {}
	impl Content for super::ClipPath {}
	impl Content for super::ColorProfile {}
	impl Content for super::Cursor {}
	impl Content for super::Defs {}
	impl Content for super::Desc {}
	impl Content for super::Discard {}
	impl Content for super::Ellipse {}
	impl Content for super::Filter {}
	impl Content for super::Font {}
	impl Content for super::FontFace {}
	impl Content for super::ForeignObject {}
	impl Content for super::Group {}
	impl Content for super::Image {}
	impl Content for super::Line {}
	impl Content for super::LinearGradient {}
	impl Content for super::Marker {}
	impl Content for super::Mask {}
	impl Content for super::Metadata {}
	impl Content for super::Mpath {}
	impl Content for super::Path {}
	impl Content for super::Pattern {}
	impl Content for super::Polygon {}
	impl Content for super::Polyline {}
	impl Content for super::RadialGradient {}
	impl Content for super::Rect {}
	impl Content for super::Script {}
	impl Content for super::Set {}
	impl Content for super::Stop {}
	impl Content for super::Style {}
	impl Content for super::Svg {}
	impl Content for super::Switch {}
	impl Content for super::Symbol {}
	impl Content for super::Text {}
	impl Content for super::Title {}
	impl Content for super::Use {}
	impl Content for super::View {}
}

#[doc = "The [`<missing-glyph>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("missing-glyph.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<a>`](A)\n",
	"- [`<altGlyphDef>`](AltGlyphDef)\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateColor>`](AnimateColor)\n",
	"- [`<animateMotion>`](AnimateMotion)\n",
	"- [`<animateTransform>`](AnimateTransform)\n",
	"- [`<circle>`](Circle)\n",
	"- [`<clipPath>`](ClipPath)\n",
	"- [`<color-profile>`](ColorProfile)\n",
	"- [`<cursor>`](Cursor)\n",
	"- [`<defs>`](Defs)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<discard>`](Discard)\n",
	"- [`<ellipse>`](Ellipse)\n",
	"- [`<filter>`](Filter)\n",
	"- [`<font>`](Font)\n",
	"- [`<font-face>`](FontFace)\n",
	"- [`<foreignObject>`](ForeignObject)\n",
	"- [`<g>`](Group)\n",
	"- [`<image>`](Image)\n",
	"- [`<line>`](Line)\n",
	"- [`<linearGradient>`](LinearGradient)\n",
	"- [`<marker>`](Marker)\n",
	"- [`<mask>`](Mask)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<mpath>`](Mpath)\n",
	"- [`<path>`](Path)\n",
	"- [`<pattern>`](Pattern)\n",
	"- [`<polygon>`](Polygon)\n",
	"- [`<polyline>`](Polyline)\n",
	"- [`<radialGradient>`](RadialGradient)\n",
	"- [`<rect>`](Rect)\n",
	"- [`<script>`](Script)\n",
	"- [`<set>`](Set)\n",
	"- [`<stop>`](Stop)\n",
	"- [`<style>`](Style)\n",
	"- [`<svg>`](Svg)\n",
	"- [`<switch>`](Switch)\n",
	"- [`<symbol>`](Symbol)\n",
	"- [`<text>`](Text)\n",
	"- [`<title>`](Title)\n",
	"- [`<use>`](Use)\n",
	"- [`<view>`](View)\n"
)]
#[doc = "\n\n [`<missing-glyph>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/missing-glyph"]
#[derive(Debug)]
pub struct MissingGlyph {
	attrs: IndexMap<MissingGlyphAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn missing_glyph_private::Content>>,
}

impl Default for MissingGlyph {
	fn default() -> Self {
		Self::new()
	}
}

impl MissingGlyph {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: missing_glyph_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: missing_glyph_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: MissingGlyphAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: MissingGlyphAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MissingGlyphAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MissingGlyphAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(MissingGlyphAttrs::Class)
	}

	/// Set the `d` attribute.
	pub fn with_d<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MissingGlyphAttrs::D, value);
		self
	}

	/// Set the `d` attribute.
	pub fn set_d<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MissingGlyphAttrs::D, value);
	}

	/// Get the `d` attribute.
	pub fn d(&self) -> Option<&dyn Value> {
		self.get_attr(MissingGlyphAttrs::D)
	}

	/// Set the `horiz-adv-x` attribute.
	pub fn with_horiz_adv_x<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MissingGlyphAttrs::HorizAdvX, value);
		self
	}

	/// Set the `horiz-adv-x` attribute.
	pub fn set_horiz_adv_x<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MissingGlyphAttrs::HorizAdvX, value);
	}

	/// Get the `horiz-adv-x` attribute.
	pub fn horiz_adv_x(&self) -> Option<&dyn Value> {
		self.get_attr(MissingGlyphAttrs::HorizAdvX)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MissingGlyphAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MissingGlyphAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(MissingGlyphAttrs::Style)
	}

	/// Set the `vert-adv-y` attribute.
	pub fn with_vert_adv_y<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MissingGlyphAttrs::VertAdvY, value);
		self
	}

	/// Set the `vert-adv-y` attribute.
	pub fn set_vert_adv_y<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MissingGlyphAttrs::VertAdvY, value);
	}

	/// Get the `vert-adv-y` attribute.
	pub fn vert_adv_y(&self) -> Option<&dyn Value> {
		self.get_attr(MissingGlyphAttrs::VertAdvY)
	}

	/// Set the `vert-origin-x` attribute.
	pub fn with_vert_origin_x<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MissingGlyphAttrs::VertOriginX, value);
		self
	}

	/// Set the `vert-origin-x` attribute.
	pub fn set_vert_origin_x<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MissingGlyphAttrs::VertOriginX, value);
	}

	/// Get the `vert-origin-x` attribute.
	pub fn vert_origin_x(&self) -> Option<&dyn Value> {
		self.get_attr(MissingGlyphAttrs::VertOriginX)
	}

	/// Set the `vert-origin-y` attribute.
	pub fn with_vert_origin_y<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MissingGlyphAttrs::VertOriginY, value);
		self
	}

	/// Set the `vert-origin-y` attribute.
	pub fn set_vert_origin_y<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MissingGlyphAttrs::VertOriginY, value);
	}

	/// Get the `vert-origin-y` attribute.
	pub fn vert_origin_y(&self) -> Option<&dyn Value> {
		self.get_attr(MissingGlyphAttrs::VertOriginY)
	}
}

impl common_attrs::CoreAttributesSetter for MissingGlyph {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MissingGlyphAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(MissingGlyphAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for MissingGlyph {}

impl common_attrs::PresentationAttributesSetter for MissingGlyph {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MissingGlyphAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(MissingGlyphAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for MissingGlyph {}

impl Tag for MissingGlyph {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("missing-glyph");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum MpathAttrs {
	ExternalResourcesRequired,
	XlinkHref,
	CoreAttributes(common_attrs::CoreAttributes),
	XLinkAttributes(common_attrs::XLinkAttributes),
}

impl From<common_attrs::CoreAttributes> for MpathAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::XLinkAttributes> for MpathAttrs {
	fn from(attr: common_attrs::XLinkAttributes) -> Self {
		Self::XLinkAttributes(attr)
	}
}

impl MpathAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::XlinkHref => "xlink:href",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::XLinkAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for MpathAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for MpathAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod mpath_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Desc {}
	impl Content for super::Metadata {}
	impl Content for super::Title {}
}

#[doc = "The [`<mpath>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("mpath.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<desc>`](Desc)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<title>`](Title)\n"
)]
#[doc = "\n\n [`<mpath>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/mpath"]
#[derive(Debug)]
pub struct Mpath {
	attrs: IndexMap<MpathAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn mpath_private::Content>>,
}

impl Default for Mpath {
	fn default() -> Self {
		Self::new()
	}
}

impl Mpath {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: mpath_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: mpath_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: MpathAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: MpathAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MpathAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MpathAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(MpathAttrs::ExternalResourcesRequired)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(MpathAttrs::XlinkHref, value);
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MpathAttrs::XlinkHref, value);
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&dyn Value> {
		self.get_attr(MpathAttrs::XlinkHref)
	}
}

impl common_attrs::CoreAttributesSetter for Mpath {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MpathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(MpathAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Mpath {}

impl common_attrs::XLinkAttributesSetter for Mpath {
	fn set_attr<V>(&mut self, attr: common_attrs::XLinkAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(MpathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&dyn Value> {
		self.get_attr(MpathAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for Mpath {}

impl Tag for Mpath {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("mpath");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum PathAttrs {
	Class,
	D,
	ExternalResourcesRequired,
	PathLength,
	Style,
	Transform,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	GlobalEventAttributes(common_attrs::GlobalEventAttributes),
	GraphicalEventAttributes(common_attrs::GraphicalEventAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::ConditionalProcessingAttributes> for PathAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for PathAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::GlobalEventAttributes> for PathAttrs {
	fn from(attr: common_attrs::GlobalEventAttributes) -> Self {
		Self::GlobalEventAttributes(attr)
	}
}

impl From<common_attrs::GraphicalEventAttributes> for PathAttrs {
	fn from(attr: common_attrs::GraphicalEventAttributes) -> Self {
		Self::GraphicalEventAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for PathAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl PathAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::D => "d",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::PathLength => "pathLength",
			Self::Style => "style",
			Self::Transform => "transform",
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::GlobalEventAttributes(attr) => attr.as_str(),
			Self::GraphicalEventAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for PathAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for PathAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod path_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::AnimateMotion {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Desc {}
	impl Content for super::Discard {}
	impl Content for super::Metadata {}
	impl Content for super::Mpath {}
	impl Content for super::Set {}
	impl Content for super::Title {}
}

#[doc = "The [`<path>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("path.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateMotion>`](AnimateMotion)\n",
	"- [`<animateTransform>`](AnimateTransform)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<discard>`](Discard)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<mpath>`](Mpath)\n",
	"- [`<set>`](Set)\n",
	"- [`<title>`](Title)\n"
)]
#[doc = "\n\n [`<path>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/path"]
#[derive(Debug)]
pub struct Path {
	attrs: IndexMap<PathAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn path_private::Content>>,
}

impl Default for Path {
	fn default() -> Self {
		Self::new()
	}
}

impl Path {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: path_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: path_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: PathAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: PathAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PathAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PathAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(PathAttrs::Class)
	}

	/// Set the `d` attribute.
	pub fn with_d<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PathAttrs::D, value);
		self
	}

	/// Set the `d` attribute.
	pub fn set_d<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PathAttrs::D, value);
	}

	/// Get the `d` attribute.
	pub fn d(&self) -> Option<&dyn Value> {
		self.get_attr(PathAttrs::D)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PathAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PathAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(PathAttrs::ExternalResourcesRequired)
	}

	/// Set the `pathLength` attribute.
	pub fn with_path_length<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PathAttrs::PathLength, value);
		self
	}

	/// Set the `pathLength` attribute.
	pub fn set_path_length<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PathAttrs::PathLength, value);
	}

	/// Get the `pathLength` attribute.
	pub fn path_length(&self) -> Option<&dyn Value> {
		self.get_attr(PathAttrs::PathLength)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PathAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PathAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(PathAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PathAttrs::Transform, value);
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PathAttrs::Transform, value);
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&dyn Value> {
		self.get_attr(PathAttrs::Transform)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Path {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(PathAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Path {}

impl common_attrs::CoreAttributesSetter for Path {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(PathAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Path {}

impl common_attrs::GlobalEventAttributesSetter for Path {
	fn set_attr<V>(&mut self, attr: common_attrs::GlobalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GlobalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(PathAttrs::from(attr))
	}
}

impl TagWithGlobalEventAttributes for Path {}

impl common_attrs::GraphicalEventAttributesSetter for Path {
	fn set_attr<V>(&mut self, attr: common_attrs::GraphicalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(PathAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Path {}

impl common_attrs::PresentationAttributesSetter for Path {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(PathAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Path {}

impl Tag for Path {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("path");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum PatternAttrs {
	Class,
	ExternalResourcesRequired,
	Height,
	PatternContentUnits,
	PatternTransform,
	PatternUnits,
	PreserveAspectRatio,
	Style,
	ViewBox,
	Width,
	X,
	XlinkHref,
	Y,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
	XLinkAttributes(common_attrs::XLinkAttributes),
}

impl From<common_attrs::ConditionalProcessingAttributes> for PatternAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for PatternAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for PatternAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl From<common_attrs::XLinkAttributes> for PatternAttrs {
	fn from(attr: common_attrs::XLinkAttributes) -> Self {
		Self::XLinkAttributes(attr)
	}
}

impl PatternAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Height => "height",
			Self::PatternContentUnits => "patternContentUnits",
			Self::PatternTransform => "patternTransform",
			Self::PatternUnits => "patternUnits",
			Self::PreserveAspectRatio => "preserveAspectRatio",
			Self::Style => "style",
			Self::ViewBox => "viewBox",
			Self::Width => "width",
			Self::X => "x",
			Self::XlinkHref => "xlink:href",
			Self::Y => "y",
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
			Self::XLinkAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for PatternAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for PatternAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod pattern_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::A {}
	impl Content for super::AltGlyphDef {}
	impl Content for super::Animate {}
	impl Content for super::AnimateColor {}
	impl Content for super::AnimateMotion {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Circle {}
	impl Content for super::ClipPath {}
	impl Content for super::ColorProfile {}
	impl Content for super::Cursor {}
	impl Content for super::Defs {}
	impl Content for super::Desc {}
	impl Content for super::Discard {}
	impl Content for super::Ellipse {}
	impl Content for super::Filter {}
	impl Content for super::Font {}
	impl Content for super::FontFace {}
	impl Content for super::ForeignObject {}
	impl Content for super::Group {}
	impl Content for super::Image {}
	impl Content for super::Line {}
	impl Content for super::LinearGradient {}
	impl Content for super::Marker {}
	impl Content for super::Mask {}
	impl Content for super::Metadata {}
	impl Content for super::Mpath {}
	impl Content for super::Path {}
	impl Content for super::Pattern {}
	impl Content for super::Polygon {}
	impl Content for super::Polyline {}
	impl Content for super::RadialGradient {}
	impl Content for super::Rect {}
	impl Content for super::Script {}
	impl Content for super::Set {}
	impl Content for super::Stop {}
	impl Content for super::Style {}
	impl Content for super::Svg {}
	impl Content for super::Switch {}
	impl Content for super::Symbol {}
	impl Content for super::Text {}
	impl Content for super::Title {}
	impl Content for super::Use {}
	impl Content for super::View {}
}

#[doc = "The [`<pattern>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("pattern.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<a>`](A)\n",
	"- [`<altGlyphDef>`](AltGlyphDef)\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateColor>`](AnimateColor)\n",
	"- [`<animateMotion>`](AnimateMotion)\n",
	"- [`<animateTransform>`](AnimateTransform)\n",
	"- [`<circle>`](Circle)\n",
	"- [`<clipPath>`](ClipPath)\n",
	"- [`<color-profile>`](ColorProfile)\n",
	"- [`<cursor>`](Cursor)\n",
	"- [`<defs>`](Defs)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<discard>`](Discard)\n",
	"- [`<ellipse>`](Ellipse)\n",
	"- [`<filter>`](Filter)\n",
	"- [`<font>`](Font)\n",
	"- [`<font-face>`](FontFace)\n",
	"- [`<foreignObject>`](ForeignObject)\n",
	"- [`<g>`](Group)\n",
	"- [`<image>`](Image)\n",
	"- [`<line>`](Line)\n",
	"- [`<linearGradient>`](LinearGradient)\n",
	"- [`<marker>`](Marker)\n",
	"- [`<mask>`](Mask)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<mpath>`](Mpath)\n",
	"- [`<path>`](Path)\n",
	"- [`<pattern>`](Pattern)\n",
	"- [`<polygon>`](Polygon)\n",
	"- [`<polyline>`](Polyline)\n",
	"- [`<radialGradient>`](RadialGradient)\n",
	"- [`<rect>`](Rect)\n",
	"- [`<script>`](Script)\n",
	"- [`<set>`](Set)\n",
	"- [`<stop>`](Stop)\n",
	"- [`<style>`](Style)\n",
	"- [`<svg>`](Svg)\n",
	"- [`<switch>`](Switch)\n",
	"- [`<symbol>`](Symbol)\n",
	"- [`<text>`](Text)\n",
	"- [`<title>`](Title)\n",
	"- [`<use>`](Use)\n",
	"- [`<view>`](View)\n"
)]
#[doc = "\n\n [`<pattern>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/pattern"]
#[derive(Debug)]
pub struct Pattern {
	attrs: IndexMap<PatternAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn pattern_private::Content>>,
}

impl Default for Pattern {
	fn default() -> Self {
		Self::new()
	}
}

impl Pattern {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: pattern_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: pattern_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: PatternAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: PatternAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(PatternAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(PatternAttrs::ExternalResourcesRequired)
	}

	/// Set the `height` attribute.
	pub fn with_height<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::Height, value);
		self
	}

	/// Set the `height` attribute.
	pub fn set_height<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::Height, value);
	}

	/// Get the `height` attribute.
	pub fn height(&self) -> Option<&dyn Value> {
		self.get_attr(PatternAttrs::Height)
	}

	/// Set the `patternContentUnits` attribute.
	pub fn with_pattern_content_units<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::PatternContentUnits, value);
		self
	}

	/// Set the `patternContentUnits` attribute.
	pub fn set_pattern_content_units<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::PatternContentUnits, value);
	}

	/// Get the `patternContentUnits` attribute.
	pub fn pattern_content_units(&self) -> Option<&dyn Value> {
		self.get_attr(PatternAttrs::PatternContentUnits)
	}

	/// Set the `patternTransform` attribute.
	pub fn with_pattern_transform<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::PatternTransform, value);
		self
	}

	/// Set the `patternTransform` attribute.
	pub fn set_pattern_transform<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::PatternTransform, value);
	}

	/// Get the `patternTransform` attribute.
	pub fn pattern_transform(&self) -> Option<&dyn Value> {
		self.get_attr(PatternAttrs::PatternTransform)
	}

	/// Set the `patternUnits` attribute.
	pub fn with_pattern_units<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::PatternUnits, value);
		self
	}

	/// Set the `patternUnits` attribute.
	pub fn set_pattern_units<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::PatternUnits, value);
	}

	/// Get the `patternUnits` attribute.
	pub fn pattern_units(&self) -> Option<&dyn Value> {
		self.get_attr(PatternAttrs::PatternUnits)
	}

	/// Set the `preserveAspectRatio` attribute.
	pub fn with_preserve_aspect_ratio<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::PreserveAspectRatio, value);
		self
	}

	/// Set the `preserveAspectRatio` attribute.
	pub fn set_preserve_aspect_ratio<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::PreserveAspectRatio, value);
	}

	/// Get the `preserveAspectRatio` attribute.
	pub fn preserve_aspect_ratio(&self) -> Option<&dyn Value> {
		self.get_attr(PatternAttrs::PreserveAspectRatio)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(PatternAttrs::Style)
	}

	/// Set the `viewBox` attribute.
	pub fn with_view_box<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::ViewBox, value);
		self
	}

	/// Set the `viewBox` attribute.
	pub fn set_view_box<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::ViewBox, value);
	}

	/// Get the `viewBox` attribute.
	pub fn view_box(&self) -> Option<&dyn Value> {
		self.get_attr(PatternAttrs::ViewBox)
	}

	/// Set the `width` attribute.
	pub fn with_width<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::Width, value);
		self
	}

	/// Set the `width` attribute.
	pub fn set_width<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::Width, value);
	}

	/// Get the `width` attribute.
	pub fn width(&self) -> Option<&dyn Value> {
		self.get_attr(PatternAttrs::Width)
	}

	/// Set the `x` attribute.
	pub fn with_x<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::X, value);
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::X, value);
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&dyn Value> {
		self.get_attr(PatternAttrs::X)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::XlinkHref, value);
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::XlinkHref, value);
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&dyn Value> {
		self.get_attr(PatternAttrs::XlinkHref)
	}

	/// Set the `y` attribute.
	pub fn with_y<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::Y, value);
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::Y, value);
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&dyn Value> {
		self.get_attr(PatternAttrs::Y)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Pattern {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(PatternAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Pattern {}

impl common_attrs::CoreAttributesSetter for Pattern {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(PatternAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Pattern {}

impl common_attrs::PresentationAttributesSetter for Pattern {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(PatternAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Pattern {}

impl common_attrs::XLinkAttributesSetter for Pattern {
	fn set_attr<V>(&mut self, attr: common_attrs::XLinkAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PatternAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&dyn Value> {
		self.get_attr(PatternAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for Pattern {}

impl Tag for Pattern {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("pattern");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum PolygonAttrs {
	Class,
	ExternalResourcesRequired,
	Points,
	Style,
	Transform,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	GlobalEventAttributes(common_attrs::GlobalEventAttributes),
	GraphicalEventAttributes(common_attrs::GraphicalEventAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::ConditionalProcessingAttributes> for PolygonAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for PolygonAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::GlobalEventAttributes> for PolygonAttrs {
	fn from(attr: common_attrs::GlobalEventAttributes) -> Self {
		Self::GlobalEventAttributes(attr)
	}
}

impl From<common_attrs::GraphicalEventAttributes> for PolygonAttrs {
	fn from(attr: common_attrs::GraphicalEventAttributes) -> Self {
		Self::GraphicalEventAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for PolygonAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl PolygonAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Points => "points",
			Self::Style => "style",
			Self::Transform => "transform",
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::GlobalEventAttributes(attr) => attr.as_str(),
			Self::GraphicalEventAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for PolygonAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for PolygonAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod polygon_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::AnimateMotion {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Desc {}
	impl Content for super::Discard {}
	impl Content for super::Metadata {}
	impl Content for super::Mpath {}
	impl Content for super::Set {}
	impl Content for super::Title {}
}

#[doc = "The [`<polygon>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("polygon.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateMotion>`](AnimateMotion)\n",
	"- [`<animateTransform>`](AnimateTransform)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<discard>`](Discard)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<mpath>`](Mpath)\n",
	"- [`<set>`](Set)\n",
	"- [`<title>`](Title)\n"
)]
#[doc = "\n\n [`<polygon>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/polygon"]
#[derive(Debug)]
pub struct Polygon {
	attrs: IndexMap<PolygonAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn polygon_private::Content>>,
}

impl Default for Polygon {
	fn default() -> Self {
		Self::new()
	}
}

impl Polygon {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: polygon_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: polygon_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: PolygonAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: PolygonAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PolygonAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PolygonAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(PolygonAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PolygonAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PolygonAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(PolygonAttrs::ExternalResourcesRequired)
	}

	/// Set the `points` attribute.
	pub fn with_points<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PolygonAttrs::Points, value);
		self
	}

	/// Set the `points` attribute.
	pub fn set_points<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PolygonAttrs::Points, value);
	}

	/// Get the `points` attribute.
	pub fn points(&self) -> Option<&dyn Value> {
		self.get_attr(PolygonAttrs::Points)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PolygonAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PolygonAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(PolygonAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PolygonAttrs::Transform, value);
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PolygonAttrs::Transform, value);
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&dyn Value> {
		self.get_attr(PolygonAttrs::Transform)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Polygon {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PolygonAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(PolygonAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Polygon {}

impl common_attrs::CoreAttributesSetter for Polygon {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PolygonAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(PolygonAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Polygon {}

impl common_attrs::GlobalEventAttributesSetter for Polygon {
	fn set_attr<V>(&mut self, attr: common_attrs::GlobalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PolygonAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GlobalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(PolygonAttrs::from(attr))
	}
}

impl TagWithGlobalEventAttributes for Polygon {}

impl common_attrs::GraphicalEventAttributesSetter for Polygon {
	fn set_attr<V>(&mut self, attr: common_attrs::GraphicalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PolygonAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(PolygonAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Polygon {}

impl common_attrs::PresentationAttributesSetter for Polygon {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PolygonAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(PolygonAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Polygon {}

impl Tag for Polygon {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("polygon");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum PolylineAttrs {
	Class,
	ExternalResourcesRequired,
	Points,
	Style,
	Transform,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	GlobalEventAttributes(common_attrs::GlobalEventAttributes),
	GraphicalEventAttributes(common_attrs::GraphicalEventAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::ConditionalProcessingAttributes> for PolylineAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for PolylineAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::GlobalEventAttributes> for PolylineAttrs {
	fn from(attr: common_attrs::GlobalEventAttributes) -> Self {
		Self::GlobalEventAttributes(attr)
	}
}

impl From<common_attrs::GraphicalEventAttributes> for PolylineAttrs {
	fn from(attr: common_attrs::GraphicalEventAttributes) -> Self {
		Self::GraphicalEventAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for PolylineAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl PolylineAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Points => "points",
			Self::Style => "style",
			Self::Transform => "transform",
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::GlobalEventAttributes(attr) => attr.as_str(),
			Self::GraphicalEventAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for PolylineAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for PolylineAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod polyline_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::AnimateMotion {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Desc {}
	impl Content for super::Discard {}
	impl Content for super::Metadata {}
	impl Content for super::Mpath {}
	impl Content for super::Set {}
	impl Content for super::Title {}
}

#[doc = "The [`<polyline>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("polyline.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateMotion>`](AnimateMotion)\n",
	"- [`<animateTransform>`](AnimateTransform)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<discard>`](Discard)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<mpath>`](Mpath)\n",
	"- [`<set>`](Set)\n",
	"- [`<title>`](Title)\n"
)]
#[doc = "\n\n [`<polyline>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/polyline"]
#[derive(Debug)]
pub struct Polyline {
	attrs: IndexMap<PolylineAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn polyline_private::Content>>,
}

impl Default for Polyline {
	fn default() -> Self {
		Self::new()
	}
}

impl Polyline {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: polyline_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: polyline_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: PolylineAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: PolylineAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PolylineAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PolylineAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(PolylineAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PolylineAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PolylineAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(PolylineAttrs::ExternalResourcesRequired)
	}

	/// Set the `points` attribute.
	pub fn with_points<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PolylineAttrs::Points, value);
		self
	}

	/// Set the `points` attribute.
	pub fn set_points<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PolylineAttrs::Points, value);
	}

	/// Get the `points` attribute.
	pub fn points(&self) -> Option<&dyn Value> {
		self.get_attr(PolylineAttrs::Points)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PolylineAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PolylineAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(PolylineAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PolylineAttrs::Transform, value);
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PolylineAttrs::Transform, value);
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&dyn Value> {
		self.get_attr(PolylineAttrs::Transform)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Polyline {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PolylineAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(PolylineAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Polyline {}

impl common_attrs::CoreAttributesSetter for Polyline {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PolylineAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(PolylineAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Polyline {}

impl common_attrs::GlobalEventAttributesSetter for Polyline {
	fn set_attr<V>(&mut self, attr: common_attrs::GlobalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PolylineAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GlobalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(PolylineAttrs::from(attr))
	}
}

impl TagWithGlobalEventAttributes for Polyline {}

impl common_attrs::GraphicalEventAttributesSetter for Polyline {
	fn set_attr<V>(&mut self, attr: common_attrs::GraphicalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PolylineAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(PolylineAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Polyline {}

impl common_attrs::PresentationAttributesSetter for Polyline {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PolylineAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(PolylineAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Polyline {}

impl Tag for Polyline {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("polyline");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum RadialGradientAttrs {
	Class,
	Cx,
	Cy,
	ExternalResourcesRequired,
	Fx,
	Fy,
	GradientTransform,
	GradientUnits,
	R,
	SpreadMethod,
	Style,
	XlinkHref,
	CoreAttributes(common_attrs::CoreAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
	XLinkAttributes(common_attrs::XLinkAttributes),
}

impl From<common_attrs::CoreAttributes> for RadialGradientAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for RadialGradientAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl From<common_attrs::XLinkAttributes> for RadialGradientAttrs {
	fn from(attr: common_attrs::XLinkAttributes) -> Self {
		Self::XLinkAttributes(attr)
	}
}

impl RadialGradientAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::Cx => "cx",
			Self::Cy => "cy",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Fx => "fx",
			Self::Fy => "fy",
			Self::GradientTransform => "gradientTransform",
			Self::GradientUnits => "gradientUnits",
			Self::R => "r",
			Self::SpreadMethod => "spreadMethod",
			Self::Style => "style",
			Self::XlinkHref => "xlink:href",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
			Self::XLinkAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for RadialGradientAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for RadialGradientAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod radial_gradient_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Desc {}
	impl Content for super::Metadata {}
	impl Content for super::Set {}
	impl Content for super::Stop {}
	impl Content for super::Title {}
}

#[doc = "The [`<radialGradient>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("radialGradient.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateTransform>`](AnimateTransform)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<set>`](Set)\n",
	"- [`<stop>`](Stop)\n",
	"- [`<title>`](Title)\n"
)]
#[doc = "\n\n [`<radialGradient>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/radialGradient"]
#[derive(Debug)]
pub struct RadialGradient {
	attrs: IndexMap<RadialGradientAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn radial_gradient_private::Content>>,
}

impl Default for RadialGradient {
	fn default() -> Self {
		Self::new()
	}
}

impl RadialGradient {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: radial_gradient_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: radial_gradient_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: RadialGradientAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: RadialGradientAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(RadialGradientAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RadialGradientAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(RadialGradientAttrs::Class)
	}

	/// Set the `cx` attribute.
	pub fn with_cx<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(RadialGradientAttrs::Cx, value);
		self
	}

	/// Set the `cx` attribute.
	pub fn set_cx<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RadialGradientAttrs::Cx, value);
	}

	/// Get the `cx` attribute.
	pub fn cx(&self) -> Option<&dyn Value> {
		self.get_attr(RadialGradientAttrs::Cx)
	}

	/// Set the `cy` attribute.
	pub fn with_cy<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(RadialGradientAttrs::Cy, value);
		self
	}

	/// Set the `cy` attribute.
	pub fn set_cy<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RadialGradientAttrs::Cy, value);
	}

	/// Get the `cy` attribute.
	pub fn cy(&self) -> Option<&dyn Value> {
		self.get_attr(RadialGradientAttrs::Cy)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(RadialGradientAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RadialGradientAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(RadialGradientAttrs::ExternalResourcesRequired)
	}

	/// Set the `fx` attribute.
	pub fn with_fx<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(RadialGradientAttrs::Fx, value);
		self
	}

	/// Set the `fx` attribute.
	pub fn set_fx<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RadialGradientAttrs::Fx, value);
	}

	/// Get the `fx` attribute.
	pub fn fx(&self) -> Option<&dyn Value> {
		self.get_attr(RadialGradientAttrs::Fx)
	}

	/// Set the `fy` attribute.
	pub fn with_fy<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(RadialGradientAttrs::Fy, value);
		self
	}

	/// Set the `fy` attribute.
	pub fn set_fy<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RadialGradientAttrs::Fy, value);
	}

	/// Get the `fy` attribute.
	pub fn fy(&self) -> Option<&dyn Value> {
		self.get_attr(RadialGradientAttrs::Fy)
	}

	/// Set the `gradientTransform` attribute.
	pub fn with_gradient_transform<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(RadialGradientAttrs::GradientTransform, value);
		self
	}

	/// Set the `gradientTransform` attribute.
	pub fn set_gradient_transform<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RadialGradientAttrs::GradientTransform, value);
	}

	/// Get the `gradientTransform` attribute.
	pub fn gradient_transform(&self) -> Option<&dyn Value> {
		self.get_attr(RadialGradientAttrs::GradientTransform)
	}

	/// Set the `gradientUnits` attribute.
	pub fn with_gradient_units<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(RadialGradientAttrs::GradientUnits, value);
		self
	}

	/// Set the `gradientUnits` attribute.
	pub fn set_gradient_units<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RadialGradientAttrs::GradientUnits, value);
	}

	/// Get the `gradientUnits` attribute.
	pub fn gradient_units(&self) -> Option<&dyn Value> {
		self.get_attr(RadialGradientAttrs::GradientUnits)
	}

	/// Set the `r` attribute.
	pub fn with_r<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(RadialGradientAttrs::R, value);
		self
	}

	/// Set the `r` attribute.
	pub fn set_r<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RadialGradientAttrs::R, value);
	}

	/// Get the `r` attribute.
	pub fn r(&self) -> Option<&dyn Value> {
		self.get_attr(RadialGradientAttrs::R)
	}

	/// Set the `spreadMethod` attribute.
	pub fn with_spread_method<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(RadialGradientAttrs::SpreadMethod, value);
		self
	}

	/// Set the `spreadMethod` attribute.
	pub fn set_spread_method<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RadialGradientAttrs::SpreadMethod, value);
	}

	/// Get the `spreadMethod` attribute.
	pub fn spread_method(&self) -> Option<&dyn Value> {
		self.get_attr(RadialGradientAttrs::SpreadMethod)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(RadialGradientAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RadialGradientAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(RadialGradientAttrs::Style)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(RadialGradientAttrs::XlinkHref, value);
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RadialGradientAttrs::XlinkHref, value);
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&dyn Value> {
		self.get_attr(RadialGradientAttrs::XlinkHref)
	}
}

impl common_attrs::CoreAttributesSetter for RadialGradient {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RadialGradientAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(RadialGradientAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for RadialGradient {}

impl common_attrs::PresentationAttributesSetter for RadialGradient {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RadialGradientAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(RadialGradientAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for RadialGradient {}

impl common_attrs::XLinkAttributesSetter for RadialGradient {
	fn set_attr<V>(&mut self, attr: common_attrs::XLinkAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RadialGradientAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&dyn Value> {
		self.get_attr(RadialGradientAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for RadialGradient {}

impl Tag for RadialGradient {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("radialGradient");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum RectAttrs {
	Class,
	ExternalResourcesRequired,
	Height,
	Rx,
	Ry,
	Style,
	Transform,
	Width,
	X,
	Y,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	GlobalEventAttributes(common_attrs::GlobalEventAttributes),
	GraphicalEventAttributes(common_attrs::GraphicalEventAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::ConditionalProcessingAttributes> for RectAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for RectAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::GlobalEventAttributes> for RectAttrs {
	fn from(attr: common_attrs::GlobalEventAttributes) -> Self {
		Self::GlobalEventAttributes(attr)
	}
}

impl From<common_attrs::GraphicalEventAttributes> for RectAttrs {
	fn from(attr: common_attrs::GraphicalEventAttributes) -> Self {
		Self::GraphicalEventAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for RectAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl RectAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Height => "height",
			Self::Rx => "rx",
			Self::Ry => "ry",
			Self::Style => "style",
			Self::Transform => "transform",
			Self::Width => "width",
			Self::X => "x",
			Self::Y => "y",
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::GlobalEventAttributes(attr) => attr.as_str(),
			Self::GraphicalEventAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for RectAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for RectAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod rect_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::AnimateMotion {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Desc {}
	impl Content for super::Discard {}
	impl Content for super::Metadata {}
	impl Content for super::Mpath {}
	impl Content for super::Set {}
	impl Content for super::Title {}
}

#[doc = "The [`<rect>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("rect.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateMotion>`](AnimateMotion)\n",
	"- [`<animateTransform>`](AnimateTransform)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<discard>`](Discard)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<mpath>`](Mpath)\n",
	"- [`<set>`](Set)\n",
	"- [`<title>`](Title)\n"
)]
#[doc = "\n\n [`<rect>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/rect"]
#[derive(Debug)]
pub struct Rect {
	attrs: IndexMap<RectAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn rect_private::Content>>,
}

impl Default for Rect {
	fn default() -> Self {
		Self::new()
	}
}

impl Rect {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: rect_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: rect_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: RectAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: RectAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(RectAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RectAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(RectAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(RectAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RectAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(RectAttrs::ExternalResourcesRequired)
	}

	/// Set the `height` attribute.
	pub fn with_height<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(RectAttrs::Height, value);
		self
	}

	/// Set the `height` attribute.
	pub fn set_height<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RectAttrs::Height, value);
	}

	/// Get the `height` attribute.
	pub fn height(&self) -> Option<&dyn Value> {
		self.get_attr(RectAttrs::Height)
	}

	/// Set the `rx` attribute.
	pub fn with_rx<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(RectAttrs::Rx, value);
		self
	}

	/// Set the `rx` attribute.
	pub fn set_rx<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RectAttrs::Rx, value);
	}

	/// Get the `rx` attribute.
	pub fn rx(&self) -> Option<&dyn Value> {
		self.get_attr(RectAttrs::Rx)
	}

	/// Set the `ry` attribute.
	pub fn with_ry<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(RectAttrs::Ry, value);
		self
	}

	/// Set the `ry` attribute.
	pub fn set_ry<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RectAttrs::Ry, value);
	}

	/// Get the `ry` attribute.
	pub fn ry(&self) -> Option<&dyn Value> {
		self.get_attr(RectAttrs::Ry)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(RectAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RectAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(RectAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(RectAttrs::Transform, value);
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RectAttrs::Transform, value);
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&dyn Value> {
		self.get_attr(RectAttrs::Transform)
	}

	/// Set the `width` attribute.
	pub fn with_width<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(RectAttrs::Width, value);
		self
	}

	/// Set the `width` attribute.
	pub fn set_width<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RectAttrs::Width, value);
	}

	/// Get the `width` attribute.
	pub fn width(&self) -> Option<&dyn Value> {
		self.get_attr(RectAttrs::Width)
	}

	/// Set the `x` attribute.
	pub fn with_x<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(RectAttrs::X, value);
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RectAttrs::X, value);
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&dyn Value> {
		self.get_attr(RectAttrs::X)
	}

	/// Set the `y` attribute.
	pub fn with_y<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(RectAttrs::Y, value);
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RectAttrs::Y, value);
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&dyn Value> {
		self.get_attr(RectAttrs::Y)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Rect {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RectAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(RectAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Rect {}

impl common_attrs::CoreAttributesSetter for Rect {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RectAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(RectAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Rect {}

impl common_attrs::GlobalEventAttributesSetter for Rect {
	fn set_attr<V>(&mut self, attr: common_attrs::GlobalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RectAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GlobalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(RectAttrs::from(attr))
	}
}

impl TagWithGlobalEventAttributes for Rect {}

impl common_attrs::GraphicalEventAttributesSetter for Rect {
	fn set_attr<V>(&mut self, attr: common_attrs::GraphicalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RectAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(RectAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Rect {}

impl common_attrs::PresentationAttributesSetter for Rect {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(RectAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(RectAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Rect {}

impl Tag for Rect {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("rect");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum ScriptAttrs {
	ExternalResourcesRequired,
	Type,
	XlinkHref,
	CoreAttributes(common_attrs::CoreAttributes),
	XLinkAttributes(common_attrs::XLinkAttributes),
}

impl From<common_attrs::CoreAttributes> for ScriptAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::XLinkAttributes> for ScriptAttrs {
	fn from(attr: common_attrs::XLinkAttributes) -> Self {
		Self::XLinkAttributes(attr)
	}
}

impl ScriptAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Type => "type",
			Self::XlinkHref => "xlink:href",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::XLinkAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for ScriptAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for ScriptAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[doc = "The [`<script>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("script.md")]
#[doc = "\n\n [`<script>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/script"]
#[derive(Debug)]
pub struct Script {
	attrs: IndexMap<ScriptAttrs, Box<dyn Value>>,
	content: String,
}

impl Default for Script {
	fn default() -> Self {
		Self::new()
	}
}

impl Script {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: String::new()
		}
	}

	pub fn push<T: Display>(&mut self, content: T) {
		write!(self.content, "{content}").unwrap();
	}

	pub fn append<T: Display>(mut self, content: T) -> Self {
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: ScriptAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: ScriptAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ScriptAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ScriptAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(ScriptAttrs::ExternalResourcesRequired)
	}

	/// Set the `type` attribute.
	pub fn with_ty<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ScriptAttrs::Type, value);
		self
	}

	/// Set the `type` attribute.
	pub fn set_ty<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ScriptAttrs::Type, value);
	}

	/// Get the `type` attribute.
	pub fn ty(&self) -> Option<&dyn Value> {
		self.get_attr(ScriptAttrs::Type)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ScriptAttrs::XlinkHref, value);
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ScriptAttrs::XlinkHref, value);
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&dyn Value> {
		self.get_attr(ScriptAttrs::XlinkHref)
	}
}

impl common_attrs::CoreAttributesSetter for Script {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ScriptAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(ScriptAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Script {}

impl common_attrs::XLinkAttributesSetter for Script {
	fn set_attr<V>(&mut self, attr: common_attrs::XLinkAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ScriptAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&dyn Value> {
		self.get_attr(ScriptAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for Script {}

impl Tag for Script {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("script");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		w.write_cdata_text(&self.content);
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum SetAttrs {
	ExternalResourcesRequired,
	To,
	AnimationAttributeTargetAttributes(common_attrs::AnimationAttributeTargetAttributes),
	AnimationEventAttributes(common_attrs::AnimationEventAttributes),
	AnimationTimingAttributes(common_attrs::AnimationTimingAttributes),
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	XLinkAttributes(common_attrs::XLinkAttributes),
}

impl From<common_attrs::AnimationAttributeTargetAttributes> for SetAttrs {
	fn from(attr: common_attrs::AnimationAttributeTargetAttributes) -> Self {
		Self::AnimationAttributeTargetAttributes(attr)
	}
}

impl From<common_attrs::AnimationEventAttributes> for SetAttrs {
	fn from(attr: common_attrs::AnimationEventAttributes) -> Self {
		Self::AnimationEventAttributes(attr)
	}
}

impl From<common_attrs::AnimationTimingAttributes> for SetAttrs {
	fn from(attr: common_attrs::AnimationTimingAttributes) -> Self {
		Self::AnimationTimingAttributes(attr)
	}
}

impl From<common_attrs::ConditionalProcessingAttributes> for SetAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for SetAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::XLinkAttributes> for SetAttrs {
	fn from(attr: common_attrs::XLinkAttributes) -> Self {
		Self::XLinkAttributes(attr)
	}
}

impl SetAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::To => "to",
			Self::AnimationAttributeTargetAttributes(attr) => attr.as_str(),
			Self::AnimationEventAttributes(attr) => attr.as_str(),
			Self::AnimationTimingAttributes(attr) => attr.as_str(),
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::XLinkAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for SetAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for SetAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod set_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Desc {}
	impl Content for super::Metadata {}
	impl Content for super::Title {}
}

#[doc = "The [`<set>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("set.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<desc>`](Desc)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<title>`](Title)\n"
)]
#[doc = "\n\n [`<set>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/set"]
#[derive(Debug)]
pub struct Set {
	attrs: IndexMap<SetAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn set_private::Content>>,
}

impl Default for Set {
	fn default() -> Self {
		Self::new()
	}
}

impl Set {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: set_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: set_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: SetAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: SetAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(SetAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SetAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(SetAttrs::ExternalResourcesRequired)
	}

	/// Set the `to` attribute.
	pub fn with_to<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(SetAttrs::To, value);
		self
	}

	/// Set the `to` attribute.
	pub fn set_to<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SetAttrs::To, value);
	}

	/// Get the `to` attribute.
	pub fn to(&self) -> Option<&dyn Value> {
		self.get_attr(SetAttrs::To)
	}
}

impl common_attrs::AnimationAttributeTargetAttributesSetter for Set {
	fn set_attr<V>(&mut self, attr: common_attrs::AnimationAttributeTargetAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SetAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationAttributeTargetAttributes) -> Option<&dyn Value> {
		self.get_attr(SetAttrs::from(attr))
	}
}

impl TagWithAnimationAttributeTargetAttributes for Set {}

impl common_attrs::AnimationEventAttributesSetter for Set {
	fn set_attr<V>(&mut self, attr: common_attrs::AnimationEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SetAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationEventAttributes) -> Option<&dyn Value> {
		self.get_attr(SetAttrs::from(attr))
	}
}

impl TagWithAnimationEventAttributes for Set {}

impl common_attrs::AnimationTimingAttributesSetter for Set {
	fn set_attr<V>(&mut self, attr: common_attrs::AnimationTimingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SetAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationTimingAttributes) -> Option<&dyn Value> {
		self.get_attr(SetAttrs::from(attr))
	}
}

impl TagWithAnimationTimingAttributes for Set {}

impl common_attrs::ConditionalProcessingAttributesSetter for Set {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SetAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(SetAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Set {}

impl common_attrs::CoreAttributesSetter for Set {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SetAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(SetAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Set {}

impl common_attrs::XLinkAttributesSetter for Set {
	fn set_attr<V>(&mut self, attr: common_attrs::XLinkAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SetAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&dyn Value> {
		self.get_attr(SetAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for Set {}

impl Tag for Set {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("set");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum StopAttrs {
	Class,
	Offset,
	StopColor,
	StopOpacity,
	Style,
	CoreAttributes(common_attrs::CoreAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::CoreAttributes> for StopAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for StopAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl StopAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::Offset => "offset",
			Self::StopColor => "stop-color",
			Self::StopOpacity => "stop-opacity",
			Self::Style => "style",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for StopAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for StopAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod stop_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::AnimateColor {}
	impl Content for super::Set {}
}

#[doc = "The [`<stop>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("stop.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateColor>`](AnimateColor)\n",
	"- [`<set>`](Set)\n"
)]
#[doc = "\n\n [`<stop>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/stop"]
#[derive(Debug)]
pub struct Stop {
	attrs: IndexMap<StopAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn stop_private::Content>>,
}

impl Default for Stop {
	fn default() -> Self {
		Self::new()
	}
}

impl Stop {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: stop_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: stop_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: StopAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: StopAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(StopAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(StopAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(StopAttrs::Class)
	}

	/// Set the `offset` attribute.
	pub fn with_offset<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(StopAttrs::Offset, value);
		self
	}

	/// Set the `offset` attribute.
	pub fn set_offset<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(StopAttrs::Offset, value);
	}

	/// Get the `offset` attribute.
	pub fn offset(&self) -> Option<&dyn Value> {
		self.get_attr(StopAttrs::Offset)
	}

	/// Set the `stop-color` attribute.
	pub fn with_stop_color<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(StopAttrs::StopColor, value);
		self
	}

	/// Set the `stop-color` attribute.
	pub fn set_stop_color<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(StopAttrs::StopColor, value);
	}

	/// Get the `stop-color` attribute.
	pub fn stop_color(&self) -> Option<&dyn Value> {
		self.get_attr(StopAttrs::StopColor)
	}

	/// Set the `stop-opacity` attribute.
	pub fn with_stop_opacity<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(StopAttrs::StopOpacity, value);
		self
	}

	/// Set the `stop-opacity` attribute.
	pub fn set_stop_opacity<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(StopAttrs::StopOpacity, value);
	}

	/// Get the `stop-opacity` attribute.
	pub fn stop_opacity(&self) -> Option<&dyn Value> {
		self.get_attr(StopAttrs::StopOpacity)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(StopAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(StopAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(StopAttrs::Style)
	}
}

impl common_attrs::CoreAttributesSetter for Stop {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(StopAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(StopAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Stop {}

impl common_attrs::PresentationAttributesSetter for Stop {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(StopAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(StopAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Stop {}

impl Tag for Stop {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("stop");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum StyleAttrs {
	Media,
	Title,
	Type,
	CoreAttributes(common_attrs::CoreAttributes),
}

impl From<common_attrs::CoreAttributes> for StyleAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl StyleAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Media => "media",
			Self::Title => "title",
			Self::Type => "type",
			Self::CoreAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for StyleAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for StyleAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[doc = "The [`<style>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("style.md")]
#[doc = "\n\n [`<style>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/style"]
#[derive(Debug)]
pub struct Style {
	attrs: IndexMap<StyleAttrs, Box<dyn Value>>,
	content: String,
}

impl Default for Style {
	fn default() -> Self {
		Self::new()
	}
}

impl Style {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: String::new()
		}
	}

	pub fn push<T: Display>(&mut self, content: T) {
		write!(self.content, "{content}").unwrap();
	}

	pub fn append<T: Display>(mut self, content: T) -> Self {
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: StyleAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: StyleAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `media` attribute.
	pub fn with_media<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(StyleAttrs::Media, value);
		self
	}

	/// Set the `media` attribute.
	pub fn set_media<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(StyleAttrs::Media, value);
	}

	/// Get the `media` attribute.
	pub fn media(&self) -> Option<&dyn Value> {
		self.get_attr(StyleAttrs::Media)
	}

	/// Set the `title` attribute.
	pub fn with_title<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(StyleAttrs::Title, value);
		self
	}

	/// Set the `title` attribute.
	pub fn set_title<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(StyleAttrs::Title, value);
	}

	/// Get the `title` attribute.
	pub fn title(&self) -> Option<&dyn Value> {
		self.get_attr(StyleAttrs::Title)
	}

	/// Set the `type` attribute.
	pub fn with_ty<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(StyleAttrs::Type, value);
		self
	}

	/// Set the `type` attribute.
	pub fn set_ty<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(StyleAttrs::Type, value);
	}

	/// Get the `type` attribute.
	pub fn ty(&self) -> Option<&dyn Value> {
		self.get_attr(StyleAttrs::Type)
	}
}

impl common_attrs::CoreAttributesSetter for Style {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(StyleAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(StyleAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Style {}

impl Tag for Style {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("style");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		w.write_cdata_text(&self.content);
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub(crate) enum SvgAttrs {
	BaseProfile,
	Class,
	ContentScriptType,
	ContentStyleType,
	ExternalResourcesRequired,
	Height,
	PreserveAspectRatio,
	Style,
	Version,
	ViewBox,
	Width,
	X,
	Y,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	DocumentEventAttributes(common_attrs::DocumentEventAttributes),
	GlobalEventAttributes(common_attrs::GlobalEventAttributes),
	GraphicalEventAttributes(common_attrs::GraphicalEventAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
	Custom(&'static str),
}

impl From<common_attrs::ConditionalProcessingAttributes> for SvgAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for SvgAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::DocumentEventAttributes> for SvgAttrs {
	fn from(attr: common_attrs::DocumentEventAttributes) -> Self {
		Self::DocumentEventAttributes(attr)
	}
}

impl From<common_attrs::GlobalEventAttributes> for SvgAttrs {
	fn from(attr: common_attrs::GlobalEventAttributes) -> Self {
		Self::GlobalEventAttributes(attr)
	}
}

impl From<common_attrs::GraphicalEventAttributes> for SvgAttrs {
	fn from(attr: common_attrs::GraphicalEventAttributes) -> Self {
		Self::GraphicalEventAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for SvgAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl From<&'static str> for SvgAttrs {
	fn from(attr: &'static str) -> Self {
		Self::Custom(attr)
	}
}

impl SvgAttrs {
	pub(crate) fn as_str(&self) -> &'static str {
		match self {
			Self::BaseProfile => "baseProfile",
			Self::Class => "class",
			Self::ContentScriptType => "contentScriptType",
			Self::ContentStyleType => "contentStyleType",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Height => "height",
			Self::PreserveAspectRatio => "preserveAspectRatio",
			Self::Style => "style",
			Self::Version => "version",
			Self::ViewBox => "viewBox",
			Self::Width => "width",
			Self::X => "x",
			Self::Y => "y",
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::DocumentEventAttributes(attr) => attr.as_str(),
			Self::GlobalEventAttributes(attr) => attr.as_str(),
			Self::GraphicalEventAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
			Self::Custom(attr) => attr,
		}
	}
}

impl Display for SvgAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for SvgAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod svg_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::A {}
	impl Content for super::AltGlyphDef {}
	impl Content for super::Animate {}
	impl Content for super::AnimateColor {}
	impl Content for super::AnimateMotion {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Circle {}
	impl Content for super::ClipPath {}
	impl Content for super::ColorProfile {}
	impl Content for super::Cursor {}
	impl Content for super::Defs {}
	impl Content for super::Desc {}
	impl Content for super::Discard {}
	impl Content for super::Ellipse {}
	impl Content for super::Filter {}
	impl Content for super::Font {}
	impl Content for super::FontFace {}
	impl Content for super::ForeignObject {}
	impl Content for super::Group {}
	impl Content for super::Image {}
	impl Content for super::Line {}
	impl Content for super::LinearGradient {}
	impl Content for super::Marker {}
	impl Content for super::Mask {}
	impl Content for super::Metadata {}
	impl Content for super::Mpath {}
	impl Content for super::Path {}
	impl Content for super::Pattern {}
	impl Content for super::Polygon {}
	impl Content for super::Polyline {}
	impl Content for super::RadialGradient {}
	impl Content for super::Rect {}
	impl Content for super::Script {}
	impl Content for super::Set {}
	impl Content for super::Stop {}
	impl Content for super::Style {}
	impl Content for super::Svg {}
	impl Content for super::Switch {}
	impl Content for super::Symbol {}
	impl Content for super::Text {}
	impl Content for super::Title {}
	impl Content for super::Use {}
	impl Content for super::View {}
}

#[doc = "The [`<svg>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("svg.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<a>`](A)\n",
	"- [`<altGlyphDef>`](AltGlyphDef)\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateColor>`](AnimateColor)\n",
	"- [`<animateMotion>`](AnimateMotion)\n",
	"- [`<animateTransform>`](AnimateTransform)\n",
	"- [`<circle>`](Circle)\n",
	"- [`<clipPath>`](ClipPath)\n",
	"- [`<color-profile>`](ColorProfile)\n",
	"- [`<cursor>`](Cursor)\n",
	"- [`<defs>`](Defs)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<discard>`](Discard)\n",
	"- [`<ellipse>`](Ellipse)\n",
	"- [`<filter>`](Filter)\n",
	"- [`<font>`](Font)\n",
	"- [`<font-face>`](FontFace)\n",
	"- [`<foreignObject>`](ForeignObject)\n",
	"- [`<g>`](Group)\n",
	"- [`<image>`](Image)\n",
	"- [`<line>`](Line)\n",
	"- [`<linearGradient>`](LinearGradient)\n",
	"- [`<marker>`](Marker)\n",
	"- [`<mask>`](Mask)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<mpath>`](Mpath)\n",
	"- [`<path>`](Path)\n",
	"- [`<pattern>`](Pattern)\n",
	"- [`<polygon>`](Polygon)\n",
	"- [`<polyline>`](Polyline)\n",
	"- [`<radialGradient>`](RadialGradient)\n",
	"- [`<rect>`](Rect)\n",
	"- [`<script>`](Script)\n",
	"- [`<set>`](Set)\n",
	"- [`<stop>`](Stop)\n",
	"- [`<style>`](Style)\n",
	"- [`<svg>`](Svg)\n",
	"- [`<switch>`](Switch)\n",
	"- [`<symbol>`](Symbol)\n",
	"- [`<text>`](Text)\n",
	"- [`<title>`](Title)\n",
	"- [`<use>`](Use)\n",
	"- [`<view>`](View)\n"
)]
#[doc = "\n\n [`<svg>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/svg"]
#[derive(Debug)]
pub struct Svg {
	attrs: IndexMap<SvgAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn svg_private::Content>>,
}

impl Default for Svg {
	fn default() -> Self {
		Self::new()
	}
}

impl Svg {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: svg_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: svg_private::Content + 'static
	{
		self.push(content);
		self
	}

	pub(crate) fn set_attr<V>(&mut self, attr: SvgAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: SvgAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `baseProfile` attribute.
	pub fn with_base_profile<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::BaseProfile, value);
		self
	}

	/// Set the `baseProfile` attribute.
	pub fn set_base_profile<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::BaseProfile, value);
	}

	/// Get the `baseProfile` attribute.
	pub fn base_profile(&self) -> Option<&dyn Value> {
		self.get_attr(SvgAttrs::BaseProfile)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(SvgAttrs::Class)
	}

	/// Set the `contentScriptType` attribute.
	pub fn with_content_script_type<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::ContentScriptType, value);
		self
	}

	/// Set the `contentScriptType` attribute.
	pub fn set_content_script_type<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::ContentScriptType, value);
	}

	/// Get the `contentScriptType` attribute.
	pub fn content_script_type(&self) -> Option<&dyn Value> {
		self.get_attr(SvgAttrs::ContentScriptType)
	}

	/// Set the `contentStyleType` attribute.
	pub fn with_content_style_type<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::ContentStyleType, value);
		self
	}

	/// Set the `contentStyleType` attribute.
	pub fn set_content_style_type<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::ContentStyleType, value);
	}

	/// Get the `contentStyleType` attribute.
	pub fn content_style_type(&self) -> Option<&dyn Value> {
		self.get_attr(SvgAttrs::ContentStyleType)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(SvgAttrs::ExternalResourcesRequired)
	}

	/// Set the `height` attribute.
	pub fn with_height<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::Height, value);
		self
	}

	/// Set the `height` attribute.
	pub fn set_height<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::Height, value);
	}

	/// Get the `height` attribute.
	pub fn height(&self) -> Option<&dyn Value> {
		self.get_attr(SvgAttrs::Height)
	}

	/// Set the `preserveAspectRatio` attribute.
	pub fn with_preserve_aspect_ratio<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::PreserveAspectRatio, value);
		self
	}

	/// Set the `preserveAspectRatio` attribute.
	pub fn set_preserve_aspect_ratio<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::PreserveAspectRatio, value);
	}

	/// Get the `preserveAspectRatio` attribute.
	pub fn preserve_aspect_ratio(&self) -> Option<&dyn Value> {
		self.get_attr(SvgAttrs::PreserveAspectRatio)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(SvgAttrs::Style)
	}

	/// Set the `version` attribute.
	pub fn with_version<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::Version, value);
		self
	}

	/// Set the `version` attribute.
	pub fn set_version<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::Version, value);
	}

	/// Get the `version` attribute.
	pub fn version(&self) -> Option<&dyn Value> {
		self.get_attr(SvgAttrs::Version)
	}

	/// Set the `viewBox` attribute.
	pub fn with_view_box<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::ViewBox, value);
		self
	}

	/// Set the `viewBox` attribute.
	pub fn set_view_box<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::ViewBox, value);
	}

	/// Get the `viewBox` attribute.
	pub fn view_box(&self) -> Option<&dyn Value> {
		self.get_attr(SvgAttrs::ViewBox)
	}

	/// Set the `width` attribute.
	pub fn with_width<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::Width, value);
		self
	}

	/// Set the `width` attribute.
	pub fn set_width<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::Width, value);
	}

	/// Get the `width` attribute.
	pub fn width(&self) -> Option<&dyn Value> {
		self.get_attr(SvgAttrs::Width)
	}

	/// Set the `x` attribute.
	pub fn with_x<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::X, value);
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::X, value);
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&dyn Value> {
		self.get_attr(SvgAttrs::X)
	}

	/// Set the `y` attribute.
	pub fn with_y<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::Y, value);
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::Y, value);
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&dyn Value> {
		self.get_attr(SvgAttrs::Y)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Svg {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(SvgAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Svg {}

impl common_attrs::CoreAttributesSetter for Svg {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(SvgAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Svg {}

impl common_attrs::DocumentEventAttributesSetter for Svg {
	fn set_attr<V>(&mut self, attr: common_attrs::DocumentEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::DocumentEventAttributes) -> Option<&dyn Value> {
		self.get_attr(SvgAttrs::from(attr))
	}
}

impl TagWithDocumentEventAttributes for Svg {}

impl common_attrs::GlobalEventAttributesSetter for Svg {
	fn set_attr<V>(&mut self, attr: common_attrs::GlobalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GlobalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(SvgAttrs::from(attr))
	}
}

impl TagWithGlobalEventAttributes for Svg {}

impl common_attrs::GraphicalEventAttributesSetter for Svg {
	fn set_attr<V>(&mut self, attr: common_attrs::GraphicalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(SvgAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Svg {}

impl common_attrs::PresentationAttributesSetter for Svg {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SvgAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(SvgAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Svg {}

impl Tag for Svg {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("svg");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum SwitchAttrs {
	AllowReorder,
	Class,
	ExternalResourcesRequired,
	Style,
	Transform,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	GlobalEventAttributes(common_attrs::GlobalEventAttributes),
	GraphicalEventAttributes(common_attrs::GraphicalEventAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::ConditionalProcessingAttributes> for SwitchAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for SwitchAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::GlobalEventAttributes> for SwitchAttrs {
	fn from(attr: common_attrs::GlobalEventAttributes) -> Self {
		Self::GlobalEventAttributes(attr)
	}
}

impl From<common_attrs::GraphicalEventAttributes> for SwitchAttrs {
	fn from(attr: common_attrs::GraphicalEventAttributes) -> Self {
		Self::GraphicalEventAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for SwitchAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl SwitchAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::AllowReorder => "allowReorder",
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Style => "style",
			Self::Transform => "transform",
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::GlobalEventAttributes(attr) => attr.as_str(),
			Self::GraphicalEventAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for SwitchAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for SwitchAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod switch_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::A {}
	impl Content for super::Animate {}
	impl Content for super::AnimateColor {}
	impl Content for super::AnimateMotion {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Circle {}
	impl Content for super::Desc {}
	impl Content for super::Discard {}
	impl Content for super::Ellipse {}
	impl Content for super::ForeignObject {}
	impl Content for super::Group {}
	impl Content for super::Image {}
	impl Content for super::Line {}
	impl Content for super::Metadata {}
	impl Content for super::Mpath {}
	impl Content for super::Path {}
	impl Content for super::Polygon {}
	impl Content for super::Polyline {}
	impl Content for super::Rect {}
	impl Content for super::Set {}
	impl Content for super::Svg {}
	impl Content for super::Switch {}
	impl Content for super::Text {}
	impl Content for super::Title {}
	impl Content for super::Use {}
}

#[doc = "The [`<switch>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("switch.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<a>`](A)\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateColor>`](AnimateColor)\n",
	"- [`<animateMotion>`](AnimateMotion)\n",
	"- [`<animateTransform>`](AnimateTransform)\n",
	"- [`<circle>`](Circle)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<discard>`](Discard)\n",
	"- [`<ellipse>`](Ellipse)\n",
	"- [`<foreignObject>`](ForeignObject)\n",
	"- [`<g>`](Group)\n",
	"- [`<image>`](Image)\n",
	"- [`<line>`](Line)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<mpath>`](Mpath)\n",
	"- [`<path>`](Path)\n",
	"- [`<polygon>`](Polygon)\n",
	"- [`<polyline>`](Polyline)\n",
	"- [`<rect>`](Rect)\n",
	"- [`<set>`](Set)\n",
	"- [`<svg>`](Svg)\n",
	"- [`<switch>`](Switch)\n",
	"- [`<text>`](Text)\n",
	"- [`<title>`](Title)\n",
	"- [`<use>`](Use)\n"
)]
#[doc = "\n\n [`<switch>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/switch"]
#[derive(Debug)]
pub struct Switch {
	attrs: IndexMap<SwitchAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn switch_private::Content>>,
}

impl Default for Switch {
	fn default() -> Self {
		Self::new()
	}
}

impl Switch {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: switch_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: switch_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: SwitchAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: SwitchAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `allowReorder` attribute.
	pub fn with_allow_reorder<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(SwitchAttrs::AllowReorder, value);
		self
	}

	/// Set the `allowReorder` attribute.
	pub fn set_allow_reorder<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SwitchAttrs::AllowReorder, value);
	}

	/// Get the `allowReorder` attribute.
	pub fn allow_reorder(&self) -> Option<&dyn Value> {
		self.get_attr(SwitchAttrs::AllowReorder)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(SwitchAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SwitchAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(SwitchAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(SwitchAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SwitchAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(SwitchAttrs::ExternalResourcesRequired)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(SwitchAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SwitchAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(SwitchAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(SwitchAttrs::Transform, value);
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SwitchAttrs::Transform, value);
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&dyn Value> {
		self.get_attr(SwitchAttrs::Transform)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Switch {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SwitchAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(SwitchAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Switch {}

impl common_attrs::CoreAttributesSetter for Switch {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SwitchAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(SwitchAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Switch {}

impl common_attrs::GlobalEventAttributesSetter for Switch {
	fn set_attr<V>(&mut self, attr: common_attrs::GlobalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SwitchAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GlobalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(SwitchAttrs::from(attr))
	}
}

impl TagWithGlobalEventAttributes for Switch {}

impl common_attrs::GraphicalEventAttributesSetter for Switch {
	fn set_attr<V>(&mut self, attr: common_attrs::GraphicalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SwitchAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(SwitchAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Switch {}

impl common_attrs::PresentationAttributesSetter for Switch {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SwitchAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(SwitchAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Switch {}

impl Tag for Switch {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("switch");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum SymbolAttrs {
	Class,
	ExternalResourcesRequired,
	PreserveAspectRatio,
	Style,
	ViewBox,
	CoreAttributes(common_attrs::CoreAttributes),
	GlobalEventAttributes(common_attrs::GlobalEventAttributes),
	GraphicalEventAttributes(common_attrs::GraphicalEventAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::CoreAttributes> for SymbolAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::GlobalEventAttributes> for SymbolAttrs {
	fn from(attr: common_attrs::GlobalEventAttributes) -> Self {
		Self::GlobalEventAttributes(attr)
	}
}

impl From<common_attrs::GraphicalEventAttributes> for SymbolAttrs {
	fn from(attr: common_attrs::GraphicalEventAttributes) -> Self {
		Self::GraphicalEventAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for SymbolAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl SymbolAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::PreserveAspectRatio => "preserveAspectRatio",
			Self::Style => "style",
			Self::ViewBox => "viewBox",
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::GlobalEventAttributes(attr) => attr.as_str(),
			Self::GraphicalEventAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for SymbolAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for SymbolAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod symbol_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::A {}
	impl Content for super::AltGlyphDef {}
	impl Content for super::Animate {}
	impl Content for super::AnimateColor {}
	impl Content for super::AnimateMotion {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Circle {}
	impl Content for super::ClipPath {}
	impl Content for super::ColorProfile {}
	impl Content for super::Cursor {}
	impl Content for super::Defs {}
	impl Content for super::Desc {}
	impl Content for super::Discard {}
	impl Content for super::Ellipse {}
	impl Content for super::Filter {}
	impl Content for super::Font {}
	impl Content for super::FontFace {}
	impl Content for super::ForeignObject {}
	impl Content for super::Group {}
	impl Content for super::Image {}
	impl Content for super::Line {}
	impl Content for super::LinearGradient {}
	impl Content for super::Marker {}
	impl Content for super::Mask {}
	impl Content for super::Metadata {}
	impl Content for super::Mpath {}
	impl Content for super::Path {}
	impl Content for super::Pattern {}
	impl Content for super::Polygon {}
	impl Content for super::Polyline {}
	impl Content for super::RadialGradient {}
	impl Content for super::Rect {}
	impl Content for super::Script {}
	impl Content for super::Set {}
	impl Content for super::Stop {}
	impl Content for super::Style {}
	impl Content for super::Svg {}
	impl Content for super::Switch {}
	impl Content for super::Symbol {}
	impl Content for super::Text {}
	impl Content for super::Title {}
	impl Content for super::Use {}
	impl Content for super::View {}
}

#[doc = "The [`<symbol>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("symbol.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<a>`](A)\n",
	"- [`<altGlyphDef>`](AltGlyphDef)\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateColor>`](AnimateColor)\n",
	"- [`<animateMotion>`](AnimateMotion)\n",
	"- [`<animateTransform>`](AnimateTransform)\n",
	"- [`<circle>`](Circle)\n",
	"- [`<clipPath>`](ClipPath)\n",
	"- [`<color-profile>`](ColorProfile)\n",
	"- [`<cursor>`](Cursor)\n",
	"- [`<defs>`](Defs)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<discard>`](Discard)\n",
	"- [`<ellipse>`](Ellipse)\n",
	"- [`<filter>`](Filter)\n",
	"- [`<font>`](Font)\n",
	"- [`<font-face>`](FontFace)\n",
	"- [`<foreignObject>`](ForeignObject)\n",
	"- [`<g>`](Group)\n",
	"- [`<image>`](Image)\n",
	"- [`<line>`](Line)\n",
	"- [`<linearGradient>`](LinearGradient)\n",
	"- [`<marker>`](Marker)\n",
	"- [`<mask>`](Mask)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<mpath>`](Mpath)\n",
	"- [`<path>`](Path)\n",
	"- [`<pattern>`](Pattern)\n",
	"- [`<polygon>`](Polygon)\n",
	"- [`<polyline>`](Polyline)\n",
	"- [`<radialGradient>`](RadialGradient)\n",
	"- [`<rect>`](Rect)\n",
	"- [`<script>`](Script)\n",
	"- [`<set>`](Set)\n",
	"- [`<stop>`](Stop)\n",
	"- [`<style>`](Style)\n",
	"- [`<svg>`](Svg)\n",
	"- [`<switch>`](Switch)\n",
	"- [`<symbol>`](Symbol)\n",
	"- [`<text>`](Text)\n",
	"- [`<title>`](Title)\n",
	"- [`<use>`](Use)\n",
	"- [`<view>`](View)\n"
)]
#[doc = "\n\n [`<symbol>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/symbol"]
#[derive(Debug)]
pub struct Symbol {
	attrs: IndexMap<SymbolAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn symbol_private::Content>>,
}

impl Default for Symbol {
	fn default() -> Self {
		Self::new()
	}
}

impl Symbol {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: symbol_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: symbol_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: SymbolAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: SymbolAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(SymbolAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SymbolAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(SymbolAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(SymbolAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SymbolAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(SymbolAttrs::ExternalResourcesRequired)
	}

	/// Set the `preserveAspectRatio` attribute.
	pub fn with_preserve_aspect_ratio<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(SymbolAttrs::PreserveAspectRatio, value);
		self
	}

	/// Set the `preserveAspectRatio` attribute.
	pub fn set_preserve_aspect_ratio<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SymbolAttrs::PreserveAspectRatio, value);
	}

	/// Get the `preserveAspectRatio` attribute.
	pub fn preserve_aspect_ratio(&self) -> Option<&dyn Value> {
		self.get_attr(SymbolAttrs::PreserveAspectRatio)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(SymbolAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SymbolAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(SymbolAttrs::Style)
	}

	/// Set the `viewBox` attribute.
	pub fn with_view_box<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(SymbolAttrs::ViewBox, value);
		self
	}

	/// Set the `viewBox` attribute.
	pub fn set_view_box<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SymbolAttrs::ViewBox, value);
	}

	/// Get the `viewBox` attribute.
	pub fn view_box(&self) -> Option<&dyn Value> {
		self.get_attr(SymbolAttrs::ViewBox)
	}
}

impl common_attrs::CoreAttributesSetter for Symbol {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SymbolAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(SymbolAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Symbol {}

impl common_attrs::GlobalEventAttributesSetter for Symbol {
	fn set_attr<V>(&mut self, attr: common_attrs::GlobalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SymbolAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GlobalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(SymbolAttrs::from(attr))
	}
}

impl TagWithGlobalEventAttributes for Symbol {}

impl common_attrs::GraphicalEventAttributesSetter for Symbol {
	fn set_attr<V>(&mut self, attr: common_attrs::GraphicalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SymbolAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(SymbolAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Symbol {}

impl common_attrs::PresentationAttributesSetter for Symbol {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(SymbolAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(SymbolAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Symbol {}

impl Tag for Symbol {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("symbol");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum TextAttrs {
	Class,
	Dx,
	Dy,
	ExternalResourcesRequired,
	LengthAdjust,
	Rotate,
	Style,
	TextAnchor,
	TextLength,
	Transform,
	X,
	Y,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	GlobalEventAttributes(common_attrs::GlobalEventAttributes),
	GraphicalEventAttributes(common_attrs::GraphicalEventAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::ConditionalProcessingAttributes> for TextAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for TextAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::GlobalEventAttributes> for TextAttrs {
	fn from(attr: common_attrs::GlobalEventAttributes) -> Self {
		Self::GlobalEventAttributes(attr)
	}
}

impl From<common_attrs::GraphicalEventAttributes> for TextAttrs {
	fn from(attr: common_attrs::GraphicalEventAttributes) -> Self {
		Self::GraphicalEventAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for TextAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl TextAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::Dx => "dx",
			Self::Dy => "dy",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::LengthAdjust => "lengthAdjust",
			Self::Rotate => "rotate",
			Self::Style => "style",
			Self::TextAnchor => "text-anchor",
			Self::TextLength => "textLength",
			Self::Transform => "transform",
			Self::X => "x",
			Self::Y => "y",
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::GlobalEventAttributes(attr) => attr.as_str(),
			Self::GraphicalEventAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for TextAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for TextAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod text_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::A {}
	impl Content for super::AltGlyph {}
	impl Content for super::Animate {}
	impl Content for super::AnimateMotion {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Desc {}
	impl Content for super::Discard {}
	impl Content for super::Metadata {}
	impl Content for super::Mpath {}
	impl Content for super::Set {}
	impl Content for super::TextPath {}
	impl Content for super::Title {}
	impl Content for super::Tref {}
	impl Content for super::TSpan {}
	impl Content for String {}
	impl Content for &'static str {}
}

#[doc = "The [`<text>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("text.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<a>`](A)\n",
	"- [`<altGlyph>`](AltGlyph)\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateMotion>`](AnimateMotion)\n",
	"- [`<animateTransform>`](AnimateTransform)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<discard>`](Discard)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<mpath>`](Mpath)\n",
	"- [`<set>`](Set)\n",
	"- [`<textPath>`](TextPath)\n",
	"- [`<title>`](Title)\n",
	"- [`<tref>`](Tref)\n",
	"- [`<tspan>`](TSpan)\n"
)]
#[doc = "\n\n [`<text>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/text"]
#[derive(Debug)]
pub struct Text {
	attrs: IndexMap<TextAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn text_private::Content>>,
}

impl Default for Text {
	fn default() -> Self {
		Self::new()
	}
}

impl Text {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: text_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: text_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: TextAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: TextAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TextAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(TextAttrs::Class)
	}

	/// Set the `dx` attribute.
	pub fn with_dx<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TextAttrs::Dx, value);
		self
	}

	/// Set the `dx` attribute.
	pub fn set_dx<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextAttrs::Dx, value);
	}

	/// Get the `dx` attribute.
	pub fn dx(&self) -> Option<&dyn Value> {
		self.get_attr(TextAttrs::Dx)
	}

	/// Set the `dy` attribute.
	pub fn with_dy<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TextAttrs::Dy, value);
		self
	}

	/// Set the `dy` attribute.
	pub fn set_dy<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextAttrs::Dy, value);
	}

	/// Get the `dy` attribute.
	pub fn dy(&self) -> Option<&dyn Value> {
		self.get_attr(TextAttrs::Dy)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TextAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(TextAttrs::ExternalResourcesRequired)
	}

	/// Set the `lengthAdjust` attribute.
	pub fn with_length_adjust<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TextAttrs::LengthAdjust, value);
		self
	}

	/// Set the `lengthAdjust` attribute.
	pub fn set_length_adjust<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextAttrs::LengthAdjust, value);
	}

	/// Get the `lengthAdjust` attribute.
	pub fn length_adjust(&self) -> Option<&dyn Value> {
		self.get_attr(TextAttrs::LengthAdjust)
	}

	/// Set the `rotate` attribute.
	pub fn with_rotate<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TextAttrs::Rotate, value);
		self
	}

	/// Set the `rotate` attribute.
	pub fn set_rotate<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextAttrs::Rotate, value);
	}

	/// Get the `rotate` attribute.
	pub fn rotate(&self) -> Option<&dyn Value> {
		self.get_attr(TextAttrs::Rotate)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TextAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(TextAttrs::Style)
	}

	/// Set the `text-anchor` attribute.
	pub fn with_text_anchor<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TextAttrs::TextAnchor, value);
		self
	}

	/// Set the `text-anchor` attribute.
	pub fn set_text_anchor<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextAttrs::TextAnchor, value);
	}

	/// Get the `text-anchor` attribute.
	pub fn text_anchor(&self) -> Option<&dyn Value> {
		self.get_attr(TextAttrs::TextAnchor)
	}

	/// Set the `textLength` attribute.
	pub fn with_text_length<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TextAttrs::TextLength, value);
		self
	}

	/// Set the `textLength` attribute.
	pub fn set_text_length<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextAttrs::TextLength, value);
	}

	/// Get the `textLength` attribute.
	pub fn text_length(&self) -> Option<&dyn Value> {
		self.get_attr(TextAttrs::TextLength)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TextAttrs::Transform, value);
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextAttrs::Transform, value);
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&dyn Value> {
		self.get_attr(TextAttrs::Transform)
	}

	/// Set the `x` attribute.
	pub fn with_x<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TextAttrs::X, value);
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextAttrs::X, value);
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&dyn Value> {
		self.get_attr(TextAttrs::X)
	}

	/// Set the `y` attribute.
	pub fn with_y<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TextAttrs::Y, value);
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextAttrs::Y, value);
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&dyn Value> {
		self.get_attr(TextAttrs::Y)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Text {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(TextAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Text {}

impl common_attrs::CoreAttributesSetter for Text {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(TextAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Text {}

impl common_attrs::GlobalEventAttributesSetter for Text {
	fn set_attr<V>(&mut self, attr: common_attrs::GlobalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GlobalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(TextAttrs::from(attr))
	}
}

impl TagWithGlobalEventAttributes for Text {}

impl common_attrs::GraphicalEventAttributesSetter for Text {
	fn set_attr<V>(&mut self, attr: common_attrs::GraphicalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(TextAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Text {}

impl common_attrs::PresentationAttributesSetter for Text {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(TextAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Text {}

impl Tag for Text {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("text");
		w.set_preserve_whitespaces(true);
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
		w.set_preserve_whitespaces(false);
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum TextPathAttrs {
	Class,
	ExternalResourcesRequired,
	Method,
	Spacing,
	StartOffset,
	Style,
	XlinkHref,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	GlobalEventAttributes(common_attrs::GlobalEventAttributes),
	GraphicalEventAttributes(common_attrs::GraphicalEventAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
	XLinkAttributes(common_attrs::XLinkAttributes),
}

impl From<common_attrs::ConditionalProcessingAttributes> for TextPathAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for TextPathAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::GlobalEventAttributes> for TextPathAttrs {
	fn from(attr: common_attrs::GlobalEventAttributes) -> Self {
		Self::GlobalEventAttributes(attr)
	}
}

impl From<common_attrs::GraphicalEventAttributes> for TextPathAttrs {
	fn from(attr: common_attrs::GraphicalEventAttributes) -> Self {
		Self::GraphicalEventAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for TextPathAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl From<common_attrs::XLinkAttributes> for TextPathAttrs {
	fn from(attr: common_attrs::XLinkAttributes) -> Self {
		Self::XLinkAttributes(attr)
	}
}

impl TextPathAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Method => "method",
			Self::Spacing => "spacing",
			Self::StartOffset => "startOffset",
			Self::Style => "style",
			Self::XlinkHref => "xlink:href",
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::GlobalEventAttributes(attr) => attr.as_str(),
			Self::GraphicalEventAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
			Self::XLinkAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for TextPathAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for TextPathAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod text_path_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::A {}
	impl Content for super::AltGlyph {}
	impl Content for super::Animate {}
	impl Content for super::AnimateColor {}
	impl Content for super::Desc {}
	impl Content for super::Metadata {}
	impl Content for super::Set {}
	impl Content for super::Title {}
	impl Content for super::Tref {}
	impl Content for super::TSpan {}
}

#[doc = "The [`<textPath>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("textPath.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<a>`](A)\n",
	"- [`<altGlyph>`](AltGlyph)\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateColor>`](AnimateColor)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<set>`](Set)\n",
	"- [`<title>`](Title)\n",
	"- [`<tref>`](Tref)\n",
	"- [`<tspan>`](TSpan)\n"
)]
#[doc = "\n\n [`<textPath>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/textPath"]
#[derive(Debug)]
pub struct TextPath {
	attrs: IndexMap<TextPathAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn text_path_private::Content>>,
}

impl Default for TextPath {
	fn default() -> Self {
		Self::new()
	}
}

impl TextPath {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: text_path_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: text_path_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: TextPathAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: TextPathAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TextPathAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextPathAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(TextPathAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TextPathAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextPathAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(TextPathAttrs::ExternalResourcesRequired)
	}

	/// Set the `method` attribute.
	pub fn with_method<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TextPathAttrs::Method, value);
		self
	}

	/// Set the `method` attribute.
	pub fn set_method<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextPathAttrs::Method, value);
	}

	/// Get the `method` attribute.
	pub fn method(&self) -> Option<&dyn Value> {
		self.get_attr(TextPathAttrs::Method)
	}

	/// Set the `spacing` attribute.
	pub fn with_spacing<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TextPathAttrs::Spacing, value);
		self
	}

	/// Set the `spacing` attribute.
	pub fn set_spacing<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextPathAttrs::Spacing, value);
	}

	/// Get the `spacing` attribute.
	pub fn spacing(&self) -> Option<&dyn Value> {
		self.get_attr(TextPathAttrs::Spacing)
	}

	/// Set the `startOffset` attribute.
	pub fn with_start_offset<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TextPathAttrs::StartOffset, value);
		self
	}

	/// Set the `startOffset` attribute.
	pub fn set_start_offset<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextPathAttrs::StartOffset, value);
	}

	/// Get the `startOffset` attribute.
	pub fn start_offset(&self) -> Option<&dyn Value> {
		self.get_attr(TextPathAttrs::StartOffset)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TextPathAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextPathAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(TextPathAttrs::Style)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TextPathAttrs::XlinkHref, value);
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextPathAttrs::XlinkHref, value);
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&dyn Value> {
		self.get_attr(TextPathAttrs::XlinkHref)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for TextPath {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextPathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(TextPathAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for TextPath {}

impl common_attrs::CoreAttributesSetter for TextPath {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextPathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(TextPathAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for TextPath {}

impl common_attrs::GlobalEventAttributesSetter for TextPath {
	fn set_attr<V>(&mut self, attr: common_attrs::GlobalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextPathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GlobalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(TextPathAttrs::from(attr))
	}
}

impl TagWithGlobalEventAttributes for TextPath {}

impl common_attrs::GraphicalEventAttributesSetter for TextPath {
	fn set_attr<V>(&mut self, attr: common_attrs::GraphicalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextPathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(TextPathAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for TextPath {}

impl common_attrs::PresentationAttributesSetter for TextPath {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextPathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(TextPathAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for TextPath {}

impl common_attrs::XLinkAttributesSetter for TextPath {
	fn set_attr<V>(&mut self, attr: common_attrs::XLinkAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TextPathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&dyn Value> {
		self.get_attr(TextPathAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for TextPath {}

impl Tag for TextPath {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("textPath");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum TitleAttrs {
	Class,
	Style,
	CoreAttributes(common_attrs::CoreAttributes),
}

impl From<common_attrs::CoreAttributes> for TitleAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl TitleAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::Style => "style",
			Self::CoreAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for TitleAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for TitleAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[doc = "The [`<title>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("title.md")]
#[doc = "\n\n [`<title>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/title"]
#[derive(Debug)]
pub struct Title {
	attrs: IndexMap<TitleAttrs, Box<dyn Value>>,
	content: String,
}

impl Default for Title {
	fn default() -> Self {
		Self::new()
	}
}

impl Title {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: String::new()
		}
	}

	pub fn push<T: Display>(&mut self, content: T) {
		write!(self.content, "{content}").unwrap();
	}

	pub fn append<T: Display>(mut self, content: T) -> Self {
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: TitleAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: TitleAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TitleAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TitleAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(TitleAttrs::Class)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TitleAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TitleAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(TitleAttrs::Style)
	}
}

impl common_attrs::CoreAttributesSetter for Title {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TitleAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(TitleAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Title {}

impl Tag for Title {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("title");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		w.write_cdata_text(&self.content);
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum TrefAttrs {
	Class,
	ExternalResourcesRequired,
	Style,
	XlinkHref,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	GlobalEventAttributes(common_attrs::GlobalEventAttributes),
	GraphicalEventAttributes(common_attrs::GraphicalEventAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
	XLinkAttributes(common_attrs::XLinkAttributes),
}

impl From<common_attrs::ConditionalProcessingAttributes> for TrefAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for TrefAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::GlobalEventAttributes> for TrefAttrs {
	fn from(attr: common_attrs::GlobalEventAttributes) -> Self {
		Self::GlobalEventAttributes(attr)
	}
}

impl From<common_attrs::GraphicalEventAttributes> for TrefAttrs {
	fn from(attr: common_attrs::GraphicalEventAttributes) -> Self {
		Self::GraphicalEventAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for TrefAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl From<common_attrs::XLinkAttributes> for TrefAttrs {
	fn from(attr: common_attrs::XLinkAttributes) -> Self {
		Self::XLinkAttributes(attr)
	}
}

impl TrefAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Style => "style",
			Self::XlinkHref => "xlink:href",
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::GlobalEventAttributes(attr) => attr.as_str(),
			Self::GraphicalEventAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
			Self::XLinkAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for TrefAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for TrefAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod tref_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::AnimateColor {}
	impl Content for super::Desc {}
	impl Content for super::Metadata {}
	impl Content for super::Set {}
	impl Content for super::Title {}
}

#[doc = "The [`<tref>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("tref.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateColor>`](AnimateColor)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<set>`](Set)\n",
	"- [`<title>`](Title)\n"
)]
#[doc = "\n\n [`<tref>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/tref"]
#[derive(Debug)]
pub struct Tref {
	attrs: IndexMap<TrefAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn tref_private::Content>>,
}

impl Default for Tref {
	fn default() -> Self {
		Self::new()
	}
}

impl Tref {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: tref_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: tref_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: TrefAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: TrefAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TrefAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TrefAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(TrefAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TrefAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TrefAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(TrefAttrs::ExternalResourcesRequired)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TrefAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TrefAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(TrefAttrs::Style)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TrefAttrs::XlinkHref, value);
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TrefAttrs::XlinkHref, value);
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&dyn Value> {
		self.get_attr(TrefAttrs::XlinkHref)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Tref {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TrefAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(TrefAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Tref {}

impl common_attrs::CoreAttributesSetter for Tref {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TrefAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(TrefAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Tref {}

impl common_attrs::GlobalEventAttributesSetter for Tref {
	fn set_attr<V>(&mut self, attr: common_attrs::GlobalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TrefAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GlobalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(TrefAttrs::from(attr))
	}
}

impl TagWithGlobalEventAttributes for Tref {}

impl common_attrs::GraphicalEventAttributesSetter for Tref {
	fn set_attr<V>(&mut self, attr: common_attrs::GraphicalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TrefAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(TrefAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Tref {}

impl common_attrs::PresentationAttributesSetter for Tref {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TrefAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(TrefAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Tref {}

impl common_attrs::XLinkAttributesSetter for Tref {
	fn set_attr<V>(&mut self, attr: common_attrs::XLinkAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TrefAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&dyn Value> {
		self.get_attr(TrefAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for Tref {}

impl Tag for Tref {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("tref");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum TSpanAttrs {
	Class,
	Dx,
	Dy,
	ExternalResourcesRequired,
	LengthAdjust,
	Rotate,
	Style,
	TextLength,
	X,
	Y,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	GlobalEventAttributes(common_attrs::GlobalEventAttributes),
	GraphicalEventAttributes(common_attrs::GraphicalEventAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::ConditionalProcessingAttributes> for TSpanAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for TSpanAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::GlobalEventAttributes> for TSpanAttrs {
	fn from(attr: common_attrs::GlobalEventAttributes) -> Self {
		Self::GlobalEventAttributes(attr)
	}
}

impl From<common_attrs::GraphicalEventAttributes> for TSpanAttrs {
	fn from(attr: common_attrs::GraphicalEventAttributes) -> Self {
		Self::GraphicalEventAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for TSpanAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl TSpanAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::Dx => "dx",
			Self::Dy => "dy",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::LengthAdjust => "lengthAdjust",
			Self::Rotate => "rotate",
			Self::Style => "style",
			Self::TextLength => "textLength",
			Self::X => "x",
			Self::Y => "y",
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::GlobalEventAttributes(attr) => attr.as_str(),
			Self::GraphicalEventAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for TSpanAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for TSpanAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod tspan_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::A {}
	impl Content for super::AltGlyph {}
	impl Content for super::Animate {}
	impl Content for super::AnimateColor {}
	impl Content for super::Desc {}
	impl Content for super::Metadata {}
	impl Content for super::Set {}
	impl Content for super::Title {}
	impl Content for super::Tref {}
	impl Content for super::TSpan {}
	impl Content for String {}
	impl Content for &'static str {}
}

#[doc = "The [`<tspan>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("tspan.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<a>`](A)\n",
	"- [`<altGlyph>`](AltGlyph)\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateColor>`](AnimateColor)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<set>`](Set)\n",
	"- [`<title>`](Title)\n",
	"- [`<tref>`](Tref)\n",
	"- [`<tspan>`](TSpan)\n"
)]
#[doc = "\n\n [`<tspan>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/tspan"]
#[derive(Debug)]
pub struct TSpan {
	attrs: IndexMap<TSpanAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn tspan_private::Content>>,
}

impl Default for TSpan {
	fn default() -> Self {
		Self::new()
	}
}

impl TSpan {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: tspan_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: tspan_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: TSpanAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: TSpanAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TSpanAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TSpanAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(TSpanAttrs::Class)
	}

	/// Set the `dx` attribute.
	pub fn with_dx<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TSpanAttrs::Dx, value);
		self
	}

	/// Set the `dx` attribute.
	pub fn set_dx<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TSpanAttrs::Dx, value);
	}

	/// Get the `dx` attribute.
	pub fn dx(&self) -> Option<&dyn Value> {
		self.get_attr(TSpanAttrs::Dx)
	}

	/// Set the `dy` attribute.
	pub fn with_dy<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TSpanAttrs::Dy, value);
		self
	}

	/// Set the `dy` attribute.
	pub fn set_dy<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TSpanAttrs::Dy, value);
	}

	/// Get the `dy` attribute.
	pub fn dy(&self) -> Option<&dyn Value> {
		self.get_attr(TSpanAttrs::Dy)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TSpanAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TSpanAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(TSpanAttrs::ExternalResourcesRequired)
	}

	/// Set the `lengthAdjust` attribute.
	pub fn with_length_adjust<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TSpanAttrs::LengthAdjust, value);
		self
	}

	/// Set the `lengthAdjust` attribute.
	pub fn set_length_adjust<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TSpanAttrs::LengthAdjust, value);
	}

	/// Get the `lengthAdjust` attribute.
	pub fn length_adjust(&self) -> Option<&dyn Value> {
		self.get_attr(TSpanAttrs::LengthAdjust)
	}

	/// Set the `rotate` attribute.
	pub fn with_rotate<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TSpanAttrs::Rotate, value);
		self
	}

	/// Set the `rotate` attribute.
	pub fn set_rotate<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TSpanAttrs::Rotate, value);
	}

	/// Get the `rotate` attribute.
	pub fn rotate(&self) -> Option<&dyn Value> {
		self.get_attr(TSpanAttrs::Rotate)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TSpanAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TSpanAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(TSpanAttrs::Style)
	}

	/// Set the `textLength` attribute.
	pub fn with_text_length<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TSpanAttrs::TextLength, value);
		self
	}

	/// Set the `textLength` attribute.
	pub fn set_text_length<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TSpanAttrs::TextLength, value);
	}

	/// Get the `textLength` attribute.
	pub fn text_length(&self) -> Option<&dyn Value> {
		self.get_attr(TSpanAttrs::TextLength)
	}

	/// Set the `x` attribute.
	pub fn with_x<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TSpanAttrs::X, value);
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TSpanAttrs::X, value);
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&dyn Value> {
		self.get_attr(TSpanAttrs::X)
	}

	/// Set the `y` attribute.
	pub fn with_y<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TSpanAttrs::Y, value);
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TSpanAttrs::Y, value);
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&dyn Value> {
		self.get_attr(TSpanAttrs::Y)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for TSpan {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TSpanAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(TSpanAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for TSpan {}

impl common_attrs::CoreAttributesSetter for TSpan {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TSpanAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(TSpanAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for TSpan {}

impl common_attrs::GlobalEventAttributesSetter for TSpan {
	fn set_attr<V>(&mut self, attr: common_attrs::GlobalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TSpanAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GlobalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(TSpanAttrs::from(attr))
	}
}

impl TagWithGlobalEventAttributes for TSpan {}

impl common_attrs::GraphicalEventAttributesSetter for TSpan {
	fn set_attr<V>(&mut self, attr: common_attrs::GraphicalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TSpanAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(TSpanAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for TSpan {}

impl common_attrs::PresentationAttributesSetter for TSpan {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TSpanAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(TSpanAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for TSpan {}

impl Tag for TSpan {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("tspan");
		w.set_preserve_whitespaces(true);
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum UseAttrs {
	Class,
	ExternalResourcesRequired,
	Height,
	Style,
	Transform,
	Width,
	X,
	XlinkHref,
	Y,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	GlobalEventAttributes(common_attrs::GlobalEventAttributes),
	GraphicalEventAttributes(common_attrs::GraphicalEventAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
	XLinkAttributes(common_attrs::XLinkAttributes),
}

impl From<common_attrs::ConditionalProcessingAttributes> for UseAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for UseAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::GlobalEventAttributes> for UseAttrs {
	fn from(attr: common_attrs::GlobalEventAttributes) -> Self {
		Self::GlobalEventAttributes(attr)
	}
}

impl From<common_attrs::GraphicalEventAttributes> for UseAttrs {
	fn from(attr: common_attrs::GraphicalEventAttributes) -> Self {
		Self::GraphicalEventAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for UseAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl From<common_attrs::XLinkAttributes> for UseAttrs {
	fn from(attr: common_attrs::XLinkAttributes) -> Self {
		Self::XLinkAttributes(attr)
	}
}

impl UseAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Height => "height",
			Self::Style => "style",
			Self::Transform => "transform",
			Self::Width => "width",
			Self::X => "x",
			Self::XlinkHref => "xlink:href",
			Self::Y => "y",
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::GlobalEventAttributes(attr) => attr.as_str(),
			Self::GraphicalEventAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
			Self::XLinkAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for UseAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for UseAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod use_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::AnimateMotion {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Desc {}
	impl Content for super::Discard {}
	impl Content for super::Metadata {}
	impl Content for super::Mpath {}
	impl Content for super::Set {}
	impl Content for super::Title {}
}

#[doc = "The [`<use>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("use.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<animate>`](Animate)\n",
	"- [`<animateMotion>`](AnimateMotion)\n",
	"- [`<animateTransform>`](AnimateTransform)\n",
	"- [`<desc>`](Desc)\n",
	"- [`<discard>`](Discard)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<mpath>`](Mpath)\n",
	"- [`<set>`](Set)\n",
	"- [`<title>`](Title)\n"
)]
#[doc = "\n\n [`<use>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/use"]
#[derive(Debug)]
pub struct Use {
	attrs: IndexMap<UseAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn use_private::Content>>,
}

impl Default for Use {
	fn default() -> Self {
		Self::new()
	}
}

impl Use {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: use_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: use_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: UseAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: UseAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `class` attribute.
	pub fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(UseAttrs::Class, value);
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(UseAttrs::Class, value);
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&dyn Value> {
		self.get_attr(UseAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(UseAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(UseAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(UseAttrs::ExternalResourcesRequired)
	}

	/// Set the `height` attribute.
	pub fn with_height<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(UseAttrs::Height, value);
		self
	}

	/// Set the `height` attribute.
	pub fn set_height<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(UseAttrs::Height, value);
	}

	/// Get the `height` attribute.
	pub fn height(&self) -> Option<&dyn Value> {
		self.get_attr(UseAttrs::Height)
	}

	/// Set the `style` attribute.
	pub fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(UseAttrs::Style, value);
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(UseAttrs::Style, value);
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&dyn Value> {
		self.get_attr(UseAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(UseAttrs::Transform, value);
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(UseAttrs::Transform, value);
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&dyn Value> {
		self.get_attr(UseAttrs::Transform)
	}

	/// Set the `width` attribute.
	pub fn with_width<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(UseAttrs::Width, value);
		self
	}

	/// Set the `width` attribute.
	pub fn set_width<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(UseAttrs::Width, value);
	}

	/// Get the `width` attribute.
	pub fn width(&self) -> Option<&dyn Value> {
		self.get_attr(UseAttrs::Width)
	}

	/// Set the `x` attribute.
	pub fn with_x<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(UseAttrs::X, value);
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(UseAttrs::X, value);
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&dyn Value> {
		self.get_attr(UseAttrs::X)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(UseAttrs::XlinkHref, value);
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(UseAttrs::XlinkHref, value);
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&dyn Value> {
		self.get_attr(UseAttrs::XlinkHref)
	}

	/// Set the `y` attribute.
	pub fn with_y<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(UseAttrs::Y, value);
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(UseAttrs::Y, value);
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&dyn Value> {
		self.get_attr(UseAttrs::Y)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Use {
	fn set_attr<V>(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(UseAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&dyn Value> {
		self.get_attr(UseAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Use {}

impl common_attrs::CoreAttributesSetter for Use {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(UseAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(UseAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Use {}

impl common_attrs::GlobalEventAttributesSetter for Use {
	fn set_attr<V>(&mut self, attr: common_attrs::GlobalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(UseAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GlobalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(UseAttrs::from(attr))
	}
}

impl TagWithGlobalEventAttributes for Use {}

impl common_attrs::GraphicalEventAttributesSetter for Use {
	fn set_attr<V>(&mut self, attr: common_attrs::GraphicalEventAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(UseAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&dyn Value> {
		self.get_attr(UseAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Use {}

impl common_attrs::PresentationAttributesSetter for Use {
	fn set_attr<V>(&mut self, attr: common_attrs::PresentationAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(UseAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&dyn Value> {
		self.get_attr(UseAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Use {}

impl common_attrs::XLinkAttributesSetter for Use {
	fn set_attr<V>(&mut self, attr: common_attrs::XLinkAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(UseAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&dyn Value> {
		self.get_attr(UseAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for Use {}

impl Tag for Use {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("use");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum ViewAttrs {
	ExternalResourcesRequired,
	PreserveAspectRatio,
	ViewBox,
	ViewTarget,
	ZoomAndPan,
	CoreAttributes(common_attrs::CoreAttributes),
}

impl From<common_attrs::CoreAttributes> for ViewAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl ViewAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::PreserveAspectRatio => "preserveAspectRatio",
			Self::ViewBox => "viewBox",
			Self::ViewTarget => "viewTarget",
			Self::ZoomAndPan => "zoomAndPan",
			Self::CoreAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for ViewAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for ViewAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod view_private {
	use crate::tag::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Desc {}
	impl Content for super::Metadata {}
	impl Content for super::Title {}
}

#[doc = "The [`<view>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("view.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n",
	"- [`<desc>`](Desc)\n",
	"- [`<metadata>`](Metadata)\n",
	"- [`<title>`](Title)\n"
)]
#[doc = "\n\n [`<view>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/view"]
#[derive(Debug)]
pub struct View {
	attrs: IndexMap<ViewAttrs, Box<dyn Value>>,
	content: Vec<Box<dyn view_private::Content>>,
}

impl Default for View {
	fn default() -> Self {
		Self::new()
	}
}

impl View {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
			content: Vec::new(),
		}
	}

	pub fn push<T>(&mut self, content: T)
	where
		T: view_private::Content + 'static
	{
		self.content.push(Box::new(content));
	}

	pub fn append<T>(mut self, content: T) -> Self
	where
		T: view_private::Content + 'static
	{
		self.push(content);
		self
	}

	fn set_attr<V>(&mut self, attr: ViewAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: ViewAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ViewAttrs::ExternalResourcesRequired, value);
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ViewAttrs::ExternalResourcesRequired, value);
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&dyn Value> {
		self.get_attr(ViewAttrs::ExternalResourcesRequired)
	}

	/// Set the `preserveAspectRatio` attribute.
	pub fn with_preserve_aspect_ratio<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ViewAttrs::PreserveAspectRatio, value);
		self
	}

	/// Set the `preserveAspectRatio` attribute.
	pub fn set_preserve_aspect_ratio<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ViewAttrs::PreserveAspectRatio, value);
	}

	/// Get the `preserveAspectRatio` attribute.
	pub fn preserve_aspect_ratio(&self) -> Option<&dyn Value> {
		self.get_attr(ViewAttrs::PreserveAspectRatio)
	}

	/// Set the `viewBox` attribute.
	pub fn with_view_box<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ViewAttrs::ViewBox, value);
		self
	}

	/// Set the `viewBox` attribute.
	pub fn set_view_box<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ViewAttrs::ViewBox, value);
	}

	/// Get the `viewBox` attribute.
	pub fn view_box(&self) -> Option<&dyn Value> {
		self.get_attr(ViewAttrs::ViewBox)
	}

	/// Set the `viewTarget` attribute.
	pub fn with_view_target<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ViewAttrs::ViewTarget, value);
		self
	}

	/// Set the `viewTarget` attribute.
	pub fn set_view_target<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ViewAttrs::ViewTarget, value);
	}

	/// Get the `viewTarget` attribute.
	pub fn view_target(&self) -> Option<&dyn Value> {
		self.get_attr(ViewAttrs::ViewTarget)
	}

	/// Set the `zoomAndPan` attribute.
	pub fn with_zoom_and_pan<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ViewAttrs::ZoomAndPan, value);
		self
	}

	/// Set the `zoomAndPan` attribute.
	pub fn set_zoom_and_pan<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ViewAttrs::ZoomAndPan, value);
	}

	/// Get the `zoomAndPan` attribute.
	pub fn zoom_and_pan(&self) -> Option<&dyn Value> {
		self.get_attr(ViewAttrs::ZoomAndPan)
	}
}

impl common_attrs::CoreAttributesSetter for View {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ViewAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(ViewAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for View {}

impl Tag for View {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("view");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum VkernAttrs {
	G1,
	G2,
	K,
	U1,
	U2,
	CoreAttributes(common_attrs::CoreAttributes),
}

impl From<common_attrs::CoreAttributes> for VkernAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl VkernAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::G1 => "g1",
			Self::G2 => "g2",
			Self::K => "k",
			Self::U1 => "u1",
			Self::U2 => "u2",
			Self::CoreAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for VkernAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for VkernAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[doc = "The [`<vkern>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("vkern.md")]
#[doc = concat!(
	"\n## Elements\n\nThese elements are allowed to appear within this tag:\n"
)]
#[doc = "\n\n [`<vkern>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/vkern"]
#[derive(Debug)]
pub struct Vkern {
	attrs: IndexMap<VkernAttrs, Box<dyn Value>>,
}

impl Default for Vkern {
	fn default() -> Self {
		Self::new()
	}
}

impl Vkern {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
		}
	}

	fn set_attr<V>(&mut self, attr: VkernAttrs, value: V)
	where
		V: Value + 'static
	{
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, Box::new(value));
	}

	fn get_attr(&self, attr: VkernAttrs) -> Option<&dyn Value> {
		self.attrs.get(&attr).map(Box::as_ref)
	}

	/// Set the `g1` attribute.
	pub fn with_g1<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(VkernAttrs::G1, value);
		self
	}

	/// Set the `g1` attribute.
	pub fn set_g1<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(VkernAttrs::G1, value);
	}

	/// Get the `g1` attribute.
	pub fn g1(&self) -> Option<&dyn Value> {
		self.get_attr(VkernAttrs::G1)
	}

	/// Set the `g2` attribute.
	pub fn with_g2<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(VkernAttrs::G2, value);
		self
	}

	/// Set the `g2` attribute.
	pub fn set_g2<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(VkernAttrs::G2, value);
	}

	/// Get the `g2` attribute.
	pub fn g2(&self) -> Option<&dyn Value> {
		self.get_attr(VkernAttrs::G2)
	}

	/// Set the `k` attribute.
	pub fn with_k<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(VkernAttrs::K, value);
		self
	}

	/// Set the `k` attribute.
	pub fn set_k<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(VkernAttrs::K, value);
	}

	/// Get the `k` attribute.
	pub fn k(&self) -> Option<&dyn Value> {
		self.get_attr(VkernAttrs::K)
	}

	/// Set the `u1` attribute.
	pub fn with_u1<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(VkernAttrs::U1, value);
		self
	}

	/// Set the `u1` attribute.
	pub fn set_u1<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(VkernAttrs::U1, value);
	}

	/// Get the `u1` attribute.
	pub fn u1(&self) -> Option<&dyn Value> {
		self.get_attr(VkernAttrs::U1)
	}

	/// Set the `u2` attribute.
	pub fn with_u2<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(VkernAttrs::U2, value);
		self
	}

	/// Set the `u2` attribute.
	pub fn set_u2<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(VkernAttrs::U2, value);
	}

	/// Get the `u2` attribute.
	pub fn u2(&self) -> Option<&dyn Value> {
		self.get_attr(VkernAttrs::U2)
	}
}

impl common_attrs::CoreAttributesSetter for Vkern {
	fn set_attr<V>(&mut self, attr: common_attrs::CoreAttributes, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(VkernAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&dyn Value> {
		self.get_attr(VkernAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Vkern {}

impl Tag for Vkern {
	fn write_to(&self, w: &mut XmlWriter, #[allow(unused_variables)] pretty: bool) {
		w.start_element("vkern");
		for (attr, value) in &self.attrs {
			let value = value.to_string(pretty);
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}
