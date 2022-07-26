// @generated

use std::fmt::{self, Debug, Display, Formatter};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub(super) enum AnimationAdditionAttributes {
	Accumulate,
	Additive,
}

impl AnimationAdditionAttributes {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Accumulate => "accumulate",
			Self::Additive => "additive",
		}
	}
}

impl Display for AnimationAdditionAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for AnimationAdditionAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub(super) enum AnimationAttributeTargetAttributes {
	AttributeName,
	AttributeType,
}

impl AnimationAttributeTargetAttributes {
	fn as_str(&self) -> &'static str {
		match self {
			Self::AttributeName => "attributeName",
			Self::AttributeType => "attributeType",
		}
	}
}

impl Display for AnimationAttributeTargetAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for AnimationAttributeTargetAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub(super) enum AnimationEventAttributes {
	Onbegin,
	Onend,
	Onrepeat,
}

impl AnimationEventAttributes {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Onbegin => "onbegin",
			Self::Onend => "onend",
			Self::Onrepeat => "onrepeat",
		}
	}
}

impl Display for AnimationEventAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for AnimationEventAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub(super) enum AnimationTimingAttributes {
	Begin,
	Dur,
	End,
	Fill,
	Max,
	Min,
	RepeatCount,
	RepeatDur,
	Restart,
}

impl AnimationTimingAttributes {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Begin => "begin",
			Self::Dur => "dur",
			Self::End => "end",
			Self::Fill => "fill",
			Self::Max => "max",
			Self::Min => "min",
			Self::RepeatCount => "repeatCount",
			Self::RepeatDur => "repeatDur",
			Self::Restart => "restart",
		}
	}
}

impl Display for AnimationTimingAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for AnimationTimingAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub(super) enum AnimationValueAttributes {
	Accelerate,
	AutoReverse,
	By,
	CalcMode,
	Decelerate,
	From,
	KeySplines,
	KeyTimes,
	To,
	Values,
}

impl AnimationValueAttributes {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Accelerate => "accelerate",
			Self::AutoReverse => "autoReverse",
			Self::By => "by",
			Self::CalcMode => "calcMode",
			Self::Decelerate => "decelerate",
			Self::From => "from",
			Self::KeySplines => "keySplines",
			Self::KeyTimes => "keyTimes",
			Self::To => "to",
			Self::Values => "values",
		}
	}
}

impl Display for AnimationValueAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for AnimationValueAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub(super) enum ConditionalProccessingAttributes {
	RequiredExtensions,
	RequiredFeatures,
	SystemLanguage,
}

impl ConditionalProccessingAttributes {
	fn as_str(&self) -> &'static str {
		match self {
			Self::RequiredExtensions => "requiredExtensions",
			Self::RequiredFeatures => "requiredFeatures",
			Self::SystemLanguage => "systemLanguage",
		}
	}
}

impl Display for ConditionalProccessingAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for ConditionalProccessingAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub(super) enum CoreAttributes {
	Id,
	Lang,
	Tabindex,
	XmlBase,
	XmlLang,
	XmlSpace,
}

impl CoreAttributes {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Id => "id",
			Self::Lang => "lang",
			Self::Tabindex => "tabindex",
			Self::XmlBase => "xml:base",
			Self::XmlLang => "xml:lang",
			Self::XmlSpace => "xml:space",
		}
	}
}

impl Display for CoreAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for CoreAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub(super) enum DocumentEventAttributes {
	Onabort,
	Onerror,
	Onresize,
	Onscroll,
	Onunload,
}

impl DocumentEventAttributes {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Onabort => "onabort",
			Self::Onerror => "onerror",
			Self::Onresize => "onresize",
			Self::Onscroll => "onscroll",
			Self::Onunload => "onunload",
		}
	}
}

impl Display for DocumentEventAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for DocumentEventAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub(super) enum FilterAttributes {
	Height,
	Result,
	Width,
	X,
	Y,
}

impl FilterAttributes {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Height => "height",
			Self::Result => "result",
			Self::Width => "width",
			Self::X => "x",
			Self::Y => "y",
		}
	}
}

impl Display for FilterAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for FilterAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub(super) enum GlobalEventAttributes {
	Oncancel,
	Oncanplay,
	Oncanplaythrough,
	Onchange,
	Onclick,
	Onclose,
	Oncuechange,
	Ondblclick,
	Ondrag,
	Ondragend,
	Ondragenter,
	Ondragleave,
	Ondragover,
	Ondragstart,
	Ondrop,
	Ondurationchange,
	Onemptied,
	Onended,
	Onerror,
	Onfocus,
	Oninput,
	Oninvalid,
	Onkeydown,
	Onkeypress,
	Onkeyup,
	Onload,
	Onloadeddata,
	Onloadedmetadata,
	Onloadstart,
	Onmousedown,
	Onmouseenter,
	Onmouseleave,
	Onmousemove,
	Onmouseout,
	Onmouseover,
	Onmouseup,
	Onmousewheel,
	Onpause,
	Onplay,
	Onplaying,
	Onprogress,
	Onratechange,
	Onreset,
	Onresize,
	Onscroll,
	Onseeked,
	Onseeking,
	Onselect,
	Onshow,
	Onstalled,
	Onsubmit,
	Onsuspend,
	Ontimeupdate,
	Ontoggle,
	Onvolumechange,
	Onwaiting,
}

impl GlobalEventAttributes {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Oncancel => "oncancel",
			Self::Oncanplay => "oncanplay",
			Self::Oncanplaythrough => "oncanplaythrough",
			Self::Onchange => "onchange",
			Self::Onclick => "onclick",
			Self::Onclose => "onclose",
			Self::Oncuechange => "oncuechange",
			Self::Ondblclick => "ondblclick",
			Self::Ondrag => "ondrag",
			Self::Ondragend => "ondragend",
			Self::Ondragenter => "ondragenter",
			Self::Ondragleave => "ondragleave",
			Self::Ondragover => "ondragover",
			Self::Ondragstart => "ondragstart",
			Self::Ondrop => "ondrop",
			Self::Ondurationchange => "ondurationchange",
			Self::Onemptied => "onemptied",
			Self::Onended => "onended",
			Self::Onerror => "onerror",
			Self::Onfocus => "onfocus",
			Self::Oninput => "oninput",
			Self::Oninvalid => "oninvalid",
			Self::Onkeydown => "onkeydown",
			Self::Onkeypress => "onkeypress",
			Self::Onkeyup => "onkeyup",
			Self::Onload => "onload",
			Self::Onloadeddata => "onloadeddata",
			Self::Onloadedmetadata => "onloadedmetadata",
			Self::Onloadstart => "onloadstart",
			Self::Onmousedown => "onmousedown",
			Self::Onmouseenter => "onmouseenter",
			Self::Onmouseleave => "onmouseleave",
			Self::Onmousemove => "onmousemove",
			Self::Onmouseout => "onmouseout",
			Self::Onmouseover => "onmouseover",
			Self::Onmouseup => "onmouseup",
			Self::Onmousewheel => "onmousewheel",
			Self::Onpause => "onpause",
			Self::Onplay => "onplay",
			Self::Onplaying => "onplaying",
			Self::Onprogress => "onprogress",
			Self::Onratechange => "onratechange",
			Self::Onreset => "onreset",
			Self::Onresize => "onresize",
			Self::Onscroll => "onscroll",
			Self::Onseeked => "onseeked",
			Self::Onseeking => "onseeking",
			Self::Onselect => "onselect",
			Self::Onshow => "onshow",
			Self::Onstalled => "onstalled",
			Self::Onsubmit => "onsubmit",
			Self::Onsuspend => "onsuspend",
			Self::Ontimeupdate => "ontimeupdate",
			Self::Ontoggle => "ontoggle",
			Self::Onvolumechange => "onvolumechange",
			Self::Onwaiting => "onwaiting",
		}
	}
}

impl Display for GlobalEventAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for GlobalEventAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub(super) enum GraphicalEventAttributes {
	Onactivate,
	Onfocusin,
	Onfocusout,
}

impl GraphicalEventAttributes {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Onactivate => "onactivate",
			Self::Onfocusin => "onfocusin",
			Self::Onfocusout => "onfocusout",
		}
	}
}

impl Display for GraphicalEventAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for GraphicalEventAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub(super) enum PresentationAttributes {
	AlignmentMinusBaseline,
	BaselineMinusShift,
	Clip,
	ClipMinusPath,
	ClipMinusRule,
	Color,
	ColorMinusInterpolation,
	ColorMinusInterpolationMinusFilters,
	ColorMinusProfile,
	ColorMinusRendering,
	Cursor,
	Direction,
	Display,
	DominantMinusBaseline,
	EnableMinusBackground,
	Fill,
	FillMinusOpacity,
	FillMinusRule,
	Filter,
	FloodMinusColor,
	FloodMinusOpacity,
	FontMinusFamily,
	FontMinusSize,
	FontMinusSizeMinusAdjust,
	FontMinusStretch,
	FontMinusStyle,
	FontMinusVariant,
	FontMinusWeight,
	GlyphMinusOrientationMinusHorizontal,
	GlyphMinusOrientationMinusVertical,
	ImageMinusRendering,
	Kerning,
	LetterMinusSpacing,
	LightingMinusColor,
	MarkerMinusEnd,
	MarkerMinusMid,
	MarkerMinusStart,
	Mask,
	Opacity,
	Overflow,
	PointerMinusEvents,
	ShapeMinusRendering,
	StopMinusColor,
	StopMinusOpacity,
	Stroke,
	StrokeMinusDasharray,
	StrokeMinusDashoffset,
	StrokeMinusLinecap,
	StrokeMinusLinejoin,
	StrokeMinusMiterlimit,
	StrokeMinusOpacity,
	StrokeMinusWidth,
	TextMinusAnchor,
	TextMinusDecoration,
	TextMinusRendering,
	Transform,
	TransformMinusOrigin,
	UnicodeMinusBidi,
	VectorMinusEffect,
	Visibility,
	WordMinusSpacing,
	WritingMinusMode,
}

impl PresentationAttributes {
	fn as_str(&self) -> &'static str {
		match self {
			Self::AlignmentMinusBaseline => "alignment-baseline",
			Self::BaselineMinusShift => "baseline-shift",
			Self::Clip => "clip",
			Self::ClipMinusPath => "clip-path",
			Self::ClipMinusRule => "clip-rule",
			Self::Color => "color",
			Self::ColorMinusInterpolation => "color-interpolation",
			Self::ColorMinusInterpolationMinusFilters => "color-interpolation-filters",
			Self::ColorMinusProfile => "color-profile",
			Self::ColorMinusRendering => "color-rendering",
			Self::Cursor => "cursor",
			Self::Direction => "direction",
			Self::Display => "display",
			Self::DominantMinusBaseline => "dominant-baseline",
			Self::EnableMinusBackground => "enable-background",
			Self::Fill => "fill",
			Self::FillMinusOpacity => "fill-opacity",
			Self::FillMinusRule => "fill-rule",
			Self::Filter => "filter",
			Self::FloodMinusColor => "flood-color",
			Self::FloodMinusOpacity => "flood-opacity",
			Self::FontMinusFamily => "font-family",
			Self::FontMinusSize => "font-size",
			Self::FontMinusSizeMinusAdjust => "font-size-adjust",
			Self::FontMinusStretch => "font-stretch",
			Self::FontMinusStyle => "font-style",
			Self::FontMinusVariant => "font-variant",
			Self::FontMinusWeight => "font-weight",
			Self::GlyphMinusOrientationMinusHorizontal => "glyph-orientation-horizontal",
			Self::GlyphMinusOrientationMinusVertical => "glyph-orientation-vertical",
			Self::ImageMinusRendering => "image-rendering",
			Self::Kerning => "kerning",
			Self::LetterMinusSpacing => "letter-spacing",
			Self::LightingMinusColor => "lighting-color",
			Self::MarkerMinusEnd => "marker-end",
			Self::MarkerMinusMid => "marker-mid",
			Self::MarkerMinusStart => "marker-start",
			Self::Mask => "mask",
			Self::Opacity => "opacity",
			Self::Overflow => "overflow",
			Self::PointerMinusEvents => "pointer-events",
			Self::ShapeMinusRendering => "shape-rendering",
			Self::StopMinusColor => "stop-color",
			Self::StopMinusOpacity => "stop-opacity",
			Self::Stroke => "stroke",
			Self::StrokeMinusDasharray => "stroke-dasharray",
			Self::StrokeMinusDashoffset => "stroke-dashoffset",
			Self::StrokeMinusLinecap => "stroke-linecap",
			Self::StrokeMinusLinejoin => "stroke-linejoin",
			Self::StrokeMinusMiterlimit => "stroke-miterlimit",
			Self::StrokeMinusOpacity => "stroke-opacity",
			Self::StrokeMinusWidth => "stroke-width",
			Self::TextMinusAnchor => "text-anchor",
			Self::TextMinusDecoration => "text-decoration",
			Self::TextMinusRendering => "text-rendering",
			Self::Transform => "transform",
			Self::TransformMinusOrigin => "transform-origin",
			Self::UnicodeMinusBidi => "unicode-bidi",
			Self::VectorMinusEffect => "vector-effect",
			Self::Visibility => "visibility",
			Self::WordMinusSpacing => "word-spacing",
			Self::WritingMinusMode => "writing-mode",
		}
	}
}

impl Display for PresentationAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for PresentationAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub(super) enum StyleAttributes {
	Class,
	Style,
}

impl StyleAttributes {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Class => "class",
			Self::Style => "style",
		}
	}
}

impl Display for StyleAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for StyleAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub(super) enum TransferFunctionAttributes {
	Amplitude,
	Exponent,
	Intercept,
	Offset,
	Slope,
	TableValues,
	Type,
}

impl TransferFunctionAttributes {
	fn as_str(&self) -> &'static str {
		match self {
			Self::Amplitude => "amplitude",
			Self::Exponent => "exponent",
			Self::Intercept => "intercept",
			Self::Offset => "offset",
			Self::Slope => "slope",
			Self::TableValues => "tableValues",
			Self::Type => "type",
		}
	}
}

impl Display for TransferFunctionAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for TransferFunctionAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub(super) enum XLinkAttributes {
	XlinkActuate,
	XlinkArcrole,
	XlinkHref,
	XlinkRole,
	XlinkShow,
	XlinkTitle,
	XlinkType,
}

impl XLinkAttributes {
	fn as_str(&self) -> &'static str {
		match self {
			Self::XlinkActuate => "xlink:actuate",
			Self::XlinkArcrole => "xlink:arcrole",
			Self::XlinkHref => "xlink:href",
			Self::XlinkRole => "xlink:role",
			Self::XlinkShow => "xlink:show",
			Self::XlinkTitle => "xlink:title",
			Self::XlinkType => "xlink:type",
		}
	}
}

impl Display for XLinkAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for XLinkAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}
