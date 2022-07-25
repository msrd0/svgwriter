// @generated

use crate::Tag;
use indexmap::IndexMap;
use std::fmt::{self, Debug, Display, Formatter};
use xmlwriter::XmlWriter;

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

#[doc = "The [`<a>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("a.md")]
#[doc = "\n\n [`<a>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/a"]
#[derive(Clone, Debug)]
pub struct A {
	attrs: IndexMap<AAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: AAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&AAttrs::Class).map(String::as_str)
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
		self.attrs.get(&AAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&AAttrs::Style).map(String::as_str)
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
		self.attrs.get(&AAttrs::Target).map(String::as_str)
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
		self.attrs.get(&AAttrs::Transform).map(String::as_str)
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
		self.attrs.get(&AAttrs::XlinkActuate).map(String::as_str)
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
		self.attrs.get(&AAttrs::XlinkHref).map(String::as_str)
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
		self.attrs.get(&AAttrs::XlinkShow).map(String::as_str)
	}
}

impl Tag for A {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("a");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
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
#[derive(Clone, Debug)]
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
		self.attrs.get(&AltGlyphAttrs::Class).map(String::as_str)
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
		self.attrs.get(&AltGlyphAttrs::Dx).map(String::as_str)
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
		self.attrs.get(&AltGlyphAttrs::Dy).map(String::as_str)
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
		self.attrs.get(&AltGlyphAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&AltGlyphAttrs::Format).map(String::as_str)
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
		self.attrs.get(&AltGlyphAttrs::GlyphRef).map(String::as_str)
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
		self.attrs.get(&AltGlyphAttrs::Rotate).map(String::as_str)
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
		self.attrs.get(&AltGlyphAttrs::Style).map(String::as_str)
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
		self.attrs.get(&AltGlyphAttrs::X).map(String::as_str)
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
		self.attrs.get(&AltGlyphAttrs::XlinkHref).map(String::as_str)
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
		self.attrs.get(&AltGlyphAttrs::Y).map(String::as_str)
	}
}

impl Tag for AltGlyph {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("altGlyph");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[doc = "The [`<altGlyphDef>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("altGlyphDef.md")]
#[doc = "\n\n [`<altGlyphDef>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/altGlyphDef"]
#[derive(Clone, Debug)]
pub struct AltGlyphDef {
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
		}
	}
}

impl Tag for AltGlyphDef {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("altGlyphDef");w.end_element();
	}
}

#[doc = "The [`<altGlyphItem>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("altGlyphItem.md")]
#[doc = "\n\n [`<altGlyphItem>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/altGlyphItem"]
#[derive(Clone, Debug)]
pub struct AltGlyphItem {
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
		}
	}
}

impl Tag for AltGlyphItem {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("altGlyphItem");w.end_element();
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
#[derive(Clone, Debug)]
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
		self.attrs.get(&AnimateAttrs::AttributeName).map(String::as_str)
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
		self.attrs.get(&AnimateAttrs::AttributeType).map(String::as_str)
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
		self.attrs.get(&AnimateAttrs::Dur).map(String::as_str)
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
		self.attrs.get(&AnimateAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&AnimateAttrs::From).map(String::as_str)
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
		self.attrs.get(&AnimateAttrs::RepeatCount).map(String::as_str)
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
		self.attrs.get(&AnimateAttrs::To).map(String::as_str)
	}
}

impl Tag for Animate {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
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
}

impl AnimateColorAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::By => "by",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::From => "from",
			Self::To => "to",
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
#[derive(Clone, Debug)]
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
		self.attrs.get(&AnimateColorAttrs::By).map(String::as_str)
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
		self.attrs.get(&AnimateColorAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&AnimateColorAttrs::From).map(String::as_str)
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
		self.attrs.get(&AnimateColorAttrs::To).map(String::as_str)
	}
}

impl Tag for AnimateColor {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
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

#[doc = "The [`<animateMotion>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("animateMotion.md")]
#[doc = "\n\n [`<animateMotion>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/animateMotion"]
#[derive(Clone, Debug)]
pub struct AnimateMotion {
	attrs: IndexMap<AnimateMotionAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: AnimateMotionAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&AnimateMotionAttrs::CalcMode).map(String::as_str)
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
		self.attrs.get(&AnimateMotionAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&AnimateMotionAttrs::KeyPoints).map(String::as_str)
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
		self.attrs.get(&AnimateMotionAttrs::Origin).map(String::as_str)
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
		self.attrs.get(&AnimateMotionAttrs::Path).map(String::as_str)
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
		self.attrs.get(&AnimateMotionAttrs::Rotate).map(String::as_str)
	}
}

impl Tag for AnimateMotion {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("animateMotion");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
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
}

impl AnimateTransformAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::By => "by",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::From => "from",
			Self::To => "to",
			Self::Type => "type",
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
#[derive(Clone, Debug)]
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
		self.attrs.get(&AnimateTransformAttrs::By).map(String::as_str)
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
		self.attrs.get(&AnimateTransformAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&AnimateTransformAttrs::From).map(String::as_str)
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
		self.attrs.get(&AnimateTransformAttrs::To).map(String::as_str)
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
		self.attrs.get(&AnimateTransformAttrs::Type).map(String::as_str)
	}
}

impl Tag for AnimateTransform {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
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
#[derive(Clone, Debug)]
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
		self.attrs.get(&CircleAttrs::Class).map(String::as_str)
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
		self.attrs.get(&CircleAttrs::Cx).map(String::as_str)
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
		self.attrs.get(&CircleAttrs::Cy).map(String::as_str)
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
		self.attrs.get(&CircleAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&CircleAttrs::R).map(String::as_str)
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
		self.attrs.get(&CircleAttrs::Style).map(String::as_str)
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
		self.attrs.get(&CircleAttrs::Transform).map(String::as_str)
	}
}

impl Tag for Circle {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
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
}

impl ClipPathAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ClipPathUnits => "clipPathUnits",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Style => "style",
			Self::Transform => "transform",
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

#[doc = "The [`<clipPath>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("clipPath.md")]
#[doc = "\n\n [`<clipPath>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/clipPath"]
#[derive(Clone, Debug)]
pub struct ClipPath {
	attrs: IndexMap<ClipPathAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: ClipPathAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&ClipPathAttrs::Class).map(String::as_str)
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
		self.attrs.get(&ClipPathAttrs::ClipPathUnits).map(String::as_str)
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
		self.attrs.get(&ClipPathAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&ClipPathAttrs::Style).map(String::as_str)
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
		self.attrs.get(&ClipPathAttrs::Transform).map(String::as_str)
	}
}

impl Tag for ClipPath {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("clipPath");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum ColorMinusProfileAttrs {
	Local,
	Name,
	RenderingMinusIntent,
	XlinkHref,
}

impl ColorMinusProfileAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Local => "local",
			Self::Name => "name",
			Self::RenderingMinusIntent => "rendering-intent",
			Self::XlinkHref => "xlink:href",
		}
	}
}

impl Display for ColorMinusProfileAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for ColorMinusProfileAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[doc = "The [`<color-profile>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("color-profile.md")]
#[doc = "\n\n [`<color-profile>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/color-profile"]
#[derive(Clone, Debug)]
pub struct ColorMinusProfile {
	attrs: IndexMap<ColorMinusProfileAttrs, String>,
}

impl Default for ColorMinusProfile {
	fn default() -> Self {
		Self::new()
	}
}

impl ColorMinusProfile {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
		}
	}

	fn set_attr(&mut self, attr: ColorMinusProfileAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	/// Set the `local` attribute.
	pub fn with_local<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ColorMinusProfileAttrs::Local, value.into());
		self
	}

	/// Set the `local` attribute.
	pub fn set_local<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ColorMinusProfileAttrs::Local, value.into());
	}

	/// Get the `local` attribute.
	pub fn local(&self) -> Option<&str> {
		self.attrs.get(&ColorMinusProfileAttrs::Local).map(String::as_str)
	}

	/// Set the `name` attribute.
	pub fn with_name<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ColorMinusProfileAttrs::Name, value.into());
		self
	}

	/// Set the `name` attribute.
	pub fn set_name<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ColorMinusProfileAttrs::Name, value.into());
	}

	/// Get the `name` attribute.
	pub fn name(&self) -> Option<&str> {
		self.attrs.get(&ColorMinusProfileAttrs::Name).map(String::as_str)
	}

	/// Set the `rendering-intent` attribute.
	pub fn with_rendering_minus_intent<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ColorMinusProfileAttrs::RenderingMinusIntent, value.into());
		self
	}

	/// Set the `rendering-intent` attribute.
	pub fn set_rendering_minus_intent<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ColorMinusProfileAttrs::RenderingMinusIntent, value.into());
	}

	/// Get the `rendering-intent` attribute.
	pub fn rendering_minus_intent(&self) -> Option<&str> {
		self.attrs.get(&ColorMinusProfileAttrs::RenderingMinusIntent).map(String::as_str)
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ColorMinusProfileAttrs::XlinkHref, value.into());
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ColorMinusProfileAttrs::XlinkHref, value.into());
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&str> {
		self.attrs.get(&ColorMinusProfileAttrs::XlinkHref).map(String::as_str)
	}
}

impl Tag for ColorMinusProfile {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
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
}

impl CursorAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::X => "x",
			Self::XlinkHref => "xlink:href",
			Self::Y => "y",
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
#[derive(Clone, Debug)]
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
		self.attrs.get(&CursorAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&CursorAttrs::X).map(String::as_str)
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
		self.attrs.get(&CursorAttrs::XlinkHref).map(String::as_str)
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
		self.attrs.get(&CursorAttrs::Y).map(String::as_str)
	}
}

impl Tag for Cursor {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
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
}

impl DefsAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Style => "style",
			Self::Transform => "transform",
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

#[doc = "The [`<defs>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("defs.md")]
#[doc = "\n\n [`<defs>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/defs"]
#[derive(Clone, Debug)]
pub struct Defs {
	attrs: IndexMap<DefsAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: DefsAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&DefsAttrs::Class).map(String::as_str)
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
		self.attrs.get(&DefsAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&DefsAttrs::Style).map(String::as_str)
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
		self.attrs.get(&DefsAttrs::Transform).map(String::as_str)
	}
}

impl Tag for Defs {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("defs");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum DescAttrs {
	Class,
	Style,
}

impl DescAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::Style => "style",
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
#[derive(Clone, Debug)]
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
		self.attrs.get(&DescAttrs::Class).map(String::as_str)
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
		self.attrs.get(&DescAttrs::Style).map(String::as_str)
	}
}

impl Tag for Desc {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
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
}

impl DiscardAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Begin => "begin",
			Self::Href => "href",
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

#[doc = "The [`<discard>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("discard.md")]
#[doc = "\n\n [`<discard>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/discard"]
#[derive(Clone, Debug)]
pub struct Discard {
	attrs: IndexMap<DiscardAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: DiscardAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&DiscardAttrs::Begin).map(String::as_str)
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
		self.attrs.get(&DiscardAttrs::Href).map(String::as_str)
	}
}

impl Tag for Discard {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("discard");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
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
#[derive(Clone, Debug)]
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
		self.attrs.get(&EllipseAttrs::Class).map(String::as_str)
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
		self.attrs.get(&EllipseAttrs::Cx).map(String::as_str)
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
		self.attrs.get(&EllipseAttrs::Cy).map(String::as_str)
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
		self.attrs.get(&EllipseAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&EllipseAttrs::Rx).map(String::as_str)
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
		self.attrs.get(&EllipseAttrs::Ry).map(String::as_str)
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
		self.attrs.get(&EllipseAttrs::Style).map(String::as_str)
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
		self.attrs.get(&EllipseAttrs::Transform).map(String::as_str)
	}
}

impl Tag for Ellipse {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
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
}

impl FeBlendAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::In => "in",
			Self::In2 => "in2",
			Self::Mode => "mode",
			Self::Style => "style",
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

#[doc = "The [`<feBlend>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feBlend.md")]
#[doc = "\n\n [`<feBlend>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feBlend"]
#[derive(Clone, Debug)]
pub struct FeBlend {
	attrs: IndexMap<FeBlendAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: FeBlendAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&FeBlendAttrs::Class).map(String::as_str)
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
		self.attrs.get(&FeBlendAttrs::In).map(String::as_str)
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
		self.attrs.get(&FeBlendAttrs::In2).map(String::as_str)
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
		self.attrs.get(&FeBlendAttrs::Mode).map(String::as_str)
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
		self.attrs.get(&FeBlendAttrs::Style).map(String::as_str)
	}
}

impl Tag for FeBlend {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("feBlend");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
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
}

impl FeColorMatrixAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::In => "in",
			Self::Style => "style",
			Self::Type => "type",
			Self::Values => "values",
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

#[doc = "The [`<feColorMatrix>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feColorMatrix.md")]
#[doc = "\n\n [`<feColorMatrix>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feColorMatrix"]
#[derive(Clone, Debug)]
pub struct FeColorMatrix {
	attrs: IndexMap<FeColorMatrixAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: FeColorMatrixAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&FeColorMatrixAttrs::Class).map(String::as_str)
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
		self.attrs.get(&FeColorMatrixAttrs::In).map(String::as_str)
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
		self.attrs.get(&FeColorMatrixAttrs::Style).map(String::as_str)
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
		self.attrs.get(&FeColorMatrixAttrs::Type).map(String::as_str)
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
		self.attrs.get(&FeColorMatrixAttrs::Values).map(String::as_str)
	}
}

impl Tag for FeColorMatrix {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("feColorMatrix");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeComponentTransferAttrs {
	Class,
	In,
	Style,
}

impl FeComponentTransferAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::In => "in",
			Self::Style => "style",
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

#[doc = "The [`<feComponentTransfer>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feComponentTransfer.md")]
#[doc = "\n\n [`<feComponentTransfer>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feComponentTransfer"]
#[derive(Clone, Debug)]
pub struct FeComponentTransfer {
	attrs: IndexMap<FeComponentTransferAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: FeComponentTransferAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&FeComponentTransferAttrs::Class).map(String::as_str)
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
		self.attrs.get(&FeComponentTransferAttrs::In).map(String::as_str)
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
		self.attrs.get(&FeComponentTransferAttrs::Style).map(String::as_str)
	}
}

impl Tag for FeComponentTransfer {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("feComponentTransfer");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
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

#[doc = "The [`<feComposite>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feComposite.md")]
#[doc = "\n\n [`<feComposite>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feComposite"]
#[derive(Clone, Debug)]
pub struct FeComposite {
	attrs: IndexMap<FeCompositeAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: FeCompositeAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&FeCompositeAttrs::Class).map(String::as_str)
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
		self.attrs.get(&FeCompositeAttrs::In).map(String::as_str)
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
		self.attrs.get(&FeCompositeAttrs::In2).map(String::as_str)
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
		self.attrs.get(&FeCompositeAttrs::K1).map(String::as_str)
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
		self.attrs.get(&FeCompositeAttrs::K2).map(String::as_str)
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
		self.attrs.get(&FeCompositeAttrs::K3).map(String::as_str)
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
		self.attrs.get(&FeCompositeAttrs::K4).map(String::as_str)
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
		self.attrs.get(&FeCompositeAttrs::Operator).map(String::as_str)
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
		self.attrs.get(&FeCompositeAttrs::Style).map(String::as_str)
	}
}

impl Tag for FeComposite {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("feComposite");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
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

#[doc = "The [`<feConvolveMatrix>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feConvolveMatrix.md")]
#[doc = "\n\n [`<feConvolveMatrix>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feConvolveMatrix"]
#[derive(Clone, Debug)]
pub struct FeConvolveMatrix {
	attrs: IndexMap<FeConvolveMatrixAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: FeConvolveMatrixAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&FeConvolveMatrixAttrs::Bias).map(String::as_str)
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
		self.attrs.get(&FeConvolveMatrixAttrs::Class).map(String::as_str)
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
		self.attrs.get(&FeConvolveMatrixAttrs::Divisor).map(String::as_str)
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
		self.attrs.get(&FeConvolveMatrixAttrs::EdgeMode).map(String::as_str)
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
		self.attrs.get(&FeConvolveMatrixAttrs::In).map(String::as_str)
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
		self.attrs.get(&FeConvolveMatrixAttrs::KernelMatrix).map(String::as_str)
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
		self.attrs.get(&FeConvolveMatrixAttrs::KernelUnitLength).map(String::as_str)
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
		self.attrs.get(&FeConvolveMatrixAttrs::Order).map(String::as_str)
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
		self.attrs.get(&FeConvolveMatrixAttrs::PreserveAlpha).map(String::as_str)
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
		self.attrs.get(&FeConvolveMatrixAttrs::Style).map(String::as_str)
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
		self.attrs.get(&FeConvolveMatrixAttrs::TargetX).map(String::as_str)
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
		self.attrs.get(&FeConvolveMatrixAttrs::TargetY).map(String::as_str)
	}
}

impl Tag for FeConvolveMatrix {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("feConvolveMatrix");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
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
#[derive(Clone, Debug)]
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
		self.attrs.get(&FeDiffuseLightingAttrs::Class).map(String::as_str)
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
		self.attrs.get(&FeDiffuseLightingAttrs::DiffuseConstant).map(String::as_str)
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
		self.attrs.get(&FeDiffuseLightingAttrs::In).map(String::as_str)
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
		self.attrs.get(&FeDiffuseLightingAttrs::KernelUnitLength).map(String::as_str)
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
		self.attrs.get(&FeDiffuseLightingAttrs::Style).map(String::as_str)
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
		self.attrs.get(&FeDiffuseLightingAttrs::SurfaceScale).map(String::as_str)
	}
}

impl Tag for FeDiffuseLighting {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
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

#[doc = "The [`<feDisplacementMap>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feDisplacementMap.md")]
#[doc = "\n\n [`<feDisplacementMap>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feDisplacementMap"]
#[derive(Clone, Debug)]
pub struct FeDisplacementMap {
	attrs: IndexMap<FeDisplacementMapAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: FeDisplacementMapAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&FeDisplacementMapAttrs::Class).map(String::as_str)
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
		self.attrs.get(&FeDisplacementMapAttrs::In).map(String::as_str)
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
		self.attrs.get(&FeDisplacementMapAttrs::In2).map(String::as_str)
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
		self.attrs.get(&FeDisplacementMapAttrs::Scale).map(String::as_str)
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
		self.attrs.get(&FeDisplacementMapAttrs::Style).map(String::as_str)
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
		self.attrs.get(&FeDisplacementMapAttrs::XChannelSelector).map(String::as_str)
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
		self.attrs.get(&FeDisplacementMapAttrs::YChannelSelector).map(String::as_str)
	}
}

impl Tag for FeDisplacementMap {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("feDisplacementMap");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeDistantLightAttrs {
	Azimuth,
	Elevation,
}

impl FeDistantLightAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Azimuth => "azimuth",
			Self::Elevation => "elevation",
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

#[doc = "The [`<feDistantLight>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feDistantLight.md")]
#[doc = "\n\n [`<feDistantLight>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feDistantLight"]
#[derive(Clone, Debug)]
pub struct FeDistantLight {
	attrs: IndexMap<FeDistantLightAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: FeDistantLightAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&FeDistantLightAttrs::Azimuth).map(String::as_str)
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
		self.attrs.get(&FeDistantLightAttrs::Elevation).map(String::as_str)
	}
}

impl Tag for FeDistantLight {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("feDistantLight");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
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

#[doc = "The [`<feDropShadow>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feDropShadow.md")]
#[doc = "\n\n [`<feDropShadow>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feDropShadow"]
#[derive(Clone, Debug)]
pub struct FeDropShadow {
	attrs: IndexMap<FeDropShadowAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: FeDropShadowAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&FeDropShadowAttrs::Class).map(String::as_str)
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
		self.attrs.get(&FeDropShadowAttrs::Dx).map(String::as_str)
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
		self.attrs.get(&FeDropShadowAttrs::Dy).map(String::as_str)
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
		self.attrs.get(&FeDropShadowAttrs::In).map(String::as_str)
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
		self.attrs.get(&FeDropShadowAttrs::StdDeviation).map(String::as_str)
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
		self.attrs.get(&FeDropShadowAttrs::Style).map(String::as_str)
	}
}

impl Tag for FeDropShadow {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("feDropShadow");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeFloodAttrs {
	Class,
	FloodMinusColor,
	FloodMinusOpacity,
	Style,
}

impl FeFloodAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::FloodMinusColor => "flood-color",
			Self::FloodMinusOpacity => "flood-opacity",
			Self::Style => "style",
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

#[doc = "The [`<feFlood>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feFlood.md")]
#[doc = "\n\n [`<feFlood>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feFlood"]
#[derive(Clone, Debug)]
pub struct FeFlood {
	attrs: IndexMap<FeFloodAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: FeFloodAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&FeFloodAttrs::Class).map(String::as_str)
	}

	/// Set the `flood-color` attribute.
	pub fn with_flood_minus_color<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeFloodAttrs::FloodMinusColor, value.into());
		self
	}

	/// Set the `flood-color` attribute.
	pub fn set_flood_minus_color<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeFloodAttrs::FloodMinusColor, value.into());
	}

	/// Get the `flood-color` attribute.
	pub fn flood_minus_color(&self) -> Option<&str> {
		self.attrs.get(&FeFloodAttrs::FloodMinusColor).map(String::as_str)
	}

	/// Set the `flood-opacity` attribute.
	pub fn with_flood_minus_opacity<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FeFloodAttrs::FloodMinusOpacity, value.into());
		self
	}

	/// Set the `flood-opacity` attribute.
	pub fn set_flood_minus_opacity<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FeFloodAttrs::FloodMinusOpacity, value.into());
	}

	/// Get the `flood-opacity` attribute.
	pub fn flood_minus_opacity(&self) -> Option<&str> {
		self.attrs.get(&FeFloodAttrs::FloodMinusOpacity).map(String::as_str)
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
		self.attrs.get(&FeFloodAttrs::Style).map(String::as_str)
	}
}

impl Tag for FeFlood {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("feFlood");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[doc = "The [`<feFuncA>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feFuncA.md")]
#[doc = "\n\n [`<feFuncA>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feFuncA"]
#[derive(Clone, Debug)]
pub struct FeFuncA {
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
		}
	}
}

impl Tag for FeFuncA {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("feFuncA");w.end_element();
	}
}

#[doc = "The [`<feFuncB>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feFuncB.md")]
#[doc = "\n\n [`<feFuncB>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feFuncB"]
#[derive(Clone, Debug)]
pub struct FeFuncB {
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
		}
	}
}

impl Tag for FeFuncB {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("feFuncB");w.end_element();
	}
}

#[doc = "The [`<feFuncG>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feFuncG.md")]
#[doc = "\n\n [`<feFuncG>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feFuncG"]
#[derive(Clone, Debug)]
pub struct FeFuncG {
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
		}
	}
}

impl Tag for FeFuncG {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("feFuncG");w.end_element();
	}
}

#[doc = "The [`<feFuncR>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feFuncR.md")]
#[doc = "\n\n [`<feFuncR>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feFuncR"]
#[derive(Clone, Debug)]
pub struct FeFuncR {
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
		}
	}
}

impl Tag for FeFuncR {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("feFuncR");w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeGaussianBlurAttrs {
	Class,
	In,
	StdDeviation,
	Style,
}

impl FeGaussianBlurAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::In => "in",
			Self::StdDeviation => "stdDeviation",
			Self::Style => "style",
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

#[doc = "The [`<feGaussianBlur>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feGaussianBlur.md")]
#[doc = "\n\n [`<feGaussianBlur>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feGaussianBlur"]
#[derive(Clone, Debug)]
pub struct FeGaussianBlur {
	attrs: IndexMap<FeGaussianBlurAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: FeGaussianBlurAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&FeGaussianBlurAttrs::Class).map(String::as_str)
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
		self.attrs.get(&FeGaussianBlurAttrs::In).map(String::as_str)
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
		self.attrs.get(&FeGaussianBlurAttrs::StdDeviation).map(String::as_str)
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
		self.attrs.get(&FeGaussianBlurAttrs::Style).map(String::as_str)
	}
}

impl Tag for FeGaussianBlur {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("feGaussianBlur");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
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
}

impl FeImageAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::PreserveAspectRatio => "preserveAspectRatio",
			Self::Style => "style",
			Self::XlinkHref => "xlink:href",
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

#[doc = "The [`<feImage>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feImage.md")]
#[doc = "\n\n [`<feImage>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feImage"]
#[derive(Clone, Debug)]
pub struct FeImage {
	attrs: IndexMap<FeImageAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: FeImageAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&FeImageAttrs::Class).map(String::as_str)
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
		self.attrs.get(&FeImageAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&FeImageAttrs::PreserveAspectRatio).map(String::as_str)
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
		self.attrs.get(&FeImageAttrs::Style).map(String::as_str)
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
		self.attrs.get(&FeImageAttrs::XlinkHref).map(String::as_str)
	}
}

impl Tag for FeImage {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("feImage");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeMergeAttrs {
	Class,
	Style,
}

impl FeMergeAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::Style => "style",
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

#[doc = "The [`<feMerge>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feMerge.md")]
#[doc = "\n\n [`<feMerge>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feMerge"]
#[derive(Clone, Debug)]
pub struct FeMerge {
	attrs: IndexMap<FeMergeAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: FeMergeAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&FeMergeAttrs::Class).map(String::as_str)
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
		self.attrs.get(&FeMergeAttrs::Style).map(String::as_str)
	}
}

impl Tag for FeMerge {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("feMerge");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeMergeNodeAttrs {
	In,
}

impl FeMergeNodeAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::In => "in",
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

#[doc = "The [`<feMergeNode>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feMergeNode.md")]
#[doc = "\n\n [`<feMergeNode>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feMergeNode"]
#[derive(Clone, Debug)]
pub struct FeMergeNode {
	attrs: IndexMap<FeMergeNodeAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: FeMergeNodeAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&FeMergeNodeAttrs::In).map(String::as_str)
	}
}

impl Tag for FeMergeNode {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("feMergeNode");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
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
}

impl FeMorphologyAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::In => "in",
			Self::Operator => "operator",
			Self::Radius => "radius",
			Self::Style => "style",
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

#[doc = "The [`<feMorphology>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feMorphology.md")]
#[doc = "\n\n [`<feMorphology>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feMorphology"]
#[derive(Clone, Debug)]
pub struct FeMorphology {
	attrs: IndexMap<FeMorphologyAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: FeMorphologyAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&FeMorphologyAttrs::Class).map(String::as_str)
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
		self.attrs.get(&FeMorphologyAttrs::In).map(String::as_str)
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
		self.attrs.get(&FeMorphologyAttrs::Operator).map(String::as_str)
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
		self.attrs.get(&FeMorphologyAttrs::Radius).map(String::as_str)
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
		self.attrs.get(&FeMorphologyAttrs::Style).map(String::as_str)
	}
}

impl Tag for FeMorphology {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("feMorphology");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
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
}

impl FeOffsetAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::Dx => "dx",
			Self::Dy => "dy",
			Self::In => "in",
			Self::Style => "style",
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

#[doc = "The [`<feOffset>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feOffset.md")]
#[doc = "\n\n [`<feOffset>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feOffset"]
#[derive(Clone, Debug)]
pub struct FeOffset {
	attrs: IndexMap<FeOffsetAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: FeOffsetAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&FeOffsetAttrs::Class).map(String::as_str)
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
		self.attrs.get(&FeOffsetAttrs::Dx).map(String::as_str)
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
		self.attrs.get(&FeOffsetAttrs::Dy).map(String::as_str)
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
		self.attrs.get(&FeOffsetAttrs::In).map(String::as_str)
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
		self.attrs.get(&FeOffsetAttrs::Style).map(String::as_str)
	}
}

impl Tag for FeOffset {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("feOffset");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FePointLightAttrs {
	X,
	Y,
	Z,
}

impl FePointLightAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::X => "x",
			Self::Y => "y",
			Self::Z => "z",
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

#[doc = "The [`<fePointLight>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("fePointLight.md")]
#[doc = "\n\n [`<fePointLight>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/fePointLight"]
#[derive(Clone, Debug)]
pub struct FePointLight {
	attrs: IndexMap<FePointLightAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: FePointLightAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&FePointLightAttrs::X).map(String::as_str)
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
		self.attrs.get(&FePointLightAttrs::Y).map(String::as_str)
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
		self.attrs.get(&FePointLightAttrs::Z).map(String::as_str)
	}
}

impl Tag for FePointLight {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("fePointLight");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
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
#[derive(Clone, Debug)]
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
		self.attrs.get(&FeSpecularLightingAttrs::Class).map(String::as_str)
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
		self.attrs.get(&FeSpecularLightingAttrs::In).map(String::as_str)
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
		self.attrs.get(&FeSpecularLightingAttrs::KernelUnitLength).map(String::as_str)
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
		self.attrs.get(&FeSpecularLightingAttrs::SpecularConstant).map(String::as_str)
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
		self.attrs.get(&FeSpecularLightingAttrs::SpecularExponent).map(String::as_str)
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
		self.attrs.get(&FeSpecularLightingAttrs::Style).map(String::as_str)
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
		self.attrs.get(&FeSpecularLightingAttrs::SurfaceScale).map(String::as_str)
	}
}

impl Tag for FeSpecularLighting {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
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

#[doc = "The [`<feSpotLight>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feSpotLight.md")]
#[doc = "\n\n [`<feSpotLight>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feSpotLight"]
#[derive(Clone, Debug)]
pub struct FeSpotLight {
	attrs: IndexMap<FeSpotLightAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: FeSpotLightAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&FeSpotLightAttrs::LimitingConeAngle).map(String::as_str)
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
		self.attrs.get(&FeSpotLightAttrs::PointsAtX).map(String::as_str)
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
		self.attrs.get(&FeSpotLightAttrs::PointsAtY).map(String::as_str)
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
		self.attrs.get(&FeSpotLightAttrs::PointsAtZ).map(String::as_str)
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
		self.attrs.get(&FeSpotLightAttrs::SpecularExponent).map(String::as_str)
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
		self.attrs.get(&FeSpotLightAttrs::X).map(String::as_str)
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
		self.attrs.get(&FeSpotLightAttrs::Y).map(String::as_str)
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
		self.attrs.get(&FeSpotLightAttrs::Z).map(String::as_str)
	}
}

impl Tag for FeSpotLight {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("feSpotLight");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FeTileAttrs {
	Class,
	In,
	Style,
}

impl FeTileAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::In => "in",
			Self::Style => "style",
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

#[doc = "The [`<feTile>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feTile.md")]
#[doc = "\n\n [`<feTile>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feTile"]
#[derive(Clone, Debug)]
pub struct FeTile {
	attrs: IndexMap<FeTileAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: FeTileAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&FeTileAttrs::Class).map(String::as_str)
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
		self.attrs.get(&FeTileAttrs::In).map(String::as_str)
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
		self.attrs.get(&FeTileAttrs::Style).map(String::as_str)
	}
}

impl Tag for FeTile {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("feTile");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
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

#[doc = "The [`<feTurbulence>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("feTurbulence.md")]
#[doc = "\n\n [`<feTurbulence>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feTurbulence"]
#[derive(Clone, Debug)]
pub struct FeTurbulence {
	attrs: IndexMap<FeTurbulenceAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: FeTurbulenceAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&FeTurbulenceAttrs::BaseFrequency).map(String::as_str)
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
		self.attrs.get(&FeTurbulenceAttrs::Class).map(String::as_str)
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
		self.attrs.get(&FeTurbulenceAttrs::NumOctaves).map(String::as_str)
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
		self.attrs.get(&FeTurbulenceAttrs::Seed).map(String::as_str)
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
		self.attrs.get(&FeTurbulenceAttrs::StitchTiles).map(String::as_str)
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
		self.attrs.get(&FeTurbulenceAttrs::Style).map(String::as_str)
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
		self.attrs.get(&FeTurbulenceAttrs::Type).map(String::as_str)
	}
}

impl Tag for FeTurbulence {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("feTurbulence");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
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

#[doc = "The [`<filter>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("filter.md")]
#[doc = "\n\n [`<filter>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/filter"]
#[derive(Clone, Debug)]
pub struct Filter {
	attrs: IndexMap<FilterAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: FilterAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&FilterAttrs::Class).map(String::as_str)
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
		self.attrs.get(&FilterAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&FilterAttrs::FilterRes).map(String::as_str)
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
		self.attrs.get(&FilterAttrs::FilterUnits).map(String::as_str)
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
		self.attrs.get(&FilterAttrs::Height).map(String::as_str)
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
		self.attrs.get(&FilterAttrs::PrimitiveUnits).map(String::as_str)
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
		self.attrs.get(&FilterAttrs::Style).map(String::as_str)
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
		self.attrs.get(&FilterAttrs::Width).map(String::as_str)
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
		self.attrs.get(&FilterAttrs::X).map(String::as_str)
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
		self.attrs.get(&FilterAttrs::XlinkHref).map(String::as_str)
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
		self.attrs.get(&FilterAttrs::Y).map(String::as_str)
	}
}

impl Tag for Filter {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("filter");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FontAttrs {
	Class,
	ExternalResourcesRequired,
	HorizMinusAdvMinusX,
	HorizMinusOriginMinusX,
	HorizMinusOriginMinusY,
	Style,
	VertMinusAdvMinusY,
	VertMinusOriginMinusX,
	VertMinusOriginMinusY,
}

impl FontAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::HorizMinusAdvMinusX => "horiz-adv-x",
			Self::HorizMinusOriginMinusX => "horiz-origin-x",
			Self::HorizMinusOriginMinusY => "horiz-origin-y",
			Self::Style => "style",
			Self::VertMinusAdvMinusY => "vert-adv-y",
			Self::VertMinusOriginMinusX => "vert-origin-x",
			Self::VertMinusOriginMinusY => "vert-origin-y",
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

#[doc = "The [`<font>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("font.md")]
#[doc = "\n\n [`<font>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/font"]
#[derive(Clone, Debug)]
pub struct Font {
	attrs: IndexMap<FontAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: FontAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&FontAttrs::Class).map(String::as_str)
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
		self.attrs.get(&FontAttrs::ExternalResourcesRequired).map(String::as_str)
	}

	/// Set the `horiz-adv-x` attribute.
	pub fn with_horiz_minus_adv_minus_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::HorizMinusAdvMinusX, value.into());
		self
	}

	/// Set the `horiz-adv-x` attribute.
	pub fn set_horiz_minus_adv_minus_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::HorizMinusAdvMinusX, value.into());
	}

	/// Get the `horiz-adv-x` attribute.
	pub fn horiz_minus_adv_minus_x(&self) -> Option<&str> {
		self.attrs.get(&FontAttrs::HorizMinusAdvMinusX).map(String::as_str)
	}

	/// Set the `horiz-origin-x` attribute.
	pub fn with_horiz_minus_origin_minus_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::HorizMinusOriginMinusX, value.into());
		self
	}

	/// Set the `horiz-origin-x` attribute.
	pub fn set_horiz_minus_origin_minus_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::HorizMinusOriginMinusX, value.into());
	}

	/// Get the `horiz-origin-x` attribute.
	pub fn horiz_minus_origin_minus_x(&self) -> Option<&str> {
		self.attrs.get(&FontAttrs::HorizMinusOriginMinusX).map(String::as_str)
	}

	/// Set the `horiz-origin-y` attribute.
	pub fn with_horiz_minus_origin_minus_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::HorizMinusOriginMinusY, value.into());
		self
	}

	/// Set the `horiz-origin-y` attribute.
	pub fn set_horiz_minus_origin_minus_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::HorizMinusOriginMinusY, value.into());
	}

	/// Get the `horiz-origin-y` attribute.
	pub fn horiz_minus_origin_minus_y(&self) -> Option<&str> {
		self.attrs.get(&FontAttrs::HorizMinusOriginMinusY).map(String::as_str)
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
		self.attrs.get(&FontAttrs::Style).map(String::as_str)
	}

	/// Set the `vert-adv-y` attribute.
	pub fn with_vert_minus_adv_minus_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::VertMinusAdvMinusY, value.into());
		self
	}

	/// Set the `vert-adv-y` attribute.
	pub fn set_vert_minus_adv_minus_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::VertMinusAdvMinusY, value.into());
	}

	/// Get the `vert-adv-y` attribute.
	pub fn vert_minus_adv_minus_y(&self) -> Option<&str> {
		self.attrs.get(&FontAttrs::VertMinusAdvMinusY).map(String::as_str)
	}

	/// Set the `vert-origin-x` attribute.
	pub fn with_vert_minus_origin_minus_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::VertMinusOriginMinusX, value.into());
		self
	}

	/// Set the `vert-origin-x` attribute.
	pub fn set_vert_minus_origin_minus_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::VertMinusOriginMinusX, value.into());
	}

	/// Get the `vert-origin-x` attribute.
	pub fn vert_minus_origin_minus_x(&self) -> Option<&str> {
		self.attrs.get(&FontAttrs::VertMinusOriginMinusX).map(String::as_str)
	}

	/// Set the `vert-origin-y` attribute.
	pub fn with_vert_minus_origin_minus_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::VertMinusOriginMinusY, value.into());
		self
	}

	/// Set the `vert-origin-y` attribute.
	pub fn set_vert_minus_origin_minus_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontAttrs::VertMinusOriginMinusY, value.into());
	}

	/// Get the `vert-origin-y` attribute.
	pub fn vert_minus_origin_minus_y(&self) -> Option<&str> {
		self.attrs.get(&FontAttrs::VertMinusOriginMinusY).map(String::as_str)
	}
}

impl Tag for Font {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("font");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FontMinusFaceAttrs {
	AccentMinusHeight,
	Alphabetic,
	Ascent,
	Bbox,
	CapMinusHeight,
	Descent,
	FontMinusFamily,
	FontMinusSize,
	FontMinusStretch,
	FontMinusStyle,
	FontMinusVariant,
	FontMinusWeight,
	Hanging,
	Ideographic,
	Mathematical,
	OverlineMinusPosition,
	OverlineMinusThickness,
	PanoseMinus1,
	Slope,
	Stemh,
	Stemv,
	StrikethroughMinusPosition,
	StrikethroughMinusThickness,
	UnderlineMinusPosition,
	UnderlineMinusThickness,
	UnicodeMinusRange,
	UnitsMinusPerMinusEm,
	VMinusAlphabetic,
	VMinusHanging,
	VMinusIdeographic,
	VMinusMathematical,
	Widths,
	XMinusHeight,
}

impl FontMinusFaceAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::AccentMinusHeight => "accent-height",
			Self::Alphabetic => "alphabetic",
			Self::Ascent => "ascent",
			Self::Bbox => "bbox",
			Self::CapMinusHeight => "cap-height",
			Self::Descent => "descent",
			Self::FontMinusFamily => "font-family",
			Self::FontMinusSize => "font-size",
			Self::FontMinusStretch => "font-stretch",
			Self::FontMinusStyle => "font-style",
			Self::FontMinusVariant => "font-variant",
			Self::FontMinusWeight => "font-weight",
			Self::Hanging => "hanging",
			Self::Ideographic => "ideographic",
			Self::Mathematical => "mathematical",
			Self::OverlineMinusPosition => "overline-position",
			Self::OverlineMinusThickness => "overline-thickness",
			Self::PanoseMinus1 => "panose-1",
			Self::Slope => "slope",
			Self::Stemh => "stemh",
			Self::Stemv => "stemv",
			Self::StrikethroughMinusPosition => "strikethrough-position",
			Self::StrikethroughMinusThickness => "strikethrough-thickness",
			Self::UnderlineMinusPosition => "underline-position",
			Self::UnderlineMinusThickness => "underline-thickness",
			Self::UnicodeMinusRange => "unicode-range",
			Self::UnitsMinusPerMinusEm => "units-per-em",
			Self::VMinusAlphabetic => "v-alphabetic",
			Self::VMinusHanging => "v-hanging",
			Self::VMinusIdeographic => "v-ideographic",
			Self::VMinusMathematical => "v-mathematical",
			Self::Widths => "widths",
			Self::XMinusHeight => "x-height",
		}
	}
}

impl Display for FontMinusFaceAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FontMinusFaceAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[doc = "The [`<font-face>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("font-face.md")]
#[doc = "\n\n [`<font-face>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/font-face"]
#[derive(Clone, Debug)]
pub struct FontMinusFace {
	attrs: IndexMap<FontMinusFaceAttrs, String>,
}

impl Default for FontMinusFace {
	fn default() -> Self {
		Self::new()
	}
}

impl FontMinusFace {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
		}
	}

	fn set_attr(&mut self, attr: FontMinusFaceAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	/// Set the `accent-height` attribute.
	pub fn with_accent_minus_height<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::AccentMinusHeight, value.into());
		self
	}

	/// Set the `accent-height` attribute.
	pub fn set_accent_minus_height<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::AccentMinusHeight, value.into());
	}

	/// Get the `accent-height` attribute.
	pub fn accent_minus_height(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::AccentMinusHeight).map(String::as_str)
	}

	/// Set the `alphabetic` attribute.
	pub fn with_alphabetic<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::Alphabetic, value.into());
		self
	}

	/// Set the `alphabetic` attribute.
	pub fn set_alphabetic<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::Alphabetic, value.into());
	}

	/// Get the `alphabetic` attribute.
	pub fn alphabetic(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::Alphabetic).map(String::as_str)
	}

	/// Set the `ascent` attribute.
	pub fn with_ascent<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::Ascent, value.into());
		self
	}

	/// Set the `ascent` attribute.
	pub fn set_ascent<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::Ascent, value.into());
	}

	/// Get the `ascent` attribute.
	pub fn ascent(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::Ascent).map(String::as_str)
	}

	/// Set the `bbox` attribute.
	pub fn with_bbox<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::Bbox, value.into());
		self
	}

	/// Set the `bbox` attribute.
	pub fn set_bbox<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::Bbox, value.into());
	}

	/// Get the `bbox` attribute.
	pub fn bbox(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::Bbox).map(String::as_str)
	}

	/// Set the `cap-height` attribute.
	pub fn with_cap_minus_height<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::CapMinusHeight, value.into());
		self
	}

	/// Set the `cap-height` attribute.
	pub fn set_cap_minus_height<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::CapMinusHeight, value.into());
	}

	/// Get the `cap-height` attribute.
	pub fn cap_minus_height(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::CapMinusHeight).map(String::as_str)
	}

	/// Set the `descent` attribute.
	pub fn with_descent<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::Descent, value.into());
		self
	}

	/// Set the `descent` attribute.
	pub fn set_descent<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::Descent, value.into());
	}

	/// Get the `descent` attribute.
	pub fn descent(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::Descent).map(String::as_str)
	}

	/// Set the `font-family` attribute.
	pub fn with_font_minus_family<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::FontMinusFamily, value.into());
		self
	}

	/// Set the `font-family` attribute.
	pub fn set_font_minus_family<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::FontMinusFamily, value.into());
	}

	/// Get the `font-family` attribute.
	pub fn font_minus_family(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::FontMinusFamily).map(String::as_str)
	}

	/// Set the `font-size` attribute.
	pub fn with_font_minus_size<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::FontMinusSize, value.into());
		self
	}

	/// Set the `font-size` attribute.
	pub fn set_font_minus_size<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::FontMinusSize, value.into());
	}

	/// Get the `font-size` attribute.
	pub fn font_minus_size(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::FontMinusSize).map(String::as_str)
	}

	/// Set the `font-stretch` attribute.
	pub fn with_font_minus_stretch<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::FontMinusStretch, value.into());
		self
	}

	/// Set the `font-stretch` attribute.
	pub fn set_font_minus_stretch<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::FontMinusStretch, value.into());
	}

	/// Get the `font-stretch` attribute.
	pub fn font_minus_stretch(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::FontMinusStretch).map(String::as_str)
	}

	/// Set the `font-style` attribute.
	pub fn with_font_minus_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::FontMinusStyle, value.into());
		self
	}

	/// Set the `font-style` attribute.
	pub fn set_font_minus_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::FontMinusStyle, value.into());
	}

	/// Get the `font-style` attribute.
	pub fn font_minus_style(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::FontMinusStyle).map(String::as_str)
	}

	/// Set the `font-variant` attribute.
	pub fn with_font_minus_variant<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::FontMinusVariant, value.into());
		self
	}

	/// Set the `font-variant` attribute.
	pub fn set_font_minus_variant<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::FontMinusVariant, value.into());
	}

	/// Get the `font-variant` attribute.
	pub fn font_minus_variant(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::FontMinusVariant).map(String::as_str)
	}

	/// Set the `font-weight` attribute.
	pub fn with_font_minus_weight<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::FontMinusWeight, value.into());
		self
	}

	/// Set the `font-weight` attribute.
	pub fn set_font_minus_weight<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::FontMinusWeight, value.into());
	}

	/// Get the `font-weight` attribute.
	pub fn font_minus_weight(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::FontMinusWeight).map(String::as_str)
	}

	/// Set the `hanging` attribute.
	pub fn with_hanging<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::Hanging, value.into());
		self
	}

	/// Set the `hanging` attribute.
	pub fn set_hanging<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::Hanging, value.into());
	}

	/// Get the `hanging` attribute.
	pub fn hanging(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::Hanging).map(String::as_str)
	}

	/// Set the `ideographic` attribute.
	pub fn with_ideographic<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::Ideographic, value.into());
		self
	}

	/// Set the `ideographic` attribute.
	pub fn set_ideographic<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::Ideographic, value.into());
	}

	/// Get the `ideographic` attribute.
	pub fn ideographic(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::Ideographic).map(String::as_str)
	}

	/// Set the `mathematical` attribute.
	pub fn with_mathematical<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::Mathematical, value.into());
		self
	}

	/// Set the `mathematical` attribute.
	pub fn set_mathematical<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::Mathematical, value.into());
	}

	/// Get the `mathematical` attribute.
	pub fn mathematical(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::Mathematical).map(String::as_str)
	}

	/// Set the `overline-position` attribute.
	pub fn with_overline_minus_position<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::OverlineMinusPosition, value.into());
		self
	}

	/// Set the `overline-position` attribute.
	pub fn set_overline_minus_position<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::OverlineMinusPosition, value.into());
	}

	/// Get the `overline-position` attribute.
	pub fn overline_minus_position(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::OverlineMinusPosition).map(String::as_str)
	}

	/// Set the `overline-thickness` attribute.
	pub fn with_overline_minus_thickness<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::OverlineMinusThickness, value.into());
		self
	}

	/// Set the `overline-thickness` attribute.
	pub fn set_overline_minus_thickness<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::OverlineMinusThickness, value.into());
	}

	/// Get the `overline-thickness` attribute.
	pub fn overline_minus_thickness(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::OverlineMinusThickness).map(String::as_str)
	}

	/// Set the `panose-1` attribute.
	pub fn with_panose_minus_1<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::PanoseMinus1, value.into());
		self
	}

	/// Set the `panose-1` attribute.
	pub fn set_panose_minus_1<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::PanoseMinus1, value.into());
	}

	/// Get the `panose-1` attribute.
	pub fn panose_minus_1(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::PanoseMinus1).map(String::as_str)
	}

	/// Set the `slope` attribute.
	pub fn with_slope<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::Slope, value.into());
		self
	}

	/// Set the `slope` attribute.
	pub fn set_slope<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::Slope, value.into());
	}

	/// Get the `slope` attribute.
	pub fn slope(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::Slope).map(String::as_str)
	}

	/// Set the `stemh` attribute.
	pub fn with_stemh<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::Stemh, value.into());
		self
	}

	/// Set the `stemh` attribute.
	pub fn set_stemh<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::Stemh, value.into());
	}

	/// Get the `stemh` attribute.
	pub fn stemh(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::Stemh).map(String::as_str)
	}

	/// Set the `stemv` attribute.
	pub fn with_stemv<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::Stemv, value.into());
		self
	}

	/// Set the `stemv` attribute.
	pub fn set_stemv<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::Stemv, value.into());
	}

	/// Get the `stemv` attribute.
	pub fn stemv(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::Stemv).map(String::as_str)
	}

	/// Set the `strikethrough-position` attribute.
	pub fn with_strikethrough_minus_position<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::StrikethroughMinusPosition, value.into());
		self
	}

	/// Set the `strikethrough-position` attribute.
	pub fn set_strikethrough_minus_position<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::StrikethroughMinusPosition, value.into());
	}

	/// Get the `strikethrough-position` attribute.
	pub fn strikethrough_minus_position(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::StrikethroughMinusPosition).map(String::as_str)
	}

	/// Set the `strikethrough-thickness` attribute.
	pub fn with_strikethrough_minus_thickness<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::StrikethroughMinusThickness, value.into());
		self
	}

	/// Set the `strikethrough-thickness` attribute.
	pub fn set_strikethrough_minus_thickness<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::StrikethroughMinusThickness, value.into());
	}

	/// Get the `strikethrough-thickness` attribute.
	pub fn strikethrough_minus_thickness(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::StrikethroughMinusThickness).map(String::as_str)
	}

	/// Set the `underline-position` attribute.
	pub fn with_underline_minus_position<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::UnderlineMinusPosition, value.into());
		self
	}

	/// Set the `underline-position` attribute.
	pub fn set_underline_minus_position<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::UnderlineMinusPosition, value.into());
	}

	/// Get the `underline-position` attribute.
	pub fn underline_minus_position(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::UnderlineMinusPosition).map(String::as_str)
	}

	/// Set the `underline-thickness` attribute.
	pub fn with_underline_minus_thickness<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::UnderlineMinusThickness, value.into());
		self
	}

	/// Set the `underline-thickness` attribute.
	pub fn set_underline_minus_thickness<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::UnderlineMinusThickness, value.into());
	}

	/// Get the `underline-thickness` attribute.
	pub fn underline_minus_thickness(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::UnderlineMinusThickness).map(String::as_str)
	}

	/// Set the `unicode-range` attribute.
	pub fn with_unicode_minus_range<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::UnicodeMinusRange, value.into());
		self
	}

	/// Set the `unicode-range` attribute.
	pub fn set_unicode_minus_range<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::UnicodeMinusRange, value.into());
	}

	/// Get the `unicode-range` attribute.
	pub fn unicode_minus_range(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::UnicodeMinusRange).map(String::as_str)
	}

	/// Set the `units-per-em` attribute.
	pub fn with_units_minus_per_minus_em<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::UnitsMinusPerMinusEm, value.into());
		self
	}

	/// Set the `units-per-em` attribute.
	pub fn set_units_minus_per_minus_em<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::UnitsMinusPerMinusEm, value.into());
	}

	/// Get the `units-per-em` attribute.
	pub fn units_minus_per_minus_em(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::UnitsMinusPerMinusEm).map(String::as_str)
	}

	/// Set the `v-alphabetic` attribute.
	pub fn with_v_minus_alphabetic<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::VMinusAlphabetic, value.into());
		self
	}

	/// Set the `v-alphabetic` attribute.
	pub fn set_v_minus_alphabetic<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::VMinusAlphabetic, value.into());
	}

	/// Get the `v-alphabetic` attribute.
	pub fn v_minus_alphabetic(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::VMinusAlphabetic).map(String::as_str)
	}

	/// Set the `v-hanging` attribute.
	pub fn with_v_minus_hanging<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::VMinusHanging, value.into());
		self
	}

	/// Set the `v-hanging` attribute.
	pub fn set_v_minus_hanging<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::VMinusHanging, value.into());
	}

	/// Get the `v-hanging` attribute.
	pub fn v_minus_hanging(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::VMinusHanging).map(String::as_str)
	}

	/// Set the `v-ideographic` attribute.
	pub fn with_v_minus_ideographic<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::VMinusIdeographic, value.into());
		self
	}

	/// Set the `v-ideographic` attribute.
	pub fn set_v_minus_ideographic<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::VMinusIdeographic, value.into());
	}

	/// Get the `v-ideographic` attribute.
	pub fn v_minus_ideographic(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::VMinusIdeographic).map(String::as_str)
	}

	/// Set the `v-mathematical` attribute.
	pub fn with_v_minus_mathematical<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::VMinusMathematical, value.into());
		self
	}

	/// Set the `v-mathematical` attribute.
	pub fn set_v_minus_mathematical<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::VMinusMathematical, value.into());
	}

	/// Get the `v-mathematical` attribute.
	pub fn v_minus_mathematical(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::VMinusMathematical).map(String::as_str)
	}

	/// Set the `widths` attribute.
	pub fn with_widths<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::Widths, value.into());
		self
	}

	/// Set the `widths` attribute.
	pub fn set_widths<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::Widths, value.into());
	}

	/// Get the `widths` attribute.
	pub fn widths(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::Widths).map(String::as_str)
	}

	/// Set the `x-height` attribute.
	pub fn with_x_minus_height<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::XMinusHeight, value.into());
		self
	}

	/// Set the `x-height` attribute.
	pub fn set_x_minus_height<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceAttrs::XMinusHeight, value.into());
	}

	/// Get the `x-height` attribute.
	pub fn x_minus_height(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceAttrs::XMinusHeight).map(String::as_str)
	}
}

impl Tag for FontMinusFace {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("font-face");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FontMinusFaceMinusFormatAttrs {
	String,
}

impl FontMinusFaceMinusFormatAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::String => "string",
		}
	}
}

impl Display for FontMinusFaceMinusFormatAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FontMinusFaceMinusFormatAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[doc = "The [`<font-face-format>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("font-face-format.md")]
#[doc = "\n\n [`<font-face-format>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/font-face-format"]
#[derive(Clone, Debug)]
pub struct FontMinusFaceMinusFormat {
	attrs: IndexMap<FontMinusFaceMinusFormatAttrs, String>,
}

impl Default for FontMinusFaceMinusFormat {
	fn default() -> Self {
		Self::new()
	}
}

impl FontMinusFaceMinusFormat {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
		}
	}

	fn set_attr(&mut self, attr: FontMinusFaceMinusFormatAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	/// Set the `string` attribute.
	pub fn with_string<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceMinusFormatAttrs::String, value.into());
		self
	}

	/// Set the `string` attribute.
	pub fn set_string<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceMinusFormatAttrs::String, value.into());
	}

	/// Get the `string` attribute.
	pub fn string(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceMinusFormatAttrs::String).map(String::as_str)
	}
}

impl Tag for FontMinusFaceMinusFormat {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("font-face-format");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FontMinusFaceMinusNameAttrs {
	Name,
}

impl FontMinusFaceMinusNameAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Name => "name",
		}
	}
}

impl Display for FontMinusFaceMinusNameAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FontMinusFaceMinusNameAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[doc = "The [`<font-face-name>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("font-face-name.md")]
#[doc = "\n\n [`<font-face-name>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/font-face-name"]
#[derive(Clone, Debug)]
pub struct FontMinusFaceMinusName {
	attrs: IndexMap<FontMinusFaceMinusNameAttrs, String>,
}

impl Default for FontMinusFaceMinusName {
	fn default() -> Self {
		Self::new()
	}
}

impl FontMinusFaceMinusName {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
		}
	}

	fn set_attr(&mut self, attr: FontMinusFaceMinusNameAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	/// Set the `name` attribute.
	pub fn with_name<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceMinusNameAttrs::Name, value.into());
		self
	}

	/// Set the `name` attribute.
	pub fn set_name<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceMinusNameAttrs::Name, value.into());
	}

	/// Get the `name` attribute.
	pub fn name(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceMinusNameAttrs::Name).map(String::as_str)
	}
}

impl Tag for FontMinusFaceMinusName {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("font-face-name");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[doc = "The [`<font-face-src>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("font-face-src.md")]
#[doc = "\n\n [`<font-face-src>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/font-face-src"]
#[derive(Clone, Debug)]
pub struct FontMinusFaceMinusSrc {
}

impl Default for FontMinusFaceMinusSrc {
	fn default() -> Self {
		Self::new()
	}
}

impl FontMinusFaceMinusSrc {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
		}
	}
}

impl Tag for FontMinusFaceMinusSrc {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("font-face-src");w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum FontMinusFaceMinusUriAttrs {
	XlinkHref,
}

impl FontMinusFaceMinusUriAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::XlinkHref => "xlink:href",
		}
	}
}

impl Display for FontMinusFaceMinusUriAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FontMinusFaceMinusUriAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[doc = "The [`<font-face-uri>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("font-face-uri.md")]
#[doc = "\n\n [`<font-face-uri>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/font-face-uri"]
#[derive(Clone, Debug)]
pub struct FontMinusFaceMinusUri {
	attrs: IndexMap<FontMinusFaceMinusUriAttrs, String>,
}

impl Default for FontMinusFaceMinusUri {
	fn default() -> Self {
		Self::new()
	}
}

impl FontMinusFaceMinusUri {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
		}
	}

	fn set_attr(&mut self, attr: FontMinusFaceMinusUriAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	/// Set the `xlink:href` attribute.
	pub fn with_xlink_href<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceMinusUriAttrs::XlinkHref, value.into());
		self
	}

	/// Set the `xlink:href` attribute.
	pub fn set_xlink_href<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FontMinusFaceMinusUriAttrs::XlinkHref, value.into());
	}

	/// Get the `xlink:href` attribute.
	pub fn xlink_href(&self) -> Option<&str> {
		self.attrs.get(&FontMinusFaceMinusUriAttrs::XlinkHref).map(String::as_str)
	}
}

impl Tag for FontMinusFaceMinusUri {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("font-face-uri");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
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
#[derive(Clone, Debug)]
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
		self.attrs.get(&ForeignObjectAttrs::Class).map(String::as_str)
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
		self.attrs.get(&ForeignObjectAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&ForeignObjectAttrs::Height).map(String::as_str)
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
		self.attrs.get(&ForeignObjectAttrs::Style).map(String::as_str)
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
		self.attrs.get(&ForeignObjectAttrs::Transform).map(String::as_str)
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
		self.attrs.get(&ForeignObjectAttrs::Width).map(String::as_str)
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
		self.attrs.get(&ForeignObjectAttrs::X).map(String::as_str)
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
		self.attrs.get(&ForeignObjectAttrs::Y).map(String::as_str)
	}
}

impl Tag for ForeignObject {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
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
}

impl GAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Style => "style",
			Self::Transform => "transform",
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

#[doc = "The [`<g>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("g.md")]
#[doc = "\n\n [`<g>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/g"]
#[derive(Clone, Debug)]
pub struct G {
	attrs: IndexMap<GAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: GAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&GAttrs::Class).map(String::as_str)
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
		self.attrs.get(&GAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&GAttrs::Style).map(String::as_str)
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
		self.attrs.get(&GAttrs::Transform).map(String::as_str)
	}
}

impl Tag for G {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("g");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum GlyphAttrs {
	ArabicMinusForm,
	Class,
	D,
	GlyphMinusName,
	HorizMinusAdvMinusX,
	Lang,
	Orientation,
	Style,
	Unicode,
	VertMinusAdvMinusY,
	VertMinusOriginMinusX,
	VertMinusOriginMinusY,
}

impl GlyphAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::ArabicMinusForm => "arabic-form",
			Self::Class => "class",
			Self::D => "d",
			Self::GlyphMinusName => "glyph-name",
			Self::HorizMinusAdvMinusX => "horiz-adv-x",
			Self::Lang => "lang",
			Self::Orientation => "orientation",
			Self::Style => "style",
			Self::Unicode => "unicode",
			Self::VertMinusAdvMinusY => "vert-adv-y",
			Self::VertMinusOriginMinusX => "vert-origin-x",
			Self::VertMinusOriginMinusY => "vert-origin-y",
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

#[doc = "The [`<glyph>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("glyph.md")]
#[doc = "\n\n [`<glyph>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/glyph"]
#[derive(Clone, Debug)]
pub struct Glyph {
	attrs: IndexMap<GlyphAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: GlyphAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	/// Set the `arabic-form` attribute.
	pub fn with_arabic_minus_form<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::ArabicMinusForm, value.into());
		self
	}

	/// Set the `arabic-form` attribute.
	pub fn set_arabic_minus_form<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::ArabicMinusForm, value.into());
	}

	/// Get the `arabic-form` attribute.
	pub fn arabic_minus_form(&self) -> Option<&str> {
		self.attrs.get(&GlyphAttrs::ArabicMinusForm).map(String::as_str)
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
		self.attrs.get(&GlyphAttrs::Class).map(String::as_str)
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
		self.attrs.get(&GlyphAttrs::D).map(String::as_str)
	}

	/// Set the `glyph-name` attribute.
	pub fn with_glyph_minus_name<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::GlyphMinusName, value.into());
		self
	}

	/// Set the `glyph-name` attribute.
	pub fn set_glyph_minus_name<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::GlyphMinusName, value.into());
	}

	/// Get the `glyph-name` attribute.
	pub fn glyph_minus_name(&self) -> Option<&str> {
		self.attrs.get(&GlyphAttrs::GlyphMinusName).map(String::as_str)
	}

	/// Set the `horiz-adv-x` attribute.
	pub fn with_horiz_minus_adv_minus_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::HorizMinusAdvMinusX, value.into());
		self
	}

	/// Set the `horiz-adv-x` attribute.
	pub fn set_horiz_minus_adv_minus_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::HorizMinusAdvMinusX, value.into());
	}

	/// Get the `horiz-adv-x` attribute.
	pub fn horiz_minus_adv_minus_x(&self) -> Option<&str> {
		self.attrs.get(&GlyphAttrs::HorizMinusAdvMinusX).map(String::as_str)
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
		self.attrs.get(&GlyphAttrs::Lang).map(String::as_str)
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
		self.attrs.get(&GlyphAttrs::Orientation).map(String::as_str)
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
		self.attrs.get(&GlyphAttrs::Style).map(String::as_str)
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
		self.attrs.get(&GlyphAttrs::Unicode).map(String::as_str)
	}

	/// Set the `vert-adv-y` attribute.
	pub fn with_vert_minus_adv_minus_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::VertMinusAdvMinusY, value.into());
		self
	}

	/// Set the `vert-adv-y` attribute.
	pub fn set_vert_minus_adv_minus_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::VertMinusAdvMinusY, value.into());
	}

	/// Get the `vert-adv-y` attribute.
	pub fn vert_minus_adv_minus_y(&self) -> Option<&str> {
		self.attrs.get(&GlyphAttrs::VertMinusAdvMinusY).map(String::as_str)
	}

	/// Set the `vert-origin-x` attribute.
	pub fn with_vert_minus_origin_minus_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::VertMinusOriginMinusX, value.into());
		self
	}

	/// Set the `vert-origin-x` attribute.
	pub fn set_vert_minus_origin_minus_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::VertMinusOriginMinusX, value.into());
	}

	/// Get the `vert-origin-x` attribute.
	pub fn vert_minus_origin_minus_x(&self) -> Option<&str> {
		self.attrs.get(&GlyphAttrs::VertMinusOriginMinusX).map(String::as_str)
	}

	/// Set the `vert-origin-y` attribute.
	pub fn with_vert_minus_origin_minus_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::VertMinusOriginMinusY, value.into());
		self
	}

	/// Set the `vert-origin-y` attribute.
	pub fn set_vert_minus_origin_minus_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlyphAttrs::VertMinusOriginMinusY, value.into());
	}

	/// Get the `vert-origin-y` attribute.
	pub fn vert_minus_origin_minus_y(&self) -> Option<&str> {
		self.attrs.get(&GlyphAttrs::VertMinusOriginMinusY).map(String::as_str)
	}
}

impl Tag for Glyph {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("glyph");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
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
#[derive(Clone, Debug)]
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
		self.attrs.get(&GlyphRefAttrs::Class).map(String::as_str)
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
		self.attrs.get(&GlyphRefAttrs::Dx).map(String::as_str)
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
		self.attrs.get(&GlyphRefAttrs::Dy).map(String::as_str)
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
		self.attrs.get(&GlyphRefAttrs::Format).map(String::as_str)
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
		self.attrs.get(&GlyphRefAttrs::GlyphRef).map(String::as_str)
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
		self.attrs.get(&GlyphRefAttrs::Style).map(String::as_str)
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
		self.attrs.get(&GlyphRefAttrs::X).map(String::as_str)
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
		self.attrs.get(&GlyphRefAttrs::XlinkHref).map(String::as_str)
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
		self.attrs.get(&GlyphRefAttrs::Y).map(String::as_str)
	}
}

impl Tag for GlyphRef {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
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

#[doc = "The [`<hatch>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("hatch.md")]
#[doc = "\n\n [`<hatch>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/hatch"]
#[derive(Clone, Debug)]
pub struct Hatch {
	attrs: IndexMap<HatchAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: HatchAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&HatchAttrs::HatchContentUnits).map(String::as_str)
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
		self.attrs.get(&HatchAttrs::HatchUnits).map(String::as_str)
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
		self.attrs.get(&HatchAttrs::Href).map(String::as_str)
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
		self.attrs.get(&HatchAttrs::Pitch).map(String::as_str)
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
		self.attrs.get(&HatchAttrs::Rotate).map(String::as_str)
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
		self.attrs.get(&HatchAttrs::Transform).map(String::as_str)
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
		self.attrs.get(&HatchAttrs::X).map(String::as_str)
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
		self.attrs.get(&HatchAttrs::Y).map(String::as_str)
	}
}

impl Tag for Hatch {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("hatch");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum HatchpathAttrs {
	D,
	Offset,
}

impl HatchpathAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::D => "d",
			Self::Offset => "offset",
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

#[doc = "The [`<hatchpath>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("hatchpath.md")]
#[doc = "\n\n [`<hatchpath>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/hatchpath"]
#[derive(Clone, Debug)]
pub struct Hatchpath {
	attrs: IndexMap<HatchpathAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: HatchpathAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&HatchpathAttrs::D).map(String::as_str)
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
		self.attrs.get(&HatchpathAttrs::Offset).map(String::as_str)
	}
}

impl Tag for Hatchpath {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("hatchpath");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
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
}

impl HkernAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::G1 => "g1",
			Self::G2 => "g2",
			Self::K => "k",
			Self::U1 => "u1",
			Self::U2 => "u2",
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
#[derive(Clone, Debug)]
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
		self.attrs.get(&HkernAttrs::G1).map(String::as_str)
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
		self.attrs.get(&HkernAttrs::G2).map(String::as_str)
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
		self.attrs.get(&HkernAttrs::K).map(String::as_str)
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
		self.attrs.get(&HkernAttrs::U1).map(String::as_str)
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
		self.attrs.get(&HkernAttrs::U2).map(String::as_str)
	}
}

impl Tag for Hkern {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
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
#[derive(Clone, Debug)]
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
		self.attrs.get(&ImageAttrs::Class).map(String::as_str)
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
		self.attrs.get(&ImageAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&ImageAttrs::Height).map(String::as_str)
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
		self.attrs.get(&ImageAttrs::PreserveAspectRatio).map(String::as_str)
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
		self.attrs.get(&ImageAttrs::Style).map(String::as_str)
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
		self.attrs.get(&ImageAttrs::Transform).map(String::as_str)
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
		self.attrs.get(&ImageAttrs::Width).map(String::as_str)
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
		self.attrs.get(&ImageAttrs::X).map(String::as_str)
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
		self.attrs.get(&ImageAttrs::XlinkHref).map(String::as_str)
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
		self.attrs.get(&ImageAttrs::Y).map(String::as_str)
	}
}

impl Tag for Image {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
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
#[derive(Clone, Debug)]
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
		self.attrs.get(&LineAttrs::Class).map(String::as_str)
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
		self.attrs.get(&LineAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&LineAttrs::Style).map(String::as_str)
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
		self.attrs.get(&LineAttrs::Transform).map(String::as_str)
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
		self.attrs.get(&LineAttrs::X1).map(String::as_str)
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
		self.attrs.get(&LineAttrs::X2).map(String::as_str)
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
		self.attrs.get(&LineAttrs::Y1).map(String::as_str)
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
		self.attrs.get(&LineAttrs::Y2).map(String::as_str)
	}
}

impl Tag for Line {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
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

#[doc = "The [`<linearGradient>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("linearGradient.md")]
#[doc = "\n\n [`<linearGradient>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/linearGradient"]
#[derive(Clone, Debug)]
pub struct LinearGradient {
	attrs: IndexMap<LinearGradientAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: LinearGradientAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&LinearGradientAttrs::Class).map(String::as_str)
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
		self.attrs.get(&LinearGradientAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&LinearGradientAttrs::GradientTransform).map(String::as_str)
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
		self.attrs.get(&LinearGradientAttrs::GradientUnits).map(String::as_str)
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
		self.attrs.get(&LinearGradientAttrs::SpreadMethod).map(String::as_str)
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
		self.attrs.get(&LinearGradientAttrs::Style).map(String::as_str)
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
		self.attrs.get(&LinearGradientAttrs::X1).map(String::as_str)
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
		self.attrs.get(&LinearGradientAttrs::X2).map(String::as_str)
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
		self.attrs.get(&LinearGradientAttrs::XlinkHref).map(String::as_str)
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
		self.attrs.get(&LinearGradientAttrs::Y1).map(String::as_str)
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
		self.attrs.get(&LinearGradientAttrs::Y2).map(String::as_str)
	}
}

impl Tag for LinearGradient {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("linearGradient");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
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

#[doc = "The [`<marker>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("marker.md")]
#[doc = "\n\n [`<marker>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/marker"]
#[derive(Clone, Debug)]
pub struct Marker {
	attrs: IndexMap<MarkerAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: MarkerAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&MarkerAttrs::Class).map(String::as_str)
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
		self.attrs.get(&MarkerAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&MarkerAttrs::MarkerHeight).map(String::as_str)
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
		self.attrs.get(&MarkerAttrs::MarkerUnits).map(String::as_str)
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
		self.attrs.get(&MarkerAttrs::MarkerWidth).map(String::as_str)
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
		self.attrs.get(&MarkerAttrs::Orient).map(String::as_str)
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
		self.attrs.get(&MarkerAttrs::PreserveAspectRatio).map(String::as_str)
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
		self.attrs.get(&MarkerAttrs::RefX).map(String::as_str)
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
		self.attrs.get(&MarkerAttrs::RefY).map(String::as_str)
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
		self.attrs.get(&MarkerAttrs::Style).map(String::as_str)
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
		self.attrs.get(&MarkerAttrs::Transform).map(String::as_str)
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
		self.attrs.get(&MarkerAttrs::ViewBox).map(String::as_str)
	}
}

impl Tag for Marker {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("marker");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
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

#[doc = "The [`<mask>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("mask.md")]
#[doc = "\n\n [`<mask>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/mask"]
#[derive(Clone, Debug)]
pub struct Mask {
	attrs: IndexMap<MaskAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: MaskAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&MaskAttrs::Class).map(String::as_str)
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
		self.attrs.get(&MaskAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&MaskAttrs::Height).map(String::as_str)
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
		self.attrs.get(&MaskAttrs::MaskContentUnits).map(String::as_str)
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
		self.attrs.get(&MaskAttrs::MaskUnits).map(String::as_str)
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
		self.attrs.get(&MaskAttrs::Style).map(String::as_str)
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
		self.attrs.get(&MaskAttrs::Width).map(String::as_str)
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
		self.attrs.get(&MaskAttrs::X).map(String::as_str)
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
		self.attrs.get(&MaskAttrs::Y).map(String::as_str)
	}
}

impl Tag for Mask {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("mask");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[doc = "The [`<metadata>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("metadata.md")]
#[doc = "\n\n [`<metadata>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/metadata"]
#[derive(Clone, Debug)]
pub struct Metadata {
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
		}
	}
}

impl Tag for Metadata {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("metadata");w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum MissingMinusGlyphAttrs {
	Class,
	D,
	HorizMinusAdvMinusX,
	Style,
	VertMinusAdvMinusY,
	VertMinusOriginMinusX,
	VertMinusOriginMinusY,
}

impl MissingMinusGlyphAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::D => "d",
			Self::HorizMinusAdvMinusX => "horiz-adv-x",
			Self::Style => "style",
			Self::VertMinusAdvMinusY => "vert-adv-y",
			Self::VertMinusOriginMinusX => "vert-origin-x",
			Self::VertMinusOriginMinusY => "vert-origin-y",
		}
	}
}

impl Display for MissingMinusGlyphAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for MissingMinusGlyphAttrs {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[doc = "The [`<missing-glyph>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("missing-glyph.md")]
#[doc = "\n\n [`<missing-glyph>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/missing-glyph"]
#[derive(Clone, Debug)]
pub struct MissingMinusGlyph {
	attrs: IndexMap<MissingMinusGlyphAttrs, String>,
}

impl Default for MissingMinusGlyph {
	fn default() -> Self {
		Self::new()
	}
}

impl MissingMinusGlyph {
	/// Create a new, empty tag.
	pub fn new() -> Self {
		Self {
			attrs: IndexMap::new(),
		}
	}

	fn set_attr(&mut self, attr: MissingMinusGlyphAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
	}

	/// Set the `class` attribute.
	pub fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MissingMinusGlyphAttrs::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	pub fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MissingMinusGlyphAttrs::Class, value.into());
	}

	/// Get the `class` attribute.
	pub fn class(&self) -> Option<&str> {
		self.attrs.get(&MissingMinusGlyphAttrs::Class).map(String::as_str)
	}

	/// Set the `d` attribute.
	pub fn with_d<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MissingMinusGlyphAttrs::D, value.into());
		self
	}

	/// Set the `d` attribute.
	pub fn set_d<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MissingMinusGlyphAttrs::D, value.into());
	}

	/// Get the `d` attribute.
	pub fn d(&self) -> Option<&str> {
		self.attrs.get(&MissingMinusGlyphAttrs::D).map(String::as_str)
	}

	/// Set the `horiz-adv-x` attribute.
	pub fn with_horiz_minus_adv_minus_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MissingMinusGlyphAttrs::HorizMinusAdvMinusX, value.into());
		self
	}

	/// Set the `horiz-adv-x` attribute.
	pub fn set_horiz_minus_adv_minus_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MissingMinusGlyphAttrs::HorizMinusAdvMinusX, value.into());
	}

	/// Get the `horiz-adv-x` attribute.
	pub fn horiz_minus_adv_minus_x(&self) -> Option<&str> {
		self.attrs.get(&MissingMinusGlyphAttrs::HorizMinusAdvMinusX).map(String::as_str)
	}

	/// Set the `style` attribute.
	pub fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MissingMinusGlyphAttrs::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	pub fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MissingMinusGlyphAttrs::Style, value.into());
	}

	/// Get the `style` attribute.
	pub fn style(&self) -> Option<&str> {
		self.attrs.get(&MissingMinusGlyphAttrs::Style).map(String::as_str)
	}

	/// Set the `vert-adv-y` attribute.
	pub fn with_vert_minus_adv_minus_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MissingMinusGlyphAttrs::VertMinusAdvMinusY, value.into());
		self
	}

	/// Set the `vert-adv-y` attribute.
	pub fn set_vert_minus_adv_minus_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MissingMinusGlyphAttrs::VertMinusAdvMinusY, value.into());
	}

	/// Get the `vert-adv-y` attribute.
	pub fn vert_minus_adv_minus_y(&self) -> Option<&str> {
		self.attrs.get(&MissingMinusGlyphAttrs::VertMinusAdvMinusY).map(String::as_str)
	}

	/// Set the `vert-origin-x` attribute.
	pub fn with_vert_minus_origin_minus_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MissingMinusGlyphAttrs::VertMinusOriginMinusX, value.into());
		self
	}

	/// Set the `vert-origin-x` attribute.
	pub fn set_vert_minus_origin_minus_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MissingMinusGlyphAttrs::VertMinusOriginMinusX, value.into());
	}

	/// Get the `vert-origin-x` attribute.
	pub fn vert_minus_origin_minus_x(&self) -> Option<&str> {
		self.attrs.get(&MissingMinusGlyphAttrs::VertMinusOriginMinusX).map(String::as_str)
	}

	/// Set the `vert-origin-y` attribute.
	pub fn with_vert_minus_origin_minus_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(MissingMinusGlyphAttrs::VertMinusOriginMinusY, value.into());
		self
	}

	/// Set the `vert-origin-y` attribute.
	pub fn set_vert_minus_origin_minus_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(MissingMinusGlyphAttrs::VertMinusOriginMinusY, value.into());
	}

	/// Get the `vert-origin-y` attribute.
	pub fn vert_minus_origin_minus_y(&self) -> Option<&str> {
		self.attrs.get(&MissingMinusGlyphAttrs::VertMinusOriginMinusY).map(String::as_str)
	}
}

impl Tag for MissingMinusGlyph {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("missing-glyph");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum MpathAttrs {
	ExternalResourcesRequired,
	XlinkHref,
}

impl MpathAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::XlinkHref => "xlink:href",
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
#[derive(Clone, Debug)]
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
		self.attrs.get(&MpathAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&MpathAttrs::XlinkHref).map(String::as_str)
	}
}

impl Tag for Mpath {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
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
#[derive(Clone, Debug)]
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
		self.attrs.get(&PathAttrs::Class).map(String::as_str)
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
		self.attrs.get(&PathAttrs::D).map(String::as_str)
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
		self.attrs.get(&PathAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&PathAttrs::PathLength).map(String::as_str)
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
		self.attrs.get(&PathAttrs::Style).map(String::as_str)
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
		self.attrs.get(&PathAttrs::Transform).map(String::as_str)
	}
}

impl Tag for Path {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
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

#[doc = "The [`<pattern>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("pattern.md")]
#[doc = "\n\n [`<pattern>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/pattern"]
#[derive(Clone, Debug)]
pub struct Pattern {
	attrs: IndexMap<PatternAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: PatternAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&PatternAttrs::Class).map(String::as_str)
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
		self.attrs.get(&PatternAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&PatternAttrs::Height).map(String::as_str)
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
		self.attrs.get(&PatternAttrs::PatternContentUnits).map(String::as_str)
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
		self.attrs.get(&PatternAttrs::PatternTransform).map(String::as_str)
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
		self.attrs.get(&PatternAttrs::PatternUnits).map(String::as_str)
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
		self.attrs.get(&PatternAttrs::PreserveAspectRatio).map(String::as_str)
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
		self.attrs.get(&PatternAttrs::Style).map(String::as_str)
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
		self.attrs.get(&PatternAttrs::ViewBox).map(String::as_str)
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
		self.attrs.get(&PatternAttrs::Width).map(String::as_str)
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
		self.attrs.get(&PatternAttrs::X).map(String::as_str)
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
		self.attrs.get(&PatternAttrs::XlinkHref).map(String::as_str)
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
		self.attrs.get(&PatternAttrs::Y).map(String::as_str)
	}
}

impl Tag for Pattern {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("pattern");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
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
}

impl PolygonAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Points => "points",
			Self::Style => "style",
			Self::Transform => "transform",
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
#[derive(Clone, Debug)]
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
		self.attrs.get(&PolygonAttrs::Class).map(String::as_str)
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
		self.attrs.get(&PolygonAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&PolygonAttrs::Points).map(String::as_str)
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
		self.attrs.get(&PolygonAttrs::Style).map(String::as_str)
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
		self.attrs.get(&PolygonAttrs::Transform).map(String::as_str)
	}
}

impl Tag for Polygon {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
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
}

impl PolylineAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Points => "points",
			Self::Style => "style",
			Self::Transform => "transform",
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
#[derive(Clone, Debug)]
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
		self.attrs.get(&PolylineAttrs::Class).map(String::as_str)
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
		self.attrs.get(&PolylineAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&PolylineAttrs::Points).map(String::as_str)
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
		self.attrs.get(&PolylineAttrs::Style).map(String::as_str)
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
		self.attrs.get(&PolylineAttrs::Transform).map(String::as_str)
	}
}

impl Tag for Polyline {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
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

#[doc = "The [`<radialGradient>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("radialGradient.md")]
#[doc = "\n\n [`<radialGradient>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/radialGradient"]
#[derive(Clone, Debug)]
pub struct RadialGradient {
	attrs: IndexMap<RadialGradientAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: RadialGradientAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&RadialGradientAttrs::Class).map(String::as_str)
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
		self.attrs.get(&RadialGradientAttrs::Cx).map(String::as_str)
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
		self.attrs.get(&RadialGradientAttrs::Cy).map(String::as_str)
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
		self.attrs.get(&RadialGradientAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&RadialGradientAttrs::Fx).map(String::as_str)
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
		self.attrs.get(&RadialGradientAttrs::Fy).map(String::as_str)
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
		self.attrs.get(&RadialGradientAttrs::GradientTransform).map(String::as_str)
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
		self.attrs.get(&RadialGradientAttrs::GradientUnits).map(String::as_str)
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
		self.attrs.get(&RadialGradientAttrs::R).map(String::as_str)
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
		self.attrs.get(&RadialGradientAttrs::SpreadMethod).map(String::as_str)
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
		self.attrs.get(&RadialGradientAttrs::Style).map(String::as_str)
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
		self.attrs.get(&RadialGradientAttrs::XlinkHref).map(String::as_str)
	}
}

impl Tag for RadialGradient {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("radialGradient");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
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
#[derive(Clone, Debug)]
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
		self.attrs.get(&RectAttrs::Class).map(String::as_str)
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
		self.attrs.get(&RectAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&RectAttrs::Height).map(String::as_str)
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
		self.attrs.get(&RectAttrs::Rx).map(String::as_str)
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
		self.attrs.get(&RectAttrs::Ry).map(String::as_str)
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
		self.attrs.get(&RectAttrs::Style).map(String::as_str)
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
		self.attrs.get(&RectAttrs::Transform).map(String::as_str)
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
		self.attrs.get(&RectAttrs::Width).map(String::as_str)
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
		self.attrs.get(&RectAttrs::X).map(String::as_str)
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
		self.attrs.get(&RectAttrs::Y).map(String::as_str)
	}
}

impl Tag for Rect {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
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
}

impl ScriptAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Type => "type",
			Self::XlinkHref => "xlink:href",
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
#[derive(Clone, Debug)]
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
		self.attrs.get(&ScriptAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&ScriptAttrs::Type).map(String::as_str)
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
		self.attrs.get(&ScriptAttrs::XlinkHref).map(String::as_str)
	}
}

impl Tag for Script {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
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
}

impl SetAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::To => "to",
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
#[derive(Clone, Debug)]
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
		self.attrs.get(&SetAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&SetAttrs::To).map(String::as_str)
	}
}

impl Tag for Set {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
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
	StopMinusColor,
	StopMinusOpacity,
	Style,
}

impl StopAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::Offset => "offset",
			Self::StopMinusColor => "stop-color",
			Self::StopMinusOpacity => "stop-opacity",
			Self::Style => "style",
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

#[doc = "The [`<stop>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("stop.md")]
#[doc = "\n\n [`<stop>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/stop"]
#[derive(Clone, Debug)]
pub struct Stop {
	attrs: IndexMap<StopAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: StopAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&StopAttrs::Class).map(String::as_str)
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
		self.attrs.get(&StopAttrs::Offset).map(String::as_str)
	}

	/// Set the `stop-color` attribute.
	pub fn with_stop_minus_color<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(StopAttrs::StopMinusColor, value.into());
		self
	}

	/// Set the `stop-color` attribute.
	pub fn set_stop_minus_color<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(StopAttrs::StopMinusColor, value.into());
	}

	/// Get the `stop-color` attribute.
	pub fn stop_minus_color(&self) -> Option<&str> {
		self.attrs.get(&StopAttrs::StopMinusColor).map(String::as_str)
	}

	/// Set the `stop-opacity` attribute.
	pub fn with_stop_minus_opacity<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(StopAttrs::StopMinusOpacity, value.into());
		self
	}

	/// Set the `stop-opacity` attribute.
	pub fn set_stop_minus_opacity<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(StopAttrs::StopMinusOpacity, value.into());
	}

	/// Get the `stop-opacity` attribute.
	pub fn stop_minus_opacity(&self) -> Option<&str> {
		self.attrs.get(&StopAttrs::StopMinusOpacity).map(String::as_str)
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
		self.attrs.get(&StopAttrs::Style).map(String::as_str)
	}
}

impl Tag for Stop {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("stop");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum StyleAttrs {
	Media,
	Title,
	Type,
}

impl StyleAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Media => "media",
			Self::Title => "title",
			Self::Type => "type",
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
#[derive(Clone, Debug)]
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
		self.attrs.get(&StyleAttrs::Media).map(String::as_str)
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
		self.attrs.get(&StyleAttrs::Title).map(String::as_str)
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
		self.attrs.get(&StyleAttrs::Type).map(String::as_str)
	}
}

impl Tag for Style {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("style");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum SvgAttrs {
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
}

impl SvgAttrs {
	fn as_str(&self) -> &'static str {
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

#[doc = "The [`<svg>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("svg.md")]
#[doc = "\n\n [`<svg>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/svg"]
#[derive(Clone, Debug)]
pub struct Svg {
	attrs: IndexMap<SvgAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: SvgAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&SvgAttrs::BaseProfile).map(String::as_str)
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
		self.attrs.get(&SvgAttrs::Class).map(String::as_str)
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
		self.attrs.get(&SvgAttrs::ContentScriptType).map(String::as_str)
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
		self.attrs.get(&SvgAttrs::ContentStyleType).map(String::as_str)
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
		self.attrs.get(&SvgAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&SvgAttrs::Height).map(String::as_str)
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
		self.attrs.get(&SvgAttrs::PreserveAspectRatio).map(String::as_str)
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
		self.attrs.get(&SvgAttrs::Style).map(String::as_str)
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
		self.attrs.get(&SvgAttrs::Version).map(String::as_str)
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
		self.attrs.get(&SvgAttrs::ViewBox).map(String::as_str)
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
		self.attrs.get(&SvgAttrs::Width).map(String::as_str)
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
		self.attrs.get(&SvgAttrs::X).map(String::as_str)
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
		self.attrs.get(&SvgAttrs::Y).map(String::as_str)
	}
}

impl Tag for Svg {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("svg");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
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
}

impl SwitchAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::AllowReorder => "allowReorder",
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Style => "style",
			Self::Transform => "transform",
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

#[doc = "The [`<switch>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("switch.md")]
#[doc = "\n\n [`<switch>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/switch"]
#[derive(Clone, Debug)]
pub struct Switch {
	attrs: IndexMap<SwitchAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: SwitchAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&SwitchAttrs::AllowReorder).map(String::as_str)
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
		self.attrs.get(&SwitchAttrs::Class).map(String::as_str)
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
		self.attrs.get(&SwitchAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&SwitchAttrs::Style).map(String::as_str)
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
		self.attrs.get(&SwitchAttrs::Transform).map(String::as_str)
	}
}

impl Tag for Switch {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("switch");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
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
}

impl SymbolAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::PreserveAspectRatio => "preserveAspectRatio",
			Self::Style => "style",
			Self::ViewBox => "viewBox",
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

#[doc = "The [`<symbol>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("symbol.md")]
#[doc = "\n\n [`<symbol>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/symbol"]
#[derive(Clone, Debug)]
pub struct Symbol {
	attrs: IndexMap<SymbolAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: SymbolAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&SymbolAttrs::Class).map(String::as_str)
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
		self.attrs.get(&SymbolAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&SymbolAttrs::PreserveAspectRatio).map(String::as_str)
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
		self.attrs.get(&SymbolAttrs::Style).map(String::as_str)
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
		self.attrs.get(&SymbolAttrs::ViewBox).map(String::as_str)
	}
}

impl Tag for Symbol {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("symbol");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
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
	TextMinusAnchor,
	TextLength,
	Transform,
	X,
	Y,
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
			Self::TextMinusAnchor => "text-anchor",
			Self::TextLength => "textLength",
			Self::Transform => "transform",
			Self::X => "x",
			Self::Y => "y",
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

#[doc = "The [`<text>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("text.md")]
#[doc = "\n\n [`<text>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/text"]
#[derive(Clone, Debug)]
pub struct Text {
	attrs: IndexMap<TextAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: TextAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&TextAttrs::Class).map(String::as_str)
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
		self.attrs.get(&TextAttrs::Dx).map(String::as_str)
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
		self.attrs.get(&TextAttrs::Dy).map(String::as_str)
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
		self.attrs.get(&TextAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&TextAttrs::LengthAdjust).map(String::as_str)
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
		self.attrs.get(&TextAttrs::Rotate).map(String::as_str)
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
		self.attrs.get(&TextAttrs::Style).map(String::as_str)
	}

	/// Set the `text-anchor` attribute.
	pub fn with_text_minus_anchor<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TextAttrs::TextMinusAnchor, value.into());
		self
	}

	/// Set the `text-anchor` attribute.
	pub fn set_text_minus_anchor<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TextAttrs::TextMinusAnchor, value.into());
	}

	/// Get the `text-anchor` attribute.
	pub fn text_minus_anchor(&self) -> Option<&str> {
		self.attrs.get(&TextAttrs::TextMinusAnchor).map(String::as_str)
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
		self.attrs.get(&TextAttrs::TextLength).map(String::as_str)
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
		self.attrs.get(&TextAttrs::Transform).map(String::as_str)
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
		self.attrs.get(&TextAttrs::X).map(String::as_str)
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
		self.attrs.get(&TextAttrs::Y).map(String::as_str)
	}
}

impl Tag for Text {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("text");
		w.set_preserve_whitespaces(true);
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
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

#[doc = "The [`<textPath>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("textPath.md")]
#[doc = "\n\n [`<textPath>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/textPath"]
#[derive(Clone, Debug)]
pub struct TextPath {
	attrs: IndexMap<TextPathAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: TextPathAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&TextPathAttrs::Class).map(String::as_str)
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
		self.attrs.get(&TextPathAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&TextPathAttrs::Method).map(String::as_str)
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
		self.attrs.get(&TextPathAttrs::Spacing).map(String::as_str)
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
		self.attrs.get(&TextPathAttrs::StartOffset).map(String::as_str)
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
		self.attrs.get(&TextPathAttrs::Style).map(String::as_str)
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
		self.attrs.get(&TextPathAttrs::XlinkHref).map(String::as_str)
	}
}

impl Tag for TextPath {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("textPath");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum TitleAttrs {
	Class,
	Style,
}

impl TitleAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::Style => "style",
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
#[derive(Clone, Debug)]
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
		self.attrs.get(&TitleAttrs::Class).map(String::as_str)
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
		self.attrs.get(&TitleAttrs::Style).map(String::as_str)
	}
}

impl Tag for Title {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
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
}

impl TrefAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::Style => "style",
			Self::XlinkHref => "xlink:href",
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

#[doc = "The [`<tref>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("tref.md")]
#[doc = "\n\n [`<tref>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/tref"]
#[derive(Clone, Debug)]
pub struct Tref {
	attrs: IndexMap<TrefAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: TrefAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&TrefAttrs::Class).map(String::as_str)
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
		self.attrs.get(&TrefAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&TrefAttrs::Style).map(String::as_str)
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
		self.attrs.get(&TrefAttrs::XlinkHref).map(String::as_str)
	}
}

impl Tag for Tref {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("tref");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
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

#[doc = "The [`<tspan>`] svg tag.\n\n# Content\n"]
#[doc = include_str!("tspan.md")]
#[doc = "\n\n [`<tspan>`]: https://developer.mozilla.org/en-US/docs/Web/SVG/Element/tspan"]
#[derive(Clone, Debug)]
pub struct Tspan {
	attrs: IndexMap<TspanAttrs, String>,
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
		}
	}

	fn set_attr(&mut self, attr: TspanAttrs, value: String) {
		self.attrs.shift_remove(&attr);
		self.attrs.insert(attr, value);
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
		self.attrs.get(&TspanAttrs::Class).map(String::as_str)
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
		self.attrs.get(&TspanAttrs::Dx).map(String::as_str)
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
		self.attrs.get(&TspanAttrs::Dy).map(String::as_str)
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
		self.attrs.get(&TspanAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&TspanAttrs::LengthAdjust).map(String::as_str)
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
		self.attrs.get(&TspanAttrs::Rotate).map(String::as_str)
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
		self.attrs.get(&TspanAttrs::Style).map(String::as_str)
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
		self.attrs.get(&TspanAttrs::TextLength).map(String::as_str)
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
		self.attrs.get(&TspanAttrs::X).map(String::as_str)
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
		self.attrs.get(&TspanAttrs::Y).map(String::as_str)
	}
}

impl Tag for Tspan {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("tspan");
		w.set_preserve_whitespaces(true);
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
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
#[derive(Clone, Debug)]
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
		self.attrs.get(&UseAttrs::Class).map(String::as_str)
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
		self.attrs.get(&UseAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&UseAttrs::Height).map(String::as_str)
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
		self.attrs.get(&UseAttrs::Style).map(String::as_str)
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
		self.attrs.get(&UseAttrs::Transform).map(String::as_str)
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
		self.attrs.get(&UseAttrs::Width).map(String::as_str)
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
		self.attrs.get(&UseAttrs::X).map(String::as_str)
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
		self.attrs.get(&UseAttrs::XlinkHref).map(String::as_str)
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
		self.attrs.get(&UseAttrs::Y).map(String::as_str)
	}
}

impl Tag for Use {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
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
}

impl ViewAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::ExternalResourcesRequired => "externalResourcesRequired",
			Self::PreserveAspectRatio => "preserveAspectRatio",
			Self::ViewBox => "viewBox",
			Self::ViewTarget => "viewTarget",
			Self::ZoomAndPan => "zoomAndPan",
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
#[derive(Clone, Debug)]
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
		self.attrs.get(&ViewAttrs::ExternalResourcesRequired).map(String::as_str)
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
		self.attrs.get(&ViewAttrs::PreserveAspectRatio).map(String::as_str)
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
		self.attrs.get(&ViewAttrs::ViewBox).map(String::as_str)
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
		self.attrs.get(&ViewAttrs::ViewTarget).map(String::as_str)
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
		self.attrs.get(&ViewAttrs::ZoomAndPan).map(String::as_str)
	}
}

impl Tag for View {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
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
}

impl VkernAttrs {
	fn as_str(&self) -> &'static str {
		match self {
			Self::G1 => "g1",
			Self::G2 => "g2",
			Self::K => "k",
			Self::U1 => "u1",
			Self::U2 => "u2",
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
#[derive(Clone, Debug)]
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
		self.attrs.get(&VkernAttrs::G1).map(String::as_str)
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
		self.attrs.get(&VkernAttrs::G2).map(String::as_str)
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
		self.attrs.get(&VkernAttrs::K).map(String::as_str)
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
		self.attrs.get(&VkernAttrs::U1).map(String::as_str)
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
		self.attrs.get(&VkernAttrs::U2).map(String::as_str)
	}
}

impl Tag for Vkern {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		w.start_element("vkern");
		for (attr, value) in &self.attrs {
			w.write_attribute(attr.as_str(), &value);
		}
		w.end_element();
	}
}