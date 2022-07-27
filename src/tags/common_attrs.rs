// @generated

use std::fmt::{self, Debug, Display, Formatter};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum AnimationAdditionAttributes {
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

pub trait AnimationAdditionAttributesSetter {
	fn set_attr(&mut self, attr: AnimationAdditionAttributes, value: String);
	fn get_attr(&self, attr: AnimationAdditionAttributes) -> Option<&str>;
}

pub trait TagWithAnimationAdditionAttributes: AnimationAdditionAttributesSetter + Sized {

	/// Set the `accumulate` attribute.
	fn with_accumulate<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimationAdditionAttributes::Accumulate, value.into());
		self
	}

	/// Set the `accumulate` attribute.
	fn set_accumulate<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimationAdditionAttributes::Accumulate, value.into());
	}

	/// Get the `accumulate` attribute.
	fn accumulate(&self) -> Option<&str> {
		self.get_attr(AnimationAdditionAttributes::Accumulate)
	}

	/// Set the `additive` attribute.
	fn with_additive<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimationAdditionAttributes::Additive, value.into());
		self
	}

	/// Set the `additive` attribute.
	fn set_additive<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimationAdditionAttributes::Additive, value.into());
	}

	/// Get the `additive` attribute.
	fn additive(&self) -> Option<&str> {
		self.get_attr(AnimationAdditionAttributes::Additive)
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum AnimationAttributeTargetAttributes {
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

pub trait AnimationAttributeTargetAttributesSetter {
	fn set_attr(&mut self, attr: AnimationAttributeTargetAttributes, value: String);
	fn get_attr(&self, attr: AnimationAttributeTargetAttributes) -> Option<&str>;
}

pub trait TagWithAnimationAttributeTargetAttributes: AnimationAttributeTargetAttributesSetter + Sized {

	/// Set the `attributeName` attribute.
	fn with_attribute_name<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimationAttributeTargetAttributes::AttributeName, value.into());
		self
	}

	/// Set the `attributeName` attribute.
	fn set_attribute_name<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimationAttributeTargetAttributes::AttributeName, value.into());
	}

	/// Get the `attributeName` attribute.
	fn attribute_name(&self) -> Option<&str> {
		self.get_attr(AnimationAttributeTargetAttributes::AttributeName)
	}

	/// Set the `attributeType` attribute.
	fn with_attribute_type<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimationAttributeTargetAttributes::AttributeType, value.into());
		self
	}

	/// Set the `attributeType` attribute.
	fn set_attribute_type<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimationAttributeTargetAttributes::AttributeType, value.into());
	}

	/// Get the `attributeType` attribute.
	fn attribute_type(&self) -> Option<&str> {
		self.get_attr(AnimationAttributeTargetAttributes::AttributeType)
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum AnimationEventAttributes {
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

pub trait AnimationEventAttributesSetter {
	fn set_attr(&mut self, attr: AnimationEventAttributes, value: String);
	fn get_attr(&self, attr: AnimationEventAttributes) -> Option<&str>;
}

pub trait TagWithAnimationEventAttributes: AnimationEventAttributesSetter + Sized {

	/// Set the `onbegin` attribute.
	fn with_onbegin<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimationEventAttributes::Onbegin, value.into());
		self
	}

	/// Set the `onbegin` attribute.
	fn set_onbegin<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimationEventAttributes::Onbegin, value.into());
	}

	/// Get the `onbegin` attribute.
	fn onbegin(&self) -> Option<&str> {
		self.get_attr(AnimationEventAttributes::Onbegin)
	}

	/// Set the `onend` attribute.
	fn with_onend<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimationEventAttributes::Onend, value.into());
		self
	}

	/// Set the `onend` attribute.
	fn set_onend<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimationEventAttributes::Onend, value.into());
	}

	/// Get the `onend` attribute.
	fn onend(&self) -> Option<&str> {
		self.get_attr(AnimationEventAttributes::Onend)
	}

	/// Set the `onrepeat` attribute.
	fn with_onrepeat<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimationEventAttributes::Onrepeat, value.into());
		self
	}

	/// Set the `onrepeat` attribute.
	fn set_onrepeat<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimationEventAttributes::Onrepeat, value.into());
	}

	/// Get the `onrepeat` attribute.
	fn onrepeat(&self) -> Option<&str> {
		self.get_attr(AnimationEventAttributes::Onrepeat)
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum AnimationTimingAttributes {
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

pub trait AnimationTimingAttributesSetter {
	fn set_attr(&mut self, attr: AnimationTimingAttributes, value: String);
	fn get_attr(&self, attr: AnimationTimingAttributes) -> Option<&str>;
}

pub trait TagWithAnimationTimingAttributes: AnimationTimingAttributesSetter + Sized {

	/// Set the `begin` attribute.
	fn with_begin<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimationTimingAttributes::Begin, value.into());
		self
	}

	/// Set the `begin` attribute.
	fn set_begin<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimationTimingAttributes::Begin, value.into());
	}

	/// Get the `begin` attribute.
	fn begin(&self) -> Option<&str> {
		self.get_attr(AnimationTimingAttributes::Begin)
	}

	/// Set the `dur` attribute.
	fn with_dur<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimationTimingAttributes::Dur, value.into());
		self
	}

	/// Set the `dur` attribute.
	fn set_dur<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimationTimingAttributes::Dur, value.into());
	}

	/// Get the `dur` attribute.
	fn dur(&self) -> Option<&str> {
		self.get_attr(AnimationTimingAttributes::Dur)
	}

	/// Set the `end` attribute.
	fn with_end<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimationTimingAttributes::End, value.into());
		self
	}

	/// Set the `end` attribute.
	fn set_end<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimationTimingAttributes::End, value.into());
	}

	/// Get the `end` attribute.
	fn end(&self) -> Option<&str> {
		self.get_attr(AnimationTimingAttributes::End)
	}

	/// Set the `fill` attribute.
	fn with_fill<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimationTimingAttributes::Fill, value.into());
		self
	}

	/// Set the `fill` attribute.
	fn set_fill<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimationTimingAttributes::Fill, value.into());
	}

	/// Get the `fill` attribute.
	fn fill(&self) -> Option<&str> {
		self.get_attr(AnimationTimingAttributes::Fill)
	}

	/// Set the `max` attribute.
	fn with_max<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimationTimingAttributes::Max, value.into());
		self
	}

	/// Set the `max` attribute.
	fn set_max<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimationTimingAttributes::Max, value.into());
	}

	/// Get the `max` attribute.
	fn max(&self) -> Option<&str> {
		self.get_attr(AnimationTimingAttributes::Max)
	}

	/// Set the `min` attribute.
	fn with_min<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimationTimingAttributes::Min, value.into());
		self
	}

	/// Set the `min` attribute.
	fn set_min<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimationTimingAttributes::Min, value.into());
	}

	/// Get the `min` attribute.
	fn min(&self) -> Option<&str> {
		self.get_attr(AnimationTimingAttributes::Min)
	}

	/// Set the `repeatCount` attribute.
	fn with_repeat_count<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimationTimingAttributes::RepeatCount, value.into());
		self
	}

	/// Set the `repeatCount` attribute.
	fn set_repeat_count<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimationTimingAttributes::RepeatCount, value.into());
	}

	/// Get the `repeatCount` attribute.
	fn repeat_count(&self) -> Option<&str> {
		self.get_attr(AnimationTimingAttributes::RepeatCount)
	}

	/// Set the `repeatDur` attribute.
	fn with_repeat_dur<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimationTimingAttributes::RepeatDur, value.into());
		self
	}

	/// Set the `repeatDur` attribute.
	fn set_repeat_dur<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimationTimingAttributes::RepeatDur, value.into());
	}

	/// Get the `repeatDur` attribute.
	fn repeat_dur(&self) -> Option<&str> {
		self.get_attr(AnimationTimingAttributes::RepeatDur)
	}

	/// Set the `restart` attribute.
	fn with_restart<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimationTimingAttributes::Restart, value.into());
		self
	}

	/// Set the `restart` attribute.
	fn set_restart<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimationTimingAttributes::Restart, value.into());
	}

	/// Get the `restart` attribute.
	fn restart(&self) -> Option<&str> {
		self.get_attr(AnimationTimingAttributes::Restart)
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum AnimationValueAttributes {
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

pub trait AnimationValueAttributesSetter {
	fn set_attr(&mut self, attr: AnimationValueAttributes, value: String);
	fn get_attr(&self, attr: AnimationValueAttributes) -> Option<&str>;
}

pub trait TagWithAnimationValueAttributes: AnimationValueAttributesSetter + Sized {

	/// Set the `accelerate` attribute.
	fn with_accelerate<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimationValueAttributes::Accelerate, value.into());
		self
	}

	/// Set the `accelerate` attribute.
	fn set_accelerate<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimationValueAttributes::Accelerate, value.into());
	}

	/// Get the `accelerate` attribute.
	fn accelerate(&self) -> Option<&str> {
		self.get_attr(AnimationValueAttributes::Accelerate)
	}

	/// Set the `autoReverse` attribute.
	fn with_auto_reverse<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimationValueAttributes::AutoReverse, value.into());
		self
	}

	/// Set the `autoReverse` attribute.
	fn set_auto_reverse<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimationValueAttributes::AutoReverse, value.into());
	}

	/// Get the `autoReverse` attribute.
	fn auto_reverse(&self) -> Option<&str> {
		self.get_attr(AnimationValueAttributes::AutoReverse)
	}

	/// Set the `by` attribute.
	fn with_by<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimationValueAttributes::By, value.into());
		self
	}

	/// Set the `by` attribute.
	fn set_by<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimationValueAttributes::By, value.into());
	}

	/// Get the `by` attribute.
	fn by(&self) -> Option<&str> {
		self.get_attr(AnimationValueAttributes::By)
	}

	/// Set the `calcMode` attribute.
	fn with_calc_mode<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimationValueAttributes::CalcMode, value.into());
		self
	}

	/// Set the `calcMode` attribute.
	fn set_calc_mode<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimationValueAttributes::CalcMode, value.into());
	}

	/// Get the `calcMode` attribute.
	fn calc_mode(&self) -> Option<&str> {
		self.get_attr(AnimationValueAttributes::CalcMode)
	}

	/// Set the `decelerate` attribute.
	fn with_decelerate<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimationValueAttributes::Decelerate, value.into());
		self
	}

	/// Set the `decelerate` attribute.
	fn set_decelerate<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimationValueAttributes::Decelerate, value.into());
	}

	/// Get the `decelerate` attribute.
	fn decelerate(&self) -> Option<&str> {
		self.get_attr(AnimationValueAttributes::Decelerate)
	}

	/// Set the `from` attribute.
	fn with_from<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimationValueAttributes::From, value.into());
		self
	}

	/// Set the `from` attribute.
	fn set_from<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimationValueAttributes::From, value.into());
	}

	/// Get the `from` attribute.
	fn from(&self) -> Option<&str> {
		self.get_attr(AnimationValueAttributes::From)
	}

	/// Set the `keySplines` attribute.
	fn with_key_splines<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimationValueAttributes::KeySplines, value.into());
		self
	}

	/// Set the `keySplines` attribute.
	fn set_key_splines<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimationValueAttributes::KeySplines, value.into());
	}

	/// Get the `keySplines` attribute.
	fn key_splines(&self) -> Option<&str> {
		self.get_attr(AnimationValueAttributes::KeySplines)
	}

	/// Set the `keyTimes` attribute.
	fn with_key_times<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimationValueAttributes::KeyTimes, value.into());
		self
	}

	/// Set the `keyTimes` attribute.
	fn set_key_times<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimationValueAttributes::KeyTimes, value.into());
	}

	/// Get the `keyTimes` attribute.
	fn key_times(&self) -> Option<&str> {
		self.get_attr(AnimationValueAttributes::KeyTimes)
	}

	/// Set the `to` attribute.
	fn with_to<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimationValueAttributes::To, value.into());
		self
	}

	/// Set the `to` attribute.
	fn set_to<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimationValueAttributes::To, value.into());
	}

	/// Get the `to` attribute.
	fn to(&self) -> Option<&str> {
		self.get_attr(AnimationValueAttributes::To)
	}

	/// Set the `values` attribute.
	fn with_values<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(AnimationValueAttributes::Values, value.into());
		self
	}

	/// Set the `values` attribute.
	fn set_values<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(AnimationValueAttributes::Values, value.into());
	}

	/// Get the `values` attribute.
	fn values(&self) -> Option<&str> {
		self.get_attr(AnimationValueAttributes::Values)
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum ConditionalProcessingAttributes {
	RequiredExtensions,
	RequiredFeatures,
	SystemLanguage,
}

impl ConditionalProcessingAttributes {
	fn as_str(&self) -> &'static str {
		match self {
			Self::RequiredExtensions => "requiredExtensions",
			Self::RequiredFeatures => "requiredFeatures",
			Self::SystemLanguage => "systemLanguage",
		}
	}
}

impl Display for ConditionalProcessingAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

impl Debug for ConditionalProcessingAttributes {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.as_str())
	}
}

pub trait ConditionalProcessingAttributesSetter {
	fn set_attr(&mut self, attr: ConditionalProcessingAttributes, value: String);
	fn get_attr(&self, attr: ConditionalProcessingAttributes) -> Option<&str>;
}

pub trait TagWithConditionalProcessingAttributes: ConditionalProcessingAttributesSetter + Sized {

	/// Set the `requiredExtensions` attribute.
	fn with_required_extensions<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ConditionalProcessingAttributes::RequiredExtensions, value.into());
		self
	}

	/// Set the `requiredExtensions` attribute.
	fn set_required_extensions<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ConditionalProcessingAttributes::RequiredExtensions, value.into());
	}

	/// Get the `requiredExtensions` attribute.
	fn required_extensions(&self) -> Option<&str> {
		self.get_attr(ConditionalProcessingAttributes::RequiredExtensions)
	}

	/// Set the `requiredFeatures` attribute.
	fn with_required_features<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ConditionalProcessingAttributes::RequiredFeatures, value.into());
		self
	}

	/// Set the `requiredFeatures` attribute.
	fn set_required_features<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ConditionalProcessingAttributes::RequiredFeatures, value.into());
	}

	/// Get the `requiredFeatures` attribute.
	fn required_features(&self) -> Option<&str> {
		self.get_attr(ConditionalProcessingAttributes::RequiredFeatures)
	}

	/// Set the `systemLanguage` attribute.
	fn with_system_language<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(ConditionalProcessingAttributes::SystemLanguage, value.into());
		self
	}

	/// Set the `systemLanguage` attribute.
	fn set_system_language<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(ConditionalProcessingAttributes::SystemLanguage, value.into());
	}

	/// Get the `systemLanguage` attribute.
	fn system_language(&self) -> Option<&str> {
		self.get_attr(ConditionalProcessingAttributes::SystemLanguage)
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum CoreAttributes {
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

pub trait CoreAttributesSetter {
	fn set_attr(&mut self, attr: CoreAttributes, value: String);
	fn get_attr(&self, attr: CoreAttributes) -> Option<&str>;
}

pub trait TagWithCoreAttributes: CoreAttributesSetter + Sized {

	/// Set the `id` attribute.
	fn with_id<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(CoreAttributes::Id, value.into());
		self
	}

	/// Set the `id` attribute.
	fn set_id<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(CoreAttributes::Id, value.into());
	}

	/// Get the `id` attribute.
	fn id(&self) -> Option<&str> {
		self.get_attr(CoreAttributes::Id)
	}

	/// Set the `lang` attribute.
	fn with_lang<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(CoreAttributes::Lang, value.into());
		self
	}

	/// Set the `lang` attribute.
	fn set_lang<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(CoreAttributes::Lang, value.into());
	}

	/// Get the `lang` attribute.
	fn lang(&self) -> Option<&str> {
		self.get_attr(CoreAttributes::Lang)
	}

	/// Set the `tabindex` attribute.
	fn with_tabindex<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(CoreAttributes::Tabindex, value.into());
		self
	}

	/// Set the `tabindex` attribute.
	fn set_tabindex<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(CoreAttributes::Tabindex, value.into());
	}

	/// Get the `tabindex` attribute.
	fn tabindex(&self) -> Option<&str> {
		self.get_attr(CoreAttributes::Tabindex)
	}

	/// Set the `xml:base` attribute.
	fn with_xml_base<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(CoreAttributes::XmlBase, value.into());
		self
	}

	/// Set the `xml:base` attribute.
	fn set_xml_base<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(CoreAttributes::XmlBase, value.into());
	}

	/// Get the `xml:base` attribute.
	fn xml_base(&self) -> Option<&str> {
		self.get_attr(CoreAttributes::XmlBase)
	}

	/// Set the `xml:lang` attribute.
	fn with_xml_lang<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(CoreAttributes::XmlLang, value.into());
		self
	}

	/// Set the `xml:lang` attribute.
	fn set_xml_lang<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(CoreAttributes::XmlLang, value.into());
	}

	/// Get the `xml:lang` attribute.
	fn xml_lang(&self) -> Option<&str> {
		self.get_attr(CoreAttributes::XmlLang)
	}

	/// Set the `xml:space` attribute.
	fn with_xml_space<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(CoreAttributes::XmlSpace, value.into());
		self
	}

	/// Set the `xml:space` attribute.
	fn set_xml_space<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(CoreAttributes::XmlSpace, value.into());
	}

	/// Get the `xml:space` attribute.
	fn xml_space(&self) -> Option<&str> {
		self.get_attr(CoreAttributes::XmlSpace)
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum DocumentEventAttributes {
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

pub trait DocumentEventAttributesSetter {
	fn set_attr(&mut self, attr: DocumentEventAttributes, value: String);
	fn get_attr(&self, attr: DocumentEventAttributes) -> Option<&str>;
}

pub trait TagWithDocumentEventAttributes: DocumentEventAttributesSetter + Sized {

	/// Set the `onabort` attribute.
	fn with_onabort<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(DocumentEventAttributes::Onabort, value.into());
		self
	}

	/// Set the `onabort` attribute.
	fn set_onabort<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(DocumentEventAttributes::Onabort, value.into());
	}

	/// Get the `onabort` attribute.
	fn onabort(&self) -> Option<&str> {
		self.get_attr(DocumentEventAttributes::Onabort)
	}

	/// Set the `onerror` attribute.
	fn with_onerror<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(DocumentEventAttributes::Onerror, value.into());
		self
	}

	/// Set the `onerror` attribute.
	fn set_onerror<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(DocumentEventAttributes::Onerror, value.into());
	}

	/// Get the `onerror` attribute.
	fn onerror(&self) -> Option<&str> {
		self.get_attr(DocumentEventAttributes::Onerror)
	}

	/// Set the `onresize` attribute.
	fn with_onresize<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(DocumentEventAttributes::Onresize, value.into());
		self
	}

	/// Set the `onresize` attribute.
	fn set_onresize<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(DocumentEventAttributes::Onresize, value.into());
	}

	/// Get the `onresize` attribute.
	fn onresize(&self) -> Option<&str> {
		self.get_attr(DocumentEventAttributes::Onresize)
	}

	/// Set the `onscroll` attribute.
	fn with_onscroll<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(DocumentEventAttributes::Onscroll, value.into());
		self
	}

	/// Set the `onscroll` attribute.
	fn set_onscroll<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(DocumentEventAttributes::Onscroll, value.into());
	}

	/// Get the `onscroll` attribute.
	fn onscroll(&self) -> Option<&str> {
		self.get_attr(DocumentEventAttributes::Onscroll)
	}

	/// Set the `onunload` attribute.
	fn with_onunload<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(DocumentEventAttributes::Onunload, value.into());
		self
	}

	/// Set the `onunload` attribute.
	fn set_onunload<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(DocumentEventAttributes::Onunload, value.into());
	}

	/// Get the `onunload` attribute.
	fn onunload(&self) -> Option<&str> {
		self.get_attr(DocumentEventAttributes::Onunload)
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum FilterAttributes {
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

pub trait FilterAttributesSetter {
	fn set_attr(&mut self, attr: FilterAttributes, value: String);
	fn get_attr(&self, attr: FilterAttributes) -> Option<&str>;
}

pub trait TagWithFilterAttributes: FilterAttributesSetter + Sized {

	/// Set the `height` attribute.
	fn with_height<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FilterAttributes::Height, value.into());
		self
	}

	/// Set the `height` attribute.
	fn set_height<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FilterAttributes::Height, value.into());
	}

	/// Get the `height` attribute.
	fn height(&self) -> Option<&str> {
		self.get_attr(FilterAttributes::Height)
	}

	/// Set the `result` attribute.
	fn with_result<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FilterAttributes::Result, value.into());
		self
	}

	/// Set the `result` attribute.
	fn set_result<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FilterAttributes::Result, value.into());
	}

	/// Get the `result` attribute.
	fn result(&self) -> Option<&str> {
		self.get_attr(FilterAttributes::Result)
	}

	/// Set the `width` attribute.
	fn with_width<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FilterAttributes::Width, value.into());
		self
	}

	/// Set the `width` attribute.
	fn set_width<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FilterAttributes::Width, value.into());
	}

	/// Get the `width` attribute.
	fn width(&self) -> Option<&str> {
		self.get_attr(FilterAttributes::Width)
	}

	/// Set the `x` attribute.
	fn with_x<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FilterAttributes::X, value.into());
		self
	}

	/// Set the `x` attribute.
	fn set_x<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FilterAttributes::X, value.into());
	}

	/// Get the `x` attribute.
	fn x(&self) -> Option<&str> {
		self.get_attr(FilterAttributes::X)
	}

	/// Set the `y` attribute.
	fn with_y<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(FilterAttributes::Y, value.into());
		self
	}

	/// Set the `y` attribute.
	fn set_y<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(FilterAttributes::Y, value.into());
	}

	/// Get the `y` attribute.
	fn y(&self) -> Option<&str> {
		self.get_attr(FilterAttributes::Y)
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum GlobalEventAttributes {
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

pub trait GlobalEventAttributesSetter {
	fn set_attr(&mut self, attr: GlobalEventAttributes, value: String);
	fn get_attr(&self, attr: GlobalEventAttributes) -> Option<&str>;
}

pub trait TagWithGlobalEventAttributes: GlobalEventAttributesSetter + Sized {

	/// Set the `oncancel` attribute.
	fn with_oncancel<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Oncancel, value.into());
		self
	}

	/// Set the `oncancel` attribute.
	fn set_oncancel<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Oncancel, value.into());
	}

	/// Get the `oncancel` attribute.
	fn oncancel(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Oncancel)
	}

	/// Set the `oncanplay` attribute.
	fn with_oncanplay<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Oncanplay, value.into());
		self
	}

	/// Set the `oncanplay` attribute.
	fn set_oncanplay<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Oncanplay, value.into());
	}

	/// Get the `oncanplay` attribute.
	fn oncanplay(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Oncanplay)
	}

	/// Set the `oncanplaythrough` attribute.
	fn with_oncanplaythrough<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Oncanplaythrough, value.into());
		self
	}

	/// Set the `oncanplaythrough` attribute.
	fn set_oncanplaythrough<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Oncanplaythrough, value.into());
	}

	/// Get the `oncanplaythrough` attribute.
	fn oncanplaythrough(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Oncanplaythrough)
	}

	/// Set the `onchange` attribute.
	fn with_onchange<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onchange, value.into());
		self
	}

	/// Set the `onchange` attribute.
	fn set_onchange<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onchange, value.into());
	}

	/// Get the `onchange` attribute.
	fn onchange(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onchange)
	}

	/// Set the `onclick` attribute.
	fn with_onclick<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onclick, value.into());
		self
	}

	/// Set the `onclick` attribute.
	fn set_onclick<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onclick, value.into());
	}

	/// Get the `onclick` attribute.
	fn onclick(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onclick)
	}

	/// Set the `onclose` attribute.
	fn with_onclose<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onclose, value.into());
		self
	}

	/// Set the `onclose` attribute.
	fn set_onclose<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onclose, value.into());
	}

	/// Get the `onclose` attribute.
	fn onclose(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onclose)
	}

	/// Set the `oncuechange` attribute.
	fn with_oncuechange<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Oncuechange, value.into());
		self
	}

	/// Set the `oncuechange` attribute.
	fn set_oncuechange<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Oncuechange, value.into());
	}

	/// Get the `oncuechange` attribute.
	fn oncuechange(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Oncuechange)
	}

	/// Set the `ondblclick` attribute.
	fn with_ondblclick<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Ondblclick, value.into());
		self
	}

	/// Set the `ondblclick` attribute.
	fn set_ondblclick<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Ondblclick, value.into());
	}

	/// Get the `ondblclick` attribute.
	fn ondblclick(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Ondblclick)
	}

	/// Set the `ondrag` attribute.
	fn with_ondrag<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Ondrag, value.into());
		self
	}

	/// Set the `ondrag` attribute.
	fn set_ondrag<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Ondrag, value.into());
	}

	/// Get the `ondrag` attribute.
	fn ondrag(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Ondrag)
	}

	/// Set the `ondragend` attribute.
	fn with_ondragend<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Ondragend, value.into());
		self
	}

	/// Set the `ondragend` attribute.
	fn set_ondragend<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Ondragend, value.into());
	}

	/// Get the `ondragend` attribute.
	fn ondragend(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Ondragend)
	}

	/// Set the `ondragenter` attribute.
	fn with_ondragenter<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Ondragenter, value.into());
		self
	}

	/// Set the `ondragenter` attribute.
	fn set_ondragenter<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Ondragenter, value.into());
	}

	/// Get the `ondragenter` attribute.
	fn ondragenter(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Ondragenter)
	}

	/// Set the `ondragleave` attribute.
	fn with_ondragleave<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Ondragleave, value.into());
		self
	}

	/// Set the `ondragleave` attribute.
	fn set_ondragleave<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Ondragleave, value.into());
	}

	/// Get the `ondragleave` attribute.
	fn ondragleave(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Ondragleave)
	}

	/// Set the `ondragover` attribute.
	fn with_ondragover<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Ondragover, value.into());
		self
	}

	/// Set the `ondragover` attribute.
	fn set_ondragover<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Ondragover, value.into());
	}

	/// Get the `ondragover` attribute.
	fn ondragover(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Ondragover)
	}

	/// Set the `ondragstart` attribute.
	fn with_ondragstart<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Ondragstart, value.into());
		self
	}

	/// Set the `ondragstart` attribute.
	fn set_ondragstart<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Ondragstart, value.into());
	}

	/// Get the `ondragstart` attribute.
	fn ondragstart(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Ondragstart)
	}

	/// Set the `ondrop` attribute.
	fn with_ondrop<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Ondrop, value.into());
		self
	}

	/// Set the `ondrop` attribute.
	fn set_ondrop<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Ondrop, value.into());
	}

	/// Get the `ondrop` attribute.
	fn ondrop(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Ondrop)
	}

	/// Set the `ondurationchange` attribute.
	fn with_ondurationchange<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Ondurationchange, value.into());
		self
	}

	/// Set the `ondurationchange` attribute.
	fn set_ondurationchange<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Ondurationchange, value.into());
	}

	/// Get the `ondurationchange` attribute.
	fn ondurationchange(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Ondurationchange)
	}

	/// Set the `onemptied` attribute.
	fn with_onemptied<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onemptied, value.into());
		self
	}

	/// Set the `onemptied` attribute.
	fn set_onemptied<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onemptied, value.into());
	}

	/// Get the `onemptied` attribute.
	fn onemptied(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onemptied)
	}

	/// Set the `onended` attribute.
	fn with_onended<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onended, value.into());
		self
	}

	/// Set the `onended` attribute.
	fn set_onended<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onended, value.into());
	}

	/// Get the `onended` attribute.
	fn onended(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onended)
	}

	/// Set the `onerror` attribute.
	fn with_onerror<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onerror, value.into());
		self
	}

	/// Set the `onerror` attribute.
	fn set_onerror<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onerror, value.into());
	}

	/// Get the `onerror` attribute.
	fn onerror(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onerror)
	}

	/// Set the `onfocus` attribute.
	fn with_onfocus<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onfocus, value.into());
		self
	}

	/// Set the `onfocus` attribute.
	fn set_onfocus<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onfocus, value.into());
	}

	/// Get the `onfocus` attribute.
	fn onfocus(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onfocus)
	}

	/// Set the `oninput` attribute.
	fn with_oninput<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Oninput, value.into());
		self
	}

	/// Set the `oninput` attribute.
	fn set_oninput<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Oninput, value.into());
	}

	/// Get the `oninput` attribute.
	fn oninput(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Oninput)
	}

	/// Set the `oninvalid` attribute.
	fn with_oninvalid<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Oninvalid, value.into());
		self
	}

	/// Set the `oninvalid` attribute.
	fn set_oninvalid<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Oninvalid, value.into());
	}

	/// Get the `oninvalid` attribute.
	fn oninvalid(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Oninvalid)
	}

	/// Set the `onkeydown` attribute.
	fn with_onkeydown<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onkeydown, value.into());
		self
	}

	/// Set the `onkeydown` attribute.
	fn set_onkeydown<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onkeydown, value.into());
	}

	/// Get the `onkeydown` attribute.
	fn onkeydown(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onkeydown)
	}

	/// Set the `onkeypress` attribute.
	fn with_onkeypress<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onkeypress, value.into());
		self
	}

	/// Set the `onkeypress` attribute.
	fn set_onkeypress<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onkeypress, value.into());
	}

	/// Get the `onkeypress` attribute.
	fn onkeypress(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onkeypress)
	}

	/// Set the `onkeyup` attribute.
	fn with_onkeyup<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onkeyup, value.into());
		self
	}

	/// Set the `onkeyup` attribute.
	fn set_onkeyup<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onkeyup, value.into());
	}

	/// Get the `onkeyup` attribute.
	fn onkeyup(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onkeyup)
	}

	/// Set the `onload` attribute.
	fn with_onload<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onload, value.into());
		self
	}

	/// Set the `onload` attribute.
	fn set_onload<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onload, value.into());
	}

	/// Get the `onload` attribute.
	fn onload(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onload)
	}

	/// Set the `onloadeddata` attribute.
	fn with_onloadeddata<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onloadeddata, value.into());
		self
	}

	/// Set the `onloadeddata` attribute.
	fn set_onloadeddata<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onloadeddata, value.into());
	}

	/// Get the `onloadeddata` attribute.
	fn onloadeddata(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onloadeddata)
	}

	/// Set the `onloadedmetadata` attribute.
	fn with_onloadedmetadata<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onloadedmetadata, value.into());
		self
	}

	/// Set the `onloadedmetadata` attribute.
	fn set_onloadedmetadata<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onloadedmetadata, value.into());
	}

	/// Get the `onloadedmetadata` attribute.
	fn onloadedmetadata(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onloadedmetadata)
	}

	/// Set the `onloadstart` attribute.
	fn with_onloadstart<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onloadstart, value.into());
		self
	}

	/// Set the `onloadstart` attribute.
	fn set_onloadstart<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onloadstart, value.into());
	}

	/// Get the `onloadstart` attribute.
	fn onloadstart(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onloadstart)
	}

	/// Set the `onmousedown` attribute.
	fn with_onmousedown<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onmousedown, value.into());
		self
	}

	/// Set the `onmousedown` attribute.
	fn set_onmousedown<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onmousedown, value.into());
	}

	/// Get the `onmousedown` attribute.
	fn onmousedown(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onmousedown)
	}

	/// Set the `onmouseenter` attribute.
	fn with_onmouseenter<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onmouseenter, value.into());
		self
	}

	/// Set the `onmouseenter` attribute.
	fn set_onmouseenter<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onmouseenter, value.into());
	}

	/// Get the `onmouseenter` attribute.
	fn onmouseenter(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onmouseenter)
	}

	/// Set the `onmouseleave` attribute.
	fn with_onmouseleave<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onmouseleave, value.into());
		self
	}

	/// Set the `onmouseleave` attribute.
	fn set_onmouseleave<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onmouseleave, value.into());
	}

	/// Get the `onmouseleave` attribute.
	fn onmouseleave(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onmouseleave)
	}

	/// Set the `onmousemove` attribute.
	fn with_onmousemove<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onmousemove, value.into());
		self
	}

	/// Set the `onmousemove` attribute.
	fn set_onmousemove<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onmousemove, value.into());
	}

	/// Get the `onmousemove` attribute.
	fn onmousemove(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onmousemove)
	}

	/// Set the `onmouseout` attribute.
	fn with_onmouseout<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onmouseout, value.into());
		self
	}

	/// Set the `onmouseout` attribute.
	fn set_onmouseout<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onmouseout, value.into());
	}

	/// Get the `onmouseout` attribute.
	fn onmouseout(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onmouseout)
	}

	/// Set the `onmouseover` attribute.
	fn with_onmouseover<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onmouseover, value.into());
		self
	}

	/// Set the `onmouseover` attribute.
	fn set_onmouseover<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onmouseover, value.into());
	}

	/// Get the `onmouseover` attribute.
	fn onmouseover(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onmouseover)
	}

	/// Set the `onmouseup` attribute.
	fn with_onmouseup<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onmouseup, value.into());
		self
	}

	/// Set the `onmouseup` attribute.
	fn set_onmouseup<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onmouseup, value.into());
	}

	/// Get the `onmouseup` attribute.
	fn onmouseup(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onmouseup)
	}

	/// Set the `onmousewheel` attribute.
	fn with_onmousewheel<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onmousewheel, value.into());
		self
	}

	/// Set the `onmousewheel` attribute.
	fn set_onmousewheel<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onmousewheel, value.into());
	}

	/// Get the `onmousewheel` attribute.
	fn onmousewheel(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onmousewheel)
	}

	/// Set the `onpause` attribute.
	fn with_onpause<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onpause, value.into());
		self
	}

	/// Set the `onpause` attribute.
	fn set_onpause<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onpause, value.into());
	}

	/// Get the `onpause` attribute.
	fn onpause(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onpause)
	}

	/// Set the `onplay` attribute.
	fn with_onplay<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onplay, value.into());
		self
	}

	/// Set the `onplay` attribute.
	fn set_onplay<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onplay, value.into());
	}

	/// Get the `onplay` attribute.
	fn onplay(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onplay)
	}

	/// Set the `onplaying` attribute.
	fn with_onplaying<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onplaying, value.into());
		self
	}

	/// Set the `onplaying` attribute.
	fn set_onplaying<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onplaying, value.into());
	}

	/// Get the `onplaying` attribute.
	fn onplaying(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onplaying)
	}

	/// Set the `onprogress` attribute.
	fn with_onprogress<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onprogress, value.into());
		self
	}

	/// Set the `onprogress` attribute.
	fn set_onprogress<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onprogress, value.into());
	}

	/// Get the `onprogress` attribute.
	fn onprogress(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onprogress)
	}

	/// Set the `onratechange` attribute.
	fn with_onratechange<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onratechange, value.into());
		self
	}

	/// Set the `onratechange` attribute.
	fn set_onratechange<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onratechange, value.into());
	}

	/// Get the `onratechange` attribute.
	fn onratechange(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onratechange)
	}

	/// Set the `onreset` attribute.
	fn with_onreset<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onreset, value.into());
		self
	}

	/// Set the `onreset` attribute.
	fn set_onreset<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onreset, value.into());
	}

	/// Get the `onreset` attribute.
	fn onreset(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onreset)
	}

	/// Set the `onresize` attribute.
	fn with_onresize<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onresize, value.into());
		self
	}

	/// Set the `onresize` attribute.
	fn set_onresize<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onresize, value.into());
	}

	/// Get the `onresize` attribute.
	fn onresize(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onresize)
	}

	/// Set the `onscroll` attribute.
	fn with_onscroll<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onscroll, value.into());
		self
	}

	/// Set the `onscroll` attribute.
	fn set_onscroll<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onscroll, value.into());
	}

	/// Get the `onscroll` attribute.
	fn onscroll(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onscroll)
	}

	/// Set the `onseeked` attribute.
	fn with_onseeked<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onseeked, value.into());
		self
	}

	/// Set the `onseeked` attribute.
	fn set_onseeked<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onseeked, value.into());
	}

	/// Get the `onseeked` attribute.
	fn onseeked(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onseeked)
	}

	/// Set the `onseeking` attribute.
	fn with_onseeking<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onseeking, value.into());
		self
	}

	/// Set the `onseeking` attribute.
	fn set_onseeking<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onseeking, value.into());
	}

	/// Get the `onseeking` attribute.
	fn onseeking(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onseeking)
	}

	/// Set the `onselect` attribute.
	fn with_onselect<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onselect, value.into());
		self
	}

	/// Set the `onselect` attribute.
	fn set_onselect<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onselect, value.into());
	}

	/// Get the `onselect` attribute.
	fn onselect(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onselect)
	}

	/// Set the `onshow` attribute.
	fn with_onshow<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onshow, value.into());
		self
	}

	/// Set the `onshow` attribute.
	fn set_onshow<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onshow, value.into());
	}

	/// Get the `onshow` attribute.
	fn onshow(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onshow)
	}

	/// Set the `onstalled` attribute.
	fn with_onstalled<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onstalled, value.into());
		self
	}

	/// Set the `onstalled` attribute.
	fn set_onstalled<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onstalled, value.into());
	}

	/// Get the `onstalled` attribute.
	fn onstalled(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onstalled)
	}

	/// Set the `onsubmit` attribute.
	fn with_onsubmit<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onsubmit, value.into());
		self
	}

	/// Set the `onsubmit` attribute.
	fn set_onsubmit<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onsubmit, value.into());
	}

	/// Get the `onsubmit` attribute.
	fn onsubmit(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onsubmit)
	}

	/// Set the `onsuspend` attribute.
	fn with_onsuspend<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onsuspend, value.into());
		self
	}

	/// Set the `onsuspend` attribute.
	fn set_onsuspend<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onsuspend, value.into());
	}

	/// Get the `onsuspend` attribute.
	fn onsuspend(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onsuspend)
	}

	/// Set the `ontimeupdate` attribute.
	fn with_ontimeupdate<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Ontimeupdate, value.into());
		self
	}

	/// Set the `ontimeupdate` attribute.
	fn set_ontimeupdate<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Ontimeupdate, value.into());
	}

	/// Get the `ontimeupdate` attribute.
	fn ontimeupdate(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Ontimeupdate)
	}

	/// Set the `ontoggle` attribute.
	fn with_ontoggle<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Ontoggle, value.into());
		self
	}

	/// Set the `ontoggle` attribute.
	fn set_ontoggle<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Ontoggle, value.into());
	}

	/// Get the `ontoggle` attribute.
	fn ontoggle(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Ontoggle)
	}

	/// Set the `onvolumechange` attribute.
	fn with_onvolumechange<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onvolumechange, value.into());
		self
	}

	/// Set the `onvolumechange` attribute.
	fn set_onvolumechange<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onvolumechange, value.into());
	}

	/// Get the `onvolumechange` attribute.
	fn onvolumechange(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onvolumechange)
	}

	/// Set the `onwaiting` attribute.
	fn with_onwaiting<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onwaiting, value.into());
		self
	}

	/// Set the `onwaiting` attribute.
	fn set_onwaiting<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GlobalEventAttributes::Onwaiting, value.into());
	}

	/// Get the `onwaiting` attribute.
	fn onwaiting(&self) -> Option<&str> {
		self.get_attr(GlobalEventAttributes::Onwaiting)
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum GraphicalEventAttributes {
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

pub trait GraphicalEventAttributesSetter {
	fn set_attr(&mut self, attr: GraphicalEventAttributes, value: String);
	fn get_attr(&self, attr: GraphicalEventAttributes) -> Option<&str>;
}

pub trait TagWithGraphicalEventAttributes: GraphicalEventAttributesSetter + Sized {

	/// Set the `onactivate` attribute.
	fn with_onactivate<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GraphicalEventAttributes::Onactivate, value.into());
		self
	}

	/// Set the `onactivate` attribute.
	fn set_onactivate<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GraphicalEventAttributes::Onactivate, value.into());
	}

	/// Get the `onactivate` attribute.
	fn onactivate(&self) -> Option<&str> {
		self.get_attr(GraphicalEventAttributes::Onactivate)
	}

	/// Set the `onfocusin` attribute.
	fn with_onfocusin<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GraphicalEventAttributes::Onfocusin, value.into());
		self
	}

	/// Set the `onfocusin` attribute.
	fn set_onfocusin<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GraphicalEventAttributes::Onfocusin, value.into());
	}

	/// Get the `onfocusin` attribute.
	fn onfocusin(&self) -> Option<&str> {
		self.get_attr(GraphicalEventAttributes::Onfocusin)
	}

	/// Set the `onfocusout` attribute.
	fn with_onfocusout<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(GraphicalEventAttributes::Onfocusout, value.into());
		self
	}

	/// Set the `onfocusout` attribute.
	fn set_onfocusout<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(GraphicalEventAttributes::Onfocusout, value.into());
	}

	/// Get the `onfocusout` attribute.
	fn onfocusout(&self) -> Option<&str> {
		self.get_attr(GraphicalEventAttributes::Onfocusout)
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum PresentationAttributes {
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

pub trait PresentationAttributesSetter {
	fn set_attr(&mut self, attr: PresentationAttributes, value: String);
	fn get_attr(&self, attr: PresentationAttributes) -> Option<&str>;
}

pub trait TagWithPresentationAttributes: PresentationAttributesSetter + Sized {

	/// Set the `alignment-baseline` attribute.
	fn with_alignment_minus_baseline<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::AlignmentMinusBaseline, value.into());
		self
	}

	/// Set the `alignment-baseline` attribute.
	fn set_alignment_minus_baseline<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::AlignmentMinusBaseline, value.into());
	}

	/// Get the `alignment-baseline` attribute.
	fn alignment_minus_baseline(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::AlignmentMinusBaseline)
	}

	/// Set the `baseline-shift` attribute.
	fn with_baseline_minus_shift<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::BaselineMinusShift, value.into());
		self
	}

	/// Set the `baseline-shift` attribute.
	fn set_baseline_minus_shift<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::BaselineMinusShift, value.into());
	}

	/// Get the `baseline-shift` attribute.
	fn baseline_minus_shift(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::BaselineMinusShift)
	}

	/// Set the `clip` attribute.
	fn with_clip<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::Clip, value.into());
		self
	}

	/// Set the `clip` attribute.
	fn set_clip<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::Clip, value.into());
	}

	/// Get the `clip` attribute.
	fn clip(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::Clip)
	}

	/// Set the `clip-path` attribute.
	fn with_clip_minus_path<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::ClipMinusPath, value.into());
		self
	}

	/// Set the `clip-path` attribute.
	fn set_clip_minus_path<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::ClipMinusPath, value.into());
	}

	/// Get the `clip-path` attribute.
	fn clip_minus_path(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::ClipMinusPath)
	}

	/// Set the `clip-rule` attribute.
	fn with_clip_minus_rule<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::ClipMinusRule, value.into());
		self
	}

	/// Set the `clip-rule` attribute.
	fn set_clip_minus_rule<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::ClipMinusRule, value.into());
	}

	/// Get the `clip-rule` attribute.
	fn clip_minus_rule(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::ClipMinusRule)
	}

	/// Set the `color` attribute.
	fn with_color<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::Color, value.into());
		self
	}

	/// Set the `color` attribute.
	fn set_color<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::Color, value.into());
	}

	/// Get the `color` attribute.
	fn color(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::Color)
	}

	/// Set the `color-interpolation` attribute.
	fn with_color_minus_interpolation<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::ColorMinusInterpolation, value.into());
		self
	}

	/// Set the `color-interpolation` attribute.
	fn set_color_minus_interpolation<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::ColorMinusInterpolation, value.into());
	}

	/// Get the `color-interpolation` attribute.
	fn color_minus_interpolation(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::ColorMinusInterpolation)
	}

	/// Set the `color-interpolation-filters` attribute.
	fn with_color_minus_interpolation_minus_filters<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::ColorMinusInterpolationMinusFilters, value.into());
		self
	}

	/// Set the `color-interpolation-filters` attribute.
	fn set_color_minus_interpolation_minus_filters<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::ColorMinusInterpolationMinusFilters, value.into());
	}

	/// Get the `color-interpolation-filters` attribute.
	fn color_minus_interpolation_minus_filters(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::ColorMinusInterpolationMinusFilters)
	}

	/// Set the `color-profile` attribute.
	fn with_color_minus_profile<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::ColorMinusProfile, value.into());
		self
	}

	/// Set the `color-profile` attribute.
	fn set_color_minus_profile<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::ColorMinusProfile, value.into());
	}

	/// Get the `color-profile` attribute.
	fn color_minus_profile(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::ColorMinusProfile)
	}

	/// Set the `color-rendering` attribute.
	fn with_color_minus_rendering<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::ColorMinusRendering, value.into());
		self
	}

	/// Set the `color-rendering` attribute.
	fn set_color_minus_rendering<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::ColorMinusRendering, value.into());
	}

	/// Get the `color-rendering` attribute.
	fn color_minus_rendering(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::ColorMinusRendering)
	}

	/// Set the `cursor` attribute.
	fn with_cursor<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::Cursor, value.into());
		self
	}

	/// Set the `cursor` attribute.
	fn set_cursor<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::Cursor, value.into());
	}

	/// Get the `cursor` attribute.
	fn cursor(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::Cursor)
	}

	/// Set the `direction` attribute.
	fn with_direction<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::Direction, value.into());
		self
	}

	/// Set the `direction` attribute.
	fn set_direction<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::Direction, value.into());
	}

	/// Get the `direction` attribute.
	fn direction(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::Direction)
	}

	/// Set the `display` attribute.
	fn with_display<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::Display, value.into());
		self
	}

	/// Set the `display` attribute.
	fn set_display<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::Display, value.into());
	}

	/// Get the `display` attribute.
	fn display(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::Display)
	}

	/// Set the `dominant-baseline` attribute.
	fn with_dominant_minus_baseline<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::DominantMinusBaseline, value.into());
		self
	}

	/// Set the `dominant-baseline` attribute.
	fn set_dominant_minus_baseline<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::DominantMinusBaseline, value.into());
	}

	/// Get the `dominant-baseline` attribute.
	fn dominant_minus_baseline(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::DominantMinusBaseline)
	}

	/// Set the `enable-background` attribute.
	fn with_enable_minus_background<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::EnableMinusBackground, value.into());
		self
	}

	/// Set the `enable-background` attribute.
	fn set_enable_minus_background<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::EnableMinusBackground, value.into());
	}

	/// Get the `enable-background` attribute.
	fn enable_minus_background(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::EnableMinusBackground)
	}

	/// Set the `fill` attribute.
	fn with_fill<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::Fill, value.into());
		self
	}

	/// Set the `fill` attribute.
	fn set_fill<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::Fill, value.into());
	}

	/// Get the `fill` attribute.
	fn fill(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::Fill)
	}

	/// Set the `fill-opacity` attribute.
	fn with_fill_minus_opacity<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::FillMinusOpacity, value.into());
		self
	}

	/// Set the `fill-opacity` attribute.
	fn set_fill_minus_opacity<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::FillMinusOpacity, value.into());
	}

	/// Get the `fill-opacity` attribute.
	fn fill_minus_opacity(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::FillMinusOpacity)
	}

	/// Set the `fill-rule` attribute.
	fn with_fill_minus_rule<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::FillMinusRule, value.into());
		self
	}

	/// Set the `fill-rule` attribute.
	fn set_fill_minus_rule<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::FillMinusRule, value.into());
	}

	/// Get the `fill-rule` attribute.
	fn fill_minus_rule(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::FillMinusRule)
	}

	/// Set the `filter` attribute.
	fn with_filter<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::Filter, value.into());
		self
	}

	/// Set the `filter` attribute.
	fn set_filter<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::Filter, value.into());
	}

	/// Get the `filter` attribute.
	fn filter(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::Filter)
	}

	/// Set the `flood-color` attribute.
	fn with_flood_minus_color<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::FloodMinusColor, value.into());
		self
	}

	/// Set the `flood-color` attribute.
	fn set_flood_minus_color<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::FloodMinusColor, value.into());
	}

	/// Get the `flood-color` attribute.
	fn flood_minus_color(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::FloodMinusColor)
	}

	/// Set the `flood-opacity` attribute.
	fn with_flood_minus_opacity<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::FloodMinusOpacity, value.into());
		self
	}

	/// Set the `flood-opacity` attribute.
	fn set_flood_minus_opacity<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::FloodMinusOpacity, value.into());
	}

	/// Get the `flood-opacity` attribute.
	fn flood_minus_opacity(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::FloodMinusOpacity)
	}

	/// Set the `font-family` attribute.
	fn with_font_minus_family<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::FontMinusFamily, value.into());
		self
	}

	/// Set the `font-family` attribute.
	fn set_font_minus_family<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::FontMinusFamily, value.into());
	}

	/// Get the `font-family` attribute.
	fn font_minus_family(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::FontMinusFamily)
	}

	/// Set the `font-size` attribute.
	fn with_font_minus_size<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::FontMinusSize, value.into());
		self
	}

	/// Set the `font-size` attribute.
	fn set_font_minus_size<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::FontMinusSize, value.into());
	}

	/// Get the `font-size` attribute.
	fn font_minus_size(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::FontMinusSize)
	}

	/// Set the `font-size-adjust` attribute.
	fn with_font_minus_size_minus_adjust<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::FontMinusSizeMinusAdjust, value.into());
		self
	}

	/// Set the `font-size-adjust` attribute.
	fn set_font_minus_size_minus_adjust<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::FontMinusSizeMinusAdjust, value.into());
	}

	/// Get the `font-size-adjust` attribute.
	fn font_minus_size_minus_adjust(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::FontMinusSizeMinusAdjust)
	}

	/// Set the `font-stretch` attribute.
	fn with_font_minus_stretch<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::FontMinusStretch, value.into());
		self
	}

	/// Set the `font-stretch` attribute.
	fn set_font_minus_stretch<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::FontMinusStretch, value.into());
	}

	/// Get the `font-stretch` attribute.
	fn font_minus_stretch(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::FontMinusStretch)
	}

	/// Set the `font-style` attribute.
	fn with_font_minus_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::FontMinusStyle, value.into());
		self
	}

	/// Set the `font-style` attribute.
	fn set_font_minus_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::FontMinusStyle, value.into());
	}

	/// Get the `font-style` attribute.
	fn font_minus_style(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::FontMinusStyle)
	}

	/// Set the `font-variant` attribute.
	fn with_font_minus_variant<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::FontMinusVariant, value.into());
		self
	}

	/// Set the `font-variant` attribute.
	fn set_font_minus_variant<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::FontMinusVariant, value.into());
	}

	/// Get the `font-variant` attribute.
	fn font_minus_variant(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::FontMinusVariant)
	}

	/// Set the `font-weight` attribute.
	fn with_font_minus_weight<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::FontMinusWeight, value.into());
		self
	}

	/// Set the `font-weight` attribute.
	fn set_font_minus_weight<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::FontMinusWeight, value.into());
	}

	/// Get the `font-weight` attribute.
	fn font_minus_weight(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::FontMinusWeight)
	}

	/// Set the `glyph-orientation-horizontal` attribute.
	fn with_glyph_minus_orientation_minus_horizontal<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::GlyphMinusOrientationMinusHorizontal, value.into());
		self
	}

	/// Set the `glyph-orientation-horizontal` attribute.
	fn set_glyph_minus_orientation_minus_horizontal<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::GlyphMinusOrientationMinusHorizontal, value.into());
	}

	/// Get the `glyph-orientation-horizontal` attribute.
	fn glyph_minus_orientation_minus_horizontal(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::GlyphMinusOrientationMinusHorizontal)
	}

	/// Set the `glyph-orientation-vertical` attribute.
	fn with_glyph_minus_orientation_minus_vertical<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::GlyphMinusOrientationMinusVertical, value.into());
		self
	}

	/// Set the `glyph-orientation-vertical` attribute.
	fn set_glyph_minus_orientation_minus_vertical<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::GlyphMinusOrientationMinusVertical, value.into());
	}

	/// Get the `glyph-orientation-vertical` attribute.
	fn glyph_minus_orientation_minus_vertical(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::GlyphMinusOrientationMinusVertical)
	}

	/// Set the `image-rendering` attribute.
	fn with_image_minus_rendering<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::ImageMinusRendering, value.into());
		self
	}

	/// Set the `image-rendering` attribute.
	fn set_image_minus_rendering<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::ImageMinusRendering, value.into());
	}

	/// Get the `image-rendering` attribute.
	fn image_minus_rendering(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::ImageMinusRendering)
	}

	/// Set the `kerning` attribute.
	fn with_kerning<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::Kerning, value.into());
		self
	}

	/// Set the `kerning` attribute.
	fn set_kerning<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::Kerning, value.into());
	}

	/// Get the `kerning` attribute.
	fn kerning(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::Kerning)
	}

	/// Set the `letter-spacing` attribute.
	fn with_letter_minus_spacing<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::LetterMinusSpacing, value.into());
		self
	}

	/// Set the `letter-spacing` attribute.
	fn set_letter_minus_spacing<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::LetterMinusSpacing, value.into());
	}

	/// Get the `letter-spacing` attribute.
	fn letter_minus_spacing(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::LetterMinusSpacing)
	}

	/// Set the `lighting-color` attribute.
	fn with_lighting_minus_color<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::LightingMinusColor, value.into());
		self
	}

	/// Set the `lighting-color` attribute.
	fn set_lighting_minus_color<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::LightingMinusColor, value.into());
	}

	/// Get the `lighting-color` attribute.
	fn lighting_minus_color(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::LightingMinusColor)
	}

	/// Set the `marker-end` attribute.
	fn with_marker_minus_end<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::MarkerMinusEnd, value.into());
		self
	}

	/// Set the `marker-end` attribute.
	fn set_marker_minus_end<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::MarkerMinusEnd, value.into());
	}

	/// Get the `marker-end` attribute.
	fn marker_minus_end(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::MarkerMinusEnd)
	}

	/// Set the `marker-mid` attribute.
	fn with_marker_minus_mid<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::MarkerMinusMid, value.into());
		self
	}

	/// Set the `marker-mid` attribute.
	fn set_marker_minus_mid<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::MarkerMinusMid, value.into());
	}

	/// Get the `marker-mid` attribute.
	fn marker_minus_mid(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::MarkerMinusMid)
	}

	/// Set the `marker-start` attribute.
	fn with_marker_minus_start<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::MarkerMinusStart, value.into());
		self
	}

	/// Set the `marker-start` attribute.
	fn set_marker_minus_start<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::MarkerMinusStart, value.into());
	}

	/// Get the `marker-start` attribute.
	fn marker_minus_start(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::MarkerMinusStart)
	}

	/// Set the `mask` attribute.
	fn with_mask<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::Mask, value.into());
		self
	}

	/// Set the `mask` attribute.
	fn set_mask<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::Mask, value.into());
	}

	/// Get the `mask` attribute.
	fn mask(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::Mask)
	}

	/// Set the `opacity` attribute.
	fn with_opacity<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::Opacity, value.into());
		self
	}

	/// Set the `opacity` attribute.
	fn set_opacity<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::Opacity, value.into());
	}

	/// Get the `opacity` attribute.
	fn opacity(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::Opacity)
	}

	/// Set the `overflow` attribute.
	fn with_overflow<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::Overflow, value.into());
		self
	}

	/// Set the `overflow` attribute.
	fn set_overflow<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::Overflow, value.into());
	}

	/// Get the `overflow` attribute.
	fn overflow(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::Overflow)
	}

	/// Set the `pointer-events` attribute.
	fn with_pointer_minus_events<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::PointerMinusEvents, value.into());
		self
	}

	/// Set the `pointer-events` attribute.
	fn set_pointer_minus_events<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::PointerMinusEvents, value.into());
	}

	/// Get the `pointer-events` attribute.
	fn pointer_minus_events(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::PointerMinusEvents)
	}

	/// Set the `shape-rendering` attribute.
	fn with_shape_minus_rendering<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::ShapeMinusRendering, value.into());
		self
	}

	/// Set the `shape-rendering` attribute.
	fn set_shape_minus_rendering<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::ShapeMinusRendering, value.into());
	}

	/// Get the `shape-rendering` attribute.
	fn shape_minus_rendering(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::ShapeMinusRendering)
	}

	/// Set the `stop-color` attribute.
	fn with_stop_minus_color<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::StopMinusColor, value.into());
		self
	}

	/// Set the `stop-color` attribute.
	fn set_stop_minus_color<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::StopMinusColor, value.into());
	}

	/// Get the `stop-color` attribute.
	fn stop_minus_color(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::StopMinusColor)
	}

	/// Set the `stop-opacity` attribute.
	fn with_stop_minus_opacity<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::StopMinusOpacity, value.into());
		self
	}

	/// Set the `stop-opacity` attribute.
	fn set_stop_minus_opacity<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::StopMinusOpacity, value.into());
	}

	/// Get the `stop-opacity` attribute.
	fn stop_minus_opacity(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::StopMinusOpacity)
	}

	/// Set the `stroke` attribute.
	fn with_stroke<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::Stroke, value.into());
		self
	}

	/// Set the `stroke` attribute.
	fn set_stroke<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::Stroke, value.into());
	}

	/// Get the `stroke` attribute.
	fn stroke(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::Stroke)
	}

	/// Set the `stroke-dasharray` attribute.
	fn with_stroke_minus_dasharray<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::StrokeMinusDasharray, value.into());
		self
	}

	/// Set the `stroke-dasharray` attribute.
	fn set_stroke_minus_dasharray<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::StrokeMinusDasharray, value.into());
	}

	/// Get the `stroke-dasharray` attribute.
	fn stroke_minus_dasharray(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::StrokeMinusDasharray)
	}

	/// Set the `stroke-dashoffset` attribute.
	fn with_stroke_minus_dashoffset<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::StrokeMinusDashoffset, value.into());
		self
	}

	/// Set the `stroke-dashoffset` attribute.
	fn set_stroke_minus_dashoffset<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::StrokeMinusDashoffset, value.into());
	}

	/// Get the `stroke-dashoffset` attribute.
	fn stroke_minus_dashoffset(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::StrokeMinusDashoffset)
	}

	/// Set the `stroke-linecap` attribute.
	fn with_stroke_minus_linecap<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::StrokeMinusLinecap, value.into());
		self
	}

	/// Set the `stroke-linecap` attribute.
	fn set_stroke_minus_linecap<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::StrokeMinusLinecap, value.into());
	}

	/// Get the `stroke-linecap` attribute.
	fn stroke_minus_linecap(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::StrokeMinusLinecap)
	}

	/// Set the `stroke-linejoin` attribute.
	fn with_stroke_minus_linejoin<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::StrokeMinusLinejoin, value.into());
		self
	}

	/// Set the `stroke-linejoin` attribute.
	fn set_stroke_minus_linejoin<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::StrokeMinusLinejoin, value.into());
	}

	/// Get the `stroke-linejoin` attribute.
	fn stroke_minus_linejoin(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::StrokeMinusLinejoin)
	}

	/// Set the `stroke-miterlimit` attribute.
	fn with_stroke_minus_miterlimit<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::StrokeMinusMiterlimit, value.into());
		self
	}

	/// Set the `stroke-miterlimit` attribute.
	fn set_stroke_minus_miterlimit<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::StrokeMinusMiterlimit, value.into());
	}

	/// Get the `stroke-miterlimit` attribute.
	fn stroke_minus_miterlimit(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::StrokeMinusMiterlimit)
	}

	/// Set the `stroke-opacity` attribute.
	fn with_stroke_minus_opacity<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::StrokeMinusOpacity, value.into());
		self
	}

	/// Set the `stroke-opacity` attribute.
	fn set_stroke_minus_opacity<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::StrokeMinusOpacity, value.into());
	}

	/// Get the `stroke-opacity` attribute.
	fn stroke_minus_opacity(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::StrokeMinusOpacity)
	}

	/// Set the `stroke-width` attribute.
	fn with_stroke_minus_width<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::StrokeMinusWidth, value.into());
		self
	}

	/// Set the `stroke-width` attribute.
	fn set_stroke_minus_width<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::StrokeMinusWidth, value.into());
	}

	/// Get the `stroke-width` attribute.
	fn stroke_minus_width(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::StrokeMinusWidth)
	}

	/// Set the `text-anchor` attribute.
	fn with_text_minus_anchor<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::TextMinusAnchor, value.into());
		self
	}

	/// Set the `text-anchor` attribute.
	fn set_text_minus_anchor<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::TextMinusAnchor, value.into());
	}

	/// Get the `text-anchor` attribute.
	fn text_minus_anchor(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::TextMinusAnchor)
	}

	/// Set the `text-decoration` attribute.
	fn with_text_minus_decoration<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::TextMinusDecoration, value.into());
		self
	}

	/// Set the `text-decoration` attribute.
	fn set_text_minus_decoration<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::TextMinusDecoration, value.into());
	}

	/// Get the `text-decoration` attribute.
	fn text_minus_decoration(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::TextMinusDecoration)
	}

	/// Set the `text-rendering` attribute.
	fn with_text_minus_rendering<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::TextMinusRendering, value.into());
		self
	}

	/// Set the `text-rendering` attribute.
	fn set_text_minus_rendering<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::TextMinusRendering, value.into());
	}

	/// Get the `text-rendering` attribute.
	fn text_minus_rendering(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::TextMinusRendering)
	}

	/// Set the `transform` attribute.
	fn with_transform<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::Transform, value.into());
		self
	}

	/// Set the `transform` attribute.
	fn set_transform<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::Transform, value.into());
	}

	/// Get the `transform` attribute.
	fn transform(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::Transform)
	}

	/// Set the `transform-origin` attribute.
	fn with_transform_minus_origin<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::TransformMinusOrigin, value.into());
		self
	}

	/// Set the `transform-origin` attribute.
	fn set_transform_minus_origin<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::TransformMinusOrigin, value.into());
	}

	/// Get the `transform-origin` attribute.
	fn transform_minus_origin(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::TransformMinusOrigin)
	}

	/// Set the `unicode-bidi` attribute.
	fn with_unicode_minus_bidi<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::UnicodeMinusBidi, value.into());
		self
	}

	/// Set the `unicode-bidi` attribute.
	fn set_unicode_minus_bidi<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::UnicodeMinusBidi, value.into());
	}

	/// Get the `unicode-bidi` attribute.
	fn unicode_minus_bidi(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::UnicodeMinusBidi)
	}

	/// Set the `vector-effect` attribute.
	fn with_vector_minus_effect<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::VectorMinusEffect, value.into());
		self
	}

	/// Set the `vector-effect` attribute.
	fn set_vector_minus_effect<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::VectorMinusEffect, value.into());
	}

	/// Get the `vector-effect` attribute.
	fn vector_minus_effect(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::VectorMinusEffect)
	}

	/// Set the `visibility` attribute.
	fn with_visibility<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::Visibility, value.into());
		self
	}

	/// Set the `visibility` attribute.
	fn set_visibility<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::Visibility, value.into());
	}

	/// Get the `visibility` attribute.
	fn visibility(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::Visibility)
	}

	/// Set the `word-spacing` attribute.
	fn with_word_minus_spacing<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::WordMinusSpacing, value.into());
		self
	}

	/// Set the `word-spacing` attribute.
	fn set_word_minus_spacing<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::WordMinusSpacing, value.into());
	}

	/// Get the `word-spacing` attribute.
	fn word_minus_spacing(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::WordMinusSpacing)
	}

	/// Set the `writing-mode` attribute.
	fn with_writing_minus_mode<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::WritingMinusMode, value.into());
		self
	}

	/// Set the `writing-mode` attribute.
	fn set_writing_minus_mode<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(PresentationAttributes::WritingMinusMode, value.into());
	}

	/// Get the `writing-mode` attribute.
	fn writing_minus_mode(&self) -> Option<&str> {
		self.get_attr(PresentationAttributes::WritingMinusMode)
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum StyleAttributes {
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

pub trait StyleAttributesSetter {
	fn set_attr(&mut self, attr: StyleAttributes, value: String);
	fn get_attr(&self, attr: StyleAttributes) -> Option<&str>;
}

pub trait TagWithStyleAttributes: StyleAttributesSetter + Sized {

	/// Set the `class` attribute.
	fn with_class<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(StyleAttributes::Class, value.into());
		self
	}

	/// Set the `class` attribute.
	fn set_class<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(StyleAttributes::Class, value.into());
	}

	/// Get the `class` attribute.
	fn class(&self) -> Option<&str> {
		self.get_attr(StyleAttributes::Class)
	}

	/// Set the `style` attribute.
	fn with_style<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(StyleAttributes::Style, value.into());
		self
	}

	/// Set the `style` attribute.
	fn set_style<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(StyleAttributes::Style, value.into());
	}

	/// Get the `style` attribute.
	fn style(&self) -> Option<&str> {
		self.get_attr(StyleAttributes::Style)
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum TransferFunctionAttributes {
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

pub trait TransferFunctionAttributesSetter {
	fn set_attr(&mut self, attr: TransferFunctionAttributes, value: String);
	fn get_attr(&self, attr: TransferFunctionAttributes) -> Option<&str>;
}

pub trait TagWithTransferFunctionAttributes: TransferFunctionAttributesSetter + Sized {

	/// Set the `amplitude` attribute.
	fn with_amplitude<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TransferFunctionAttributes::Amplitude, value.into());
		self
	}

	/// Set the `amplitude` attribute.
	fn set_amplitude<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TransferFunctionAttributes::Amplitude, value.into());
	}

	/// Get the `amplitude` attribute.
	fn amplitude(&self) -> Option<&str> {
		self.get_attr(TransferFunctionAttributes::Amplitude)
	}

	/// Set the `exponent` attribute.
	fn with_exponent<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TransferFunctionAttributes::Exponent, value.into());
		self
	}

	/// Set the `exponent` attribute.
	fn set_exponent<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TransferFunctionAttributes::Exponent, value.into());
	}

	/// Get the `exponent` attribute.
	fn exponent(&self) -> Option<&str> {
		self.get_attr(TransferFunctionAttributes::Exponent)
	}

	/// Set the `intercept` attribute.
	fn with_intercept<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TransferFunctionAttributes::Intercept, value.into());
		self
	}

	/// Set the `intercept` attribute.
	fn set_intercept<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TransferFunctionAttributes::Intercept, value.into());
	}

	/// Get the `intercept` attribute.
	fn intercept(&self) -> Option<&str> {
		self.get_attr(TransferFunctionAttributes::Intercept)
	}

	/// Set the `offset` attribute.
	fn with_offset<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TransferFunctionAttributes::Offset, value.into());
		self
	}

	/// Set the `offset` attribute.
	fn set_offset<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TransferFunctionAttributes::Offset, value.into());
	}

	/// Get the `offset` attribute.
	fn offset(&self) -> Option<&str> {
		self.get_attr(TransferFunctionAttributes::Offset)
	}

	/// Set the `slope` attribute.
	fn with_slope<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TransferFunctionAttributes::Slope, value.into());
		self
	}

	/// Set the `slope` attribute.
	fn set_slope<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TransferFunctionAttributes::Slope, value.into());
	}

	/// Get the `slope` attribute.
	fn slope(&self) -> Option<&str> {
		self.get_attr(TransferFunctionAttributes::Slope)
	}

	/// Set the `tableValues` attribute.
	fn with_table_values<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TransferFunctionAttributes::TableValues, value.into());
		self
	}

	/// Set the `tableValues` attribute.
	fn set_table_values<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TransferFunctionAttributes::TableValues, value.into());
	}

	/// Get the `tableValues` attribute.
	fn table_values(&self) -> Option<&str> {
		self.get_attr(TransferFunctionAttributes::TableValues)
	}

	/// Set the `type` attribute.
	fn with_ty<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(TransferFunctionAttributes::Type, value.into());
		self
	}

	/// Set the `type` attribute.
	fn set_ty<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(TransferFunctionAttributes::Type, value.into());
	}

	/// Get the `type` attribute.
	fn ty(&self) -> Option<&str> {
		self.get_attr(TransferFunctionAttributes::Type)
	}
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum XLinkAttributes {
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

pub trait XLinkAttributesSetter {
	fn set_attr(&mut self, attr: XLinkAttributes, value: String);
	fn get_attr(&self, attr: XLinkAttributes) -> Option<&str>;
}

pub trait TagWithXLinkAttributes: XLinkAttributesSetter + Sized {

	/// Set the `xlink:actuate` attribute.
	fn with_xlink_actuate<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(XLinkAttributes::XlinkActuate, value.into());
		self
	}

	/// Set the `xlink:actuate` attribute.
	fn set_xlink_actuate<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(XLinkAttributes::XlinkActuate, value.into());
	}

	/// Get the `xlink:actuate` attribute.
	fn xlink_actuate(&self) -> Option<&str> {
		self.get_attr(XLinkAttributes::XlinkActuate)
	}

	/// Set the `xlink:arcrole` attribute.
	fn with_xlink_arcrole<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(XLinkAttributes::XlinkArcrole, value.into());
		self
	}

	/// Set the `xlink:arcrole` attribute.
	fn set_xlink_arcrole<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(XLinkAttributes::XlinkArcrole, value.into());
	}

	/// Get the `xlink:arcrole` attribute.
	fn xlink_arcrole(&self) -> Option<&str> {
		self.get_attr(XLinkAttributes::XlinkArcrole)
	}

	/// Set the `xlink:href` attribute.
	fn with_xlink_href<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(XLinkAttributes::XlinkHref, value.into());
		self
	}

	/// Set the `xlink:href` attribute.
	fn set_xlink_href<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(XLinkAttributes::XlinkHref, value.into());
	}

	/// Get the `xlink:href` attribute.
	fn xlink_href(&self) -> Option<&str> {
		self.get_attr(XLinkAttributes::XlinkHref)
	}

	/// Set the `xlink:role` attribute.
	fn with_xlink_role<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(XLinkAttributes::XlinkRole, value.into());
		self
	}

	/// Set the `xlink:role` attribute.
	fn set_xlink_role<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(XLinkAttributes::XlinkRole, value.into());
	}

	/// Get the `xlink:role` attribute.
	fn xlink_role(&self) -> Option<&str> {
		self.get_attr(XLinkAttributes::XlinkRole)
	}

	/// Set the `xlink:show` attribute.
	fn with_xlink_show<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(XLinkAttributes::XlinkShow, value.into());
		self
	}

	/// Set the `xlink:show` attribute.
	fn set_xlink_show<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(XLinkAttributes::XlinkShow, value.into());
	}

	/// Get the `xlink:show` attribute.
	fn xlink_show(&self) -> Option<&str> {
		self.get_attr(XLinkAttributes::XlinkShow)
	}

	/// Set the `xlink:title` attribute.
	fn with_xlink_title<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(XLinkAttributes::XlinkTitle, value.into());
		self
	}

	/// Set the `xlink:title` attribute.
	fn set_xlink_title<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(XLinkAttributes::XlinkTitle, value.into());
	}

	/// Get the `xlink:title` attribute.
	fn xlink_title(&self) -> Option<&str> {
		self.get_attr(XLinkAttributes::XlinkTitle)
	}

	/// Set the `xlink:type` attribute.
	fn with_xlink_type<T>(mut self, value: T) -> Self
	where
		T: Into<String>
	{
		self.set_attr(XLinkAttributes::XlinkType, value.into());
		self
	}

	/// Set the `xlink:type` attribute.
	fn set_xlink_type<T>(&mut self, value: T)
	where
		T: Into<String>
	{
		self.set_attr(XLinkAttributes::XlinkType, value.into());
	}

	/// Get the `xlink:type` attribute.
	fn xlink_type(&self) -> Option<&str> {
		self.get_attr(XLinkAttributes::XlinkType)
	}
}

pub(super) mod prelude {
	pub use super::{
		TagWithAnimationAdditionAttributes,
		TagWithAnimationAttributeTargetAttributes,
		TagWithAnimationEventAttributes,
		TagWithAnimationTimingAttributes,
		TagWithAnimationValueAttributes,
		TagWithConditionalProcessingAttributes,
		TagWithCoreAttributes,
		TagWithDocumentEventAttributes,
		TagWithFilterAttributes,
		TagWithGlobalEventAttributes,
		TagWithGraphicalEventAttributes,
		TagWithPresentationAttributes,
		TagWithStyleAttributes,
		TagWithTransferFunctionAttributes,
		TagWithXLinkAttributes,
	};
}
