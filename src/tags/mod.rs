// @generated

use crate::Tag;
use indexmap::IndexMap;
use std::fmt::{self, Debug, Display, Formatter};
use xmlwriter::XmlWriter;

mod common_attrs;
pub use common_attrs::prelude::*;

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::A {}
	impl Content for super::AltGlyphDef {}
	impl Content for super::ClipPath {}
	impl Content for super::ColorProfile {}
	impl Content for super::Cursor {}
	impl Content for super::Filter {}
	impl Content for super::Font {}
	impl Content for super::FontFace {}
	impl Content for super::ForeignObject {}
	impl Content for super::Image {}
	impl Content for super::Marker {}
	impl Content for super::Mask {}
	impl Content for super::Pattern {}
	impl Content for super::Script {}
	impl Content for super::Style {}
	impl Content for super::Switch {}
	impl Content for super::Text {}
	impl Content for super::View {}
}

#[doc = "The [`<a>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("a.md")]
#[doc = "\n\n [`<a>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/a"]
#[derive(Debug)]
pub struct A {
	attrs: IndexMap<AAttrs, String>,
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

	fn set_attr(&mut self, attr: AAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: AAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(AAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(AAttrs::ExternalResourcesRequired)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(AAttrs::Style)
	}

	/// Set the `target` attribute.
	pub fn with_target<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AAttrs::Target, value.into());
		self
	}

	/// Set the `target` attribute.
	pub fn set_target<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AAttrs::Target, value.into());
	}

	/// Get the `target` attribute.
	pub fn target(&self) -> Option<&str> {
		self.get_attr(AAttrs::Target)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AAttrs::Transform, value.into());
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AAttrs::Transform, value.into());
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&str> {
		self.get_attr(AAttrs::Transform)
	}

	/// Set the `xlink:actuate` attribute.
	pub fn with_xlink_actuate<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AAttrs::XlinkActuate, value.into());
		self
	}

	/// Set the `xlink:actuate` attribute.
	pub fn set_xlink_actuate<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AAttrs::XlinkActuate, value.into());
	}

	/// Get the `xlink:actuate` attribute.
	pub fn xlink_actuate(&self) -> Option<&str> {
		self.get_attr(AAttrs::XlinkActuate)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AAttrs::XlinkHref, value.into());
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AAttrs::XlinkHref, value.into());
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&str> {
		self.get_attr(AAttrs::XlinkHref)
	}

	/// Set the `xlink:show` attribute.
	pub fn with_xlink_show<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AAttrs::XlinkShow, value.into());
		self
	}

	/// Set the `xlink:show` attribute.
	pub fn set_xlink_show<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AAttrs::XlinkShow, value.into());
	}

	/// Get the `xlink:show` attribute.
	pub fn xlink_show(&self) -> Option<&str> {
		self.get_attr(AAttrs::XlinkShow)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for A {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(AAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(AAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for A {}

impl common_attrs::CoreAttributesSetter for A {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(AAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(AAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for A {}

impl common_attrs::GraphicalEventAttributesSetter for A {
	fn set_attr(&mut self, attr: common_attrs::GraphicalEventAttributes, value: String) {
		self.set_attr(AAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&str> {
		self.get_attr(AAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for A {}

impl common_attrs::PresentationAttributesSetter for A {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(AAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(AAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for A {}

impl common_attrs::XLinkAttributesSetter for A {
	fn set_attr(&mut self, attr: common_attrs::XLinkAttributes, value: String) {
		self.set_attr(AAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&str> {
		self.get_attr(AAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for A {}

impl Tag for A {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("a");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	attrs: IndexMap<AltGlyphAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: AltGlyphAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: AltGlyphAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AltGlyphAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AltGlyphAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(AltGlyphAttrs::Class)
	}

	/// Set the `dx` attribute.
	pub fn with_dx<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AltGlyphAttrs::Dx, value.into());
		self
	}

	/// Set the `dx` attribute.
	pub fn set_dx<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AltGlyphAttrs::Dx, value.into());
	}

	/// Get the `dx` attribute.
	pub fn dx(&self) -> Option<&str> {
		self.get_attr(AltGlyphAttrs::Dx)
	}

	/// Set the `dy` attribute.
	pub fn with_dy<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AltGlyphAttrs::Dy, value.into());
		self
	}

	/// Set the `dy` attribute.
	pub fn set_dy<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AltGlyphAttrs::Dy, value.into());
	}

	/// Get the `dy` attribute.
	pub fn dy(&self) -> Option<&str> {
		self.get_attr(AltGlyphAttrs::Dy)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AltGlyphAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AltGlyphAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(AltGlyphAttrs::ExternalResourcesRequired)
	}

	/// Set the `format` attribute.
	pub fn with_format<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AltGlyphAttrs::Format, value.into());
		self
	}

	/// Set the `format` attribute.
	pub fn set_format<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AltGlyphAttrs::Format, value.into());
	}

	/// Get the `format` attribute.
	pub fn format(&self) -> Option<&str> {
		self.get_attr(AltGlyphAttrs::Format)
	}

	/// Set the `glyphRef` attribute.
	pub fn with_glyph_ref<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AltGlyphAttrs::GlyphRef, value.into());
		self
	}

	/// Set the `glyphRef` attribute.
	pub fn set_glyph_ref<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AltGlyphAttrs::GlyphRef, value.into());
	}

	/// Get the `glyphRef` attribute.
	pub fn glyph_ref(&self) -> Option<&str> {
		self.get_attr(AltGlyphAttrs::GlyphRef)
	}

	/// Set the `rotate` attribute.
	pub fn with_rotate<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AltGlyphAttrs::Rotate, value.into());
		self
	}

	/// Set the `rotate` attribute.
	pub fn set_rotate<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AltGlyphAttrs::Rotate, value.into());
	}

	/// Get the `rotate` attribute.
	pub fn rotate(&self) -> Option<&str> {
		self.get_attr(AltGlyphAttrs::Rotate)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AltGlyphAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AltGlyphAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(AltGlyphAttrs::Style)
	}

	/// Set the `x` attribute.
	pub fn with_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AltGlyphAttrs::X, value.into());
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AltGlyphAttrs::X, value.into());
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&str> {
		self.get_attr(AltGlyphAttrs::X)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AltGlyphAttrs::XlinkHref, value.into());
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AltGlyphAttrs::XlinkHref, value.into());
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&str> {
		self.get_attr(AltGlyphAttrs::XlinkHref)
	}

	/// Set the `y` attribute.
	pub fn with_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AltGlyphAttrs::Y, value.into());
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AltGlyphAttrs::Y, value.into());
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&str> {
		self.get_attr(AltGlyphAttrs::Y)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for AltGlyph {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(AltGlyphAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(AltGlyphAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for AltGlyph {}

impl common_attrs::CoreAttributesSetter for AltGlyph {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(AltGlyphAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(AltGlyphAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for AltGlyph {}

impl common_attrs::GraphicalEventAttributesSetter for AltGlyph {
	fn set_attr(&mut self, attr: common_attrs::GraphicalEventAttributes, value: String) {
		self.set_attr(AltGlyphAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&str> {
		self.get_attr(AltGlyphAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for AltGlyph {}

impl common_attrs::PresentationAttributesSetter for AltGlyph {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(AltGlyphAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(AltGlyphAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for AltGlyph {}

impl common_attrs::XLinkAttributesSetter for AltGlyph {
	fn set_attr(&mut self, attr: common_attrs::XLinkAttributes, value: String) {
		self.set_attr(AltGlyphAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&str> {
		self.get_attr(AltGlyphAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for AltGlyph {}

impl Tag for AltGlyph {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("altGlyph");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

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
#[doc = "\n\n [`<altGlyphDef>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/altGlyphDef"]
#[derive(Debug)]
pub struct AltGlyphDef {
	attrs: IndexMap<AltGlyphDefAttrs, String>,
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

	fn set_attr(&mut self, attr: AltGlyphDefAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: AltGlyphDefAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}
}

impl common_attrs::CoreAttributesSetter for AltGlyphDef {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(AltGlyphDefAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(AltGlyphDefAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for AltGlyphDef {}

impl Tag for AltGlyphDef {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("altGlyphDef");
		w.end_element();
	}
}

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
#[doc = "\n\n [`<altGlyphItem>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/altGlyphItem"]
#[derive(Debug)]
pub struct AltGlyphItem {
	attrs: IndexMap<AltGlyphItemAttrs, String>,
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

	fn set_attr(&mut self, attr: AltGlyphItemAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: AltGlyphItemAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}
}

impl common_attrs::CoreAttributesSetter for AltGlyphItem {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(AltGlyphItemAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(AltGlyphItemAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for AltGlyphItem {}

impl Tag for AltGlyphItem {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("altGlyphItem");
		w.end_element();
	}
}

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

#[doc = "The [`<animate>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("animate.md")]
#[doc = "\n\n [`<animate>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/animate"]
#[derive(Debug)]
pub struct Animate {
	attrs: IndexMap<AnimateAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: AnimateAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: AnimateAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `attributeName` attribute.
	pub fn with_attribute_name<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimateAttrs::AttributeName, value.into());
		self
	}

	/// Set the `attributeName` attribute.
	pub fn set_attribute_name<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimateAttrs::AttributeName, value.into());
	}

	/// Get the `attributeName` attribute.
	pub fn attribute_name(&self) -> Option<&str> {
		self.get_attr(AnimateAttrs::AttributeName)
	}

	/// Set the `attributeType` attribute.
	pub fn with_attribute_type<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimateAttrs::AttributeType, value.into());
		self
	}

	/// Set the `attributeType` attribute.
	pub fn set_attribute_type<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimateAttrs::AttributeType, value.into());
	}

	/// Get the `attributeType` attribute.
	pub fn attribute_type(&self) -> Option<&str> {
		self.get_attr(AnimateAttrs::AttributeType)
	}

	/// Set the `dur` attribute.
	pub fn with_dur<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimateAttrs::Dur, value.into());
		self
	}

	/// Set the `dur` attribute.
	pub fn set_dur<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimateAttrs::Dur, value.into());
	}

	/// Get the `dur` attribute.
	pub fn dur(&self) -> Option<&str> {
		self.get_attr(AnimateAttrs::Dur)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimateAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimateAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(AnimateAttrs::ExternalResourcesRequired)
	}

	/// Set the `from` attribute.
	pub fn with_from<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimateAttrs::From, value.into());
		self
	}

	/// Set the `from` attribute.
	pub fn set_from<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimateAttrs::From, value.into());
	}

	/// Get the `from` attribute.
	pub fn from(&self) -> Option<&str> {
		self.get_attr(AnimateAttrs::From)
	}

	/// Set the `repeatCount` attribute.
	pub fn with_repeat_count<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimateAttrs::RepeatCount, value.into());
		self
	}

	/// Set the `repeatCount` attribute.
	pub fn set_repeat_count<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimateAttrs::RepeatCount, value.into());
	}

	/// Get the `repeatCount` attribute.
	pub fn repeat_count(&self) -> Option<&str> {
		self.get_attr(AnimateAttrs::RepeatCount)
	}

	/// Set the `to` attribute.
	pub fn with_to<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimateAttrs::To, value.into());
		self
	}

	/// Set the `to` attribute.
	pub fn set_to<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimateAttrs::To, value.into());
	}

	/// Get the `to` attribute.
	pub fn to(&self) -> Option<&str> {
		self.get_attr(AnimateAttrs::To)
	}
}

impl common_attrs::AnimationAdditionAttributesSetter for Animate {
	fn set_attr(&mut self, attr: common_attrs::AnimationAdditionAttributes, value: String) {
		self.set_attr(AnimateAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationAdditionAttributes) -> Option<&str> {
		self.get_attr(AnimateAttrs::from(attr))
	}
}

impl TagWithAnimationAdditionAttributes for Animate {}

impl common_attrs::AnimationAttributeTargetAttributesSetter for Animate {
	fn set_attr(&mut self, attr: common_attrs::AnimationAttributeTargetAttributes, value: String) {
		self.set_attr(AnimateAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationAttributeTargetAttributes) -> Option<&str> {
		self.get_attr(AnimateAttrs::from(attr))
	}
}

impl TagWithAnimationAttributeTargetAttributes for Animate {}

impl common_attrs::AnimationEventAttributesSetter for Animate {
	fn set_attr(&mut self, attr: common_attrs::AnimationEventAttributes, value: String) {
		self.set_attr(AnimateAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationEventAttributes) -> Option<&str> {
		self.get_attr(AnimateAttrs::from(attr))
	}
}

impl TagWithAnimationEventAttributes for Animate {}

impl common_attrs::AnimationTimingAttributesSetter for Animate {
	fn set_attr(&mut self, attr: common_attrs::AnimationTimingAttributes, value: String) {
		self.set_attr(AnimateAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationTimingAttributes) -> Option<&str> {
		self.get_attr(AnimateAttrs::from(attr))
	}
}

impl TagWithAnimationTimingAttributes for Animate {}

impl common_attrs::AnimationValueAttributesSetter for Animate {
	fn set_attr(&mut self, attr: common_attrs::AnimationValueAttributes, value: String) {
		self.set_attr(AnimateAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationValueAttributes) -> Option<&str> {
		self.get_attr(AnimateAttrs::from(attr))
	}
}

impl TagWithAnimationValueAttributes for Animate {}

impl common_attrs::ConditionalProcessingAttributesSetter for Animate {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(AnimateAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(AnimateAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Animate {}

impl common_attrs::CoreAttributesSetter for Animate {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(AnimateAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(AnimateAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Animate {}

impl common_attrs::XLinkAttributesSetter for Animate {
	fn set_attr(&mut self, attr: common_attrs::XLinkAttributes, value: String) {
		self.set_attr(AnimateAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&str> {
		self.get_attr(AnimateAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for Animate {}

impl Tag for Animate {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("animate");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

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

#[doc = "The [`<animateColor>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("animateColor.md")]
#[doc = "\n\n [`<animateColor>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/animateColor"]
#[derive(Debug)]
pub struct AnimateColor {
	attrs: IndexMap<AnimateColorAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: AnimateColorAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: AnimateColorAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `by` attribute.
	pub fn with_by<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimateColorAttrs::By, value.into());
		self
	}

	/// Set the `by` attribute.
	pub fn set_by<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimateColorAttrs::By, value.into());
	}

	/// Get the `by` attribute.
	pub fn by(&self) -> Option<&str> {
		self.get_attr(AnimateColorAttrs::By)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimateColorAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimateColorAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(AnimateColorAttrs::ExternalResourcesRequired)
	}

	/// Set the `from` attribute.
	pub fn with_from<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimateColorAttrs::From, value.into());
		self
	}

	/// Set the `from` attribute.
	pub fn set_from<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimateColorAttrs::From, value.into());
	}

	/// Get the `from` attribute.
	pub fn from(&self) -> Option<&str> {
		self.get_attr(AnimateColorAttrs::From)
	}

	/// Set the `to` attribute.
	pub fn with_to<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimateColorAttrs::To, value.into());
		self
	}

	/// Set the `to` attribute.
	pub fn set_to<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimateColorAttrs::To, value.into());
	}

	/// Get the `to` attribute.
	pub fn to(&self) -> Option<&str> {
		self.get_attr(AnimateColorAttrs::To)
	}
}

impl common_attrs::AnimationAdditionAttributesSetter for AnimateColor {
	fn set_attr(&mut self, attr: common_attrs::AnimationAdditionAttributes, value: String) {
		self.set_attr(AnimateColorAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationAdditionAttributes) -> Option<&str> {
		self.get_attr(AnimateColorAttrs::from(attr))
	}
}

impl TagWithAnimationAdditionAttributes for AnimateColor {}

impl common_attrs::AnimationAttributeTargetAttributesSetter for AnimateColor {
	fn set_attr(&mut self, attr: common_attrs::AnimationAttributeTargetAttributes, value: String) {
		self.set_attr(AnimateColorAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationAttributeTargetAttributes) -> Option<&str> {
		self.get_attr(AnimateColorAttrs::from(attr))
	}
}

impl TagWithAnimationAttributeTargetAttributes for AnimateColor {}

impl common_attrs::AnimationEventAttributesSetter for AnimateColor {
	fn set_attr(&mut self, attr: common_attrs::AnimationEventAttributes, value: String) {
		self.set_attr(AnimateColorAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationEventAttributes) -> Option<&str> {
		self.get_attr(AnimateColorAttrs::from(attr))
	}
}

impl TagWithAnimationEventAttributes for AnimateColor {}

impl common_attrs::AnimationTimingAttributesSetter for AnimateColor {
	fn set_attr(&mut self, attr: common_attrs::AnimationTimingAttributes, value: String) {
		self.set_attr(AnimateColorAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationTimingAttributes) -> Option<&str> {
		self.get_attr(AnimateColorAttrs::from(attr))
	}
}

impl TagWithAnimationTimingAttributes for AnimateColor {}

impl common_attrs::AnimationValueAttributesSetter for AnimateColor {
	fn set_attr(&mut self, attr: common_attrs::AnimationValueAttributes, value: String) {
		self.set_attr(AnimateColorAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationValueAttributes) -> Option<&str> {
		self.get_attr(AnimateColorAttrs::from(attr))
	}
}

impl TagWithAnimationValueAttributes for AnimateColor {}

impl common_attrs::ConditionalProcessingAttributesSetter for AnimateColor {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(AnimateColorAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(AnimateColorAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for AnimateColor {}

impl common_attrs::CoreAttributesSetter for AnimateColor {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(AnimateColorAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(AnimateColorAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for AnimateColor {}

impl common_attrs::XLinkAttributesSetter for AnimateColor {
	fn set_attr(&mut self, attr: common_attrs::XLinkAttributes, value: String) {
		self.set_attr(AnimateColorAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&str> {
		self.get_attr(AnimateColorAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for AnimateColor {}

impl Tag for AnimateColor {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("animateColor");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Mpath {}
}

#[doc = "The [`<animateMotion>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("animateMotion.md")]
#[doc = "\n\n [`<animateMotion>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/animateMotion"]
#[derive(Debug)]
pub struct AnimateMotion {
	attrs: IndexMap<AnimateMotionAttrs, String>,
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

	fn set_attr(&mut self, attr: AnimateMotionAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: AnimateMotionAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `calcMode` attribute.
	pub fn with_calc_mode<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimateMotionAttrs::CalcMode, value.into());
		self
	}

	/// Set the `calcMode` attribute.
	pub fn set_calc_mode<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimateMotionAttrs::CalcMode, value.into());
	}

	/// Get the `calcMode` attribute.
	pub fn calc_mode(&self) -> Option<&str> {
		self.get_attr(AnimateMotionAttrs::CalcMode)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimateMotionAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimateMotionAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(AnimateMotionAttrs::ExternalResourcesRequired)
	}

	/// Set the `keyPoints` attribute.
	pub fn with_key_points<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimateMotionAttrs::KeyPoints, value.into());
		self
	}

	/// Set the `keyPoints` attribute.
	pub fn set_key_points<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimateMotionAttrs::KeyPoints, value.into());
	}

	/// Get the `keyPoints` attribute.
	pub fn key_points(&self) -> Option<&str> {
		self.get_attr(AnimateMotionAttrs::KeyPoints)
	}

	/// Set the `origin` attribute.
	pub fn with_origin<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimateMotionAttrs::Origin, value.into());
		self
	}

	/// Set the `origin` attribute.
	pub fn set_origin<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimateMotionAttrs::Origin, value.into());
	}

	/// Get the `origin` attribute.
	pub fn origin(&self) -> Option<&str> {
		self.get_attr(AnimateMotionAttrs::Origin)
	}

	/// Set the `path` attribute.
	pub fn with_path<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimateMotionAttrs::Path, value.into());
		self
	}

	/// Set the `path` attribute.
	pub fn set_path<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimateMotionAttrs::Path, value.into());
	}

	/// Get the `path` attribute.
	pub fn path(&self) -> Option<&str> {
		self.get_attr(AnimateMotionAttrs::Path)
	}

	/// Set the `rotate` attribute.
	pub fn with_rotate<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimateMotionAttrs::Rotate, value.into());
		self
	}

	/// Set the `rotate` attribute.
	pub fn set_rotate<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimateMotionAttrs::Rotate, value.into());
	}

	/// Get the `rotate` attribute.
	pub fn rotate(&self) -> Option<&str> {
		self.get_attr(AnimateMotionAttrs::Rotate)
	}
}

impl common_attrs::AnimationAdditionAttributesSetter for AnimateMotion {
	fn set_attr(&mut self, attr: common_attrs::AnimationAdditionAttributes, value: String) {
		self.set_attr(AnimateMotionAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationAdditionAttributes) -> Option<&str> {
		self.get_attr(AnimateMotionAttrs::from(attr))
	}
}

impl TagWithAnimationAdditionAttributes for AnimateMotion {}

impl common_attrs::AnimationEventAttributesSetter for AnimateMotion {
	fn set_attr(&mut self, attr: common_attrs::AnimationEventAttributes, value: String) {
		self.set_attr(AnimateMotionAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationEventAttributes) -> Option<&str> {
		self.get_attr(AnimateMotionAttrs::from(attr))
	}
}

impl TagWithAnimationEventAttributes for AnimateMotion {}

impl common_attrs::AnimationTimingAttributesSetter for AnimateMotion {
	fn set_attr(&mut self, attr: common_attrs::AnimationTimingAttributes, value: String) {
		self.set_attr(AnimateMotionAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationTimingAttributes) -> Option<&str> {
		self.get_attr(AnimateMotionAttrs::from(attr))
	}
}

impl TagWithAnimationTimingAttributes for AnimateMotion {}

impl common_attrs::AnimationValueAttributesSetter for AnimateMotion {
	fn set_attr(&mut self, attr: common_attrs::AnimationValueAttributes, value: String) {
		self.set_attr(AnimateMotionAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationValueAttributes) -> Option<&str> {
		self.get_attr(AnimateMotionAttrs::from(attr))
	}
}

impl TagWithAnimationValueAttributes for AnimateMotion {}

impl common_attrs::ConditionalProcessingAttributesSetter for AnimateMotion {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(AnimateMotionAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(AnimateMotionAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for AnimateMotion {}

impl common_attrs::CoreAttributesSetter for AnimateMotion {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(AnimateMotionAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(AnimateMotionAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for AnimateMotion {}

impl common_attrs::XLinkAttributesSetter for AnimateMotion {
	fn set_attr(&mut self, attr: common_attrs::XLinkAttributes, value: String) {
		self.set_attr(AnimateMotionAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&str> {
		self.get_attr(AnimateMotionAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for AnimateMotion {}

impl Tag for AnimateMotion {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("animateMotion");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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

#[doc = "The [`<animateTransform>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("animateTransform.md")]
#[doc = "\n\n [`<animateTransform>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/animateTransform"]
#[derive(Debug)]
pub struct AnimateTransform {
	attrs: IndexMap<AnimateTransformAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: AnimateTransformAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: AnimateTransformAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `by` attribute.
	pub fn with_by<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimateTransformAttrs::By, value.into());
		self
	}

	/// Set the `by` attribute.
	pub fn set_by<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimateTransformAttrs::By, value.into());
	}

	/// Get the `by` attribute.
	pub fn by(&self) -> Option<&str> {
		self.get_attr(AnimateTransformAttrs::By)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimateTransformAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimateTransformAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(AnimateTransformAttrs::ExternalResourcesRequired)
	}

	/// Set the `from` attribute.
	pub fn with_from<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimateTransformAttrs::From, value.into());
		self
	}

	/// Set the `from` attribute.
	pub fn set_from<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimateTransformAttrs::From, value.into());
	}

	/// Get the `from` attribute.
	pub fn from(&self) -> Option<&str> {
		self.get_attr(AnimateTransformAttrs::From)
	}

	/// Set the `to` attribute.
	pub fn with_to<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimateTransformAttrs::To, value.into());
		self
	}

	/// Set the `to` attribute.
	pub fn set_to<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimateTransformAttrs::To, value.into());
	}

	/// Get the `to` attribute.
	pub fn to(&self) -> Option<&str> {
		self.get_attr(AnimateTransformAttrs::To)
	}

	/// Set the `type` attribute.
	pub fn with_ty<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimateTransformAttrs::Type, value.into());
		self
	}

	/// Set the `type` attribute.
	pub fn set_ty<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimateTransformAttrs::Type, value.into());
	}

	/// Get the `type` attribute.
	pub fn ty(&self) -> Option<&str> {
		self.get_attr(AnimateTransformAttrs::Type)
	}
}

impl common_attrs::AnimationAdditionAttributesSetter for AnimateTransform {
	fn set_attr(&mut self, attr: common_attrs::AnimationAdditionAttributes, value: String) {
		self.set_attr(AnimateTransformAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationAdditionAttributes) -> Option<&str> {
		self.get_attr(AnimateTransformAttrs::from(attr))
	}
}

impl TagWithAnimationAdditionAttributes for AnimateTransform {}

impl common_attrs::AnimationAttributeTargetAttributesSetter for AnimateTransform {
	fn set_attr(&mut self, attr: common_attrs::AnimationAttributeTargetAttributes, value: String) {
		self.set_attr(AnimateTransformAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationAttributeTargetAttributes) -> Option<&str> {
		self.get_attr(AnimateTransformAttrs::from(attr))
	}
}

impl TagWithAnimationAttributeTargetAttributes for AnimateTransform {}

impl common_attrs::AnimationEventAttributesSetter for AnimateTransform {
	fn set_attr(&mut self, attr: common_attrs::AnimationEventAttributes, value: String) {
		self.set_attr(AnimateTransformAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationEventAttributes) -> Option<&str> {
		self.get_attr(AnimateTransformAttrs::from(attr))
	}
}

impl TagWithAnimationEventAttributes for AnimateTransform {}

impl common_attrs::AnimationTimingAttributesSetter for AnimateTransform {
	fn set_attr(&mut self, attr: common_attrs::AnimationTimingAttributes, value: String) {
		self.set_attr(AnimateTransformAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationTimingAttributes) -> Option<&str> {
		self.get_attr(AnimateTransformAttrs::from(attr))
	}
}

impl TagWithAnimationTimingAttributes for AnimateTransform {}

impl common_attrs::AnimationValueAttributesSetter for AnimateTransform {
	fn set_attr(&mut self, attr: common_attrs::AnimationValueAttributes, value: String) {
		self.set_attr(AnimateTransformAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationValueAttributes) -> Option<&str> {
		self.get_attr(AnimateTransformAttrs::from(attr))
	}
}

impl TagWithAnimationValueAttributes for AnimateTransform {}

impl common_attrs::ConditionalProcessingAttributesSetter for AnimateTransform {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(AnimateTransformAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(AnimateTransformAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for AnimateTransform {}

impl common_attrs::CoreAttributesSetter for AnimateTransform {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(AnimateTransformAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(AnimateTransformAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for AnimateTransform {}

impl common_attrs::XLinkAttributesSetter for AnimateTransform {
	fn set_attr(&mut self, attr: common_attrs::XLinkAttributes, value: String) {
		self.set_attr(AnimateTransformAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&str> {
		self.get_attr(AnimateTransformAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for AnimateTransform {}

impl Tag for AnimateTransform {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("animateTransform");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

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

#[doc = "The [`<circle>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("circle.md")]
#[doc = "\n\n [`<circle>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/circle"]
#[derive(Debug)]
pub struct Circle {
	attrs: IndexMap<CircleAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: CircleAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: CircleAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(CircleAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(CircleAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(CircleAttrs::Class)
	}

	/// Set the `cx` attribute.
	pub fn with_cx<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(CircleAttrs::Cx, value.into());
		self
	}

	/// Set the `cx` attribute.
	pub fn set_cx<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(CircleAttrs::Cx, value.into());
	}

	/// Get the `cx` attribute.
	pub fn cx(&self) -> Option<&str> {
		self.get_attr(CircleAttrs::Cx)
	}

	/// Set the `cy` attribute.
	pub fn with_cy<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(CircleAttrs::Cy, value.into());
		self
	}

	/// Set the `cy` attribute.
	pub fn set_cy<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(CircleAttrs::Cy, value.into());
	}

	/// Get the `cy` attribute.
	pub fn cy(&self) -> Option<&str> {
		self.get_attr(CircleAttrs::Cy)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(CircleAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(CircleAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(CircleAttrs::ExternalResourcesRequired)
	}

	/// Set the `r` attribute.
	pub fn with_r<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(CircleAttrs::R, value.into());
		self
	}

	/// Set the `r` attribute.
	pub fn set_r<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(CircleAttrs::R, value.into());
	}

	/// Get the `r` attribute.
	pub fn r(&self) -> Option<&str> {
		self.get_attr(CircleAttrs::R)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(CircleAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(CircleAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(CircleAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(CircleAttrs::Transform, value.into());
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(CircleAttrs::Transform, value.into());
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&str> {
		self.get_attr(CircleAttrs::Transform)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Circle {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(CircleAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(CircleAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Circle {}

impl common_attrs::CoreAttributesSetter for Circle {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(CircleAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(CircleAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Circle {}

impl common_attrs::GraphicalEventAttributesSetter for Circle {
	fn set_attr(&mut self, attr: common_attrs::GraphicalEventAttributes, value: String) {
		self.set_attr(CircleAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&str> {
		self.get_attr(CircleAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Circle {}

impl common_attrs::PresentationAttributesSetter for Circle {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(CircleAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(CircleAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Circle {}

impl Tag for Circle {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("circle");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Text {}
	impl Content for super::Use {}
}

#[doc = "The [`<clipPath>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("clipPath.md")]
#[doc = "\n\n [`<clipPath>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/clipPath"]
#[derive(Debug)]
pub struct ClipPath {
	attrs: IndexMap<ClipPathAttrs, String>,
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

	fn set_attr(&mut self, attr: ClipPathAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: ClipPathAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ClipPathAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ClipPathAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(ClipPathAttrs::Class)
	}

	/// Set the `clipPathUnits` attribute.
	pub fn with_clip_path_units<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ClipPathAttrs::ClipPathUnits, value.into());
		self
	}

	/// Set the `clipPathUnits` attribute.
	pub fn set_clip_path_units<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ClipPathAttrs::ClipPathUnits, value.into());
	}

	/// Get the `clipPathUnits` attribute.
	pub fn clip_path_units(&self) -> Option<&str> {
		self.get_attr(ClipPathAttrs::ClipPathUnits)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ClipPathAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ClipPathAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(ClipPathAttrs::ExternalResourcesRequired)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ClipPathAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ClipPathAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(ClipPathAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ClipPathAttrs::Transform, value.into());
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ClipPathAttrs::Transform, value.into());
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&str> {
		self.get_attr(ClipPathAttrs::Transform)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for ClipPath {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(ClipPathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(ClipPathAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for ClipPath {}

impl common_attrs::CoreAttributesSetter for ClipPath {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(ClipPathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(ClipPathAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for ClipPath {}

impl common_attrs::PresentationAttributesSetter for ClipPath {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(ClipPathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(ClipPathAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for ClipPath {}

impl Tag for ClipPath {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("clipPath");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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

#[doc = "The [`<color-profile>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("color-profile.md")]
#[doc = "\n\n [`<color-profile>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/color-profile"]
#[derive(Debug)]
pub struct ColorProfile {
	attrs: IndexMap<ColorProfileAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: ColorProfileAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: ColorProfileAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `local` attribute.
	pub fn with_local<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ColorProfileAttrs::Local, value.into());
		self
	}

	/// Set the `local` attribute.
	pub fn set_local<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ColorProfileAttrs::Local, value.into());
	}

	/// Get the `local` attribute.
	pub fn local(&self) -> Option<&str> {
		self.get_attr(ColorProfileAttrs::Local)
	}

	/// Set the `name` attribute.
	pub fn with_name<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ColorProfileAttrs::Name, value.into());
		self
	}

	/// Set the `name` attribute.
	pub fn set_name<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ColorProfileAttrs::Name, value.into());
	}

	/// Get the `name` attribute.
	pub fn name(&self) -> Option<&str> {
		self.get_attr(ColorProfileAttrs::Name)
	}

	/// Set the `rendering-intent` attribute.
	pub fn with_rendering_intent<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ColorProfileAttrs::RenderingIntent, value.into());
		self
	}

	/// Set the `rendering-intent` attribute.
	pub fn set_rendering_intent<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ColorProfileAttrs::RenderingIntent, value.into());
	}

	/// Get the `rendering-intent` attribute.
	pub fn rendering_intent(&self) -> Option<&str> {
		self.get_attr(ColorProfileAttrs::RenderingIntent)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ColorProfileAttrs::XlinkHref, value.into());
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ColorProfileAttrs::XlinkHref, value.into());
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&str> {
		self.get_attr(ColorProfileAttrs::XlinkHref)
	}
}

impl common_attrs::CoreAttributesSetter for ColorProfile {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(ColorProfileAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(ColorProfileAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for ColorProfile {}

impl common_attrs::XLinkAttributesSetter for ColorProfile {
	fn set_attr(&mut self, attr: common_attrs::XLinkAttributes, value: String) {
		self.set_attr(ColorProfileAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&str> {
		self.get_attr(ColorProfileAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for ColorProfile {}

impl Tag for ColorProfile {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("color-profile");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

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

#[doc = "The [`<cursor>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("cursor.md")]
#[doc = "\n\n [`<cursor>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/cursor"]
#[derive(Debug)]
pub struct Cursor {
	attrs: IndexMap<CursorAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: CursorAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: CursorAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(CursorAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(CursorAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(CursorAttrs::ExternalResourcesRequired)
	}

	/// Set the `x` attribute.
	pub fn with_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(CursorAttrs::X, value.into());
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(CursorAttrs::X, value.into());
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&str> {
		self.get_attr(CursorAttrs::X)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(CursorAttrs::XlinkHref, value.into());
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(CursorAttrs::XlinkHref, value.into());
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&str> {
		self.get_attr(CursorAttrs::XlinkHref)
	}

	/// Set the `y` attribute.
	pub fn with_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(CursorAttrs::Y, value.into());
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(CursorAttrs::Y, value.into());
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&str> {
		self.get_attr(CursorAttrs::Y)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Cursor {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(CursorAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(CursorAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Cursor {}

impl common_attrs::CoreAttributesSetter for Cursor {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(CursorAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(CursorAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Cursor {}

impl common_attrs::XLinkAttributesSetter for Cursor {
	fn set_attr(&mut self, attr: common_attrs::XLinkAttributes, value: String) {
		self.set_attr(CursorAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&str> {
		self.get_attr(CursorAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for Cursor {}

impl Tag for Cursor {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("cursor");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum DefsAttrs {
	Class,
	ExternalResourcesRequired,
	Style,
	Transform,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::A {}
	impl Content for super::AltGlyphDef {}
	impl Content for super::ClipPath {}
	impl Content for super::ColorProfile {}
	impl Content for super::Cursor {}
	impl Content for super::Filter {}
	impl Content for super::Font {}
	impl Content for super::FontFace {}
	impl Content for super::ForeignObject {}
	impl Content for super::Image {}
	impl Content for super::Marker {}
	impl Content for super::Mask {}
	impl Content for super::Pattern {}
	impl Content for super::Script {}
	impl Content for super::Style {}
	impl Content for super::Switch {}
	impl Content for super::Text {}
	impl Content for super::View {}
}

#[doc = "The [`<defs>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("defs.md")]
#[doc = "\n\n [`<defs>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/defs"]
#[derive(Debug)]
pub struct Defs {
	attrs: IndexMap<DefsAttrs, String>,
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

	fn set_attr(&mut self, attr: DefsAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: DefsAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(DefsAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(DefsAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(DefsAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(DefsAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(DefsAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(DefsAttrs::ExternalResourcesRequired)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(DefsAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(DefsAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(DefsAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(DefsAttrs::Transform, value.into());
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(DefsAttrs::Transform, value.into());
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&str> {
		self.get_attr(DefsAttrs::Transform)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Defs {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(DefsAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(DefsAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Defs {}

impl common_attrs::CoreAttributesSetter for Defs {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(DefsAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(DefsAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Defs {}

impl common_attrs::GraphicalEventAttributesSetter for Defs {
	fn set_attr(&mut self, attr: common_attrs::GraphicalEventAttributes, value: String) {
		self.set_attr(DefsAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&str> {
		self.get_attr(DefsAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Defs {}

impl common_attrs::PresentationAttributesSetter for Defs {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(DefsAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(DefsAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Defs {}

impl Tag for Defs {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("defs");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	attrs: IndexMap<DescAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: DescAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: DescAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(DescAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(DescAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(DescAttrs::Class)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(DescAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(DescAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(DescAttrs::Style)
	}
}

impl common_attrs::CoreAttributesSetter for Desc {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(DescAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(DescAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Desc {}

impl Tag for Desc {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("desc");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Script {}
}

#[doc = "The [`<discard>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("discard.md")]
#[doc = "\n\n [`<discard>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/discard"]
#[derive(Debug)]
pub struct Discard {
	attrs: IndexMap<DiscardAttrs, String>,
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

	fn set_attr(&mut self, attr: DiscardAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: DiscardAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `begin` attribute.
	pub fn with_begin<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(DiscardAttrs::Begin, value.into());
		self
	}

	/// Set the `begin` attribute.
	pub fn set_begin<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(DiscardAttrs::Begin, value.into());
	}

	/// Get the `begin` attribute.
	pub fn begin(&self) -> Option<&str> {
		self.get_attr(DiscardAttrs::Begin)
	}

	/// Set the `href` attribute.
	pub fn with_href<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(DiscardAttrs::Href, value.into());
		self
	}

	/// Set the `href` attribute.
	pub fn set_href<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(DiscardAttrs::Href, value.into());
	}

	/// Get the `href` attribute.
	pub fn href(&self) -> Option<&str> {
		self.get_attr(DiscardAttrs::Href)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Discard {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(DiscardAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(DiscardAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Discard {}

impl common_attrs::CoreAttributesSetter for Discard {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(DiscardAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(DiscardAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Discard {}

impl Tag for Discard {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("discard");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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

#[doc = "The [`<ellipse>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("ellipse.md")]
#[doc = "\n\n [`<ellipse>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/ellipse"]
#[derive(Debug)]
pub struct Ellipse {
	attrs: IndexMap<EllipseAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: EllipseAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: EllipseAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(EllipseAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(EllipseAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(EllipseAttrs::Class)
	}

	/// Set the `cx` attribute.
	pub fn with_cx<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(EllipseAttrs::Cx, value.into());
		self
	}

	/// Set the `cx` attribute.
	pub fn set_cx<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(EllipseAttrs::Cx, value.into());
	}

	/// Get the `cx` attribute.
	pub fn cx(&self) -> Option<&str> {
		self.get_attr(EllipseAttrs::Cx)
	}

	/// Set the `cy` attribute.
	pub fn with_cy<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(EllipseAttrs::Cy, value.into());
		self
	}

	/// Set the `cy` attribute.
	pub fn set_cy<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(EllipseAttrs::Cy, value.into());
	}

	/// Get the `cy` attribute.
	pub fn cy(&self) -> Option<&str> {
		self.get_attr(EllipseAttrs::Cy)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(EllipseAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(EllipseAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(EllipseAttrs::ExternalResourcesRequired)
	}

	/// Set the `rx` attribute.
	pub fn with_rx<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(EllipseAttrs::Rx, value.into());
		self
	}

	/// Set the `rx` attribute.
	pub fn set_rx<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(EllipseAttrs::Rx, value.into());
	}

	/// Get the `rx` attribute.
	pub fn rx(&self) -> Option<&str> {
		self.get_attr(EllipseAttrs::Rx)
	}

	/// Set the `ry` attribute.
	pub fn with_ry<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(EllipseAttrs::Ry, value.into());
		self
	}

	/// Set the `ry` attribute.
	pub fn set_ry<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(EllipseAttrs::Ry, value.into());
	}

	/// Get the `ry` attribute.
	pub fn ry(&self) -> Option<&str> {
		self.get_attr(EllipseAttrs::Ry)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(EllipseAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(EllipseAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(EllipseAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(EllipseAttrs::Transform, value.into());
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(EllipseAttrs::Transform, value.into());
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&str> {
		self.get_attr(EllipseAttrs::Transform)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Ellipse {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(EllipseAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(EllipseAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Ellipse {}

impl common_attrs::CoreAttributesSetter for Ellipse {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(EllipseAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(EllipseAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Ellipse {}

impl common_attrs::GraphicalEventAttributesSetter for Ellipse {
	fn set_attr(&mut self, attr: common_attrs::GraphicalEventAttributes, value: String) {
		self.set_attr(EllipseAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&str> {
		self.get_attr(EllipseAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Ellipse {}

impl common_attrs::PresentationAttributesSetter for Ellipse {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(EllipseAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(EllipseAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Ellipse {}

impl Tag for Ellipse {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("ellipse");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feBlend>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feBlend.md")]
#[doc = "\n\n [`<feBlend>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feBlend"]
#[derive(Debug)]
pub struct FeBlend {
	attrs: IndexMap<FeBlendAttrs, String>,
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

	fn set_attr(&mut self, attr: FeBlendAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FeBlendAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeBlendAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeBlendAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(FeBlendAttrs::Class)
	}

	/// Set the `in` attribute.
	pub fn with_in1<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeBlendAttrs::In, value.into());
		self
	}

	/// Set the `in` attribute.
	pub fn set_in1<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeBlendAttrs::In, value.into());
	}

	/// Get the `in` attribute.
	pub fn in1(&self) -> Option<&str> {
		self.get_attr(FeBlendAttrs::In)
	}

	/// Set the `in2` attribute.
	pub fn with_in2<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeBlendAttrs::In2, value.into());
		self
	}

	/// Set the `in2` attribute.
	pub fn set_in2<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeBlendAttrs::In2, value.into());
	}

	/// Get the `in2` attribute.
	pub fn in2(&self) -> Option<&str> {
		self.get_attr(FeBlendAttrs::In2)
	}

	/// Set the `mode` attribute.
	pub fn with_mode<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeBlendAttrs::Mode, value.into());
		self
	}

	/// Set the `mode` attribute.
	pub fn set_mode<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeBlendAttrs::Mode, value.into());
	}

	/// Get the `mode` attribute.
	pub fn mode(&self) -> Option<&str> {
		self.get_attr(FeBlendAttrs::Mode)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeBlendAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeBlendAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(FeBlendAttrs::Style)
	}
}

impl common_attrs::CoreAttributesSetter for FeBlend {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FeBlendAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FeBlendAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeBlend {}

impl common_attrs::FilterAttributesSetter for FeBlend {
	fn set_attr(&mut self, attr: common_attrs::FilterAttributes, value: String) {
		self.set_attr(FeBlendAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&str> {
		self.get_attr(FeBlendAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeBlend {}

impl common_attrs::PresentationAttributesSetter for FeBlend {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(FeBlendAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(FeBlendAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeBlend {}

impl Tag for FeBlend {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("feBlend");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feColorMatrix>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feColorMatrix.md")]
#[doc = "\n\n [`<feColorMatrix>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feColorMatrix"]
#[derive(Debug)]
pub struct FeColorMatrix {
	attrs: IndexMap<FeColorMatrixAttrs, String>,
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

	fn set_attr(&mut self, attr: FeColorMatrixAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FeColorMatrixAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeColorMatrixAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeColorMatrixAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(FeColorMatrixAttrs::Class)
	}

	/// Set the `in` attribute.
	pub fn with_in1<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeColorMatrixAttrs::In, value.into());
		self
	}

	/// Set the `in` attribute.
	pub fn set_in1<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeColorMatrixAttrs::In, value.into());
	}

	/// Get the `in` attribute.
	pub fn in1(&self) -> Option<&str> {
		self.get_attr(FeColorMatrixAttrs::In)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeColorMatrixAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeColorMatrixAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(FeColorMatrixAttrs::Style)
	}

	/// Set the `type` attribute.
	pub fn with_ty<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeColorMatrixAttrs::Type, value.into());
		self
	}

	/// Set the `type` attribute.
	pub fn set_ty<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeColorMatrixAttrs::Type, value.into());
	}

	/// Get the `type` attribute.
	pub fn ty(&self) -> Option<&str> {
		self.get_attr(FeColorMatrixAttrs::Type)
	}

	/// Set the `values` attribute.
	pub fn with_values<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeColorMatrixAttrs::Values, value.into());
		self
	}

	/// Set the `values` attribute.
	pub fn set_values<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeColorMatrixAttrs::Values, value.into());
	}

	/// Get the `values` attribute.
	pub fn values(&self) -> Option<&str> {
		self.get_attr(FeColorMatrixAttrs::Values)
	}
}

impl common_attrs::CoreAttributesSetter for FeColorMatrix {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FeColorMatrixAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FeColorMatrixAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeColorMatrix {}

impl common_attrs::FilterAttributesSetter for FeColorMatrix {
	fn set_attr(&mut self, attr: common_attrs::FilterAttributes, value: String) {
		self.set_attr(FeColorMatrixAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&str> {
		self.get_attr(FeColorMatrixAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeColorMatrix {}

impl common_attrs::PresentationAttributesSetter for FeColorMatrix {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(FeColorMatrixAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(FeColorMatrixAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeColorMatrix {}

impl Tag for FeColorMatrix {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("feColorMatrix");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::FeFuncA {}
	impl Content for super::FeFuncB {}
	impl Content for super::FeFuncG {}
	impl Content for super::FeFuncR {}
}

#[doc = "The [`<feComponentTransfer>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feComponentTransfer.md")]
#[doc = "\n\n [`<feComponentTransfer>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feComponentTransfer"]
#[derive(Debug)]
pub struct FeComponentTransfer {
	attrs: IndexMap<FeComponentTransferAttrs, String>,
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

	fn set_attr(&mut self, attr: FeComponentTransferAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FeComponentTransferAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeComponentTransferAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeComponentTransferAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(FeComponentTransferAttrs::Class)
	}

	/// Set the `in` attribute.
	pub fn with_in1<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeComponentTransferAttrs::In, value.into());
		self
	}

	/// Set the `in` attribute.
	pub fn set_in1<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeComponentTransferAttrs::In, value.into());
	}

	/// Get the `in` attribute.
	pub fn in1(&self) -> Option<&str> {
		self.get_attr(FeComponentTransferAttrs::In)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeComponentTransferAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeComponentTransferAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(FeComponentTransferAttrs::Style)
	}
}

impl common_attrs::CoreAttributesSetter for FeComponentTransfer {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FeComponentTransferAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FeComponentTransferAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeComponentTransfer {}

impl common_attrs::FilterAttributesSetter for FeComponentTransfer {
	fn set_attr(&mut self, attr: common_attrs::FilterAttributes, value: String) {
		self.set_attr(FeComponentTransferAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&str> {
		self.get_attr(FeComponentTransferAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeComponentTransfer {}

impl common_attrs::PresentationAttributesSetter for FeComponentTransfer {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(FeComponentTransferAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(FeComponentTransferAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeComponentTransfer {}

impl Tag for FeComponentTransfer {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("feComponentTransfer");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feComposite>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feComposite.md")]
#[doc = "\n\n [`<feComposite>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feComposite"]
#[derive(Debug)]
pub struct FeComposite {
	attrs: IndexMap<FeCompositeAttrs, String>,
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

	fn set_attr(&mut self, attr: FeCompositeAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FeCompositeAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeCompositeAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeCompositeAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(FeCompositeAttrs::Class)
	}

	/// Set the `in` attribute.
	pub fn with_in1<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeCompositeAttrs::In, value.into());
		self
	}

	/// Set the `in` attribute.
	pub fn set_in1<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeCompositeAttrs::In, value.into());
	}

	/// Get the `in` attribute.
	pub fn in1(&self) -> Option<&str> {
		self.get_attr(FeCompositeAttrs::In)
	}

	/// Set the `in2` attribute.
	pub fn with_in2<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeCompositeAttrs::In2, value.into());
		self
	}

	/// Set the `in2` attribute.
	pub fn set_in2<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeCompositeAttrs::In2, value.into());
	}

	/// Get the `in2` attribute.
	pub fn in2(&self) -> Option<&str> {
		self.get_attr(FeCompositeAttrs::In2)
	}

	/// Set the `k1` attribute.
	pub fn with_k1<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeCompositeAttrs::K1, value.into());
		self
	}

	/// Set the `k1` attribute.
	pub fn set_k1<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeCompositeAttrs::K1, value.into());
	}

	/// Get the `k1` attribute.
	pub fn k1(&self) -> Option<&str> {
		self.get_attr(FeCompositeAttrs::K1)
	}

	/// Set the `k2` attribute.
	pub fn with_k2<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeCompositeAttrs::K2, value.into());
		self
	}

	/// Set the `k2` attribute.
	pub fn set_k2<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeCompositeAttrs::K2, value.into());
	}

	/// Get the `k2` attribute.
	pub fn k2(&self) -> Option<&str> {
		self.get_attr(FeCompositeAttrs::K2)
	}

	/// Set the `k3` attribute.
	pub fn with_k3<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeCompositeAttrs::K3, value.into());
		self
	}

	/// Set the `k3` attribute.
	pub fn set_k3<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeCompositeAttrs::K3, value.into());
	}

	/// Get the `k3` attribute.
	pub fn k3(&self) -> Option<&str> {
		self.get_attr(FeCompositeAttrs::K3)
	}

	/// Set the `k4` attribute.
	pub fn with_k4<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeCompositeAttrs::K4, value.into());
		self
	}

	/// Set the `k4` attribute.
	pub fn set_k4<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeCompositeAttrs::K4, value.into());
	}

	/// Get the `k4` attribute.
	pub fn k4(&self) -> Option<&str> {
		self.get_attr(FeCompositeAttrs::K4)
	}

	/// Set the `operator` attribute.
	pub fn with_operator<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeCompositeAttrs::Operator, value.into());
		self
	}

	/// Set the `operator` attribute.
	pub fn set_operator<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeCompositeAttrs::Operator, value.into());
	}

	/// Get the `operator` attribute.
	pub fn operator(&self) -> Option<&str> {
		self.get_attr(FeCompositeAttrs::Operator)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeCompositeAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeCompositeAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(FeCompositeAttrs::Style)
	}
}

impl common_attrs::CoreAttributesSetter for FeComposite {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FeCompositeAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FeCompositeAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeComposite {}

impl common_attrs::FilterAttributesSetter for FeComposite {
	fn set_attr(&mut self, attr: common_attrs::FilterAttributes, value: String) {
		self.set_attr(FeCompositeAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&str> {
		self.get_attr(FeCompositeAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeComposite {}

impl common_attrs::PresentationAttributesSetter for FeComposite {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(FeCompositeAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(FeCompositeAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeComposite {}

impl Tag for FeComposite {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("feComposite");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feConvolveMatrix>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feConvolveMatrix.md")]
#[doc = "\n\n [`<feConvolveMatrix>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feConvolveMatrix"]
#[derive(Debug)]
pub struct FeConvolveMatrix {
	attrs: IndexMap<FeConvolveMatrixAttrs, String>,
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

	fn set_attr(&mut self, attr: FeConvolveMatrixAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FeConvolveMatrixAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `bias` attribute.
	pub fn with_bias<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeConvolveMatrixAttrs::Bias, value.into());
		self
	}

	/// Set the `bias` attribute.
	pub fn set_bias<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeConvolveMatrixAttrs::Bias, value.into());
	}

	/// Get the `bias` attribute.
	pub fn bias(&self) -> Option<&str> {
		self.get_attr(FeConvolveMatrixAttrs::Bias)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeConvolveMatrixAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeConvolveMatrixAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(FeConvolveMatrixAttrs::Class)
	}

	/// Set the `divisor` attribute.
	pub fn with_divisor<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeConvolveMatrixAttrs::Divisor, value.into());
		self
	}

	/// Set the `divisor` attribute.
	pub fn set_divisor<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeConvolveMatrixAttrs::Divisor, value.into());
	}

	/// Get the `divisor` attribute.
	pub fn divisor(&self) -> Option<&str> {
		self.get_attr(FeConvolveMatrixAttrs::Divisor)
	}

	/// Set the `edgeMode` attribute.
	pub fn with_edge_mode<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeConvolveMatrixAttrs::EdgeMode, value.into());
		self
	}

	/// Set the `edgeMode` attribute.
	pub fn set_edge_mode<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeConvolveMatrixAttrs::EdgeMode, value.into());
	}

	/// Get the `edgeMode` attribute.
	pub fn edge_mode(&self) -> Option<&str> {
		self.get_attr(FeConvolveMatrixAttrs::EdgeMode)
	}

	/// Set the `in` attribute.
	pub fn with_in1<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeConvolveMatrixAttrs::In, value.into());
		self
	}

	/// Set the `in` attribute.
	pub fn set_in1<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeConvolveMatrixAttrs::In, value.into());
	}

	/// Get the `in` attribute.
	pub fn in1(&self) -> Option<&str> {
		self.get_attr(FeConvolveMatrixAttrs::In)
	}

	/// Set the `kernelMatrix` attribute.
	pub fn with_kernel_matrix<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeConvolveMatrixAttrs::KernelMatrix, value.into());
		self
	}

	/// Set the `kernelMatrix` attribute.
	pub fn set_kernel_matrix<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeConvolveMatrixAttrs::KernelMatrix, value.into());
	}

	/// Get the `kernelMatrix` attribute.
	pub fn kernel_matrix(&self) -> Option<&str> {
		self.get_attr(FeConvolveMatrixAttrs::KernelMatrix)
	}

	/// Set the `kernelUnitLength` attribute.
	pub fn with_kernel_unit_length<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeConvolveMatrixAttrs::KernelUnitLength, value.into());
		self
	}

	/// Set the `kernelUnitLength` attribute.
	pub fn set_kernel_unit_length<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeConvolveMatrixAttrs::KernelUnitLength, value.into());
	}

	/// Get the `kernelUnitLength` attribute.
	pub fn kernel_unit_length(&self) -> Option<&str> {
		self.get_attr(FeConvolveMatrixAttrs::KernelUnitLength)
	}

	/// Set the `order` attribute.
	pub fn with_order<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeConvolveMatrixAttrs::Order, value.into());
		self
	}

	/// Set the `order` attribute.
	pub fn set_order<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeConvolveMatrixAttrs::Order, value.into());
	}

	/// Get the `order` attribute.
	pub fn order(&self) -> Option<&str> {
		self.get_attr(FeConvolveMatrixAttrs::Order)
	}

	/// Set the `preserveAlpha` attribute.
	pub fn with_preserve_alpha<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeConvolveMatrixAttrs::PreserveAlpha, value.into());
		self
	}

	/// Set the `preserveAlpha` attribute.
	pub fn set_preserve_alpha<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeConvolveMatrixAttrs::PreserveAlpha, value.into());
	}

	/// Get the `preserveAlpha` attribute.
	pub fn preserve_alpha(&self) -> Option<&str> {
		self.get_attr(FeConvolveMatrixAttrs::PreserveAlpha)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeConvolveMatrixAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeConvolveMatrixAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(FeConvolveMatrixAttrs::Style)
	}

	/// Set the `targetX` attribute.
	pub fn with_target_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeConvolveMatrixAttrs::TargetX, value.into());
		self
	}

	/// Set the `targetX` attribute.
	pub fn set_target_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeConvolveMatrixAttrs::TargetX, value.into());
	}

	/// Get the `targetX` attribute.
	pub fn target_x(&self) -> Option<&str> {
		self.get_attr(FeConvolveMatrixAttrs::TargetX)
	}

	/// Set the `targetY` attribute.
	pub fn with_target_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeConvolveMatrixAttrs::TargetY, value.into());
		self
	}

	/// Set the `targetY` attribute.
	pub fn set_target_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeConvolveMatrixAttrs::TargetY, value.into());
	}

	/// Get the `targetY` attribute.
	pub fn target_y(&self) -> Option<&str> {
		self.get_attr(FeConvolveMatrixAttrs::TargetY)
	}
}

impl common_attrs::CoreAttributesSetter for FeConvolveMatrix {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FeConvolveMatrixAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FeConvolveMatrixAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeConvolveMatrix {}

impl common_attrs::FilterAttributesSetter for FeConvolveMatrix {
	fn set_attr(&mut self, attr: common_attrs::FilterAttributes, value: String) {
		self.set_attr(FeConvolveMatrixAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&str> {
		self.get_attr(FeConvolveMatrixAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeConvolveMatrix {}

impl common_attrs::PresentationAttributesSetter for FeConvolveMatrix {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(FeConvolveMatrixAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(FeConvolveMatrixAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeConvolveMatrix {}

impl Tag for FeConvolveMatrix {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("feConvolveMatrix");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
#[doc = "\n\n [`<feDiffuseLighting>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feDiffuseLighting"]
#[derive(Debug)]
pub struct FeDiffuseLighting {
	attrs: IndexMap<FeDiffuseLightingAttrs, String>,
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

	fn set_attr(&mut self, attr: FeDiffuseLightingAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FeDiffuseLightingAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeDiffuseLightingAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeDiffuseLightingAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(FeDiffuseLightingAttrs::Class)
	}

	/// Set the `diffuseConstant` attribute.
	pub fn with_diffuse_constant<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeDiffuseLightingAttrs::DiffuseConstant, value.into());
		self
	}

	/// Set the `diffuseConstant` attribute.
	pub fn set_diffuse_constant<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeDiffuseLightingAttrs::DiffuseConstant, value.into());
	}

	/// Get the `diffuseConstant` attribute.
	pub fn diffuse_constant(&self) -> Option<&str> {
		self.get_attr(FeDiffuseLightingAttrs::DiffuseConstant)
	}

	/// Set the `in` attribute.
	pub fn with_in1<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeDiffuseLightingAttrs::In, value.into());
		self
	}

	/// Set the `in` attribute.
	pub fn set_in1<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeDiffuseLightingAttrs::In, value.into());
	}

	/// Get the `in` attribute.
	pub fn in1(&self) -> Option<&str> {
		self.get_attr(FeDiffuseLightingAttrs::In)
	}

	/// Set the `kernelUnitLength` attribute.
	pub fn with_kernel_unit_length<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeDiffuseLightingAttrs::KernelUnitLength, value.into());
		self
	}

	/// Set the `kernelUnitLength` attribute.
	pub fn set_kernel_unit_length<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeDiffuseLightingAttrs::KernelUnitLength, value.into());
	}

	/// Get the `kernelUnitLength` attribute.
	pub fn kernel_unit_length(&self) -> Option<&str> {
		self.get_attr(FeDiffuseLightingAttrs::KernelUnitLength)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeDiffuseLightingAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeDiffuseLightingAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(FeDiffuseLightingAttrs::Style)
	}

	/// Set the `surfaceScale` attribute.
	pub fn with_surface_scale<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeDiffuseLightingAttrs::SurfaceScale, value.into());
		self
	}

	/// Set the `surfaceScale` attribute.
	pub fn set_surface_scale<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeDiffuseLightingAttrs::SurfaceScale, value.into());
	}

	/// Get the `surfaceScale` attribute.
	pub fn surface_scale(&self) -> Option<&str> {
		self.get_attr(FeDiffuseLightingAttrs::SurfaceScale)
	}
}

impl common_attrs::CoreAttributesSetter for FeDiffuseLighting {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FeDiffuseLightingAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FeDiffuseLightingAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeDiffuseLighting {}

impl common_attrs::FilterAttributesSetter for FeDiffuseLighting {
	fn set_attr(&mut self, attr: common_attrs::FilterAttributes, value: String) {
		self.set_attr(FeDiffuseLightingAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&str> {
		self.get_attr(FeDiffuseLightingAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeDiffuseLighting {}

impl common_attrs::PresentationAttributesSetter for FeDiffuseLighting {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(FeDiffuseLightingAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(FeDiffuseLightingAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeDiffuseLighting {}

impl Tag for FeDiffuseLighting {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("feDiffuseLighting");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feDisplacementMap>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feDisplacementMap.md")]
#[doc = "\n\n [`<feDisplacementMap>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feDisplacementMap"]
#[derive(Debug)]
pub struct FeDisplacementMap {
	attrs: IndexMap<FeDisplacementMapAttrs, String>,
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

	fn set_attr(&mut self, attr: FeDisplacementMapAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FeDisplacementMapAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeDisplacementMapAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeDisplacementMapAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(FeDisplacementMapAttrs::Class)
	}

	/// Set the `in` attribute.
	pub fn with_in1<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeDisplacementMapAttrs::In, value.into());
		self
	}

	/// Set the `in` attribute.
	pub fn set_in1<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeDisplacementMapAttrs::In, value.into());
	}

	/// Get the `in` attribute.
	pub fn in1(&self) -> Option<&str> {
		self.get_attr(FeDisplacementMapAttrs::In)
	}

	/// Set the `in2` attribute.
	pub fn with_in2<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeDisplacementMapAttrs::In2, value.into());
		self
	}

	/// Set the `in2` attribute.
	pub fn set_in2<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeDisplacementMapAttrs::In2, value.into());
	}

	/// Get the `in2` attribute.
	pub fn in2(&self) -> Option<&str> {
		self.get_attr(FeDisplacementMapAttrs::In2)
	}

	/// Set the `scale` attribute.
	pub fn with_scale<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeDisplacementMapAttrs::Scale, value.into());
		self
	}

	/// Set the `scale` attribute.
	pub fn set_scale<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeDisplacementMapAttrs::Scale, value.into());
	}

	/// Get the `scale` attribute.
	pub fn scale(&self) -> Option<&str> {
		self.get_attr(FeDisplacementMapAttrs::Scale)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeDisplacementMapAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeDisplacementMapAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(FeDisplacementMapAttrs::Style)
	}

	/// Set the `xChannelSelector` attribute.
	pub fn with_x_channel_selector<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeDisplacementMapAttrs::XChannelSelector, value.into());
		self
	}

	/// Set the `xChannelSelector` attribute.
	pub fn set_x_channel_selector<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeDisplacementMapAttrs::XChannelSelector, value.into());
	}

	/// Get the `xChannelSelector` attribute.
	pub fn x_channel_selector(&self) -> Option<&str> {
		self.get_attr(FeDisplacementMapAttrs::XChannelSelector)
	}

	/// Set the `yChannelSelector` attribute.
	pub fn with_y_channel_selector<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeDisplacementMapAttrs::YChannelSelector, value.into());
		self
	}

	/// Set the `yChannelSelector` attribute.
	pub fn set_y_channel_selector<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeDisplacementMapAttrs::YChannelSelector, value.into());
	}

	/// Get the `yChannelSelector` attribute.
	pub fn y_channel_selector(&self) -> Option<&str> {
		self.get_attr(FeDisplacementMapAttrs::YChannelSelector)
	}
}

impl common_attrs::CoreAttributesSetter for FeDisplacementMap {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FeDisplacementMapAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FeDisplacementMapAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeDisplacementMap {}

impl common_attrs::FilterAttributesSetter for FeDisplacementMap {
	fn set_attr(&mut self, attr: common_attrs::FilterAttributes, value: String) {
		self.set_attr(FeDisplacementMapAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&str> {
		self.get_attr(FeDisplacementMapAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeDisplacementMap {}

impl common_attrs::PresentationAttributesSetter for FeDisplacementMap {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(FeDisplacementMapAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(FeDisplacementMapAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeDisplacementMap {}

impl Tag for FeDisplacementMap {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("feDisplacementMap");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feDistantLight>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feDistantLight.md")]
#[doc = "\n\n [`<feDistantLight>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feDistantLight"]
#[derive(Debug)]
pub struct FeDistantLight {
	attrs: IndexMap<FeDistantLightAttrs, String>,
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

	fn set_attr(&mut self, attr: FeDistantLightAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FeDistantLightAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `azimuth` attribute.
	pub fn with_azimuth<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeDistantLightAttrs::Azimuth, value.into());
		self
	}

	/// Set the `azimuth` attribute.
	pub fn set_azimuth<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeDistantLightAttrs::Azimuth, value.into());
	}

	/// Get the `azimuth` attribute.
	pub fn azimuth(&self) -> Option<&str> {
		self.get_attr(FeDistantLightAttrs::Azimuth)
	}

	/// Set the `elevation` attribute.
	pub fn with_elevation<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeDistantLightAttrs::Elevation, value.into());
		self
	}

	/// Set the `elevation` attribute.
	pub fn set_elevation<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeDistantLightAttrs::Elevation, value.into());
	}

	/// Get the `elevation` attribute.
	pub fn elevation(&self) -> Option<&str> {
		self.get_attr(FeDistantLightAttrs::Elevation)
	}
}

impl common_attrs::CoreAttributesSetter for FeDistantLight {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FeDistantLightAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FeDistantLightAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeDistantLight {}

impl Tag for FeDistantLight {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("feDistantLight");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Script {}
	impl Content for super::Set {}
}

#[doc = "The [`<feDropShadow>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feDropShadow.md")]
#[doc = "\n\n [`<feDropShadow>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feDropShadow"]
#[derive(Debug)]
pub struct FeDropShadow {
	attrs: IndexMap<FeDropShadowAttrs, String>,
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

	fn set_attr(&mut self, attr: FeDropShadowAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FeDropShadowAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeDropShadowAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeDropShadowAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(FeDropShadowAttrs::Class)
	}

	/// Set the `dx` attribute.
	pub fn with_dx<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeDropShadowAttrs::Dx, value.into());
		self
	}

	/// Set the `dx` attribute.
	pub fn set_dx<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeDropShadowAttrs::Dx, value.into());
	}

	/// Get the `dx` attribute.
	pub fn dx(&self) -> Option<&str> {
		self.get_attr(FeDropShadowAttrs::Dx)
	}

	/// Set the `dy` attribute.
	pub fn with_dy<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeDropShadowAttrs::Dy, value.into());
		self
	}

	/// Set the `dy` attribute.
	pub fn set_dy<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeDropShadowAttrs::Dy, value.into());
	}

	/// Get the `dy` attribute.
	pub fn dy(&self) -> Option<&str> {
		self.get_attr(FeDropShadowAttrs::Dy)
	}

	/// Set the `in` attribute.
	pub fn with_in1<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeDropShadowAttrs::In, value.into());
		self
	}

	/// Set the `in` attribute.
	pub fn set_in1<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeDropShadowAttrs::In, value.into());
	}

	/// Get the `in` attribute.
	pub fn in1(&self) -> Option<&str> {
		self.get_attr(FeDropShadowAttrs::In)
	}

	/// Set the `stdDeviation` attribute.
	pub fn with_std_deviation<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeDropShadowAttrs::StdDeviation, value.into());
		self
	}

	/// Set the `stdDeviation` attribute.
	pub fn set_std_deviation<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeDropShadowAttrs::StdDeviation, value.into());
	}

	/// Get the `stdDeviation` attribute.
	pub fn std_deviation(&self) -> Option<&str> {
		self.get_attr(FeDropShadowAttrs::StdDeviation)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeDropShadowAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeDropShadowAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(FeDropShadowAttrs::Style)
	}
}

impl common_attrs::CoreAttributesSetter for FeDropShadow {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FeDropShadowAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FeDropShadowAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeDropShadow {}

impl common_attrs::FilterAttributesSetter for FeDropShadow {
	fn set_attr(&mut self, attr: common_attrs::FilterAttributes, value: String) {
		self.set_attr(FeDropShadowAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&str> {
		self.get_attr(FeDropShadowAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeDropShadow {}

impl common_attrs::PresentationAttributesSetter for FeDropShadow {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(FeDropShadowAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(FeDropShadowAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeDropShadow {}

impl Tag for FeDropShadow {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("feDropShadow");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::AnimateColor {}
	impl Content for super::Set {}
}

#[doc = "The [`<feFlood>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feFlood.md")]
#[doc = "\n\n [`<feFlood>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feFlood"]
#[derive(Debug)]
pub struct FeFlood {
	attrs: IndexMap<FeFloodAttrs, String>,
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

	fn set_attr(&mut self, attr: FeFloodAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FeFloodAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeFloodAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeFloodAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(FeFloodAttrs::Class)
	}

	/// Set the `flood-color` attribute.
	pub fn with_flood_color<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeFloodAttrs::FloodColor, value.into());
		self
	}

	/// Set the `flood-color` attribute.
	pub fn set_flood_color<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeFloodAttrs::FloodColor, value.into());
	}

	/// Get the `flood-color` attribute.
	pub fn flood_color(&self) -> Option<&str> {
		self.get_attr(FeFloodAttrs::FloodColor)
	}

	/// Set the `flood-opacity` attribute.
	pub fn with_flood_opacity<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeFloodAttrs::FloodOpacity, value.into());
		self
	}

	/// Set the `flood-opacity` attribute.
	pub fn set_flood_opacity<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeFloodAttrs::FloodOpacity, value.into());
	}

	/// Get the `flood-opacity` attribute.
	pub fn flood_opacity(&self) -> Option<&str> {
		self.get_attr(FeFloodAttrs::FloodOpacity)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeFloodAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeFloodAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(FeFloodAttrs::Style)
	}
}

impl common_attrs::CoreAttributesSetter for FeFlood {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FeFloodAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FeFloodAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeFlood {}

impl common_attrs::FilterAttributesSetter for FeFlood {
	fn set_attr(&mut self, attr: common_attrs::FilterAttributes, value: String) {
		self.set_attr(FeFloodAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&str> {
		self.get_attr(FeFloodAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeFlood {}

impl common_attrs::PresentationAttributesSetter for FeFlood {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(FeFloodAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(FeFloodAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeFlood {}

impl Tag for FeFlood {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("feFlood");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feFuncA>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feFuncA.md")]
#[doc = "\n\n [`<feFuncA>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feFuncA"]
#[derive(Debug)]
pub struct FeFuncA {
	attrs: IndexMap<FeFuncAAttrs, String>,
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

	fn set_attr(&mut self, attr: FeFuncAAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FeFuncAAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}
}

impl common_attrs::CoreAttributesSetter for FeFuncA {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FeFuncAAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FeFuncAAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeFuncA {}

impl common_attrs::TransferFunctionAttributesSetter for FeFuncA {
	fn set_attr(&mut self, attr: common_attrs::TransferFunctionAttributes, value: String) {
		self.set_attr(FeFuncAAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::TransferFunctionAttributes) -> Option<&str> {
		self.get_attr(FeFuncAAttrs::from(attr))
	}
}

impl TagWithTransferFunctionAttributes for FeFuncA {}

impl Tag for FeFuncA {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("feFuncA");
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feFuncB>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feFuncB.md")]
#[doc = "\n\n [`<feFuncB>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feFuncB"]
#[derive(Debug)]
pub struct FeFuncB {
	attrs: IndexMap<FeFuncBAttrs, String>,
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

	fn set_attr(&mut self, attr: FeFuncBAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FeFuncBAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}
}

impl common_attrs::CoreAttributesSetter for FeFuncB {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FeFuncBAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FeFuncBAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeFuncB {}

impl common_attrs::TransferFunctionAttributesSetter for FeFuncB {
	fn set_attr(&mut self, attr: common_attrs::TransferFunctionAttributes, value: String) {
		self.set_attr(FeFuncBAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::TransferFunctionAttributes) -> Option<&str> {
		self.get_attr(FeFuncBAttrs::from(attr))
	}
}

impl TagWithTransferFunctionAttributes for FeFuncB {}

impl Tag for FeFuncB {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("feFuncB");
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feFuncG>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feFuncG.md")]
#[doc = "\n\n [`<feFuncG>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feFuncG"]
#[derive(Debug)]
pub struct FeFuncG {
	attrs: IndexMap<FeFuncGAttrs, String>,
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

	fn set_attr(&mut self, attr: FeFuncGAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FeFuncGAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}
}

impl common_attrs::CoreAttributesSetter for FeFuncG {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FeFuncGAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FeFuncGAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeFuncG {}

impl common_attrs::TransferFunctionAttributesSetter for FeFuncG {
	fn set_attr(&mut self, attr: common_attrs::TransferFunctionAttributes, value: String) {
		self.set_attr(FeFuncGAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::TransferFunctionAttributes) -> Option<&str> {
		self.get_attr(FeFuncGAttrs::from(attr))
	}
}

impl TagWithTransferFunctionAttributes for FeFuncG {}

impl Tag for FeFuncG {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("feFuncG");
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feFuncR>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feFuncR.md")]
#[doc = "\n\n [`<feFuncR>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feFuncR"]
#[derive(Debug)]
pub struct FeFuncR {
	attrs: IndexMap<FeFuncRAttrs, String>,
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

	fn set_attr(&mut self, attr: FeFuncRAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FeFuncRAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}
}

impl common_attrs::CoreAttributesSetter for FeFuncR {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FeFuncRAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FeFuncRAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeFuncR {}

impl common_attrs::TransferFunctionAttributesSetter for FeFuncR {
	fn set_attr(&mut self, attr: common_attrs::TransferFunctionAttributes, value: String) {
		self.set_attr(FeFuncRAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::TransferFunctionAttributes) -> Option<&str> {
		self.get_attr(FeFuncRAttrs::from(attr))
	}
}

impl TagWithTransferFunctionAttributes for FeFuncR {}

impl Tag for FeFuncR {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("feFuncR");
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feGaussianBlur>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feGaussianBlur.md")]
#[doc = "\n\n [`<feGaussianBlur>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feGaussianBlur"]
#[derive(Debug)]
pub struct FeGaussianBlur {
	attrs: IndexMap<FeGaussianBlurAttrs, String>,
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

	fn set_attr(&mut self, attr: FeGaussianBlurAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FeGaussianBlurAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeGaussianBlurAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeGaussianBlurAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(FeGaussianBlurAttrs::Class)
	}

	/// Set the `in` attribute.
	pub fn with_in1<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeGaussianBlurAttrs::In, value.into());
		self
	}

	/// Set the `in` attribute.
	pub fn set_in1<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeGaussianBlurAttrs::In, value.into());
	}

	/// Get the `in` attribute.
	pub fn in1(&self) -> Option<&str> {
		self.get_attr(FeGaussianBlurAttrs::In)
	}

	/// Set the `stdDeviation` attribute.
	pub fn with_std_deviation<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeGaussianBlurAttrs::StdDeviation, value.into());
		self
	}

	/// Set the `stdDeviation` attribute.
	pub fn set_std_deviation<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeGaussianBlurAttrs::StdDeviation, value.into());
	}

	/// Get the `stdDeviation` attribute.
	pub fn std_deviation(&self) -> Option<&str> {
		self.get_attr(FeGaussianBlurAttrs::StdDeviation)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeGaussianBlurAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeGaussianBlurAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(FeGaussianBlurAttrs::Style)
	}
}

impl common_attrs::CoreAttributesSetter for FeGaussianBlur {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FeGaussianBlurAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FeGaussianBlurAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeGaussianBlur {}

impl common_attrs::FilterAttributesSetter for FeGaussianBlur {
	fn set_attr(&mut self, attr: common_attrs::FilterAttributes, value: String) {
		self.set_attr(FeGaussianBlurAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&str> {
		self.get_attr(FeGaussianBlurAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeGaussianBlur {}

impl common_attrs::PresentationAttributesSetter for FeGaussianBlur {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(FeGaussianBlurAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(FeGaussianBlurAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeGaussianBlur {}

impl Tag for FeGaussianBlur {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("feGaussianBlur");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Set {}
}

#[doc = "The [`<feImage>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feImage.md")]
#[doc = "\n\n [`<feImage>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feImage"]
#[derive(Debug)]
pub struct FeImage {
	attrs: IndexMap<FeImageAttrs, String>,
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

	fn set_attr(&mut self, attr: FeImageAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FeImageAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeImageAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeImageAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(FeImageAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeImageAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeImageAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(FeImageAttrs::ExternalResourcesRequired)
	}

	/// Set the `preserveAspectRatio` attribute.
	pub fn with_preserve_aspect_ratio<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeImageAttrs::PreserveAspectRatio, value.into());
		self
	}

	/// Set the `preserveAspectRatio` attribute.
	pub fn set_preserve_aspect_ratio<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeImageAttrs::PreserveAspectRatio, value.into());
	}

	/// Get the `preserveAspectRatio` attribute.
	pub fn preserve_aspect_ratio(&self) -> Option<&str> {
		self.get_attr(FeImageAttrs::PreserveAspectRatio)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeImageAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeImageAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(FeImageAttrs::Style)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeImageAttrs::XlinkHref, value.into());
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeImageAttrs::XlinkHref, value.into());
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&str> {
		self.get_attr(FeImageAttrs::XlinkHref)
	}
}

impl common_attrs::CoreAttributesSetter for FeImage {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FeImageAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FeImageAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeImage {}

impl common_attrs::FilterAttributesSetter for FeImage {
	fn set_attr(&mut self, attr: common_attrs::FilterAttributes, value: String) {
		self.set_attr(FeImageAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&str> {
		self.get_attr(FeImageAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeImage {}

impl common_attrs::PresentationAttributesSetter for FeImage {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(FeImageAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(FeImageAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeImage {}

impl common_attrs::XLinkAttributesSetter for FeImage {
	fn set_attr(&mut self, attr: common_attrs::XLinkAttributes, value: String) {
		self.set_attr(FeImageAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&str> {
		self.get_attr(FeImageAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for FeImage {}

impl Tag for FeImage {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("feImage");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::FeMergeNode {}
}

#[doc = "The [`<feMerge>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feMerge.md")]
#[doc = "\n\n [`<feMerge>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feMerge"]
#[derive(Debug)]
pub struct FeMerge {
	attrs: IndexMap<FeMergeAttrs, String>,
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

	fn set_attr(&mut self, attr: FeMergeAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FeMergeAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeMergeAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeMergeAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(FeMergeAttrs::Class)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeMergeAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeMergeAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(FeMergeAttrs::Style)
	}
}

impl common_attrs::CoreAttributesSetter for FeMerge {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FeMergeAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FeMergeAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeMerge {}

impl common_attrs::FilterAttributesSetter for FeMerge {
	fn set_attr(&mut self, attr: common_attrs::FilterAttributes, value: String) {
		self.set_attr(FeMergeAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&str> {
		self.get_attr(FeMergeAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeMerge {}

impl common_attrs::PresentationAttributesSetter for FeMerge {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(FeMergeAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(FeMergeAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeMerge {}

impl Tag for FeMerge {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("feMerge");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feMergeNode>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feMergeNode.md")]
#[doc = "\n\n [`<feMergeNode>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feMergeNode"]
#[derive(Debug)]
pub struct FeMergeNode {
	attrs: IndexMap<FeMergeNodeAttrs, String>,
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

	fn set_attr(&mut self, attr: FeMergeNodeAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FeMergeNodeAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `in` attribute.
	pub fn with_in1<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeMergeNodeAttrs::In, value.into());
		self
	}

	/// Set the `in` attribute.
	pub fn set_in1<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeMergeNodeAttrs::In, value.into());
	}

	/// Get the `in` attribute.
	pub fn in1(&self) -> Option<&str> {
		self.get_attr(FeMergeNodeAttrs::In)
	}
}

impl common_attrs::CoreAttributesSetter for FeMergeNode {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FeMergeNodeAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FeMergeNodeAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeMergeNode {}

impl Tag for FeMergeNode {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("feMergeNode");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feMorphology>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feMorphology.md")]
#[doc = "\n\n [`<feMorphology>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feMorphology"]
#[derive(Debug)]
pub struct FeMorphology {
	attrs: IndexMap<FeMorphologyAttrs, String>,
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

	fn set_attr(&mut self, attr: FeMorphologyAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FeMorphologyAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeMorphologyAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeMorphologyAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(FeMorphologyAttrs::Class)
	}

	/// Set the `in` attribute.
	pub fn with_in1<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeMorphologyAttrs::In, value.into());
		self
	}

	/// Set the `in` attribute.
	pub fn set_in1<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeMorphologyAttrs::In, value.into());
	}

	/// Get the `in` attribute.
	pub fn in1(&self) -> Option<&str> {
		self.get_attr(FeMorphologyAttrs::In)
	}

	/// Set the `operator` attribute.
	pub fn with_operator<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeMorphologyAttrs::Operator, value.into());
		self
	}

	/// Set the `operator` attribute.
	pub fn set_operator<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeMorphologyAttrs::Operator, value.into());
	}

	/// Get the `operator` attribute.
	pub fn operator(&self) -> Option<&str> {
		self.get_attr(FeMorphologyAttrs::Operator)
	}

	/// Set the `radius` attribute.
	pub fn with_radius<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeMorphologyAttrs::Radius, value.into());
		self
	}

	/// Set the `radius` attribute.
	pub fn set_radius<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeMorphologyAttrs::Radius, value.into());
	}

	/// Get the `radius` attribute.
	pub fn radius(&self) -> Option<&str> {
		self.get_attr(FeMorphologyAttrs::Radius)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeMorphologyAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeMorphologyAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(FeMorphologyAttrs::Style)
	}
}

impl common_attrs::CoreAttributesSetter for FeMorphology {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FeMorphologyAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FeMorphologyAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeMorphology {}

impl common_attrs::FilterAttributesSetter for FeMorphology {
	fn set_attr(&mut self, attr: common_attrs::FilterAttributes, value: String) {
		self.set_attr(FeMorphologyAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&str> {
		self.get_attr(FeMorphologyAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeMorphology {}

impl common_attrs::PresentationAttributesSetter for FeMorphology {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(FeMorphologyAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(FeMorphologyAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeMorphology {}

impl Tag for FeMorphology {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("feMorphology");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feOffset>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feOffset.md")]
#[doc = "\n\n [`<feOffset>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feOffset"]
#[derive(Debug)]
pub struct FeOffset {
	attrs: IndexMap<FeOffsetAttrs, String>,
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

	fn set_attr(&mut self, attr: FeOffsetAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FeOffsetAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeOffsetAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeOffsetAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(FeOffsetAttrs::Class)
	}

	/// Set the `dx` attribute.
	pub fn with_dx<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeOffsetAttrs::Dx, value.into());
		self
	}

	/// Set the `dx` attribute.
	pub fn set_dx<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeOffsetAttrs::Dx, value.into());
	}

	/// Get the `dx` attribute.
	pub fn dx(&self) -> Option<&str> {
		self.get_attr(FeOffsetAttrs::Dx)
	}

	/// Set the `dy` attribute.
	pub fn with_dy<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeOffsetAttrs::Dy, value.into());
		self
	}

	/// Set the `dy` attribute.
	pub fn set_dy<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeOffsetAttrs::Dy, value.into());
	}

	/// Get the `dy` attribute.
	pub fn dy(&self) -> Option<&str> {
		self.get_attr(FeOffsetAttrs::Dy)
	}

	/// Set the `in` attribute.
	pub fn with_in1<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeOffsetAttrs::In, value.into());
		self
	}

	/// Set the `in` attribute.
	pub fn set_in1<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeOffsetAttrs::In, value.into());
	}

	/// Get the `in` attribute.
	pub fn in1(&self) -> Option<&str> {
		self.get_attr(FeOffsetAttrs::In)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeOffsetAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeOffsetAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(FeOffsetAttrs::Style)
	}
}

impl common_attrs::CoreAttributesSetter for FeOffset {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FeOffsetAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FeOffsetAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeOffset {}

impl common_attrs::FilterAttributesSetter for FeOffset {
	fn set_attr(&mut self, attr: common_attrs::FilterAttributes, value: String) {
		self.set_attr(FeOffsetAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&str> {
		self.get_attr(FeOffsetAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeOffset {}

impl common_attrs::PresentationAttributesSetter for FeOffset {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(FeOffsetAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(FeOffsetAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeOffset {}

impl Tag for FeOffset {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("feOffset");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<fePointLight>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("fePointLight.md")]
#[doc = "\n\n [`<fePointLight>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/fePointLight"]
#[derive(Debug)]
pub struct FePointLight {
	attrs: IndexMap<FePointLightAttrs, String>,
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

	fn set_attr(&mut self, attr: FePointLightAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FePointLightAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `x` attribute.
	pub fn with_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FePointLightAttrs::X, value.into());
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FePointLightAttrs::X, value.into());
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&str> {
		self.get_attr(FePointLightAttrs::X)
	}

	/// Set the `y` attribute.
	pub fn with_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FePointLightAttrs::Y, value.into());
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FePointLightAttrs::Y, value.into());
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&str> {
		self.get_attr(FePointLightAttrs::Y)
	}

	/// Set the `z` attribute.
	pub fn with_z<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FePointLightAttrs::Z, value.into());
		self
	}

	/// Set the `z` attribute.
	pub fn set_z<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FePointLightAttrs::Z, value.into());
	}

	/// Get the `z` attribute.
	pub fn z(&self) -> Option<&str> {
		self.get_attr(FePointLightAttrs::Z)
	}
}

impl common_attrs::CoreAttributesSetter for FePointLight {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FePointLightAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FePointLightAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FePointLight {}

impl Tag for FePointLight {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("fePointLight");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
#[doc = "\n\n [`<feSpecularLighting>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feSpecularLighting"]
#[derive(Debug)]
pub struct FeSpecularLighting {
	attrs: IndexMap<FeSpecularLightingAttrs, String>,
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

	fn set_attr(&mut self, attr: FeSpecularLightingAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FeSpecularLightingAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeSpecularLightingAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeSpecularLightingAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(FeSpecularLightingAttrs::Class)
	}

	/// Set the `in` attribute.
	pub fn with_in1<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeSpecularLightingAttrs::In, value.into());
		self
	}

	/// Set the `in` attribute.
	pub fn set_in1<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeSpecularLightingAttrs::In, value.into());
	}

	/// Get the `in` attribute.
	pub fn in1(&self) -> Option<&str> {
		self.get_attr(FeSpecularLightingAttrs::In)
	}

	/// Set the `kernelUnitLength` attribute.
	pub fn with_kernel_unit_length<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeSpecularLightingAttrs::KernelUnitLength, value.into());
		self
	}

	/// Set the `kernelUnitLength` attribute.
	pub fn set_kernel_unit_length<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeSpecularLightingAttrs::KernelUnitLength, value.into());
	}

	/// Get the `kernelUnitLength` attribute.
	pub fn kernel_unit_length(&self) -> Option<&str> {
		self.get_attr(FeSpecularLightingAttrs::KernelUnitLength)
	}

	/// Set the `specularConstant` attribute.
	pub fn with_specular_constant<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeSpecularLightingAttrs::SpecularConstant, value.into());
		self
	}

	/// Set the `specularConstant` attribute.
	pub fn set_specular_constant<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeSpecularLightingAttrs::SpecularConstant, value.into());
	}

	/// Get the `specularConstant` attribute.
	pub fn specular_constant(&self) -> Option<&str> {
		self.get_attr(FeSpecularLightingAttrs::SpecularConstant)
	}

	/// Set the `specularExponent` attribute.
	pub fn with_specular_exponent<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeSpecularLightingAttrs::SpecularExponent, value.into());
		self
	}

	/// Set the `specularExponent` attribute.
	pub fn set_specular_exponent<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeSpecularLightingAttrs::SpecularExponent, value.into());
	}

	/// Get the `specularExponent` attribute.
	pub fn specular_exponent(&self) -> Option<&str> {
		self.get_attr(FeSpecularLightingAttrs::SpecularExponent)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeSpecularLightingAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeSpecularLightingAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(FeSpecularLightingAttrs::Style)
	}

	/// Set the `surfaceScale` attribute.
	pub fn with_surface_scale<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeSpecularLightingAttrs::SurfaceScale, value.into());
		self
	}

	/// Set the `surfaceScale` attribute.
	pub fn set_surface_scale<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeSpecularLightingAttrs::SurfaceScale, value.into());
	}

	/// Get the `surfaceScale` attribute.
	pub fn surface_scale(&self) -> Option<&str> {
		self.get_attr(FeSpecularLightingAttrs::SurfaceScale)
	}
}

impl common_attrs::CoreAttributesSetter for FeSpecularLighting {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FeSpecularLightingAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FeSpecularLightingAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeSpecularLighting {}

impl common_attrs::FilterAttributesSetter for FeSpecularLighting {
	fn set_attr(&mut self, attr: common_attrs::FilterAttributes, value: String) {
		self.set_attr(FeSpecularLightingAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&str> {
		self.get_attr(FeSpecularLightingAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeSpecularLighting {}

impl common_attrs::PresentationAttributesSetter for FeSpecularLighting {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(FeSpecularLightingAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(FeSpecularLightingAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeSpecularLighting {}

impl Tag for FeSpecularLighting {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("feSpecularLighting");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feSpotLight>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feSpotLight.md")]
#[doc = "\n\n [`<feSpotLight>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feSpotLight"]
#[derive(Debug)]
pub struct FeSpotLight {
	attrs: IndexMap<FeSpotLightAttrs, String>,
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

	fn set_attr(&mut self, attr: FeSpotLightAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FeSpotLightAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `limitingConeAngle` attribute.
	pub fn with_limiting_cone_angle<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeSpotLightAttrs::LimitingConeAngle, value.into());
		self
	}

	/// Set the `limitingConeAngle` attribute.
	pub fn set_limiting_cone_angle<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeSpotLightAttrs::LimitingConeAngle, value.into());
	}

	/// Get the `limitingConeAngle` attribute.
	pub fn limiting_cone_angle(&self) -> Option<&str> {
		self.get_attr(FeSpotLightAttrs::LimitingConeAngle)
	}

	/// Set the `pointsAtX` attribute.
	pub fn with_points_at_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeSpotLightAttrs::PointsAtX, value.into());
		self
	}

	/// Set the `pointsAtX` attribute.
	pub fn set_points_at_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeSpotLightAttrs::PointsAtX, value.into());
	}

	/// Get the `pointsAtX` attribute.
	pub fn points_at_x(&self) -> Option<&str> {
		self.get_attr(FeSpotLightAttrs::PointsAtX)
	}

	/// Set the `pointsAtY` attribute.
	pub fn with_points_at_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeSpotLightAttrs::PointsAtY, value.into());
		self
	}

	/// Set the `pointsAtY` attribute.
	pub fn set_points_at_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeSpotLightAttrs::PointsAtY, value.into());
	}

	/// Get the `pointsAtY` attribute.
	pub fn points_at_y(&self) -> Option<&str> {
		self.get_attr(FeSpotLightAttrs::PointsAtY)
	}

	/// Set the `pointsAtZ` attribute.
	pub fn with_points_at_z<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeSpotLightAttrs::PointsAtZ, value.into());
		self
	}

	/// Set the `pointsAtZ` attribute.
	pub fn set_points_at_z<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeSpotLightAttrs::PointsAtZ, value.into());
	}

	/// Get the `pointsAtZ` attribute.
	pub fn points_at_z(&self) -> Option<&str> {
		self.get_attr(FeSpotLightAttrs::PointsAtZ)
	}

	/// Set the `specularExponent` attribute.
	pub fn with_specular_exponent<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeSpotLightAttrs::SpecularExponent, value.into());
		self
	}

	/// Set the `specularExponent` attribute.
	pub fn set_specular_exponent<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeSpotLightAttrs::SpecularExponent, value.into());
	}

	/// Get the `specularExponent` attribute.
	pub fn specular_exponent(&self) -> Option<&str> {
		self.get_attr(FeSpotLightAttrs::SpecularExponent)
	}

	/// Set the `x` attribute.
	pub fn with_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeSpotLightAttrs::X, value.into());
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeSpotLightAttrs::X, value.into());
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&str> {
		self.get_attr(FeSpotLightAttrs::X)
	}

	/// Set the `y` attribute.
	pub fn with_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeSpotLightAttrs::Y, value.into());
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeSpotLightAttrs::Y, value.into());
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&str> {
		self.get_attr(FeSpotLightAttrs::Y)
	}

	/// Set the `z` attribute.
	pub fn with_z<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeSpotLightAttrs::Z, value.into());
		self
	}

	/// Set the `z` attribute.
	pub fn set_z<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeSpotLightAttrs::Z, value.into());
	}

	/// Get the `z` attribute.
	pub fn z(&self) -> Option<&str> {
		self.get_attr(FeSpotLightAttrs::Z)
	}
}

impl common_attrs::CoreAttributesSetter for FeSpotLight {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FeSpotLightAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FeSpotLightAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeSpotLight {}

impl Tag for FeSpotLight {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("feSpotLight");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feTile>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feTile.md")]
#[doc = "\n\n [`<feTile>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feTile"]
#[derive(Debug)]
pub struct FeTile {
	attrs: IndexMap<FeTileAttrs, String>,
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

	fn set_attr(&mut self, attr: FeTileAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FeTileAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeTileAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeTileAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(FeTileAttrs::Class)
	}

	/// Set the `in` attribute.
	pub fn with_in1<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeTileAttrs::In, value.into());
		self
	}

	/// Set the `in` attribute.
	pub fn set_in1<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeTileAttrs::In, value.into());
	}

	/// Get the `in` attribute.
	pub fn in1(&self) -> Option<&str> {
		self.get_attr(FeTileAttrs::In)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeTileAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeTileAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(FeTileAttrs::Style)
	}
}

impl common_attrs::CoreAttributesSetter for FeTile {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FeTileAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FeTileAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeTile {}

impl common_attrs::FilterAttributesSetter for FeTile {
	fn set_attr(&mut self, attr: common_attrs::FilterAttributes, value: String) {
		self.set_attr(FeTileAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&str> {
		self.get_attr(FeTileAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeTile {}

impl common_attrs::PresentationAttributesSetter for FeTile {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(FeTileAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(FeTileAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeTile {}

impl Tag for FeTile {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("feTile");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<feTurbulence>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feTurbulence.md")]
#[doc = "\n\n [`<feTurbulence>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feTurbulence"]
#[derive(Debug)]
pub struct FeTurbulence {
	attrs: IndexMap<FeTurbulenceAttrs, String>,
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

	fn set_attr(&mut self, attr: FeTurbulenceAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FeTurbulenceAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `baseFrequency` attribute.
	pub fn with_base_frequency<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeTurbulenceAttrs::BaseFrequency, value.into());
		self
	}

	/// Set the `baseFrequency` attribute.
	pub fn set_base_frequency<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeTurbulenceAttrs::BaseFrequency, value.into());
	}

	/// Get the `baseFrequency` attribute.
	pub fn base_frequency(&self) -> Option<&str> {
		self.get_attr(FeTurbulenceAttrs::BaseFrequency)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeTurbulenceAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeTurbulenceAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(FeTurbulenceAttrs::Class)
	}

	/// Set the `numOctaves` attribute.
	pub fn with_num_octaves<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeTurbulenceAttrs::NumOctaves, value.into());
		self
	}

	/// Set the `numOctaves` attribute.
	pub fn set_num_octaves<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeTurbulenceAttrs::NumOctaves, value.into());
	}

	/// Get the `numOctaves` attribute.
	pub fn num_octaves(&self) -> Option<&str> {
		self.get_attr(FeTurbulenceAttrs::NumOctaves)
	}

	/// Set the `seed` attribute.
	pub fn with_seed<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeTurbulenceAttrs::Seed, value.into());
		self
	}

	/// Set the `seed` attribute.
	pub fn set_seed<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeTurbulenceAttrs::Seed, value.into());
	}

	/// Get the `seed` attribute.
	pub fn seed(&self) -> Option<&str> {
		self.get_attr(FeTurbulenceAttrs::Seed)
	}

	/// Set the `stitchTiles` attribute.
	pub fn with_stitch_tiles<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeTurbulenceAttrs::StitchTiles, value.into());
		self
	}

	/// Set the `stitchTiles` attribute.
	pub fn set_stitch_tiles<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeTurbulenceAttrs::StitchTiles, value.into());
	}

	/// Get the `stitchTiles` attribute.
	pub fn stitch_tiles(&self) -> Option<&str> {
		self.get_attr(FeTurbulenceAttrs::StitchTiles)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeTurbulenceAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeTurbulenceAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(FeTurbulenceAttrs::Style)
	}

	/// Set the `type` attribute.
	pub fn with_ty<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeTurbulenceAttrs::Type, value.into());
		self
	}

	/// Set the `type` attribute.
	pub fn set_ty<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeTurbulenceAttrs::Type, value.into());
	}

	/// Get the `type` attribute.
	pub fn ty(&self) -> Option<&str> {
		self.get_attr(FeTurbulenceAttrs::Type)
	}
}

impl common_attrs::CoreAttributesSetter for FeTurbulence {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FeTurbulenceAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FeTurbulenceAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FeTurbulence {}

impl common_attrs::FilterAttributesSetter for FeTurbulence {
	fn set_attr(&mut self, attr: common_attrs::FilterAttributes, value: String) {
		self.set_attr(FeTurbulenceAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::FilterAttributes) -> Option<&str> {
		self.get_attr(FeTurbulenceAttrs::from(attr))
	}
}

impl TagWithFilterAttributes for FeTurbulence {}

impl common_attrs::PresentationAttributesSetter for FeTurbulence {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(FeTurbulenceAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(FeTurbulenceAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for FeTurbulence {}

impl Tag for FeTurbulence {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("feTurbulence");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::Set {}
}

#[doc = "The [`<filter>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("filter.md")]
#[doc = "\n\n [`<filter>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/filter"]
#[derive(Debug)]
pub struct Filter {
	attrs: IndexMap<FilterAttrs, String>,
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

	fn set_attr(&mut self, attr: FilterAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FilterAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FilterAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FilterAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(FilterAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FilterAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FilterAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(FilterAttrs::ExternalResourcesRequired)
	}

	/// Set the `filterRes` attribute.
	pub fn with_filter_res<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FilterAttrs::FilterRes, value.into());
		self
	}

	/// Set the `filterRes` attribute.
	pub fn set_filter_res<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FilterAttrs::FilterRes, value.into());
	}

	/// Get the `filterRes` attribute.
	pub fn filter_res(&self) -> Option<&str> {
		self.get_attr(FilterAttrs::FilterRes)
	}

	/// Set the `filterUnits` attribute.
	pub fn with_filter_units<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FilterAttrs::FilterUnits, value.into());
		self
	}

	/// Set the `filterUnits` attribute.
	pub fn set_filter_units<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FilterAttrs::FilterUnits, value.into());
	}

	/// Get the `filterUnits` attribute.
	pub fn filter_units(&self) -> Option<&str> {
		self.get_attr(FilterAttrs::FilterUnits)
	}

	/// Set the `height` attribute.
	pub fn with_height<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FilterAttrs::Height, value.into());
		self
	}

	/// Set the `height` attribute.
	pub fn set_height<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FilterAttrs::Height, value.into());
	}

	/// Get the `height` attribute.
	pub fn height(&self) -> Option<&str> {
		self.get_attr(FilterAttrs::Height)
	}

	/// Set the `primitiveUnits` attribute.
	pub fn with_primitive_units<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FilterAttrs::PrimitiveUnits, value.into());
		self
	}

	/// Set the `primitiveUnits` attribute.
	pub fn set_primitive_units<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FilterAttrs::PrimitiveUnits, value.into());
	}

	/// Get the `primitiveUnits` attribute.
	pub fn primitive_units(&self) -> Option<&str> {
		self.get_attr(FilterAttrs::PrimitiveUnits)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FilterAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FilterAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(FilterAttrs::Style)
	}

	/// Set the `width` attribute.
	pub fn with_width<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FilterAttrs::Width, value.into());
		self
	}

	/// Set the `width` attribute.
	pub fn set_width<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FilterAttrs::Width, value.into());
	}

	/// Get the `width` attribute.
	pub fn width(&self) -> Option<&str> {
		self.get_attr(FilterAttrs::Width)
	}

	/// Set the `x` attribute.
	pub fn with_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FilterAttrs::X, value.into());
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FilterAttrs::X, value.into());
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&str> {
		self.get_attr(FilterAttrs::X)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FilterAttrs::XlinkHref, value.into());
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FilterAttrs::XlinkHref, value.into());
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&str> {
		self.get_attr(FilterAttrs::XlinkHref)
	}

	/// Set the `y` attribute.
	pub fn with_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FilterAttrs::Y, value.into());
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FilterAttrs::Y, value.into());
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&str> {
		self.get_attr(FilterAttrs::Y)
	}
}

impl common_attrs::CoreAttributesSetter for Filter {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FilterAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FilterAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Filter {}

impl common_attrs::PresentationAttributesSetter for Filter {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(FilterAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(FilterAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Filter {}

impl common_attrs::XLinkAttributesSetter for Filter {
	fn set_attr(&mut self, attr: common_attrs::XLinkAttributes, value: String) {
		self.set_attr(FilterAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&str> {
		self.get_attr(FilterAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for Filter {}

impl Tag for Filter {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("filter");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::FontFace {}
	impl Content for super::Glyph {}
	impl Content for super::Hkern {}
	impl Content for super::MissingGlyph {}
	impl Content for super::Vkern {}
}

#[doc = "The [`<font>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("font.md")]
#[doc = "\n\n [`<font>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/font"]
#[derive(Debug)]
pub struct Font {
	attrs: IndexMap<FontAttrs, String>,
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

	fn set_attr(&mut self, attr: FontAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FontAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(FontAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(FontAttrs::ExternalResourcesRequired)
	}

	/// Set the `horiz-adv-x` attribute.
	pub fn with_horiz_adv_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::HorizAdvX, value.into());
		self
	}

	/// Set the `horiz-adv-x` attribute.
	pub fn set_horiz_adv_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::HorizAdvX, value.into());
	}

	/// Get the `horiz-adv-x` attribute.
	pub fn horiz_adv_x(&self) -> Option<&str> {
		self.get_attr(FontAttrs::HorizAdvX)
	}

	/// Set the `horiz-origin-x` attribute.
	pub fn with_horiz_origin_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::HorizOriginX, value.into());
		self
	}

	/// Set the `horiz-origin-x` attribute.
	pub fn set_horiz_origin_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::HorizOriginX, value.into());
	}

	/// Get the `horiz-origin-x` attribute.
	pub fn horiz_origin_x(&self) -> Option<&str> {
		self.get_attr(FontAttrs::HorizOriginX)
	}

	/// Set the `horiz-origin-y` attribute.
	pub fn with_horiz_origin_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::HorizOriginY, value.into());
		self
	}

	/// Set the `horiz-origin-y` attribute.
	pub fn set_horiz_origin_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::HorizOriginY, value.into());
	}

	/// Get the `horiz-origin-y` attribute.
	pub fn horiz_origin_y(&self) -> Option<&str> {
		self.get_attr(FontAttrs::HorizOriginY)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(FontAttrs::Style)
	}

	/// Set the `vert-adv-y` attribute.
	pub fn with_vert_adv_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::VertAdvY, value.into());
		self
	}

	/// Set the `vert-adv-y` attribute.
	pub fn set_vert_adv_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::VertAdvY, value.into());
	}

	/// Get the `vert-adv-y` attribute.
	pub fn vert_adv_y(&self) -> Option<&str> {
		self.get_attr(FontAttrs::VertAdvY)
	}

	/// Set the `vert-origin-x` attribute.
	pub fn with_vert_origin_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::VertOriginX, value.into());
		self
	}

	/// Set the `vert-origin-x` attribute.
	pub fn set_vert_origin_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::VertOriginX, value.into());
	}

	/// Get the `vert-origin-x` attribute.
	pub fn vert_origin_x(&self) -> Option<&str> {
		self.get_attr(FontAttrs::VertOriginX)
	}

	/// Set the `vert-origin-y` attribute.
	pub fn with_vert_origin_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::VertOriginY, value.into());
		self
	}

	/// Set the `vert-origin-y` attribute.
	pub fn set_vert_origin_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::VertOriginY, value.into());
	}

	/// Get the `vert-origin-y` attribute.
	pub fn vert_origin_y(&self) -> Option<&str> {
		self.get_attr(FontAttrs::VertOriginY)
	}
}

impl common_attrs::CoreAttributesSetter for Font {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FontAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FontAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Font {}

impl common_attrs::PresentationAttributesSetter for Font {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(FontAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(FontAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Font {}

impl Tag for Font {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("font");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
#[doc = "\n\n [`<font-face>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/font-face"]
#[derive(Debug)]
pub struct FontFace {
	attrs: IndexMap<FontFaceAttrs, String>,
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

	fn set_attr(&mut self, attr: FontFaceAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FontFaceAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `accent-height` attribute.
	pub fn with_accent_height<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::AccentHeight, value.into());
		self
	}

	/// Set the `accent-height` attribute.
	pub fn set_accent_height<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::AccentHeight, value.into());
	}

	/// Get the `accent-height` attribute.
	pub fn accent_height(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::AccentHeight)
	}

	/// Set the `alphabetic` attribute.
	pub fn with_alphabetic<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::Alphabetic, value.into());
		self
	}

	/// Set the `alphabetic` attribute.
	pub fn set_alphabetic<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::Alphabetic, value.into());
	}

	/// Get the `alphabetic` attribute.
	pub fn alphabetic(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::Alphabetic)
	}

	/// Set the `ascent` attribute.
	pub fn with_ascent<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::Ascent, value.into());
		self
	}

	/// Set the `ascent` attribute.
	pub fn set_ascent<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::Ascent, value.into());
	}

	/// Get the `ascent` attribute.
	pub fn ascent(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::Ascent)
	}

	/// Set the `bbox` attribute.
	pub fn with_bbox<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::Bbox, value.into());
		self
	}

	/// Set the `bbox` attribute.
	pub fn set_bbox<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::Bbox, value.into());
	}

	/// Get the `bbox` attribute.
	pub fn bbox(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::Bbox)
	}

	/// Set the `cap-height` attribute.
	pub fn with_cap_height<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::CapHeight, value.into());
		self
	}

	/// Set the `cap-height` attribute.
	pub fn set_cap_height<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::CapHeight, value.into());
	}

	/// Get the `cap-height` attribute.
	pub fn cap_height(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::CapHeight)
	}

	/// Set the `descent` attribute.
	pub fn with_descent<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::Descent, value.into());
		self
	}

	/// Set the `descent` attribute.
	pub fn set_descent<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::Descent, value.into());
	}

	/// Get the `descent` attribute.
	pub fn descent(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::Descent)
	}

	/// Set the `font-family` attribute.
	pub fn with_font_family<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::FontFamily, value.into());
		self
	}

	/// Set the `font-family` attribute.
	pub fn set_font_family<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::FontFamily, value.into());
	}

	/// Get the `font-family` attribute.
	pub fn font_family(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::FontFamily)
	}

	/// Set the `font-size` attribute.
	pub fn with_font_size<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::FontSize, value.into());
		self
	}

	/// Set the `font-size` attribute.
	pub fn set_font_size<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::FontSize, value.into());
	}

	/// Get the `font-size` attribute.
	pub fn font_size(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::FontSize)
	}

	/// Set the `font-stretch` attribute.
	pub fn with_font_stretch<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::FontStretch, value.into());
		self
	}

	/// Set the `font-stretch` attribute.
	pub fn set_font_stretch<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::FontStretch, value.into());
	}

	/// Get the `font-stretch` attribute.
	pub fn font_stretch(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::FontStretch)
	}

	/// Set the `font-style` attribute.
	pub fn with_font_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::FontStyle, value.into());
		self
	}

	/// Set the `font-style` attribute.
	pub fn set_font_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::FontStyle, value.into());
	}

	/// Get the `font-style` attribute.
	pub fn font_style(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::FontStyle)
	}

	/// Set the `font-variant` attribute.
	pub fn with_font_variant<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::FontVariant, value.into());
		self
	}

	/// Set the `font-variant` attribute.
	pub fn set_font_variant<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::FontVariant, value.into());
	}

	/// Get the `font-variant` attribute.
	pub fn font_variant(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::FontVariant)
	}

	/// Set the `font-weight` attribute.
	pub fn with_font_weight<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::FontWeight, value.into());
		self
	}

	/// Set the `font-weight` attribute.
	pub fn set_font_weight<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::FontWeight, value.into());
	}

	/// Get the `font-weight` attribute.
	pub fn font_weight(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::FontWeight)
	}

	/// Set the `hanging` attribute.
	pub fn with_hanging<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::Hanging, value.into());
		self
	}

	/// Set the `hanging` attribute.
	pub fn set_hanging<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::Hanging, value.into());
	}

	/// Get the `hanging` attribute.
	pub fn hanging(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::Hanging)
	}

	/// Set the `ideographic` attribute.
	pub fn with_ideographic<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::Ideographic, value.into());
		self
	}

	/// Set the `ideographic` attribute.
	pub fn set_ideographic<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::Ideographic, value.into());
	}

	/// Get the `ideographic` attribute.
	pub fn ideographic(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::Ideographic)
	}

	/// Set the `mathematical` attribute.
	pub fn with_mathematical<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::Mathematical, value.into());
		self
	}

	/// Set the `mathematical` attribute.
	pub fn set_mathematical<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::Mathematical, value.into());
	}

	/// Get the `mathematical` attribute.
	pub fn mathematical(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::Mathematical)
	}

	/// Set the `overline-position` attribute.
	pub fn with_overline_position<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::OverlinePosition, value.into());
		self
	}

	/// Set the `overline-position` attribute.
	pub fn set_overline_position<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::OverlinePosition, value.into());
	}

	/// Get the `overline-position` attribute.
	pub fn overline_position(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::OverlinePosition)
	}

	/// Set the `overline-thickness` attribute.
	pub fn with_overline_thickness<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::OverlineThickness, value.into());
		self
	}

	/// Set the `overline-thickness` attribute.
	pub fn set_overline_thickness<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::OverlineThickness, value.into());
	}

	/// Get the `overline-thickness` attribute.
	pub fn overline_thickness(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::OverlineThickness)
	}

	/// Set the `panose-1` attribute.
	pub fn with_panose_1<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::Panose1, value.into());
		self
	}

	/// Set the `panose-1` attribute.
	pub fn set_panose_1<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::Panose1, value.into());
	}

	/// Get the `panose-1` attribute.
	pub fn panose_1(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::Panose1)
	}

	/// Set the `slope` attribute.
	pub fn with_slope<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::Slope, value.into());
		self
	}

	/// Set the `slope` attribute.
	pub fn set_slope<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::Slope, value.into());
	}

	/// Get the `slope` attribute.
	pub fn slope(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::Slope)
	}

	/// Set the `stemh` attribute.
	pub fn with_stemh<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::Stemh, value.into());
		self
	}

	/// Set the `stemh` attribute.
	pub fn set_stemh<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::Stemh, value.into());
	}

	/// Get the `stemh` attribute.
	pub fn stemh(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::Stemh)
	}

	/// Set the `stemv` attribute.
	pub fn with_stemv<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::Stemv, value.into());
		self
	}

	/// Set the `stemv` attribute.
	pub fn set_stemv<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::Stemv, value.into());
	}

	/// Get the `stemv` attribute.
	pub fn stemv(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::Stemv)
	}

	/// Set the `strikethrough-position` attribute.
	pub fn with_strikethrough_position<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::StrikethroughPosition, value.into());
		self
	}

	/// Set the `strikethrough-position` attribute.
	pub fn set_strikethrough_position<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::StrikethroughPosition, value.into());
	}

	/// Get the `strikethrough-position` attribute.
	pub fn strikethrough_position(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::StrikethroughPosition)
	}

	/// Set the `strikethrough-thickness` attribute.
	pub fn with_strikethrough_thickness<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::StrikethroughThickness, value.into());
		self
	}

	/// Set the `strikethrough-thickness` attribute.
	pub fn set_strikethrough_thickness<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::StrikethroughThickness, value.into());
	}

	/// Get the `strikethrough-thickness` attribute.
	pub fn strikethrough_thickness(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::StrikethroughThickness)
	}

	/// Set the `underline-position` attribute.
	pub fn with_underline_position<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::UnderlinePosition, value.into());
		self
	}

	/// Set the `underline-position` attribute.
	pub fn set_underline_position<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::UnderlinePosition, value.into());
	}

	/// Get the `underline-position` attribute.
	pub fn underline_position(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::UnderlinePosition)
	}

	/// Set the `underline-thickness` attribute.
	pub fn with_underline_thickness<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::UnderlineThickness, value.into());
		self
	}

	/// Set the `underline-thickness` attribute.
	pub fn set_underline_thickness<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::UnderlineThickness, value.into());
	}

	/// Get the `underline-thickness` attribute.
	pub fn underline_thickness(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::UnderlineThickness)
	}

	/// Set the `unicode-range` attribute.
	pub fn with_unicode_range<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::UnicodeRange, value.into());
		self
	}

	/// Set the `unicode-range` attribute.
	pub fn set_unicode_range<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::UnicodeRange, value.into());
	}

	/// Get the `unicode-range` attribute.
	pub fn unicode_range(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::UnicodeRange)
	}

	/// Set the `units-per-em` attribute.
	pub fn with_units_per_em<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::UnitsPerEm, value.into());
		self
	}

	/// Set the `units-per-em` attribute.
	pub fn set_units_per_em<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::UnitsPerEm, value.into());
	}

	/// Get the `units-per-em` attribute.
	pub fn units_per_em(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::UnitsPerEm)
	}

	/// Set the `v-alphabetic` attribute.
	pub fn with_v_alphabetic<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::VAlphabetic, value.into());
		self
	}

	/// Set the `v-alphabetic` attribute.
	pub fn set_v_alphabetic<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::VAlphabetic, value.into());
	}

	/// Get the `v-alphabetic` attribute.
	pub fn v_alphabetic(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::VAlphabetic)
	}

	/// Set the `v-hanging` attribute.
	pub fn with_v_hanging<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::VHanging, value.into());
		self
	}

	/// Set the `v-hanging` attribute.
	pub fn set_v_hanging<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::VHanging, value.into());
	}

	/// Get the `v-hanging` attribute.
	pub fn v_hanging(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::VHanging)
	}

	/// Set the `v-ideographic` attribute.
	pub fn with_v_ideographic<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::VIdeographic, value.into());
		self
	}

	/// Set the `v-ideographic` attribute.
	pub fn set_v_ideographic<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::VIdeographic, value.into());
	}

	/// Get the `v-ideographic` attribute.
	pub fn v_ideographic(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::VIdeographic)
	}

	/// Set the `v-mathematical` attribute.
	pub fn with_v_mathematical<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::VMathematical, value.into());
		self
	}

	/// Set the `v-mathematical` attribute.
	pub fn set_v_mathematical<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::VMathematical, value.into());
	}

	/// Get the `v-mathematical` attribute.
	pub fn v_mathematical(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::VMathematical)
	}

	/// Set the `widths` attribute.
	pub fn with_widths<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::Widths, value.into());
		self
	}

	/// Set the `widths` attribute.
	pub fn set_widths<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::Widths, value.into());
	}

	/// Get the `widths` attribute.
	pub fn widths(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::Widths)
	}

	/// Set the `x-height` attribute.
	pub fn with_x_height<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::XHeight, value.into());
		self
	}

	/// Set the `x-height` attribute.
	pub fn set_x_height<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceAttrs::XHeight, value.into());
	}

	/// Get the `x-height` attribute.
	pub fn x_height(&self) -> Option<&str> {
		self.get_attr(FontFaceAttrs::XHeight)
	}
}

impl common_attrs::CoreAttributesSetter for FontFace {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FontFaceAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FontFaceAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FontFace {}

impl Tag for FontFace {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("font-face");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

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
#[doc = "\n\n [`<font-face-format>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/font-face-format"]
#[derive(Debug)]
pub struct FontFaceFormat {
	attrs: IndexMap<FontFaceFormatAttrs, String>,
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

	fn set_attr(&mut self, attr: FontFaceFormatAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FontFaceFormatAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `string` attribute.
	pub fn with_string<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceFormatAttrs::String, value.into());
		self
	}

	/// Set the `string` attribute.
	pub fn set_string<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceFormatAttrs::String, value.into());
	}

	/// Get the `string` attribute.
	pub fn string(&self) -> Option<&str> {
		self.get_attr(FontFaceFormatAttrs::String)
	}
}

impl common_attrs::CoreAttributesSetter for FontFaceFormat {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FontFaceFormatAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FontFaceFormatAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FontFaceFormat {}

impl Tag for FontFaceFormat {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("font-face-format");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

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
#[doc = "\n\n [`<font-face-name>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/font-face-name"]
#[derive(Debug)]
pub struct FontFaceName {
	attrs: IndexMap<FontFaceNameAttrs, String>,
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

	fn set_attr(&mut self, attr: FontFaceNameAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FontFaceNameAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `name` attribute.
	pub fn with_name<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceNameAttrs::Name, value.into());
		self
	}

	/// Set the `name` attribute.
	pub fn set_name<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceNameAttrs::Name, value.into());
	}

	/// Get the `name` attribute.
	pub fn name(&self) -> Option<&str> {
		self.get_attr(FontFaceNameAttrs::Name)
	}
}

impl common_attrs::CoreAttributesSetter for FontFaceName {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FontFaceNameAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FontFaceNameAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FontFaceName {}

impl Tag for FontFaceName {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("font-face-name");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::FontFaceName {}
	impl Content for super::FontFaceUri {}
}

#[doc = "The [`<font-face-src>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("font-face-src.md")]
#[doc = "\n\n [`<font-face-src>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/font-face-src"]
#[derive(Debug)]
pub struct FontFaceSrc {
	attrs: IndexMap<FontFaceSrcAttrs, String>,
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

	fn set_attr(&mut self, attr: FontFaceSrcAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FontFaceSrcAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}
}

impl common_attrs::CoreAttributesSetter for FontFaceSrc {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FontFaceSrcAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FontFaceSrcAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FontFaceSrc {}

impl Tag for FontFaceSrc {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("font-face-src");
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::FontFaceFormat {}
}

#[doc = "The [`<font-face-uri>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("font-face-uri.md")]
#[doc = "\n\n [`<font-face-uri>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/font-face-uri"]
#[derive(Debug)]
pub struct FontFaceUri {
	attrs: IndexMap<FontFaceUriAttrs, String>,
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

	fn set_attr(&mut self, attr: FontFaceUriAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: FontFaceUriAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontFaceUriAttrs::XlinkHref, value.into());
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontFaceUriAttrs::XlinkHref, value.into());
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&str> {
		self.get_attr(FontFaceUriAttrs::XlinkHref)
	}
}

impl common_attrs::CoreAttributesSetter for FontFaceUri {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(FontFaceUriAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(FontFaceUriAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for FontFaceUri {}

impl common_attrs::XLinkAttributesSetter for FontFaceUri {
	fn set_attr(&mut self, attr: common_attrs::XLinkAttributes, value: String) {
		self.set_attr(FontFaceUriAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&str> {
		self.get_attr(FontFaceUriAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for FontFaceUri {}

impl Tag for FontFaceUri {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("font-face-uri");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	attrs: IndexMap<ForeignObjectAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: ForeignObjectAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: ForeignObjectAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ForeignObjectAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ForeignObjectAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(ForeignObjectAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ForeignObjectAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ForeignObjectAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(ForeignObjectAttrs::ExternalResourcesRequired)
	}

	/// Set the `height` attribute.
	pub fn with_height<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ForeignObjectAttrs::Height, value.into());
		self
	}

	/// Set the `height` attribute.
	pub fn set_height<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ForeignObjectAttrs::Height, value.into());
	}

	/// Get the `height` attribute.
	pub fn height(&self) -> Option<&str> {
		self.get_attr(ForeignObjectAttrs::Height)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ForeignObjectAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ForeignObjectAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(ForeignObjectAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ForeignObjectAttrs::Transform, value.into());
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ForeignObjectAttrs::Transform, value.into());
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&str> {
		self.get_attr(ForeignObjectAttrs::Transform)
	}

	/// Set the `width` attribute.
	pub fn with_width<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ForeignObjectAttrs::Width, value.into());
		self
	}

	/// Set the `width` attribute.
	pub fn set_width<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ForeignObjectAttrs::Width, value.into());
	}

	/// Get the `width` attribute.
	pub fn width(&self) -> Option<&str> {
		self.get_attr(ForeignObjectAttrs::Width)
	}

	/// Set the `x` attribute.
	pub fn with_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ForeignObjectAttrs::X, value.into());
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ForeignObjectAttrs::X, value.into());
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&str> {
		self.get_attr(ForeignObjectAttrs::X)
	}

	/// Set the `y` attribute.
	pub fn with_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ForeignObjectAttrs::Y, value.into());
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ForeignObjectAttrs::Y, value.into());
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&str> {
		self.get_attr(ForeignObjectAttrs::Y)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for ForeignObject {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(ForeignObjectAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(ForeignObjectAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for ForeignObject {}

impl common_attrs::CoreAttributesSetter for ForeignObject {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(ForeignObjectAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(ForeignObjectAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for ForeignObject {}

impl common_attrs::GraphicalEventAttributesSetter for ForeignObject {
	fn set_attr(&mut self, attr: common_attrs::GraphicalEventAttributes, value: String) {
		self.set_attr(ForeignObjectAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&str> {
		self.get_attr(ForeignObjectAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for ForeignObject {}

impl common_attrs::PresentationAttributesSetter for ForeignObject {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(ForeignObjectAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(ForeignObjectAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for ForeignObject {}

impl Tag for ForeignObject {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("foreignObject");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum GAttrs {
	Class,
	ExternalResourcesRequired,
	Style,
	Transform,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
	GraphicalEventAttributes(common_attrs::GraphicalEventAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::ConditionalProcessingAttributes> for GAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for GAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::GraphicalEventAttributes> for GAttrs {
	fn from(attr: common_attrs::GraphicalEventAttributes) -> Self {
		Self::GraphicalEventAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for GAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl GAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Style => "style",
			Self::Transform => "transform",
			Self::ConditionalProcessingAttributes(attr) => attr.as_str(),
			Self::CoreAttributes(attr) => attr.as_str(),
			Self::GraphicalEventAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for GAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for GAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod g_private {
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::A {}
	impl Content for super::AltGlyphDef {}
	impl Content for super::ClipPath {}
	impl Content for super::ColorProfile {}
	impl Content for super::Cursor {}
	impl Content for super::Filter {}
	impl Content for super::Font {}
	impl Content for super::FontFace {}
	impl Content for super::ForeignObject {}
	impl Content for super::Image {}
	impl Content for super::Marker {}
	impl Content for super::Mask {}
	impl Content for super::Pattern {}
	impl Content for super::Script {}
	impl Content for super::Style {}
	impl Content for super::Switch {}
	impl Content for super::Text {}
	impl Content for super::View {}
}

#[doc = "The [`<g>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("g.md")]
#[doc = "\n\n [`<g>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/g"]
#[derive(Debug)]
pub struct G {
	attrs: IndexMap<GAttrs, String>,
	content: Vec<Box<dyn g_private::Content>>,
}

impl Default for G {
	fn default() -> Self {
		Self::new()
	}
}

impl G {
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

	fn set_attr(&mut self, attr: GAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: GAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(GAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(GAttrs::ExternalResourcesRequired)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(GAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GAttrs::Transform, value.into());
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GAttrs::Transform, value.into());
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&str> {
		self.get_attr(GAttrs::Transform)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for G {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(GAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(GAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for G {}

impl common_attrs::CoreAttributesSetter for G {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(GAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(GAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for G {}

impl common_attrs::GraphicalEventAttributesSetter for G {
	fn set_attr(&mut self, attr: common_attrs::GraphicalEventAttributes, value: String) {
		self.set_attr(GAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&str> {
		self.get_attr(GAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for G {}

impl common_attrs::PresentationAttributesSetter for G {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(GAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(GAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for G {}

impl Tag for G {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("g");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::A {}
	impl Content for super::AltGlyphDef {}
	impl Content for super::ClipPath {}
	impl Content for super::ColorProfile {}
	impl Content for super::Cursor {}
	impl Content for super::Filter {}
	impl Content for super::Font {}
	impl Content for super::FontFace {}
	impl Content for super::ForeignObject {}
	impl Content for super::Image {}
	impl Content for super::Marker {}
	impl Content for super::Mask {}
	impl Content for super::Pattern {}
	impl Content for super::Script {}
	impl Content for super::Style {}
	impl Content for super::Switch {}
	impl Content for super::Text {}
	impl Content for super::View {}
}

#[doc = "The [`<glyph>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("glyph.md")]
#[doc = "\n\n [`<glyph>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/glyph"]
#[derive(Debug)]
pub struct Glyph {
	attrs: IndexMap<GlyphAttrs, String>,
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

	fn set_attr(&mut self, attr: GlyphAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: GlyphAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `arabic-form` attribute.
	pub fn with_arabic_form<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::ArabicForm, value.into());
		self
	}

	/// Set the `arabic-form` attribute.
	pub fn set_arabic_form<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::ArabicForm, value.into());
	}

	/// Get the `arabic-form` attribute.
	pub fn arabic_form(&self) -> Option<&str> {
		self.get_attr(GlyphAttrs::ArabicForm)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(GlyphAttrs::Class)
	}

	/// Set the `d` attribute.
	pub fn with_d<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::D, value.into());
		self
	}

	/// Set the `d` attribute.
	pub fn set_d<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::D, value.into());
	}

	/// Get the `d` attribute.
	pub fn d(&self) -> Option<&str> {
		self.get_attr(GlyphAttrs::D)
	}

	/// Set the `glyph-name` attribute.
	pub fn with_glyph_name<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::GlyphName, value.into());
		self
	}

	/// Set the `glyph-name` attribute.
	pub fn set_glyph_name<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::GlyphName, value.into());
	}

	/// Get the `glyph-name` attribute.
	pub fn glyph_name(&self) -> Option<&str> {
		self.get_attr(GlyphAttrs::GlyphName)
	}

	/// Set the `horiz-adv-x` attribute.
	pub fn with_horiz_adv_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::HorizAdvX, value.into());
		self
	}

	/// Set the `horiz-adv-x` attribute.
	pub fn set_horiz_adv_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::HorizAdvX, value.into());
	}

	/// Get the `horiz-adv-x` attribute.
	pub fn horiz_adv_x(&self) -> Option<&str> {
		self.get_attr(GlyphAttrs::HorizAdvX)
	}

	/// Set the `lang` attribute.
	pub fn with_lang<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::Lang, value.into());
		self
	}

	/// Set the `lang` attribute.
	pub fn set_lang<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::Lang, value.into());
	}

	/// Get the `lang` attribute.
	pub fn lang(&self) -> Option<&str> {
		self.get_attr(GlyphAttrs::Lang)
	}

	/// Set the `orientation` attribute.
	pub fn with_orientation<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::Orientation, value.into());
		self
	}

	/// Set the `orientation` attribute.
	pub fn set_orientation<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::Orientation, value.into());
	}

	/// Get the `orientation` attribute.
	pub fn orientation(&self) -> Option<&str> {
		self.get_attr(GlyphAttrs::Orientation)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(GlyphAttrs::Style)
	}

	/// Set the `unicode` attribute.
	pub fn with_unicode<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::Unicode, value.into());
		self
	}

	/// Set the `unicode` attribute.
	pub fn set_unicode<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::Unicode, value.into());
	}

	/// Get the `unicode` attribute.
	pub fn unicode(&self) -> Option<&str> {
		self.get_attr(GlyphAttrs::Unicode)
	}

	/// Set the `vert-adv-y` attribute.
	pub fn with_vert_adv_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::VertAdvY, value.into());
		self
	}

	/// Set the `vert-adv-y` attribute.
	pub fn set_vert_adv_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::VertAdvY, value.into());
	}

	/// Get the `vert-adv-y` attribute.
	pub fn vert_adv_y(&self) -> Option<&str> {
		self.get_attr(GlyphAttrs::VertAdvY)
	}

	/// Set the `vert-origin-x` attribute.
	pub fn with_vert_origin_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::VertOriginX, value.into());
		self
	}

	/// Set the `vert-origin-x` attribute.
	pub fn set_vert_origin_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::VertOriginX, value.into());
	}

	/// Get the `vert-origin-x` attribute.
	pub fn vert_origin_x(&self) -> Option<&str> {
		self.get_attr(GlyphAttrs::VertOriginX)
	}

	/// Set the `vert-origin-y` attribute.
	pub fn with_vert_origin_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::VertOriginY, value.into());
		self
	}

	/// Set the `vert-origin-y` attribute.
	pub fn set_vert_origin_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::VertOriginY, value.into());
	}

	/// Get the `vert-origin-y` attribute.
	pub fn vert_origin_y(&self) -> Option<&str> {
		self.get_attr(GlyphAttrs::VertOriginY)
	}
}

impl common_attrs::CoreAttributesSetter for Glyph {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(GlyphAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(GlyphAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Glyph {}

impl common_attrs::PresentationAttributesSetter for Glyph {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(GlyphAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(GlyphAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Glyph {}

impl Tag for Glyph {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("glyph");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
#[doc = "\n\n [`<glyphRef>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/glyphRef"]
#[derive(Debug)]
pub struct GlyphRef {
	attrs: IndexMap<GlyphRefAttrs, String>,
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

	fn set_attr(&mut self, attr: GlyphRefAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: GlyphRefAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlyphRefAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlyphRefAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(GlyphRefAttrs::Class)
	}

	/// Set the `dx` attribute.
	pub fn with_dx<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlyphRefAttrs::Dx, value.into());
		self
	}

	/// Set the `dx` attribute.
	pub fn set_dx<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlyphRefAttrs::Dx, value.into());
	}

	/// Get the `dx` attribute.
	pub fn dx(&self) -> Option<&str> {
		self.get_attr(GlyphRefAttrs::Dx)
	}

	/// Set the `dy` attribute.
	pub fn with_dy<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlyphRefAttrs::Dy, value.into());
		self
	}

	/// Set the `dy` attribute.
	pub fn set_dy<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlyphRefAttrs::Dy, value.into());
	}

	/// Get the `dy` attribute.
	pub fn dy(&self) -> Option<&str> {
		self.get_attr(GlyphRefAttrs::Dy)
	}

	/// Set the `format` attribute.
	pub fn with_format<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlyphRefAttrs::Format, value.into());
		self
	}

	/// Set the `format` attribute.
	pub fn set_format<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlyphRefAttrs::Format, value.into());
	}

	/// Get the `format` attribute.
	pub fn format(&self) -> Option<&str> {
		self.get_attr(GlyphRefAttrs::Format)
	}

	/// Set the `glyphRef` attribute.
	pub fn with_glyph_ref<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlyphRefAttrs::GlyphRef, value.into());
		self
	}

	/// Set the `glyphRef` attribute.
	pub fn set_glyph_ref<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlyphRefAttrs::GlyphRef, value.into());
	}

	/// Get the `glyphRef` attribute.
	pub fn glyph_ref(&self) -> Option<&str> {
		self.get_attr(GlyphRefAttrs::GlyphRef)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlyphRefAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlyphRefAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(GlyphRefAttrs::Style)
	}

	/// Set the `x` attribute.
	pub fn with_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlyphRefAttrs::X, value.into());
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlyphRefAttrs::X, value.into());
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&str> {
		self.get_attr(GlyphRefAttrs::X)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlyphRefAttrs::XlinkHref, value.into());
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlyphRefAttrs::XlinkHref, value.into());
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&str> {
		self.get_attr(GlyphRefAttrs::XlinkHref)
	}

	/// Set the `y` attribute.
	pub fn with_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlyphRefAttrs::Y, value.into());
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlyphRefAttrs::Y, value.into());
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&str> {
		self.get_attr(GlyphRefAttrs::Y)
	}
}

impl common_attrs::CoreAttributesSetter for GlyphRef {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(GlyphRefAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(GlyphRefAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for GlyphRef {}

impl common_attrs::PresentationAttributesSetter for GlyphRef {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(GlyphRefAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(GlyphRefAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for GlyphRef {}

impl common_attrs::XLinkAttributesSetter for GlyphRef {
	fn set_attr(&mut self, attr: common_attrs::XLinkAttributes, value: String) {
		self.set_attr(GlyphRefAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&str> {
		self.get_attr(GlyphRefAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for GlyphRef {}

impl Tag for GlyphRef {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("glyphRef");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Hatchpath {}
	impl Content for super::Script {}
	impl Content for super::Style {}
}

#[doc = "The [`<hatch>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("hatch.md")]
#[doc = "\n\n [`<hatch>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/hatch"]
#[derive(Debug)]
pub struct Hatch {
	attrs: IndexMap<HatchAttrs, String>,
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

	fn set_attr(&mut self, attr: HatchAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: HatchAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `hatchContentUnits` attribute.
	pub fn with_hatch_content_units<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(HatchAttrs::HatchContentUnits, value.into());
		self
	}

	/// Set the `hatchContentUnits` attribute.
	pub fn set_hatch_content_units<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(HatchAttrs::HatchContentUnits, value.into());
	}

	/// Get the `hatchContentUnits` attribute.
	pub fn hatch_content_units(&self) -> Option<&str> {
		self.get_attr(HatchAttrs::HatchContentUnits)
	}

	/// Set the `hatchUnits` attribute.
	pub fn with_hatch_units<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(HatchAttrs::HatchUnits, value.into());
		self
	}

	/// Set the `hatchUnits` attribute.
	pub fn set_hatch_units<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(HatchAttrs::HatchUnits, value.into());
	}

	/// Get the `hatchUnits` attribute.
	pub fn hatch_units(&self) -> Option<&str> {
		self.get_attr(HatchAttrs::HatchUnits)
	}

	/// Set the `href` attribute.
	pub fn with_href<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(HatchAttrs::Href, value.into());
		self
	}

	/// Set the `href` attribute.
	pub fn set_href<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(HatchAttrs::Href, value.into());
	}

	/// Get the `href` attribute.
	pub fn href(&self) -> Option<&str> {
		self.get_attr(HatchAttrs::Href)
	}

	/// Set the `pitch` attribute.
	pub fn with_pitch<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(HatchAttrs::Pitch, value.into());
		self
	}

	/// Set the `pitch` attribute.
	pub fn set_pitch<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(HatchAttrs::Pitch, value.into());
	}

	/// Get the `pitch` attribute.
	pub fn pitch(&self) -> Option<&str> {
		self.get_attr(HatchAttrs::Pitch)
	}

	/// Set the `rotate` attribute.
	pub fn with_rotate<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(HatchAttrs::Rotate, value.into());
		self
	}

	/// Set the `rotate` attribute.
	pub fn set_rotate<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(HatchAttrs::Rotate, value.into());
	}

	/// Get the `rotate` attribute.
	pub fn rotate(&self) -> Option<&str> {
		self.get_attr(HatchAttrs::Rotate)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(HatchAttrs::Transform, value.into());
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(HatchAttrs::Transform, value.into());
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&str> {
		self.get_attr(HatchAttrs::Transform)
	}

	/// Set the `x` attribute.
	pub fn with_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(HatchAttrs::X, value.into());
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(HatchAttrs::X, value.into());
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&str> {
		self.get_attr(HatchAttrs::X)
	}

	/// Set the `y` attribute.
	pub fn with_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(HatchAttrs::Y, value.into());
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(HatchAttrs::Y, value.into());
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&str> {
		self.get_attr(HatchAttrs::Y)
	}
}

impl common_attrs::CoreAttributesSetter for Hatch {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(HatchAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(HatchAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Hatch {}

impl common_attrs::GlobalEventAttributesSetter for Hatch {
	fn set_attr(&mut self, attr: common_attrs::GlobalEventAttributes, value: String) {
		self.set_attr(HatchAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GlobalEventAttributes) -> Option<&str> {
		self.get_attr(HatchAttrs::from(attr))
	}
}

impl TagWithGlobalEventAttributes for Hatch {}

impl common_attrs::PresentationAttributesSetter for Hatch {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(HatchAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(HatchAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Hatch {}

impl common_attrs::StyleAttributesSetter for Hatch {
	fn set_attr(&mut self, attr: common_attrs::StyleAttributes, value: String) {
		self.set_attr(HatchAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::StyleAttributes) -> Option<&str> {
		self.get_attr(HatchAttrs::from(attr))
	}
}

impl TagWithStyleAttributes for Hatch {}

impl Tag for Hatch {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("hatch");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Script {}
	impl Content for super::Style {}
}

#[doc = "The [`<hatchpath>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("hatchpath.md")]
#[doc = "\n\n [`<hatchpath>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/hatchpath"]
#[derive(Debug)]
pub struct Hatchpath {
	attrs: IndexMap<HatchpathAttrs, String>,
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

	fn set_attr(&mut self, attr: HatchpathAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: HatchpathAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `d` attribute.
	pub fn with_d<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(HatchpathAttrs::D, value.into());
		self
	}

	/// Set the `d` attribute.
	pub fn set_d<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(HatchpathAttrs::D, value.into());
	}

	/// Get the `d` attribute.
	pub fn d(&self) -> Option<&str> {
		self.get_attr(HatchpathAttrs::D)
	}

	/// Set the `offset` attribute.
	pub fn with_offset<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(HatchpathAttrs::Offset, value.into());
		self
	}

	/// Set the `offset` attribute.
	pub fn set_offset<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(HatchpathAttrs::Offset, value.into());
	}

	/// Get the `offset` attribute.
	pub fn offset(&self) -> Option<&str> {
		self.get_attr(HatchpathAttrs::Offset)
	}
}

impl common_attrs::CoreAttributesSetter for Hatchpath {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(HatchpathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(HatchpathAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Hatchpath {}

impl common_attrs::GlobalEventAttributesSetter for Hatchpath {
	fn set_attr(&mut self, attr: common_attrs::GlobalEventAttributes, value: String) {
		self.set_attr(HatchpathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GlobalEventAttributes) -> Option<&str> {
		self.get_attr(HatchpathAttrs::from(attr))
	}
}

impl TagWithGlobalEventAttributes for Hatchpath {}

impl common_attrs::PresentationAttributesSetter for Hatchpath {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(HatchpathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(HatchpathAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Hatchpath {}

impl common_attrs::StyleAttributesSetter for Hatchpath {
	fn set_attr(&mut self, attr: common_attrs::StyleAttributes, value: String) {
		self.set_attr(HatchpathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::StyleAttributes) -> Option<&str> {
		self.get_attr(HatchpathAttrs::from(attr))
	}
}

impl TagWithStyleAttributes for Hatchpath {}

impl Tag for Hatchpath {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("hatchpath");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
#[doc = "\n\n [`<hkern>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/hkern"]
#[derive(Debug)]
pub struct Hkern {
	attrs: IndexMap<HkernAttrs, String>,
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

	fn set_attr(&mut self, attr: HkernAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: HkernAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `g1` attribute.
	pub fn with_g1<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(HkernAttrs::G1, value.into());
		self
	}

	/// Set the `g1` attribute.
	pub fn set_g1<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(HkernAttrs::G1, value.into());
	}

	/// Get the `g1` attribute.
	pub fn g1(&self) -> Option<&str> {
		self.get_attr(HkernAttrs::G1)
	}

	/// Set the `g2` attribute.
	pub fn with_g2<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(HkernAttrs::G2, value.into());
		self
	}

	/// Set the `g2` attribute.
	pub fn set_g2<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(HkernAttrs::G2, value.into());
	}

	/// Get the `g2` attribute.
	pub fn g2(&self) -> Option<&str> {
		self.get_attr(HkernAttrs::G2)
	}

	/// Set the `k` attribute.
	pub fn with_k<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(HkernAttrs::K, value.into());
		self
	}

	/// Set the `k` attribute.
	pub fn set_k<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(HkernAttrs::K, value.into());
	}

	/// Get the `k` attribute.
	pub fn k(&self) -> Option<&str> {
		self.get_attr(HkernAttrs::K)
	}

	/// Set the `u1` attribute.
	pub fn with_u1<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(HkernAttrs::U1, value.into());
		self
	}

	/// Set the `u1` attribute.
	pub fn set_u1<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(HkernAttrs::U1, value.into());
	}

	/// Get the `u1` attribute.
	pub fn u1(&self) -> Option<&str> {
		self.get_attr(HkernAttrs::U1)
	}

	/// Set the `u2` attribute.
	pub fn with_u2<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(HkernAttrs::U2, value.into());
		self
	}

	/// Set the `u2` attribute.
	pub fn set_u2<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(HkernAttrs::U2, value.into());
	}

	/// Get the `u2` attribute.
	pub fn u2(&self) -> Option<&str> {
		self.get_attr(HkernAttrs::U2)
	}
}

impl common_attrs::CoreAttributesSetter for Hkern {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(HkernAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(HkernAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Hkern {}

impl Tag for Hkern {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("hkern");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

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

#[doc = "The [`<image>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("image.md")]
#[doc = "\n\n [`<image>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/image"]
#[derive(Debug)]
pub struct Image {
	attrs: IndexMap<ImageAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: ImageAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: ImageAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ImageAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ImageAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(ImageAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ImageAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ImageAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(ImageAttrs::ExternalResourcesRequired)
	}

	/// Set the `height` attribute.
	pub fn with_height<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ImageAttrs::Height, value.into());
		self
	}

	/// Set the `height` attribute.
	pub fn set_height<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ImageAttrs::Height, value.into());
	}

	/// Get the `height` attribute.
	pub fn height(&self) -> Option<&str> {
		self.get_attr(ImageAttrs::Height)
	}

	/// Set the `preserveAspectRatio` attribute.
	pub fn with_preserve_aspect_ratio<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ImageAttrs::PreserveAspectRatio, value.into());
		self
	}

	/// Set the `preserveAspectRatio` attribute.
	pub fn set_preserve_aspect_ratio<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ImageAttrs::PreserveAspectRatio, value.into());
	}

	/// Get the `preserveAspectRatio` attribute.
	pub fn preserve_aspect_ratio(&self) -> Option<&str> {
		self.get_attr(ImageAttrs::PreserveAspectRatio)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ImageAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ImageAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(ImageAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ImageAttrs::Transform, value.into());
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ImageAttrs::Transform, value.into());
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&str> {
		self.get_attr(ImageAttrs::Transform)
	}

	/// Set the `width` attribute.
	pub fn with_width<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ImageAttrs::Width, value.into());
		self
	}

	/// Set the `width` attribute.
	pub fn set_width<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ImageAttrs::Width, value.into());
	}

	/// Get the `width` attribute.
	pub fn width(&self) -> Option<&str> {
		self.get_attr(ImageAttrs::Width)
	}

	/// Set the `x` attribute.
	pub fn with_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ImageAttrs::X, value.into());
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ImageAttrs::X, value.into());
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&str> {
		self.get_attr(ImageAttrs::X)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ImageAttrs::XlinkHref, value.into());
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ImageAttrs::XlinkHref, value.into());
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&str> {
		self.get_attr(ImageAttrs::XlinkHref)
	}

	/// Set the `y` attribute.
	pub fn with_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ImageAttrs::Y, value.into());
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ImageAttrs::Y, value.into());
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&str> {
		self.get_attr(ImageAttrs::Y)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Image {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(ImageAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(ImageAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Image {}

impl common_attrs::CoreAttributesSetter for Image {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(ImageAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(ImageAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Image {}

impl common_attrs::GraphicalEventAttributesSetter for Image {
	fn set_attr(&mut self, attr: common_attrs::GraphicalEventAttributes, value: String) {
		self.set_attr(ImageAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&str> {
		self.get_attr(ImageAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Image {}

impl common_attrs::PresentationAttributesSetter for Image {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(ImageAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(ImageAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Image {}

impl common_attrs::XLinkAttributesSetter for Image {
	fn set_attr(&mut self, attr: common_attrs::XLinkAttributes, value: String) {
		self.set_attr(ImageAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&str> {
		self.get_attr(ImageAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for Image {}

impl Tag for Image {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("image");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

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

#[doc = "The [`<line>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("line.md")]
#[doc = "\n\n [`<line>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/line"]
#[derive(Debug)]
pub struct Line {
	attrs: IndexMap<LineAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: LineAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: LineAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(LineAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(LineAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(LineAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(LineAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(LineAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(LineAttrs::ExternalResourcesRequired)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(LineAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(LineAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(LineAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(LineAttrs::Transform, value.into());
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(LineAttrs::Transform, value.into());
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&str> {
		self.get_attr(LineAttrs::Transform)
	}

	/// Set the `x1` attribute.
	pub fn with_x1<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(LineAttrs::X1, value.into());
		self
	}

	/// Set the `x1` attribute.
	pub fn set_x1<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(LineAttrs::X1, value.into());
	}

	/// Get the `x1` attribute.
	pub fn x1(&self) -> Option<&str> {
		self.get_attr(LineAttrs::X1)
	}

	/// Set the `x2` attribute.
	pub fn with_x2<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(LineAttrs::X2, value.into());
		self
	}

	/// Set the `x2` attribute.
	pub fn set_x2<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(LineAttrs::X2, value.into());
	}

	/// Get the `x2` attribute.
	pub fn x2(&self) -> Option<&str> {
		self.get_attr(LineAttrs::X2)
	}

	/// Set the `y1` attribute.
	pub fn with_y1<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(LineAttrs::Y1, value.into());
		self
	}

	/// Set the `y1` attribute.
	pub fn set_y1<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(LineAttrs::Y1, value.into());
	}

	/// Get the `y1` attribute.
	pub fn y1(&self) -> Option<&str> {
		self.get_attr(LineAttrs::Y1)
	}

	/// Set the `y2` attribute.
	pub fn with_y2<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(LineAttrs::Y2, value.into());
		self
	}

	/// Set the `y2` attribute.
	pub fn set_y2<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(LineAttrs::Y2, value.into());
	}

	/// Get the `y2` attribute.
	pub fn y2(&self) -> Option<&str> {
		self.get_attr(LineAttrs::Y2)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Line {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(LineAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(LineAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Line {}

impl common_attrs::CoreAttributesSetter for Line {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(LineAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(LineAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Line {}

impl common_attrs::GraphicalEventAttributesSetter for Line {
	fn set_attr(&mut self, attr: common_attrs::GraphicalEventAttributes, value: String) {
		self.set_attr(LineAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&str> {
		self.get_attr(LineAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Line {}

impl common_attrs::PresentationAttributesSetter for Line {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(LineAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(LineAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Line {}

impl Tag for Line {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("line");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Set {}
	impl Content for super::Stop {}
}

#[doc = "The [`<linearGradient>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("linearGradient.md")]
#[doc = "\n\n [`<linearGradient>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/linearGradient"]
#[derive(Debug)]
pub struct LinearGradient {
	attrs: IndexMap<LinearGradientAttrs, String>,
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

	fn set_attr(&mut self, attr: LinearGradientAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: LinearGradientAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(LinearGradientAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(LinearGradientAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(LinearGradientAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(LinearGradientAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(LinearGradientAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(LinearGradientAttrs::ExternalResourcesRequired)
	}

	/// Set the `gradientTransform` attribute.
	pub fn with_gradient_transform<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(LinearGradientAttrs::GradientTransform, value.into());
		self
	}

	/// Set the `gradientTransform` attribute.
	pub fn set_gradient_transform<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(LinearGradientAttrs::GradientTransform, value.into());
	}

	/// Get the `gradientTransform` attribute.
	pub fn gradient_transform(&self) -> Option<&str> {
		self.get_attr(LinearGradientAttrs::GradientTransform)
	}

	/// Set the `gradientUnits` attribute.
	pub fn with_gradient_units<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(LinearGradientAttrs::GradientUnits, value.into());
		self
	}

	/// Set the `gradientUnits` attribute.
	pub fn set_gradient_units<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(LinearGradientAttrs::GradientUnits, value.into());
	}

	/// Get the `gradientUnits` attribute.
	pub fn gradient_units(&self) -> Option<&str> {
		self.get_attr(LinearGradientAttrs::GradientUnits)
	}

	/// Set the `spreadMethod` attribute.
	pub fn with_spread_method<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(LinearGradientAttrs::SpreadMethod, value.into());
		self
	}

	/// Set the `spreadMethod` attribute.
	pub fn set_spread_method<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(LinearGradientAttrs::SpreadMethod, value.into());
	}

	/// Get the `spreadMethod` attribute.
	pub fn spread_method(&self) -> Option<&str> {
		self.get_attr(LinearGradientAttrs::SpreadMethod)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(LinearGradientAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(LinearGradientAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(LinearGradientAttrs::Style)
	}

	/// Set the `x1` attribute.
	pub fn with_x1<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(LinearGradientAttrs::X1, value.into());
		self
	}

	/// Set the `x1` attribute.
	pub fn set_x1<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(LinearGradientAttrs::X1, value.into());
	}

	/// Get the `x1` attribute.
	pub fn x1(&self) -> Option<&str> {
		self.get_attr(LinearGradientAttrs::X1)
	}

	/// Set the `x2` attribute.
	pub fn with_x2<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(LinearGradientAttrs::X2, value.into());
		self
	}

	/// Set the `x2` attribute.
	pub fn set_x2<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(LinearGradientAttrs::X2, value.into());
	}

	/// Get the `x2` attribute.
	pub fn x2(&self) -> Option<&str> {
		self.get_attr(LinearGradientAttrs::X2)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(LinearGradientAttrs::XlinkHref, value.into());
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(LinearGradientAttrs::XlinkHref, value.into());
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&str> {
		self.get_attr(LinearGradientAttrs::XlinkHref)
	}

	/// Set the `y1` attribute.
	pub fn with_y1<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(LinearGradientAttrs::Y1, value.into());
		self
	}

	/// Set the `y1` attribute.
	pub fn set_y1<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(LinearGradientAttrs::Y1, value.into());
	}

	/// Get the `y1` attribute.
	pub fn y1(&self) -> Option<&str> {
		self.get_attr(LinearGradientAttrs::Y1)
	}

	/// Set the `y2` attribute.
	pub fn with_y2<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(LinearGradientAttrs::Y2, value.into());
		self
	}

	/// Set the `y2` attribute.
	pub fn set_y2<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(LinearGradientAttrs::Y2, value.into());
	}

	/// Get the `y2` attribute.
	pub fn y2(&self) -> Option<&str> {
		self.get_attr(LinearGradientAttrs::Y2)
	}
}

impl common_attrs::CoreAttributesSetter for LinearGradient {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(LinearGradientAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(LinearGradientAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for LinearGradient {}

impl common_attrs::PresentationAttributesSetter for LinearGradient {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(LinearGradientAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(LinearGradientAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for LinearGradient {}

impl common_attrs::XLinkAttributesSetter for LinearGradient {
	fn set_attr(&mut self, attr: common_attrs::XLinkAttributes, value: String) {
		self.set_attr(LinearGradientAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&str> {
		self.get_attr(LinearGradientAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for LinearGradient {}

impl Tag for LinearGradient {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("linearGradient");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::A {}
	impl Content for super::AltGlyphDef {}
	impl Content for super::ClipPath {}
	impl Content for super::ColorProfile {}
	impl Content for super::Cursor {}
	impl Content for super::Filter {}
	impl Content for super::Font {}
	impl Content for super::FontFace {}
	impl Content for super::ForeignObject {}
	impl Content for super::Image {}
	impl Content for super::Marker {}
	impl Content for super::Mask {}
	impl Content for super::Pattern {}
	impl Content for super::Script {}
	impl Content for super::Style {}
	impl Content for super::Switch {}
	impl Content for super::Text {}
	impl Content for super::View {}
}

#[doc = "The [`<marker>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("marker.md")]
#[doc = "\n\n [`<marker>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/marker"]
#[derive(Debug)]
pub struct Marker {
	attrs: IndexMap<MarkerAttrs, String>,
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

	fn set_attr(&mut self, attr: MarkerAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: MarkerAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MarkerAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MarkerAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(MarkerAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MarkerAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MarkerAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(MarkerAttrs::ExternalResourcesRequired)
	}

	/// Set the `markerHeight` attribute.
	pub fn with_marker_height<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MarkerAttrs::MarkerHeight, value.into());
		self
	}

	/// Set the `markerHeight` attribute.
	pub fn set_marker_height<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MarkerAttrs::MarkerHeight, value.into());
	}

	/// Get the `markerHeight` attribute.
	pub fn marker_height(&self) -> Option<&str> {
		self.get_attr(MarkerAttrs::MarkerHeight)
	}

	/// Set the `markerUnits` attribute.
	pub fn with_marker_units<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MarkerAttrs::MarkerUnits, value.into());
		self
	}

	/// Set the `markerUnits` attribute.
	pub fn set_marker_units<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MarkerAttrs::MarkerUnits, value.into());
	}

	/// Get the `markerUnits` attribute.
	pub fn marker_units(&self) -> Option<&str> {
		self.get_attr(MarkerAttrs::MarkerUnits)
	}

	/// Set the `markerWidth` attribute.
	pub fn with_marker_width<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MarkerAttrs::MarkerWidth, value.into());
		self
	}

	/// Set the `markerWidth` attribute.
	pub fn set_marker_width<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MarkerAttrs::MarkerWidth, value.into());
	}

	/// Get the `markerWidth` attribute.
	pub fn marker_width(&self) -> Option<&str> {
		self.get_attr(MarkerAttrs::MarkerWidth)
	}

	/// Set the `orient` attribute.
	pub fn with_orient<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MarkerAttrs::Orient, value.into());
		self
	}

	/// Set the `orient` attribute.
	pub fn set_orient<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MarkerAttrs::Orient, value.into());
	}

	/// Get the `orient` attribute.
	pub fn orient(&self) -> Option<&str> {
		self.get_attr(MarkerAttrs::Orient)
	}

	/// Set the `preserveAspectRatio` attribute.
	pub fn with_preserve_aspect_ratio<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MarkerAttrs::PreserveAspectRatio, value.into());
		self
	}

	/// Set the `preserveAspectRatio` attribute.
	pub fn set_preserve_aspect_ratio<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MarkerAttrs::PreserveAspectRatio, value.into());
	}

	/// Get the `preserveAspectRatio` attribute.
	pub fn preserve_aspect_ratio(&self) -> Option<&str> {
		self.get_attr(MarkerAttrs::PreserveAspectRatio)
	}

	/// Set the `refX` attribute.
	pub fn with_ref_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MarkerAttrs::RefX, value.into());
		self
	}

	/// Set the `refX` attribute.
	pub fn set_ref_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MarkerAttrs::RefX, value.into());
	}

	/// Get the `refX` attribute.
	pub fn ref_x(&self) -> Option<&str> {
		self.get_attr(MarkerAttrs::RefX)
	}

	/// Set the `refY` attribute.
	pub fn with_ref_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MarkerAttrs::RefY, value.into());
		self
	}

	/// Set the `refY` attribute.
	pub fn set_ref_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MarkerAttrs::RefY, value.into());
	}

	/// Get the `refY` attribute.
	pub fn ref_y(&self) -> Option<&str> {
		self.get_attr(MarkerAttrs::RefY)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MarkerAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MarkerAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(MarkerAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MarkerAttrs::Transform, value.into());
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MarkerAttrs::Transform, value.into());
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&str> {
		self.get_attr(MarkerAttrs::Transform)
	}

	/// Set the `viewBox` attribute.
	pub fn with_view_box<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MarkerAttrs::ViewBox, value.into());
		self
	}

	/// Set the `viewBox` attribute.
	pub fn set_view_box<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MarkerAttrs::ViewBox, value.into());
	}

	/// Get the `viewBox` attribute.
	pub fn view_box(&self) -> Option<&str> {
		self.get_attr(MarkerAttrs::ViewBox)
	}
}

impl common_attrs::CoreAttributesSetter for Marker {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(MarkerAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(MarkerAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Marker {}

impl common_attrs::PresentationAttributesSetter for Marker {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(MarkerAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(MarkerAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Marker {}

impl Tag for Marker {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("marker");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::A {}
	impl Content for super::AltGlyphDef {}
	impl Content for super::ClipPath {}
	impl Content for super::ColorProfile {}
	impl Content for super::Cursor {}
	impl Content for super::Filter {}
	impl Content for super::Font {}
	impl Content for super::FontFace {}
	impl Content for super::ForeignObject {}
	impl Content for super::Image {}
	impl Content for super::Marker {}
	impl Content for super::Mask {}
	impl Content for super::Pattern {}
	impl Content for super::Script {}
	impl Content for super::Style {}
	impl Content for super::Switch {}
	impl Content for super::Text {}
	impl Content for super::View {}
}

#[doc = "The [`<mask>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("mask.md")]
#[doc = "\n\n [`<mask>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/mask"]
#[derive(Debug)]
pub struct Mask {
	attrs: IndexMap<MaskAttrs, String>,
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

	fn set_attr(&mut self, attr: MaskAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: MaskAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MaskAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MaskAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(MaskAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MaskAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MaskAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(MaskAttrs::ExternalResourcesRequired)
	}

	/// Set the `height` attribute.
	pub fn with_height<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MaskAttrs::Height, value.into());
		self
	}

	/// Set the `height` attribute.
	pub fn set_height<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MaskAttrs::Height, value.into());
	}

	/// Get the `height` attribute.
	pub fn height(&self) -> Option<&str> {
		self.get_attr(MaskAttrs::Height)
	}

	/// Set the `maskContentUnits` attribute.
	pub fn with_mask_content_units<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MaskAttrs::MaskContentUnits, value.into());
		self
	}

	/// Set the `maskContentUnits` attribute.
	pub fn set_mask_content_units<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MaskAttrs::MaskContentUnits, value.into());
	}

	/// Get the `maskContentUnits` attribute.
	pub fn mask_content_units(&self) -> Option<&str> {
		self.get_attr(MaskAttrs::MaskContentUnits)
	}

	/// Set the `maskUnits` attribute.
	pub fn with_mask_units<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MaskAttrs::MaskUnits, value.into());
		self
	}

	/// Set the `maskUnits` attribute.
	pub fn set_mask_units<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MaskAttrs::MaskUnits, value.into());
	}

	/// Get the `maskUnits` attribute.
	pub fn mask_units(&self) -> Option<&str> {
		self.get_attr(MaskAttrs::MaskUnits)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MaskAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MaskAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(MaskAttrs::Style)
	}

	/// Set the `width` attribute.
	pub fn with_width<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MaskAttrs::Width, value.into());
		self
	}

	/// Set the `width` attribute.
	pub fn set_width<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MaskAttrs::Width, value.into());
	}

	/// Get the `width` attribute.
	pub fn width(&self) -> Option<&str> {
		self.get_attr(MaskAttrs::Width)
	}

	/// Set the `x` attribute.
	pub fn with_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MaskAttrs::X, value.into());
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MaskAttrs::X, value.into());
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&str> {
		self.get_attr(MaskAttrs::X)
	}

	/// Set the `y` attribute.
	pub fn with_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MaskAttrs::Y, value.into());
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MaskAttrs::Y, value.into());
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&str> {
		self.get_attr(MaskAttrs::Y)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Mask {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(MaskAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(MaskAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Mask {}

impl common_attrs::CoreAttributesSetter for Mask {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(MaskAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(MaskAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Mask {}

impl common_attrs::PresentationAttributesSetter for Mask {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(MaskAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(MaskAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Mask {}

impl Tag for Mask {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("mask");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	attrs: IndexMap<MetadataAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: MetadataAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: MetadataAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}
}

impl common_attrs::CoreAttributesSetter for Metadata {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(MetadataAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(MetadataAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Metadata {}

impl Tag for Metadata {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("metadata");
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::A {}
	impl Content for super::AltGlyphDef {}
	impl Content for super::ClipPath {}
	impl Content for super::ColorProfile {}
	impl Content for super::Cursor {}
	impl Content for super::Filter {}
	impl Content for super::Font {}
	impl Content for super::FontFace {}
	impl Content for super::ForeignObject {}
	impl Content for super::Image {}
	impl Content for super::Marker {}
	impl Content for super::Mask {}
	impl Content for super::Pattern {}
	impl Content for super::Script {}
	impl Content for super::Style {}
	impl Content for super::Switch {}
	impl Content for super::Text {}
	impl Content for super::View {}
}

#[doc = "The [`<missing-glyph>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("missing-glyph.md")]
#[doc = "\n\n [`<missing-glyph>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/missing-glyph"]
#[derive(Debug)]
pub struct MissingGlyph {
	attrs: IndexMap<MissingGlyphAttrs, String>,
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

	fn set_attr(&mut self, attr: MissingGlyphAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: MissingGlyphAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MissingGlyphAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MissingGlyphAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(MissingGlyphAttrs::Class)
	}

	/// Set the `d` attribute.
	pub fn with_d<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MissingGlyphAttrs::D, value.into());
		self
	}

	/// Set the `d` attribute.
	pub fn set_d<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MissingGlyphAttrs::D, value.into());
	}

	/// Get the `d` attribute.
	pub fn d(&self) -> Option<&str> {
		self.get_attr(MissingGlyphAttrs::D)
	}

	/// Set the `horiz-adv-x` attribute.
	pub fn with_horiz_adv_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MissingGlyphAttrs::HorizAdvX, value.into());
		self
	}

	/// Set the `horiz-adv-x` attribute.
	pub fn set_horiz_adv_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MissingGlyphAttrs::HorizAdvX, value.into());
	}

	/// Get the `horiz-adv-x` attribute.
	pub fn horiz_adv_x(&self) -> Option<&str> {
		self.get_attr(MissingGlyphAttrs::HorizAdvX)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MissingGlyphAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MissingGlyphAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(MissingGlyphAttrs::Style)
	}

	/// Set the `vert-adv-y` attribute.
	pub fn with_vert_adv_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MissingGlyphAttrs::VertAdvY, value.into());
		self
	}

	/// Set the `vert-adv-y` attribute.
	pub fn set_vert_adv_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MissingGlyphAttrs::VertAdvY, value.into());
	}

	/// Get the `vert-adv-y` attribute.
	pub fn vert_adv_y(&self) -> Option<&str> {
		self.get_attr(MissingGlyphAttrs::VertAdvY)
	}

	/// Set the `vert-origin-x` attribute.
	pub fn with_vert_origin_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MissingGlyphAttrs::VertOriginX, value.into());
		self
	}

	/// Set the `vert-origin-x` attribute.
	pub fn set_vert_origin_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MissingGlyphAttrs::VertOriginX, value.into());
	}

	/// Get the `vert-origin-x` attribute.
	pub fn vert_origin_x(&self) -> Option<&str> {
		self.get_attr(MissingGlyphAttrs::VertOriginX)
	}

	/// Set the `vert-origin-y` attribute.
	pub fn with_vert_origin_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MissingGlyphAttrs::VertOriginY, value.into());
		self
	}

	/// Set the `vert-origin-y` attribute.
	pub fn set_vert_origin_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MissingGlyphAttrs::VertOriginY, value.into());
	}

	/// Get the `vert-origin-y` attribute.
	pub fn vert_origin_y(&self) -> Option<&str> {
		self.get_attr(MissingGlyphAttrs::VertOriginY)
	}
}

impl common_attrs::CoreAttributesSetter for MissingGlyph {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(MissingGlyphAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(MissingGlyphAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for MissingGlyph {}

impl common_attrs::PresentationAttributesSetter for MissingGlyph {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(MissingGlyphAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(MissingGlyphAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for MissingGlyph {}

impl Tag for MissingGlyph {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("missing-glyph");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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

#[doc = "The [`<mpath>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("mpath.md")]
#[doc = "\n\n [`<mpath>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/mpath"]
#[derive(Debug)]
pub struct Mpath {
	attrs: IndexMap<MpathAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: MpathAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: MpathAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MpathAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MpathAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(MpathAttrs::ExternalResourcesRequired)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MpathAttrs::XlinkHref, value.into());
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MpathAttrs::XlinkHref, value.into());
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&str> {
		self.get_attr(MpathAttrs::XlinkHref)
	}
}

impl common_attrs::CoreAttributesSetter for Mpath {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(MpathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(MpathAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Mpath {}

impl common_attrs::XLinkAttributesSetter for Mpath {
	fn set_attr(&mut self, attr: common_attrs::XLinkAttributes, value: String) {
		self.set_attr(MpathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&str> {
		self.get_attr(MpathAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for Mpath {}

impl Tag for Mpath {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("mpath");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

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

#[doc = "The [`<path>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("path.md")]
#[doc = "\n\n [`<path>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/path"]
#[derive(Debug)]
pub struct Path {
	attrs: IndexMap<PathAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: PathAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: PathAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PathAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PathAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(PathAttrs::Class)
	}

	/// Set the `d` attribute.
	pub fn with_d<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PathAttrs::D, value.into());
		self
	}

	/// Set the `d` attribute.
	pub fn set_d<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PathAttrs::D, value.into());
	}

	/// Get the `d` attribute.
	pub fn d(&self) -> Option<&str> {
		self.get_attr(PathAttrs::D)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PathAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PathAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(PathAttrs::ExternalResourcesRequired)
	}

	/// Set the `pathLength` attribute.
	pub fn with_path_length<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PathAttrs::PathLength, value.into());
		self
	}

	/// Set the `pathLength` attribute.
	pub fn set_path_length<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PathAttrs::PathLength, value.into());
	}

	/// Get the `pathLength` attribute.
	pub fn path_length(&self) -> Option<&str> {
		self.get_attr(PathAttrs::PathLength)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PathAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PathAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(PathAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PathAttrs::Transform, value.into());
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PathAttrs::Transform, value.into());
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&str> {
		self.get_attr(PathAttrs::Transform)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Path {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(PathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(PathAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Path {}

impl common_attrs::CoreAttributesSetter for Path {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(PathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(PathAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Path {}

impl common_attrs::GraphicalEventAttributesSetter for Path {
	fn set_attr(&mut self, attr: common_attrs::GraphicalEventAttributes, value: String) {
		self.set_attr(PathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&str> {
		self.get_attr(PathAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Path {}

impl common_attrs::PresentationAttributesSetter for Path {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(PathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(PathAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Path {}

impl Tag for Path {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("path");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::A {}
	impl Content for super::AltGlyphDef {}
	impl Content for super::ClipPath {}
	impl Content for super::ColorProfile {}
	impl Content for super::Cursor {}
	impl Content for super::Filter {}
	impl Content for super::Font {}
	impl Content for super::FontFace {}
	impl Content for super::ForeignObject {}
	impl Content for super::Image {}
	impl Content for super::Marker {}
	impl Content for super::Mask {}
	impl Content for super::Pattern {}
	impl Content for super::Script {}
	impl Content for super::Style {}
	impl Content for super::Switch {}
	impl Content for super::Text {}
	impl Content for super::View {}
}

#[doc = "The [`<pattern>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("pattern.md")]
#[doc = "\n\n [`<pattern>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/pattern"]
#[derive(Debug)]
pub struct Pattern {
	attrs: IndexMap<PatternAttrs, String>,
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

	fn set_attr(&mut self, attr: PatternAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: PatternAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PatternAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PatternAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(PatternAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PatternAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PatternAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(PatternAttrs::ExternalResourcesRequired)
	}

	/// Set the `height` attribute.
	pub fn with_height<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PatternAttrs::Height, value.into());
		self
	}

	/// Set the `height` attribute.
	pub fn set_height<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PatternAttrs::Height, value.into());
	}

	/// Get the `height` attribute.
	pub fn height(&self) -> Option<&str> {
		self.get_attr(PatternAttrs::Height)
	}

	/// Set the `patternContentUnits` attribute.
	pub fn with_pattern_content_units<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PatternAttrs::PatternContentUnits, value.into());
		self
	}

	/// Set the `patternContentUnits` attribute.
	pub fn set_pattern_content_units<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PatternAttrs::PatternContentUnits, value.into());
	}

	/// Get the `patternContentUnits` attribute.
	pub fn pattern_content_units(&self) -> Option<&str> {
		self.get_attr(PatternAttrs::PatternContentUnits)
	}

	/// Set the `patternTransform` attribute.
	pub fn with_pattern_transform<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PatternAttrs::PatternTransform, value.into());
		self
	}

	/// Set the `patternTransform` attribute.
	pub fn set_pattern_transform<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PatternAttrs::PatternTransform, value.into());
	}

	/// Get the `patternTransform` attribute.
	pub fn pattern_transform(&self) -> Option<&str> {
		self.get_attr(PatternAttrs::PatternTransform)
	}

	/// Set the `patternUnits` attribute.
	pub fn with_pattern_units<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PatternAttrs::PatternUnits, value.into());
		self
	}

	/// Set the `patternUnits` attribute.
	pub fn set_pattern_units<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PatternAttrs::PatternUnits, value.into());
	}

	/// Get the `patternUnits` attribute.
	pub fn pattern_units(&self) -> Option<&str> {
		self.get_attr(PatternAttrs::PatternUnits)
	}

	/// Set the `preserveAspectRatio` attribute.
	pub fn with_preserve_aspect_ratio<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PatternAttrs::PreserveAspectRatio, value.into());
		self
	}

	/// Set the `preserveAspectRatio` attribute.
	pub fn set_preserve_aspect_ratio<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PatternAttrs::PreserveAspectRatio, value.into());
	}

	/// Get the `preserveAspectRatio` attribute.
	pub fn preserve_aspect_ratio(&self) -> Option<&str> {
		self.get_attr(PatternAttrs::PreserveAspectRatio)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PatternAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PatternAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(PatternAttrs::Style)
	}

	/// Set the `viewBox` attribute.
	pub fn with_view_box<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PatternAttrs::ViewBox, value.into());
		self
	}

	/// Set the `viewBox` attribute.
	pub fn set_view_box<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PatternAttrs::ViewBox, value.into());
	}

	/// Get the `viewBox` attribute.
	pub fn view_box(&self) -> Option<&str> {
		self.get_attr(PatternAttrs::ViewBox)
	}

	/// Set the `width` attribute.
	pub fn with_width<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PatternAttrs::Width, value.into());
		self
	}

	/// Set the `width` attribute.
	pub fn set_width<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PatternAttrs::Width, value.into());
	}

	/// Get the `width` attribute.
	pub fn width(&self) -> Option<&str> {
		self.get_attr(PatternAttrs::Width)
	}

	/// Set the `x` attribute.
	pub fn with_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PatternAttrs::X, value.into());
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PatternAttrs::X, value.into());
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&str> {
		self.get_attr(PatternAttrs::X)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PatternAttrs::XlinkHref, value.into());
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PatternAttrs::XlinkHref, value.into());
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&str> {
		self.get_attr(PatternAttrs::XlinkHref)
	}

	/// Set the `y` attribute.
	pub fn with_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PatternAttrs::Y, value.into());
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PatternAttrs::Y, value.into());
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&str> {
		self.get_attr(PatternAttrs::Y)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Pattern {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(PatternAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(PatternAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Pattern {}

impl common_attrs::CoreAttributesSetter for Pattern {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(PatternAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(PatternAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Pattern {}

impl common_attrs::PresentationAttributesSetter for Pattern {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(PatternAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(PatternAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Pattern {}

impl common_attrs::XLinkAttributesSetter for Pattern {
	fn set_attr(&mut self, attr: common_attrs::XLinkAttributes, value: String) {
		self.set_attr(PatternAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&str> {
		self.get_attr(PatternAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for Pattern {}

impl Tag for Pattern {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("pattern");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum PolygonAttrs {
	Class,
	ExternalResourcesRequired,
	Points,
	Style,
	Transform,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
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

#[doc = "The [`<polygon>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("polygon.md")]
#[doc = "\n\n [`<polygon>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/polygon"]
#[derive(Debug)]
pub struct Polygon {
	attrs: IndexMap<PolygonAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: PolygonAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: PolygonAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PolygonAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PolygonAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(PolygonAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PolygonAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PolygonAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(PolygonAttrs::ExternalResourcesRequired)
	}

	/// Set the `points` attribute.
	pub fn with_points<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PolygonAttrs::Points, value.into());
		self
	}

	/// Set the `points` attribute.
	pub fn set_points<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PolygonAttrs::Points, value.into());
	}

	/// Get the `points` attribute.
	pub fn points(&self) -> Option<&str> {
		self.get_attr(PolygonAttrs::Points)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PolygonAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PolygonAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(PolygonAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PolygonAttrs::Transform, value.into());
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PolygonAttrs::Transform, value.into());
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&str> {
		self.get_attr(PolygonAttrs::Transform)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Polygon {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(PolygonAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(PolygonAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Polygon {}

impl common_attrs::CoreAttributesSetter for Polygon {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(PolygonAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(PolygonAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Polygon {}

impl common_attrs::GraphicalEventAttributesSetter for Polygon {
	fn set_attr(&mut self, attr: common_attrs::GraphicalEventAttributes, value: String) {
		self.set_attr(PolygonAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&str> {
		self.get_attr(PolygonAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Polygon {}

impl common_attrs::PresentationAttributesSetter for Polygon {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(PolygonAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(PolygonAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Polygon {}

impl Tag for Polygon {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("polygon");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum PolylineAttrs {
	Class,
	ExternalResourcesRequired,
	Points,
	Style,
	Transform,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
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

#[doc = "The [`<polyline>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("polyline.md")]
#[doc = "\n\n [`<polyline>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/polyline"]
#[derive(Debug)]
pub struct Polyline {
	attrs: IndexMap<PolylineAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: PolylineAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: PolylineAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PolylineAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PolylineAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(PolylineAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PolylineAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PolylineAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(PolylineAttrs::ExternalResourcesRequired)
	}

	/// Set the `points` attribute.
	pub fn with_points<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PolylineAttrs::Points, value.into());
		self
	}

	/// Set the `points` attribute.
	pub fn set_points<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PolylineAttrs::Points, value.into());
	}

	/// Get the `points` attribute.
	pub fn points(&self) -> Option<&str> {
		self.get_attr(PolylineAttrs::Points)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PolylineAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PolylineAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(PolylineAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PolylineAttrs::Transform, value.into());
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PolylineAttrs::Transform, value.into());
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&str> {
		self.get_attr(PolylineAttrs::Transform)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Polyline {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(PolylineAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(PolylineAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Polyline {}

impl common_attrs::CoreAttributesSetter for Polyline {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(PolylineAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(PolylineAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Polyline {}

impl common_attrs::GraphicalEventAttributesSetter for Polyline {
	fn set_attr(&mut self, attr: common_attrs::GraphicalEventAttributes, value: String) {
		self.set_attr(PolylineAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&str> {
		self.get_attr(PolylineAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Polyline {}

impl common_attrs::PresentationAttributesSetter for Polyline {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(PolylineAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(PolylineAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Polyline {}

impl Tag for Polyline {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("polyline");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::AnimateTransform {}
	impl Content for super::Set {}
	impl Content for super::Stop {}
}

#[doc = "The [`<radialGradient>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("radialGradient.md")]
#[doc = "\n\n [`<radialGradient>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/radialGradient"]
#[derive(Debug)]
pub struct RadialGradient {
	attrs: IndexMap<RadialGradientAttrs, String>,
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

	fn set_attr(&mut self, attr: RadialGradientAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: RadialGradientAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(RadialGradientAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(RadialGradientAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(RadialGradientAttrs::Class)
	}

	/// Set the `cx` attribute.
	pub fn with_cx<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(RadialGradientAttrs::Cx, value.into());
		self
	}

	/// Set the `cx` attribute.
	pub fn set_cx<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(RadialGradientAttrs::Cx, value.into());
	}

	/// Get the `cx` attribute.
	pub fn cx(&self) -> Option<&str> {
		self.get_attr(RadialGradientAttrs::Cx)
	}

	/// Set the `cy` attribute.
	pub fn with_cy<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(RadialGradientAttrs::Cy, value.into());
		self
	}

	/// Set the `cy` attribute.
	pub fn set_cy<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(RadialGradientAttrs::Cy, value.into());
	}

	/// Get the `cy` attribute.
	pub fn cy(&self) -> Option<&str> {
		self.get_attr(RadialGradientAttrs::Cy)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(RadialGradientAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(RadialGradientAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(RadialGradientAttrs::ExternalResourcesRequired)
	}

	/// Set the `fx` attribute.
	pub fn with_fx<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(RadialGradientAttrs::Fx, value.into());
		self
	}

	/// Set the `fx` attribute.
	pub fn set_fx<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(RadialGradientAttrs::Fx, value.into());
	}

	/// Get the `fx` attribute.
	pub fn fx(&self) -> Option<&str> {
		self.get_attr(RadialGradientAttrs::Fx)
	}

	/// Set the `fy` attribute.
	pub fn with_fy<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(RadialGradientAttrs::Fy, value.into());
		self
	}

	/// Set the `fy` attribute.
	pub fn set_fy<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(RadialGradientAttrs::Fy, value.into());
	}

	/// Get the `fy` attribute.
	pub fn fy(&self) -> Option<&str> {
		self.get_attr(RadialGradientAttrs::Fy)
	}

	/// Set the `gradientTransform` attribute.
	pub fn with_gradient_transform<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(RadialGradientAttrs::GradientTransform, value.into());
		self
	}

	/// Set the `gradientTransform` attribute.
	pub fn set_gradient_transform<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(RadialGradientAttrs::GradientTransform, value.into());
	}

	/// Get the `gradientTransform` attribute.
	pub fn gradient_transform(&self) -> Option<&str> {
		self.get_attr(RadialGradientAttrs::GradientTransform)
	}

	/// Set the `gradientUnits` attribute.
	pub fn with_gradient_units<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(RadialGradientAttrs::GradientUnits, value.into());
		self
	}

	/// Set the `gradientUnits` attribute.
	pub fn set_gradient_units<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(RadialGradientAttrs::GradientUnits, value.into());
	}

	/// Get the `gradientUnits` attribute.
	pub fn gradient_units(&self) -> Option<&str> {
		self.get_attr(RadialGradientAttrs::GradientUnits)
	}

	/// Set the `r` attribute.
	pub fn with_r<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(RadialGradientAttrs::R, value.into());
		self
	}

	/// Set the `r` attribute.
	pub fn set_r<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(RadialGradientAttrs::R, value.into());
	}

	/// Get the `r` attribute.
	pub fn r(&self) -> Option<&str> {
		self.get_attr(RadialGradientAttrs::R)
	}

	/// Set the `spreadMethod` attribute.
	pub fn with_spread_method<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(RadialGradientAttrs::SpreadMethod, value.into());
		self
	}

	/// Set the `spreadMethod` attribute.
	pub fn set_spread_method<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(RadialGradientAttrs::SpreadMethod, value.into());
	}

	/// Get the `spreadMethod` attribute.
	pub fn spread_method(&self) -> Option<&str> {
		self.get_attr(RadialGradientAttrs::SpreadMethod)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(RadialGradientAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(RadialGradientAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(RadialGradientAttrs::Style)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(RadialGradientAttrs::XlinkHref, value.into());
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(RadialGradientAttrs::XlinkHref, value.into());
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&str> {
		self.get_attr(RadialGradientAttrs::XlinkHref)
	}
}

impl common_attrs::CoreAttributesSetter for RadialGradient {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(RadialGradientAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(RadialGradientAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for RadialGradient {}

impl common_attrs::PresentationAttributesSetter for RadialGradient {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(RadialGradientAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(RadialGradientAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for RadialGradient {}

impl common_attrs::XLinkAttributesSetter for RadialGradient {
	fn set_attr(&mut self, attr: common_attrs::XLinkAttributes, value: String) {
		self.set_attr(RadialGradientAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&str> {
		self.get_attr(RadialGradientAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for RadialGradient {}

impl Tag for RadialGradient {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("radialGradient");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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

#[doc = "The [`<rect>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("rect.md")]
#[doc = "\n\n [`<rect>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/rect"]
#[derive(Debug)]
pub struct Rect {
	attrs: IndexMap<RectAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: RectAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: RectAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(RectAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(RectAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(RectAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(RectAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(RectAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(RectAttrs::ExternalResourcesRequired)
	}

	/// Set the `height` attribute.
	pub fn with_height<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(RectAttrs::Height, value.into());
		self
	}

	/// Set the `height` attribute.
	pub fn set_height<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(RectAttrs::Height, value.into());
	}

	/// Get the `height` attribute.
	pub fn height(&self) -> Option<&str> {
		self.get_attr(RectAttrs::Height)
	}

	/// Set the `rx` attribute.
	pub fn with_rx<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(RectAttrs::Rx, value.into());
		self
	}

	/// Set the `rx` attribute.
	pub fn set_rx<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(RectAttrs::Rx, value.into());
	}

	/// Get the `rx` attribute.
	pub fn rx(&self) -> Option<&str> {
		self.get_attr(RectAttrs::Rx)
	}

	/// Set the `ry` attribute.
	pub fn with_ry<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(RectAttrs::Ry, value.into());
		self
	}

	/// Set the `ry` attribute.
	pub fn set_ry<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(RectAttrs::Ry, value.into());
	}

	/// Get the `ry` attribute.
	pub fn ry(&self) -> Option<&str> {
		self.get_attr(RectAttrs::Ry)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(RectAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(RectAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(RectAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(RectAttrs::Transform, value.into());
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(RectAttrs::Transform, value.into());
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&str> {
		self.get_attr(RectAttrs::Transform)
	}

	/// Set the `width` attribute.
	pub fn with_width<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(RectAttrs::Width, value.into());
		self
	}

	/// Set the `width` attribute.
	pub fn set_width<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(RectAttrs::Width, value.into());
	}

	/// Get the `width` attribute.
	pub fn width(&self) -> Option<&str> {
		self.get_attr(RectAttrs::Width)
	}

	/// Set the `x` attribute.
	pub fn with_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(RectAttrs::X, value.into());
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(RectAttrs::X, value.into());
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&str> {
		self.get_attr(RectAttrs::X)
	}

	/// Set the `y` attribute.
	pub fn with_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(RectAttrs::Y, value.into());
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(RectAttrs::Y, value.into());
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&str> {
		self.get_attr(RectAttrs::Y)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Rect {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(RectAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(RectAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Rect {}

impl common_attrs::CoreAttributesSetter for Rect {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(RectAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(RectAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Rect {}

impl common_attrs::GraphicalEventAttributesSetter for Rect {
	fn set_attr(&mut self, attr: common_attrs::GraphicalEventAttributes, value: String) {
		self.set_attr(RectAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&str> {
		self.get_attr(RectAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Rect {}

impl common_attrs::PresentationAttributesSetter for Rect {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(RectAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(RectAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Rect {}

impl Tag for Rect {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("rect");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

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
	attrs: IndexMap<ScriptAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: ScriptAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: ScriptAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ScriptAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ScriptAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(ScriptAttrs::ExternalResourcesRequired)
	}

	/// Set the `type` attribute.
	pub fn with_ty<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ScriptAttrs::Type, value.into());
		self
	}

	/// Set the `type` attribute.
	pub fn set_ty<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ScriptAttrs::Type, value.into());
	}

	/// Get the `type` attribute.
	pub fn ty(&self) -> Option<&str> {
		self.get_attr(ScriptAttrs::Type)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ScriptAttrs::XlinkHref, value.into());
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ScriptAttrs::XlinkHref, value.into());
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&str> {
		self.get_attr(ScriptAttrs::XlinkHref)
	}
}

impl common_attrs::CoreAttributesSetter for Script {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(ScriptAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(ScriptAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Script {}

impl common_attrs::XLinkAttributesSetter for Script {
	fn set_attr(&mut self, attr: common_attrs::XLinkAttributes, value: String) {
		self.set_attr(ScriptAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&str> {
		self.get_attr(ScriptAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for Script {}

impl Tag for Script {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("script");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

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

#[doc = "The [`<set>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("set.md")]
#[doc = "\n\n [`<set>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/set"]
#[derive(Debug)]
pub struct Set {
	attrs: IndexMap<SetAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: SetAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: SetAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(SetAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(SetAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(SetAttrs::ExternalResourcesRequired)
	}

	/// Set the `to` attribute.
	pub fn with_to<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(SetAttrs::To, value.into());
		self
	}

	/// Set the `to` attribute.
	pub fn set_to<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(SetAttrs::To, value.into());
	}

	/// Get the `to` attribute.
	pub fn to(&self) -> Option<&str> {
		self.get_attr(SetAttrs::To)
	}
}

impl common_attrs::AnimationAttributeTargetAttributesSetter for Set {
	fn set_attr(&mut self, attr: common_attrs::AnimationAttributeTargetAttributes, value: String) {
		self.set_attr(SetAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationAttributeTargetAttributes) -> Option<&str> {
		self.get_attr(SetAttrs::from(attr))
	}
}

impl TagWithAnimationAttributeTargetAttributes for Set {}

impl common_attrs::AnimationEventAttributesSetter for Set {
	fn set_attr(&mut self, attr: common_attrs::AnimationEventAttributes, value: String) {
		self.set_attr(SetAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationEventAttributes) -> Option<&str> {
		self.get_attr(SetAttrs::from(attr))
	}
}

impl TagWithAnimationEventAttributes for Set {}

impl common_attrs::AnimationTimingAttributesSetter for Set {
	fn set_attr(&mut self, attr: common_attrs::AnimationTimingAttributes, value: String) {
		self.set_attr(SetAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::AnimationTimingAttributes) -> Option<&str> {
		self.get_attr(SetAttrs::from(attr))
	}
}

impl TagWithAnimationTimingAttributes for Set {}

impl common_attrs::ConditionalProcessingAttributesSetter for Set {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(SetAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(SetAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Set {}

impl common_attrs::CoreAttributesSetter for Set {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(SetAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(SetAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Set {}

impl common_attrs::XLinkAttributesSetter for Set {
	fn set_attr(&mut self, attr: common_attrs::XLinkAttributes, value: String) {
		self.set_attr(SetAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&str> {
		self.get_attr(SetAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for Set {}

impl Tag for Set {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("set");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::AnimateColor {}
	impl Content for super::Set {}
}

#[doc = "The [`<stop>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("stop.md")]
#[doc = "\n\n [`<stop>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/stop"]
#[derive(Debug)]
pub struct Stop {
	attrs: IndexMap<StopAttrs, String>,
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

	fn set_attr(&mut self, attr: StopAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: StopAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(StopAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(StopAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(StopAttrs::Class)
	}

	/// Set the `offset` attribute.
	pub fn with_offset<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(StopAttrs::Offset, value.into());
		self
	}

	/// Set the `offset` attribute.
	pub fn set_offset<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(StopAttrs::Offset, value.into());
	}

	/// Get the `offset` attribute.
	pub fn offset(&self) -> Option<&str> {
		self.get_attr(StopAttrs::Offset)
	}

	/// Set the `stop-color` attribute.
	pub fn with_stop_color<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(StopAttrs::StopColor, value.into());
		self
	}

	/// Set the `stop-color` attribute.
	pub fn set_stop_color<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(StopAttrs::StopColor, value.into());
	}

	/// Get the `stop-color` attribute.
	pub fn stop_color(&self) -> Option<&str> {
		self.get_attr(StopAttrs::StopColor)
	}

	/// Set the `stop-opacity` attribute.
	pub fn with_stop_opacity<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(StopAttrs::StopOpacity, value.into());
		self
	}

	/// Set the `stop-opacity` attribute.
	pub fn set_stop_opacity<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(StopAttrs::StopOpacity, value.into());
	}

	/// Get the `stop-opacity` attribute.
	pub fn stop_opacity(&self) -> Option<&str> {
		self.get_attr(StopAttrs::StopOpacity)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(StopAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(StopAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(StopAttrs::Style)
	}
}

impl common_attrs::CoreAttributesSetter for Stop {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(StopAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(StopAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Stop {}

impl common_attrs::PresentationAttributesSetter for Stop {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(StopAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(StopAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Stop {}

impl Tag for Stop {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("stop");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	attrs: IndexMap<StyleAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: StyleAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: StyleAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `media` attribute.
	pub fn with_media<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(StyleAttrs::Media, value.into());
		self
	}

	/// Set the `media` attribute.
	pub fn set_media<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(StyleAttrs::Media, value.into());
	}

	/// Get the `media` attribute.
	pub fn media(&self) -> Option<&str> {
		self.get_attr(StyleAttrs::Media)
	}

	/// Set the `title` attribute.
	pub fn with_title<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(StyleAttrs::Title, value.into());
		self
	}

	/// Set the `title` attribute.
	pub fn set_title<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(StyleAttrs::Title, value.into());
	}

	/// Get the `title` attribute.
	pub fn title(&self) -> Option<&str> {
		self.get_attr(StyleAttrs::Title)
	}

	/// Set the `type` attribute.
	pub fn with_ty<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(StyleAttrs::Type, value.into());
		self
	}

	/// Set the `type` attribute.
	pub fn set_ty<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(StyleAttrs::Type, value.into());
	}

	/// Get the `type` attribute.
	pub fn ty(&self) -> Option<&str> {
		self.get_attr(StyleAttrs::Type)
	}
}

impl common_attrs::CoreAttributesSetter for Style {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(StyleAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(StyleAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Style {}

impl Tag for Style {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("style");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::A {}
	impl Content for super::AltGlyphDef {}
	impl Content for super::ClipPath {}
	impl Content for super::ColorProfile {}
	impl Content for super::Cursor {}
	impl Content for super::Filter {}
	impl Content for super::Font {}
	impl Content for super::FontFace {}
	impl Content for super::ForeignObject {}
	impl Content for super::Image {}
	impl Content for super::Marker {}
	impl Content for super::Mask {}
	impl Content for super::Pattern {}
	impl Content for super::Script {}
	impl Content for super::Style {}
	impl Content for super::Switch {}
	impl Content for super::Text {}
	impl Content for super::View {}
}

#[doc = "The [`<svg>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("svg.md")]
#[doc = "\n\n [`<svg>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/svg"]
#[derive(Debug)]
pub struct Svg {
	attrs: IndexMap<SvgAttrs, String>,
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

	pub(crate) fn set_attr(&mut self, attr: SvgAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: SvgAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `baseProfile` attribute.
	pub fn with_base_profile<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(SvgAttrs::BaseProfile, value.into());
		self
	}

	/// Set the `baseProfile` attribute.
	pub fn set_base_profile<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(SvgAttrs::BaseProfile, value.into());
	}

	/// Get the `baseProfile` attribute.
	pub fn base_profile(&self) -> Option<&str> {
		self.get_attr(SvgAttrs::BaseProfile)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(SvgAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(SvgAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(SvgAttrs::Class)
	}

	/// Set the `contentScriptType` attribute.
	pub fn with_content_script_type<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(SvgAttrs::ContentScriptType, value.into());
		self
	}

	/// Set the `contentScriptType` attribute.
	pub fn set_content_script_type<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(SvgAttrs::ContentScriptType, value.into());
	}

	/// Get the `contentScriptType` attribute.
	pub fn content_script_type(&self) -> Option<&str> {
		self.get_attr(SvgAttrs::ContentScriptType)
	}

	/// Set the `contentStyleType` attribute.
	pub fn with_content_style_type<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(SvgAttrs::ContentStyleType, value.into());
		self
	}

	/// Set the `contentStyleType` attribute.
	pub fn set_content_style_type<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(SvgAttrs::ContentStyleType, value.into());
	}

	/// Get the `contentStyleType` attribute.
	pub fn content_style_type(&self) -> Option<&str> {
		self.get_attr(SvgAttrs::ContentStyleType)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(SvgAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(SvgAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(SvgAttrs::ExternalResourcesRequired)
	}

	/// Set the `height` attribute.
	pub fn with_height<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(SvgAttrs::Height, value.into());
		self
	}

	/// Set the `height` attribute.
	pub fn set_height<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(SvgAttrs::Height, value.into());
	}

	/// Get the `height` attribute.
	pub fn height(&self) -> Option<&str> {
		self.get_attr(SvgAttrs::Height)
	}

	/// Set the `preserveAspectRatio` attribute.
	pub fn with_preserve_aspect_ratio<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(SvgAttrs::PreserveAspectRatio, value.into());
		self
	}

	/// Set the `preserveAspectRatio` attribute.
	pub fn set_preserve_aspect_ratio<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(SvgAttrs::PreserveAspectRatio, value.into());
	}

	/// Get the `preserveAspectRatio` attribute.
	pub fn preserve_aspect_ratio(&self) -> Option<&str> {
		self.get_attr(SvgAttrs::PreserveAspectRatio)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(SvgAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(SvgAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(SvgAttrs::Style)
	}

	/// Set the `version` attribute.
	pub fn with_version<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(SvgAttrs::Version, value.into());
		self
	}

	/// Set the `version` attribute.
	pub fn set_version<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(SvgAttrs::Version, value.into());
	}

	/// Get the `version` attribute.
	pub fn version(&self) -> Option<&str> {
		self.get_attr(SvgAttrs::Version)
	}

	/// Set the `viewBox` attribute.
	pub fn with_view_box<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(SvgAttrs::ViewBox, value.into());
		self
	}

	/// Set the `viewBox` attribute.
	pub fn set_view_box<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(SvgAttrs::ViewBox, value.into());
	}

	/// Get the `viewBox` attribute.
	pub fn view_box(&self) -> Option<&str> {
		self.get_attr(SvgAttrs::ViewBox)
	}

	/// Set the `width` attribute.
	pub fn with_width<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(SvgAttrs::Width, value.into());
		self
	}

	/// Set the `width` attribute.
	pub fn set_width<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(SvgAttrs::Width, value.into());
	}

	/// Get the `width` attribute.
	pub fn width(&self) -> Option<&str> {
		self.get_attr(SvgAttrs::Width)
	}

	/// Set the `x` attribute.
	pub fn with_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(SvgAttrs::X, value.into());
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(SvgAttrs::X, value.into());
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&str> {
		self.get_attr(SvgAttrs::X)
	}

	/// Set the `y` attribute.
	pub fn with_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(SvgAttrs::Y, value.into());
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(SvgAttrs::Y, value.into());
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&str> {
		self.get_attr(SvgAttrs::Y)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Svg {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(SvgAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(SvgAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Svg {}

impl common_attrs::CoreAttributesSetter for Svg {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(SvgAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(SvgAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Svg {}

impl common_attrs::DocumentEventAttributesSetter for Svg {
	fn set_attr(&mut self, attr: common_attrs::DocumentEventAttributes, value: String) {
		self.set_attr(SvgAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::DocumentEventAttributes) -> Option<&str> {
		self.get_attr(SvgAttrs::from(attr))
	}
}

impl TagWithDocumentEventAttributes for Svg {}

impl common_attrs::GlobalEventAttributesSetter for Svg {
	fn set_attr(&mut self, attr: common_attrs::GlobalEventAttributes, value: String) {
		self.set_attr(SvgAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GlobalEventAttributes) -> Option<&str> {
		self.get_attr(SvgAttrs::from(attr))
	}
}

impl TagWithGlobalEventAttributes for Svg {}

impl common_attrs::GraphicalEventAttributesSetter for Svg {
	fn set_attr(&mut self, attr: common_attrs::GraphicalEventAttributes, value: String) {
		self.set_attr(SvgAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&str> {
		self.get_attr(SvgAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Svg {}

impl common_attrs::PresentationAttributesSetter for Svg {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(SvgAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(SvgAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Svg {}

impl Tag for Svg {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("svg");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum SwitchAttrs {
	AllowReorder,
	Class,
	ExternalResourcesRequired,
	Style,
	Transform,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::A {}
	impl Content for super::ForeignObject {}
	impl Content for super::G {}
	impl Content for super::Image {}
	impl Content for super::Svg {}
	impl Content for super::Switch {}
	impl Content for super::Text {}
	impl Content for super::Use {}
}

#[doc = "The [`<switch>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("switch.md")]
#[doc = "\n\n [`<switch>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/switch"]
#[derive(Debug)]
pub struct Switch {
	attrs: IndexMap<SwitchAttrs, String>,
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

	fn set_attr(&mut self, attr: SwitchAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: SwitchAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `allowReorder` attribute.
	pub fn with_allow_reorder<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(SwitchAttrs::AllowReorder, value.into());
		self
	}

	/// Set the `allowReorder` attribute.
	pub fn set_allow_reorder<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(SwitchAttrs::AllowReorder, value.into());
	}

	/// Get the `allowReorder` attribute.
	pub fn allow_reorder(&self) -> Option<&str> {
		self.get_attr(SwitchAttrs::AllowReorder)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(SwitchAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(SwitchAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(SwitchAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(SwitchAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(SwitchAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(SwitchAttrs::ExternalResourcesRequired)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(SwitchAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(SwitchAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(SwitchAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(SwitchAttrs::Transform, value.into());
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(SwitchAttrs::Transform, value.into());
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&str> {
		self.get_attr(SwitchAttrs::Transform)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Switch {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(SwitchAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(SwitchAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Switch {}

impl common_attrs::CoreAttributesSetter for Switch {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(SwitchAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(SwitchAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Switch {}

impl common_attrs::GraphicalEventAttributesSetter for Switch {
	fn set_attr(&mut self, attr: common_attrs::GraphicalEventAttributes, value: String) {
		self.set_attr(SwitchAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&str> {
		self.get_attr(SwitchAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Switch {}

impl common_attrs::PresentationAttributesSetter for Switch {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(SwitchAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(SwitchAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Switch {}

impl Tag for Switch {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("switch");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum SymbolAttrs {
	Class,
	ExternalResourcesRequired,
	PreserveAspectRatio,
	Style,
	ViewBox,
	CoreAttributes(common_attrs::CoreAttributes),
	GraphicalEventAttributes(common_attrs::GraphicalEventAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::CoreAttributes> for SymbolAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::A {}
	impl Content for super::AltGlyphDef {}
	impl Content for super::ClipPath {}
	impl Content for super::ColorProfile {}
	impl Content for super::Cursor {}
	impl Content for super::Filter {}
	impl Content for super::Font {}
	impl Content for super::FontFace {}
	impl Content for super::ForeignObject {}
	impl Content for super::Image {}
	impl Content for super::Marker {}
	impl Content for super::Mask {}
	impl Content for super::Pattern {}
	impl Content for super::Script {}
	impl Content for super::Style {}
	impl Content for super::Switch {}
	impl Content for super::Text {}
	impl Content for super::View {}
}

#[doc = "The [`<symbol>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("symbol.md")]
#[doc = "\n\n [`<symbol>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/symbol"]
#[derive(Debug)]
pub struct Symbol {
	attrs: IndexMap<SymbolAttrs, String>,
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

	fn set_attr(&mut self, attr: SymbolAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: SymbolAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(SymbolAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(SymbolAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(SymbolAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(SymbolAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(SymbolAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(SymbolAttrs::ExternalResourcesRequired)
	}

	/// Set the `preserveAspectRatio` attribute.
	pub fn with_preserve_aspect_ratio<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(SymbolAttrs::PreserveAspectRatio, value.into());
		self
	}

	/// Set the `preserveAspectRatio` attribute.
	pub fn set_preserve_aspect_ratio<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(SymbolAttrs::PreserveAspectRatio, value.into());
	}

	/// Get the `preserveAspectRatio` attribute.
	pub fn preserve_aspect_ratio(&self) -> Option<&str> {
		self.get_attr(SymbolAttrs::PreserveAspectRatio)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(SymbolAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(SymbolAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(SymbolAttrs::Style)
	}

	/// Set the `viewBox` attribute.
	pub fn with_view_box<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(SymbolAttrs::ViewBox, value.into());
		self
	}

	/// Set the `viewBox` attribute.
	pub fn set_view_box<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(SymbolAttrs::ViewBox, value.into());
	}

	/// Get the `viewBox` attribute.
	pub fn view_box(&self) -> Option<&str> {
		self.get_attr(SymbolAttrs::ViewBox)
	}
}

impl common_attrs::CoreAttributesSetter for Symbol {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(SymbolAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(SymbolAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Symbol {}

impl common_attrs::GraphicalEventAttributesSetter for Symbol {
	fn set_attr(&mut self, attr: common_attrs::GraphicalEventAttributes, value: String) {
		self.set_attr(SymbolAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&str> {
		self.get_attr(SymbolAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Symbol {}

impl common_attrs::PresentationAttributesSetter for Symbol {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(SymbolAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(SymbolAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Symbol {}

impl Tag for Symbol {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("symbol");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::A {}
	impl Content for String {}
	impl Content for &'static str {}
}

#[doc = "The [`<text>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("text.md")]
#[doc = "\n\n [`<text>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/text"]
#[derive(Debug)]
pub struct Text {
	attrs: IndexMap<TextAttrs, String>,
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

	fn set_attr(&mut self, attr: TextAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: TextAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TextAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TextAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(TextAttrs::Class)
	}

	/// Set the `dx` attribute.
	pub fn with_dx<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TextAttrs::Dx, value.into());
		self
	}

	/// Set the `dx` attribute.
	pub fn set_dx<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TextAttrs::Dx, value.into());
	}

	/// Get the `dx` attribute.
	pub fn dx(&self) -> Option<&str> {
		self.get_attr(TextAttrs::Dx)
	}

	/// Set the `dy` attribute.
	pub fn with_dy<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TextAttrs::Dy, value.into());
		self
	}

	/// Set the `dy` attribute.
	pub fn set_dy<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TextAttrs::Dy, value.into());
	}

	/// Get the `dy` attribute.
	pub fn dy(&self) -> Option<&str> {
		self.get_attr(TextAttrs::Dy)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TextAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TextAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(TextAttrs::ExternalResourcesRequired)
	}

	/// Set the `lengthAdjust` attribute.
	pub fn with_length_adjust<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TextAttrs::LengthAdjust, value.into());
		self
	}

	/// Set the `lengthAdjust` attribute.
	pub fn set_length_adjust<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TextAttrs::LengthAdjust, value.into());
	}

	/// Get the `lengthAdjust` attribute.
	pub fn length_adjust(&self) -> Option<&str> {
		self.get_attr(TextAttrs::LengthAdjust)
	}

	/// Set the `rotate` attribute.
	pub fn with_rotate<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TextAttrs::Rotate, value.into());
		self
	}

	/// Set the `rotate` attribute.
	pub fn set_rotate<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TextAttrs::Rotate, value.into());
	}

	/// Get the `rotate` attribute.
	pub fn rotate(&self) -> Option<&str> {
		self.get_attr(TextAttrs::Rotate)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TextAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TextAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(TextAttrs::Style)
	}

	/// Set the `text-anchor` attribute.
	pub fn with_text_anchor<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TextAttrs::TextAnchor, value.into());
		self
	}

	/// Set the `text-anchor` attribute.
	pub fn set_text_anchor<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TextAttrs::TextAnchor, value.into());
	}

	/// Get the `text-anchor` attribute.
	pub fn text_anchor(&self) -> Option<&str> {
		self.get_attr(TextAttrs::TextAnchor)
	}

	/// Set the `textLength` attribute.
	pub fn with_text_length<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TextAttrs::TextLength, value.into());
		self
	}

	/// Set the `textLength` attribute.
	pub fn set_text_length<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TextAttrs::TextLength, value.into());
	}

	/// Get the `textLength` attribute.
	pub fn text_length(&self) -> Option<&str> {
		self.get_attr(TextAttrs::TextLength)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TextAttrs::Transform, value.into());
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TextAttrs::Transform, value.into());
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&str> {
		self.get_attr(TextAttrs::Transform)
	}

	/// Set the `x` attribute.
	pub fn with_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TextAttrs::X, value.into());
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TextAttrs::X, value.into());
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&str> {
		self.get_attr(TextAttrs::X)
	}

	/// Set the `y` attribute.
	pub fn with_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TextAttrs::Y, value.into());
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TextAttrs::Y, value.into());
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&str> {
		self.get_attr(TextAttrs::Y)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Text {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(TextAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(TextAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Text {}

impl common_attrs::CoreAttributesSetter for Text {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(TextAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(TextAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Text {}

impl common_attrs::GraphicalEventAttributesSetter for Text {
	fn set_attr(&mut self, attr: common_attrs::GraphicalEventAttributes, value: String) {
		self.set_attr(TextAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&str> {
		self.get_attr(TextAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Text {}

impl common_attrs::PresentationAttributesSetter for Text {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(TextAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(TextAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Text {}

impl Tag for Text {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("text");
		w.set_preserve_whitespaces(true);
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
		w.set_preserve_whitespaces(false);
	}
}

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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::A {}
	impl Content for super::AltGlyph {}
	impl Content for super::Animate {}
	impl Content for super::AnimateColor {}
	impl Content for super::Set {}
	impl Content for super::Tref {}
	impl Content for super::Tspan {}
}

#[doc = "The [`<textPath>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("textPath.md")]
#[doc = "\n\n [`<textPath>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/textPath"]
#[derive(Debug)]
pub struct TextPath {
	attrs: IndexMap<TextPathAttrs, String>,
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

	fn set_attr(&mut self, attr: TextPathAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: TextPathAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TextPathAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TextPathAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(TextPathAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TextPathAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TextPathAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(TextPathAttrs::ExternalResourcesRequired)
	}

	/// Set the `method` attribute.
	pub fn with_method<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TextPathAttrs::Method, value.into());
		self
	}

	/// Set the `method` attribute.
	pub fn set_method<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TextPathAttrs::Method, value.into());
	}

	/// Get the `method` attribute.
	pub fn method(&self) -> Option<&str> {
		self.get_attr(TextPathAttrs::Method)
	}

	/// Set the `spacing` attribute.
	pub fn with_spacing<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TextPathAttrs::Spacing, value.into());
		self
	}

	/// Set the `spacing` attribute.
	pub fn set_spacing<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TextPathAttrs::Spacing, value.into());
	}

	/// Get the `spacing` attribute.
	pub fn spacing(&self) -> Option<&str> {
		self.get_attr(TextPathAttrs::Spacing)
	}

	/// Set the `startOffset` attribute.
	pub fn with_start_offset<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TextPathAttrs::StartOffset, value.into());
		self
	}

	/// Set the `startOffset` attribute.
	pub fn set_start_offset<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TextPathAttrs::StartOffset, value.into());
	}

	/// Get the `startOffset` attribute.
	pub fn start_offset(&self) -> Option<&str> {
		self.get_attr(TextPathAttrs::StartOffset)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TextPathAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TextPathAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(TextPathAttrs::Style)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TextPathAttrs::XlinkHref, value.into());
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TextPathAttrs::XlinkHref, value.into());
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&str> {
		self.get_attr(TextPathAttrs::XlinkHref)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for TextPath {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(TextPathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(TextPathAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for TextPath {}

impl common_attrs::CoreAttributesSetter for TextPath {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(TextPathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(TextPathAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for TextPath {}

impl common_attrs::GraphicalEventAttributesSetter for TextPath {
	fn set_attr(&mut self, attr: common_attrs::GraphicalEventAttributes, value: String) {
		self.set_attr(TextPathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&str> {
		self.get_attr(TextPathAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for TextPath {}

impl common_attrs::PresentationAttributesSetter for TextPath {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(TextPathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(TextPathAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for TextPath {}

impl common_attrs::XLinkAttributesSetter for TextPath {
	fn set_attr(&mut self, attr: common_attrs::XLinkAttributes, value: String) {
		self.set_attr(TextPathAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&str> {
		self.get_attr(TextPathAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for TextPath {}

impl Tag for TextPath {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("textPath");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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
	attrs: IndexMap<TitleAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: TitleAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: TitleAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TitleAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TitleAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(TitleAttrs::Class)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TitleAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TitleAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(TitleAttrs::Style)
	}
}

impl common_attrs::CoreAttributesSetter for Title {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(TitleAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(TitleAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Title {}

impl Tag for Title {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("title");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum TrefAttrs {
	Class,
	ExternalResourcesRequired,
	Style,
	XlinkHref,
	ConditionalProcessingAttributes(common_attrs::ConditionalProcessingAttributes),
	CoreAttributes(common_attrs::CoreAttributes),
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
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::Animate {}
	impl Content for super::AnimateColor {}
	impl Content for super::Set {}
}

#[doc = "The [`<tref>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("tref.md")]
#[doc = "\n\n [`<tref>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/tref"]
#[derive(Debug)]
pub struct Tref {
	attrs: IndexMap<TrefAttrs, String>,
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

	fn set_attr(&mut self, attr: TrefAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: TrefAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TrefAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TrefAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(TrefAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TrefAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TrefAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(TrefAttrs::ExternalResourcesRequired)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TrefAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TrefAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(TrefAttrs::Style)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TrefAttrs::XlinkHref, value.into());
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TrefAttrs::XlinkHref, value.into());
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&str> {
		self.get_attr(TrefAttrs::XlinkHref)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Tref {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(TrefAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(TrefAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Tref {}

impl common_attrs::CoreAttributesSetter for Tref {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(TrefAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(TrefAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Tref {}

impl common_attrs::GraphicalEventAttributesSetter for Tref {
	fn set_attr(&mut self, attr: common_attrs::GraphicalEventAttributes, value: String) {
		self.set_attr(TrefAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&str> {
		self.get_attr(TrefAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Tref {}

impl common_attrs::PresentationAttributesSetter for Tref {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(TrefAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(TrefAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Tref {}

impl common_attrs::XLinkAttributesSetter for Tref {
	fn set_attr(&mut self, attr: common_attrs::XLinkAttributes, value: String) {
		self.set_attr(TrefAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&str> {
		self.get_attr(TrefAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for Tref {}

impl Tag for Tref {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("tref");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum TspanAttrs {
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
	GraphicalEventAttributes(common_attrs::GraphicalEventAttributes),
	PresentationAttributes(common_attrs::PresentationAttributes),
}

impl From<common_attrs::ConditionalProcessingAttributes> for TspanAttrs {
	fn from(attr: common_attrs::ConditionalProcessingAttributes) -> Self {
		Self::ConditionalProcessingAttributes(attr)
	}
}

impl From<common_attrs::CoreAttributes> for TspanAttrs {
	fn from(attr: common_attrs::CoreAttributes) -> Self {
		Self::CoreAttributes(attr)
	}
}

impl From<common_attrs::GraphicalEventAttributes> for TspanAttrs {
	fn from(attr: common_attrs::GraphicalEventAttributes) -> Self {
		Self::GraphicalEventAttributes(attr)
	}
}

impl From<common_attrs::PresentationAttributes> for TspanAttrs {
	fn from(attr: common_attrs::PresentationAttributes) -> Self {
		Self::PresentationAttributes(attr)
	}
}

impl TspanAttrs {
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
			Self::GraphicalEventAttributes(attr) => attr.as_str(),
			Self::PresentationAttributes(attr) => attr.as_str(),
		}
	}
}

impl Display for TspanAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for TspanAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

mod tspan_private {
	use crate::Tag;
	use std::fmt::Debug;

	pub trait Content: Tag + Debug {}
	impl Content for super::A {}
	impl Content for super::AltGlyph {}
	impl Content for super::Animate {}
	impl Content for super::AnimateColor {}
	impl Content for super::Set {}
	impl Content for super::Tref {}
	impl Content for super::Tspan {}
	impl Content for String {}
	impl Content for &'static str {}
}

#[doc = "The [`<tspan>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("tspan.md")]
#[doc = "\n\n [`<tspan>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/tspan"]
#[derive(Debug)]
pub struct Tspan {
	attrs: IndexMap<TspanAttrs, String>,
	content: Vec<Box<dyn tspan_private::Content>>,
}

impl Default for Tspan {
	fn default() -> Self {
		Self::new()
	}
}

impl Tspan {
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

	fn set_attr(&mut self, attr: TspanAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: TspanAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TspanAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TspanAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(TspanAttrs::Class)
	}

	/// Set the `dx` attribute.
	pub fn with_dx<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TspanAttrs::Dx, value.into());
		self
	}

	/// Set the `dx` attribute.
	pub fn set_dx<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TspanAttrs::Dx, value.into());
	}

	/// Get the `dx` attribute.
	pub fn dx(&self) -> Option<&str> {
		self.get_attr(TspanAttrs::Dx)
	}

	/// Set the `dy` attribute.
	pub fn with_dy<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TspanAttrs::Dy, value.into());
		self
	}

	/// Set the `dy` attribute.
	pub fn set_dy<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TspanAttrs::Dy, value.into());
	}

	/// Get the `dy` attribute.
	pub fn dy(&self) -> Option<&str> {
		self.get_attr(TspanAttrs::Dy)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TspanAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TspanAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(TspanAttrs::ExternalResourcesRequired)
	}

	/// Set the `lengthAdjust` attribute.
	pub fn with_length_adjust<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TspanAttrs::LengthAdjust, value.into());
		self
	}

	/// Set the `lengthAdjust` attribute.
	pub fn set_length_adjust<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TspanAttrs::LengthAdjust, value.into());
	}

	/// Get the `lengthAdjust` attribute.
	pub fn length_adjust(&self) -> Option<&str> {
		self.get_attr(TspanAttrs::LengthAdjust)
	}

	/// Set the `rotate` attribute.
	pub fn with_rotate<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TspanAttrs::Rotate, value.into());
		self
	}

	/// Set the `rotate` attribute.
	pub fn set_rotate<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TspanAttrs::Rotate, value.into());
	}

	/// Get the `rotate` attribute.
	pub fn rotate(&self) -> Option<&str> {
		self.get_attr(TspanAttrs::Rotate)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TspanAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TspanAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(TspanAttrs::Style)
	}

	/// Set the `textLength` attribute.
	pub fn with_text_length<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TspanAttrs::TextLength, value.into());
		self
	}

	/// Set the `textLength` attribute.
	pub fn set_text_length<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TspanAttrs::TextLength, value.into());
	}

	/// Get the `textLength` attribute.
	pub fn text_length(&self) -> Option<&str> {
		self.get_attr(TspanAttrs::TextLength)
	}

	/// Set the `x` attribute.
	pub fn with_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TspanAttrs::X, value.into());
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TspanAttrs::X, value.into());
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&str> {
		self.get_attr(TspanAttrs::X)
	}

	/// Set the `y` attribute.
	pub fn with_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TspanAttrs::Y, value.into());
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TspanAttrs::Y, value.into());
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&str> {
		self.get_attr(TspanAttrs::Y)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Tspan {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(TspanAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(TspanAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Tspan {}

impl common_attrs::CoreAttributesSetter for Tspan {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(TspanAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(TspanAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Tspan {}

impl common_attrs::GraphicalEventAttributesSetter for Tspan {
	fn set_attr(&mut self, attr: common_attrs::GraphicalEventAttributes, value: String) {
		self.set_attr(TspanAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&str> {
		self.get_attr(TspanAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Tspan {}

impl common_attrs::PresentationAttributesSetter for Tspan {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(TspanAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(TspanAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Tspan {}

impl Tag for Tspan {
	fn write_to(&self, w: &mut XmlWriter,pretty: bool) {
		w.start_element("tspan");
		w.set_preserve_whitespaces(true);
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		for tag in &self.content {
			tag.write_to(w, pretty);
		}
		w.end_element();
	}
}

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

#[doc = "The [`<use>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("use.md")]
#[doc = "\n\n [`<use>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/use"]
#[derive(Debug)]
pub struct Use {
	attrs: IndexMap<UseAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: UseAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: UseAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(UseAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(UseAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.get_attr(UseAttrs::Class)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(UseAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(UseAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(UseAttrs::ExternalResourcesRequired)
	}

	/// Set the `height` attribute.
	pub fn with_height<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(UseAttrs::Height, value.into());
		self
	}

	/// Set the `height` attribute.
	pub fn set_height<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(UseAttrs::Height, value.into());
	}

	/// Get the `height` attribute.
	pub fn height(&self) -> Option<&str> {
		self.get_attr(UseAttrs::Height)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(UseAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(UseAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.get_attr(UseAttrs::Style)
	}

	/// Set the `transform` attribute.
	pub fn with_transform<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(UseAttrs::Transform, value.into());
		self
	}

	/// Set the `transform` attribute.
	pub fn set_transform<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(UseAttrs::Transform, value.into());
	}

	/// Get the `transform` attribute.
	pub fn transform(&self) -> Option<&str> {
		self.get_attr(UseAttrs::Transform)
	}

	/// Set the `width` attribute.
	pub fn with_width<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(UseAttrs::Width, value.into());
		self
	}

	/// Set the `width` attribute.
	pub fn set_width<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(UseAttrs::Width, value.into());
	}

	/// Get the `width` attribute.
	pub fn width(&self) -> Option<&str> {
		self.get_attr(UseAttrs::Width)
	}

	/// Set the `x` attribute.
	pub fn with_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(UseAttrs::X, value.into());
		self
	}

	/// Set the `x` attribute.
	pub fn set_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(UseAttrs::X, value.into());
	}

	/// Get the `x` attribute.
	pub fn x(&self) -> Option<&str> {
		self.get_attr(UseAttrs::X)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(UseAttrs::XlinkHref, value.into());
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(UseAttrs::XlinkHref, value.into());
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&str> {
		self.get_attr(UseAttrs::XlinkHref)
	}

	/// Set the `y` attribute.
	pub fn with_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(UseAttrs::Y, value.into());
		self
	}

	/// Set the `y` attribute.
	pub fn set_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(UseAttrs::Y, value.into());
	}

	/// Get the `y` attribute.
	pub fn y(&self) -> Option<&str> {
		self.get_attr(UseAttrs::Y)
	}
}

impl common_attrs::ConditionalProcessingAttributesSetter for Use {
	fn set_attr(&mut self, attr: common_attrs::ConditionalProcessingAttributes, value: String) {
		self.set_attr(UseAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::ConditionalProcessingAttributes) -> Option<&str> {
		self.get_attr(UseAttrs::from(attr))
	}
}

impl TagWithConditionalProcessingAttributes for Use {}

impl common_attrs::CoreAttributesSetter for Use {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(UseAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(UseAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Use {}

impl common_attrs::GraphicalEventAttributesSetter for Use {
	fn set_attr(&mut self, attr: common_attrs::GraphicalEventAttributes, value: String) {
		self.set_attr(UseAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::GraphicalEventAttributes) -> Option<&str> {
		self.get_attr(UseAttrs::from(attr))
	}
}

impl TagWithGraphicalEventAttributes for Use {}

impl common_attrs::PresentationAttributesSetter for Use {
	fn set_attr(&mut self, attr: common_attrs::PresentationAttributes, value: String) {
		self.set_attr(UseAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::PresentationAttributes) -> Option<&str> {
		self.get_attr(UseAttrs::from(attr))
	}
}

impl TagWithPresentationAttributes for Use {}

impl common_attrs::XLinkAttributesSetter for Use {
	fn set_attr(&mut self, attr: common_attrs::XLinkAttributes, value: String) {
		self.set_attr(UseAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::XLinkAttributes) -> Option<&str> {
		self.get_attr(UseAttrs::from(attr))
	}
}

impl TagWithXLinkAttributes for Use {}

impl Tag for Use {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("use");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

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

#[doc = "The [`<view>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("view.md")]
#[doc = "\n\n [`<view>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/view"]
#[derive(Debug)]
pub struct View {
	attrs: IndexMap<ViewAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: ViewAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: ViewAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn with_external_resources_required<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ViewAttrs::ExternalResourcesRequired, value.into());
		self
	}

	/// Set the `externalResourcesRequired` attribute.
	pub fn set_external_resources_required<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ViewAttrs::ExternalResourcesRequired, value.into());
	}

	/// Get the `externalResourcesRequired` attribute.
	pub fn external_resources_required(&self) -> Option<&str> {
		self.get_attr(ViewAttrs::ExternalResourcesRequired)
	}

	/// Set the `preserveAspectRatio` attribute.
	pub fn with_preserve_aspect_ratio<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ViewAttrs::PreserveAspectRatio, value.into());
		self
	}

	/// Set the `preserveAspectRatio` attribute.
	pub fn set_preserve_aspect_ratio<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ViewAttrs::PreserveAspectRatio, value.into());
	}

	/// Get the `preserveAspectRatio` attribute.
	pub fn preserve_aspect_ratio(&self) -> Option<&str> {
		self.get_attr(ViewAttrs::PreserveAspectRatio)
	}

	/// Set the `viewBox` attribute.
	pub fn with_view_box<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ViewAttrs::ViewBox, value.into());
		self
	}

	/// Set the `viewBox` attribute.
	pub fn set_view_box<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ViewAttrs::ViewBox, value.into());
	}

	/// Get the `viewBox` attribute.
	pub fn view_box(&self) -> Option<&str> {
		self.get_attr(ViewAttrs::ViewBox)
	}

	/// Set the `viewTarget` attribute.
	pub fn with_view_target<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ViewAttrs::ViewTarget, value.into());
		self
	}

	/// Set the `viewTarget` attribute.
	pub fn set_view_target<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ViewAttrs::ViewTarget, value.into());
	}

	/// Get the `viewTarget` attribute.
	pub fn view_target(&self) -> Option<&str> {
		self.get_attr(ViewAttrs::ViewTarget)
	}

	/// Set the `zoomAndPan` attribute.
	pub fn with_zoom_and_pan<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ViewAttrs::ZoomAndPan, value.into());
		self
	}

	/// Set the `zoomAndPan` attribute.
	pub fn set_zoom_and_pan<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ViewAttrs::ZoomAndPan, value.into());
	}

	/// Get the `zoomAndPan` attribute.
	pub fn zoom_and_pan(&self) -> Option<&str> {
		self.get_attr(ViewAttrs::ZoomAndPan)
	}
}

impl common_attrs::CoreAttributesSetter for View {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(ViewAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(ViewAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for View {}

impl Tag for View {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("view");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

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
#[doc = "\n\n [`<vkern>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/vkern"]
#[derive(Debug)]
pub struct Vkern {
	attrs: IndexMap<VkernAttrs, String>,
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

	fn set_attr(&mut self, attr: VkernAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	fn get_attr(&self, attr: VkernAttrs) -> Option<&str> {
		self.attrs.get(&attr).map(String::as_str)
	}

	/// Set the `g1` attribute.
	pub fn with_g1<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(VkernAttrs::G1, value.into());
		self
	}

	/// Set the `g1` attribute.
	pub fn set_g1<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(VkernAttrs::G1, value.into());
	}

	/// Get the `g1` attribute.
	pub fn g1(&self) -> Option<&str> {
		self.get_attr(VkernAttrs::G1)
	}

	/// Set the `g2` attribute.
	pub fn with_g2<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(VkernAttrs::G2, value.into());
		self
	}

	/// Set the `g2` attribute.
	pub fn set_g2<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(VkernAttrs::G2, value.into());
	}

	/// Get the `g2` attribute.
	pub fn g2(&self) -> Option<&str> {
		self.get_attr(VkernAttrs::G2)
	}

	/// Set the `k` attribute.
	pub fn with_k<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(VkernAttrs::K, value.into());
		self
	}

	/// Set the `k` attribute.
	pub fn set_k<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(VkernAttrs::K, value.into());
	}

	/// Get the `k` attribute.
	pub fn k(&self) -> Option<&str> {
		self.get_attr(VkernAttrs::K)
	}

	/// Set the `u1` attribute.
	pub fn with_u1<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(VkernAttrs::U1, value.into());
		self
	}

	/// Set the `u1` attribute.
	pub fn set_u1<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(VkernAttrs::U1, value.into());
	}

	/// Get the `u1` attribute.
	pub fn u1(&self) -> Option<&str> {
		self.get_attr(VkernAttrs::U1)
	}

	/// Set the `u2` attribute.
	pub fn with_u2<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(VkernAttrs::U2, value.into());
		self
	}

	/// Set the `u2` attribute.
	pub fn set_u2<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(VkernAttrs::U2, value.into());
	}

	/// Get the `u2` attribute.
	pub fn u2(&self) -> Option<&str> {
		self.get_attr(VkernAttrs::U2)
	}
}

impl common_attrs::CoreAttributesSetter for Vkern {
	fn set_attr(&mut self, attr: common_attrs::CoreAttributes, value: String) {
		self.set_attr(VkernAttrs::from(attr), value);
	}

	fn get_attr(&self, attr: common_attrs::CoreAttributes) -> Option<&str> {
		self.get_attr(VkernAttrs::from(attr))
	}
}

impl TagWithCoreAttributes for Vkern {}

impl Tag for Vkern {
	fn write_to(&self, w: &mut XmlWriter,_pretty: bool) {
		w.start_element("vkern");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}
