// @generated

use crate::value::Value;
use std::fmt::{self, Debug, Display, Formatter};

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum AnimationAdditionAttributes {
	Accumulate,
	Additive,
}

impl AnimationAdditionAttributes {
	pub fn as_str(&self) -> &'static str {
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
	fn set_attr<V>(&mut self, attr: AnimationAdditionAttributes, value: V)
	where
		V: Value + 'static;
	
	fn get_attr(&self, attr: AnimationAdditionAttributes) -> Option<&dyn Value>;
}

pub trait TagWithAnimationAdditionAttributes: AnimationAdditionAttributesSetter + Sized {

	/// Set the `accumulate` attribute.
	fn with_accumulate<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimationAdditionAttributes::Accumulate, value);
		self
	}

	/// Set the `accumulate` attribute.
	fn set_accumulate<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimationAdditionAttributes::Accumulate, value);
	}

	/// Get the `accumulate` attribute.
	fn accumulate(&self) -> Option<&dyn Value> {
		self.get_attr(AnimationAdditionAttributes::Accumulate)
	}

	/// Set the `additive` attribute.
	fn with_additive<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimationAdditionAttributes::Additive, value);
		self
	}

	/// Set the `additive` attribute.
	fn set_additive<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimationAdditionAttributes::Additive, value);
	}

	/// Get the `additive` attribute.
	fn additive(&self) -> Option<&dyn Value> {
		self.get_attr(AnimationAdditionAttributes::Additive)
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum AnimationAttributeTargetAttributes {
	AttributeName,
	AttributeType,
}

impl AnimationAttributeTargetAttributes {
	pub fn as_str(&self) -> &'static str {
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
	fn set_attr<V>(&mut self, attr: AnimationAttributeTargetAttributes, value: V)
	where
		V: Value + 'static;
	
	fn get_attr(&self, attr: AnimationAttributeTargetAttributes) -> Option<&dyn Value>;
}

pub trait TagWithAnimationAttributeTargetAttributes: AnimationAttributeTargetAttributesSetter + Sized {

	/// Set the `attributeName` attribute.
	fn with_attribute_name<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimationAttributeTargetAttributes::AttributeName, value);
		self
	}

	/// Set the `attributeName` attribute.
	fn set_attribute_name<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimationAttributeTargetAttributes::AttributeName, value);
	}

	/// Get the `attributeName` attribute.
	fn attribute_name(&self) -> Option<&dyn Value> {
		self.get_attr(AnimationAttributeTargetAttributes::AttributeName)
	}

	/// Set the `attributeType` attribute.
	fn with_attribute_type<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimationAttributeTargetAttributes::AttributeType, value);
		self
	}

	/// Set the `attributeType` attribute.
	fn set_attribute_type<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimationAttributeTargetAttributes::AttributeType, value);
	}

	/// Get the `attributeType` attribute.
	fn attribute_type(&self) -> Option<&dyn Value> {
		self.get_attr(AnimationAttributeTargetAttributes::AttributeType)
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum AnimationEventAttributes {
	Onbegin,
	Onend,
	Onrepeat,
}

impl AnimationEventAttributes {
	pub fn as_str(&self) -> &'static str {
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
	fn set_attr<V>(&mut self, attr: AnimationEventAttributes, value: V)
	where
		V: Value + 'static;
	
	fn get_attr(&self, attr: AnimationEventAttributes) -> Option<&dyn Value>;
}

pub trait TagWithAnimationEventAttributes: AnimationEventAttributesSetter + Sized {

	/// Set the `onbegin` attribute.
	fn with_onbegin<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimationEventAttributes::Onbegin, value);
		self
	}

	/// Set the `onbegin` attribute.
	fn set_onbegin<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimationEventAttributes::Onbegin, value);
	}

	/// Get the `onbegin` attribute.
	fn onbegin(&self) -> Option<&dyn Value> {
		self.get_attr(AnimationEventAttributes::Onbegin)
	}

	/// Set the `onend` attribute.
	fn with_onend<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimationEventAttributes::Onend, value);
		self
	}

	/// Set the `onend` attribute.
	fn set_onend<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimationEventAttributes::Onend, value);
	}

	/// Get the `onend` attribute.
	fn onend(&self) -> Option<&dyn Value> {
		self.get_attr(AnimationEventAttributes::Onend)
	}

	/// Set the `onrepeat` attribute.
	fn with_onrepeat<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimationEventAttributes::Onrepeat, value);
		self
	}

	/// Set the `onrepeat` attribute.
	fn set_onrepeat<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimationEventAttributes::Onrepeat, value);
	}

	/// Get the `onrepeat` attribute.
	fn onrepeat(&self) -> Option<&dyn Value> {
		self.get_attr(AnimationEventAttributes::Onrepeat)
	}
}

#[allow(clippy::enum_variant_names)]
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
	pub fn as_str(&self) -> &'static str {
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
	fn set_attr<V>(&mut self, attr: AnimationTimingAttributes, value: V)
	where
		V: Value + 'static;
	
	fn get_attr(&self, attr: AnimationTimingAttributes) -> Option<&dyn Value>;
}

pub trait TagWithAnimationTimingAttributes: AnimationTimingAttributesSetter + Sized {

	/// Set the `begin` attribute.
	fn with_begin<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimationTimingAttributes::Begin, value);
		self
	}

	/// Set the `begin` attribute.
	fn set_begin<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimationTimingAttributes::Begin, value);
	}

	/// Get the `begin` attribute.
	fn begin(&self) -> Option<&dyn Value> {
		self.get_attr(AnimationTimingAttributes::Begin)
	}

	/// Set the `dur` attribute.
	fn with_dur<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimationTimingAttributes::Dur, value);
		self
	}

	/// Set the `dur` attribute.
	fn set_dur<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimationTimingAttributes::Dur, value);
	}

	/// Get the `dur` attribute.
	fn dur(&self) -> Option<&dyn Value> {
		self.get_attr(AnimationTimingAttributes::Dur)
	}

	/// Set the `end` attribute.
	fn with_end<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimationTimingAttributes::End, value);
		self
	}

	/// Set the `end` attribute.
	fn set_end<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimationTimingAttributes::End, value);
	}

	/// Get the `end` attribute.
	fn end(&self) -> Option<&dyn Value> {
		self.get_attr(AnimationTimingAttributes::End)
	}

	/// Set the `fill` attribute.
	fn with_fill<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimationTimingAttributes::Fill, value);
		self
	}

	/// Set the `fill` attribute.
	fn set_fill<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimationTimingAttributes::Fill, value);
	}

	/// Get the `fill` attribute.
	fn fill(&self) -> Option<&dyn Value> {
		self.get_attr(AnimationTimingAttributes::Fill)
	}

	/// Set the `max` attribute.
	fn with_max<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimationTimingAttributes::Max, value);
		self
	}

	/// Set the `max` attribute.
	fn set_max<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimationTimingAttributes::Max, value);
	}

	/// Get the `max` attribute.
	fn max(&self) -> Option<&dyn Value> {
		self.get_attr(AnimationTimingAttributes::Max)
	}

	/// Set the `min` attribute.
	fn with_min<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimationTimingAttributes::Min, value);
		self
	}

	/// Set the `min` attribute.
	fn set_min<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimationTimingAttributes::Min, value);
	}

	/// Get the `min` attribute.
	fn min(&self) -> Option<&dyn Value> {
		self.get_attr(AnimationTimingAttributes::Min)
	}

	/// Set the `repeatCount` attribute.
	fn with_repeat_count<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimationTimingAttributes::RepeatCount, value);
		self
	}

	/// Set the `repeatCount` attribute.
	fn set_repeat_count<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimationTimingAttributes::RepeatCount, value);
	}

	/// Get the `repeatCount` attribute.
	fn repeat_count(&self) -> Option<&dyn Value> {
		self.get_attr(AnimationTimingAttributes::RepeatCount)
	}

	/// Set the `repeatDur` attribute.
	fn with_repeat_dur<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimationTimingAttributes::RepeatDur, value);
		self
	}

	/// Set the `repeatDur` attribute.
	fn set_repeat_dur<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimationTimingAttributes::RepeatDur, value);
	}

	/// Get the `repeatDur` attribute.
	fn repeat_dur(&self) -> Option<&dyn Value> {
		self.get_attr(AnimationTimingAttributes::RepeatDur)
	}

	/// Set the `restart` attribute.
	fn with_restart<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimationTimingAttributes::Restart, value);
		self
	}

	/// Set the `restart` attribute.
	fn set_restart<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimationTimingAttributes::Restart, value);
	}

	/// Get the `restart` attribute.
	fn restart(&self) -> Option<&dyn Value> {
		self.get_attr(AnimationTimingAttributes::Restart)
	}
}

#[allow(clippy::enum_variant_names)]
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
	pub fn as_str(&self) -> &'static str {
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
	fn set_attr<V>(&mut self, attr: AnimationValueAttributes, value: V)
	where
		V: Value + 'static;
	
	fn get_attr(&self, attr: AnimationValueAttributes) -> Option<&dyn Value>;
}

pub trait TagWithAnimationValueAttributes: AnimationValueAttributesSetter + Sized {

	/// Set the `accelerate` attribute.
	fn with_accelerate<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimationValueAttributes::Accelerate, value);
		self
	}

	/// Set the `accelerate` attribute.
	fn set_accelerate<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimationValueAttributes::Accelerate, value);
	}

	/// Get the `accelerate` attribute.
	fn accelerate(&self) -> Option<&dyn Value> {
		self.get_attr(AnimationValueAttributes::Accelerate)
	}

	/// Set the `autoReverse` attribute.
	fn with_auto_reverse<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimationValueAttributes::AutoReverse, value);
		self
	}

	/// Set the `autoReverse` attribute.
	fn set_auto_reverse<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimationValueAttributes::AutoReverse, value);
	}

	/// Get the `autoReverse` attribute.
	fn auto_reverse(&self) -> Option<&dyn Value> {
		self.get_attr(AnimationValueAttributes::AutoReverse)
	}

	/// Set the `by` attribute.
	fn with_by<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimationValueAttributes::By, value);
		self
	}

	/// Set the `by` attribute.
	fn set_by<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimationValueAttributes::By, value);
	}

	/// Get the `by` attribute.
	fn by(&self) -> Option<&dyn Value> {
		self.get_attr(AnimationValueAttributes::By)
	}

	/// Set the `calcMode` attribute.
	fn with_calc_mode<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimationValueAttributes::CalcMode, value);
		self
	}

	/// Set the `calcMode` attribute.
	fn set_calc_mode<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimationValueAttributes::CalcMode, value);
	}

	/// Get the `calcMode` attribute.
	fn calc_mode(&self) -> Option<&dyn Value> {
		self.get_attr(AnimationValueAttributes::CalcMode)
	}

	/// Set the `decelerate` attribute.
	fn with_decelerate<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimationValueAttributes::Decelerate, value);
		self
	}

	/// Set the `decelerate` attribute.
	fn set_decelerate<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimationValueAttributes::Decelerate, value);
	}

	/// Get the `decelerate` attribute.
	fn decelerate(&self) -> Option<&dyn Value> {
		self.get_attr(AnimationValueAttributes::Decelerate)
	}

	/// Set the `from` attribute.
	fn with_from<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimationValueAttributes::From, value);
		self
	}

	/// Set the `from` attribute.
	fn set_from<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimationValueAttributes::From, value);
	}

	/// Get the `from` attribute.
	fn from(&self) -> Option<&dyn Value> {
		self.get_attr(AnimationValueAttributes::From)
	}

	/// Set the `keySplines` attribute.
	fn with_key_splines<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimationValueAttributes::KeySplines, value);
		self
	}

	/// Set the `keySplines` attribute.
	fn set_key_splines<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimationValueAttributes::KeySplines, value);
	}

	/// Get the `keySplines` attribute.
	fn key_splines(&self) -> Option<&dyn Value> {
		self.get_attr(AnimationValueAttributes::KeySplines)
	}

	/// Set the `keyTimes` attribute.
	fn with_key_times<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimationValueAttributes::KeyTimes, value);
		self
	}

	/// Set the `keyTimes` attribute.
	fn set_key_times<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimationValueAttributes::KeyTimes, value);
	}

	/// Get the `keyTimes` attribute.
	fn key_times(&self) -> Option<&dyn Value> {
		self.get_attr(AnimationValueAttributes::KeyTimes)
	}

	/// Set the `to` attribute.
	fn with_to<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimationValueAttributes::To, value);
		self
	}

	/// Set the `to` attribute.
	fn set_to<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimationValueAttributes::To, value);
	}

	/// Get the `to` attribute.
	fn to(&self) -> Option<&dyn Value> {
		self.get_attr(AnimationValueAttributes::To)
	}

	/// Set the `values` attribute.
	fn with_values<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(AnimationValueAttributes::Values, value);
		self
	}

	/// Set the `values` attribute.
	fn set_values<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(AnimationValueAttributes::Values, value);
	}

	/// Get the `values` attribute.
	fn values(&self) -> Option<&dyn Value> {
		self.get_attr(AnimationValueAttributes::Values)
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum ConditionalProcessingAttributes {
	RequiredExtensions,
	RequiredFeatures,
	SystemLanguage,
}

impl ConditionalProcessingAttributes {
	pub fn as_str(&self) -> &'static str {
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
	fn set_attr<V>(&mut self, attr: ConditionalProcessingAttributes, value: V)
	where
		V: Value + 'static;
	
	fn get_attr(&self, attr: ConditionalProcessingAttributes) -> Option<&dyn Value>;
}

pub trait TagWithConditionalProcessingAttributes: ConditionalProcessingAttributesSetter + Sized {

	/// Set the `requiredExtensions` attribute.
	fn with_required_extensions<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ConditionalProcessingAttributes::RequiredExtensions, value);
		self
	}

	/// Set the `requiredExtensions` attribute.
	fn set_required_extensions<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ConditionalProcessingAttributes::RequiredExtensions, value);
	}

	/// Get the `requiredExtensions` attribute.
	fn required_extensions(&self) -> Option<&dyn Value> {
		self.get_attr(ConditionalProcessingAttributes::RequiredExtensions)
	}

	/// Set the `requiredFeatures` attribute.
	fn with_required_features<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ConditionalProcessingAttributes::RequiredFeatures, value);
		self
	}

	/// Set the `requiredFeatures` attribute.
	fn set_required_features<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ConditionalProcessingAttributes::RequiredFeatures, value);
	}

	/// Get the `requiredFeatures` attribute.
	fn required_features(&self) -> Option<&dyn Value> {
		self.get_attr(ConditionalProcessingAttributes::RequiredFeatures)
	}

	/// Set the `systemLanguage` attribute.
	fn with_system_language<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(ConditionalProcessingAttributes::SystemLanguage, value);
		self
	}

	/// Set the `systemLanguage` attribute.
	fn set_system_language<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(ConditionalProcessingAttributes::SystemLanguage, value);
	}

	/// Get the `systemLanguage` attribute.
	fn system_language(&self) -> Option<&dyn Value> {
		self.get_attr(ConditionalProcessingAttributes::SystemLanguage)
	}
}

#[allow(clippy::enum_variant_names)]
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
	pub fn as_str(&self) -> &'static str {
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
	fn set_attr<V>(&mut self, attr: CoreAttributes, value: V)
	where
		V: Value + 'static;
	
	fn get_attr(&self, attr: CoreAttributes) -> Option<&dyn Value>;
}

pub trait TagWithCoreAttributes: CoreAttributesSetter + Sized {

	/// Set the `id` attribute.
	fn with_id<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(CoreAttributes::Id, value);
		self
	}

	/// Set the `id` attribute.
	fn set_id<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(CoreAttributes::Id, value);
	}

	/// Get the `id` attribute.
	fn id(&self) -> Option<&dyn Value> {
		self.get_attr(CoreAttributes::Id)
	}

	/// Set the `lang` attribute.
	fn with_lang<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(CoreAttributes::Lang, value);
		self
	}

	/// Set the `lang` attribute.
	fn set_lang<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(CoreAttributes::Lang, value);
	}

	/// Get the `lang` attribute.
	fn lang(&self) -> Option<&dyn Value> {
		self.get_attr(CoreAttributes::Lang)
	}

	/// Set the `tabindex` attribute.
	fn with_tabindex<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(CoreAttributes::Tabindex, value);
		self
	}

	/// Set the `tabindex` attribute.
	fn set_tabindex<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(CoreAttributes::Tabindex, value);
	}

	/// Get the `tabindex` attribute.
	fn tabindex(&self) -> Option<&dyn Value> {
		self.get_attr(CoreAttributes::Tabindex)
	}

	/// Set the `xml:base` attribute.
	fn with_xml_base<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(CoreAttributes::XmlBase, value);
		self
	}

	/// Set the `xml:base` attribute.
	fn set_xml_base<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(CoreAttributes::XmlBase, value);
	}

	/// Get the `xml:base` attribute.
	fn xml_base(&self) -> Option<&dyn Value> {
		self.get_attr(CoreAttributes::XmlBase)
	}

	/// Set the `xml:lang` attribute.
	fn with_xml_lang<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(CoreAttributes::XmlLang, value);
		self
	}

	/// Set the `xml:lang` attribute.
	fn set_xml_lang<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(CoreAttributes::XmlLang, value);
	}

	/// Get the `xml:lang` attribute.
	fn xml_lang(&self) -> Option<&dyn Value> {
		self.get_attr(CoreAttributes::XmlLang)
	}

	/// Set the `xml:space` attribute.
	fn with_xml_space<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(CoreAttributes::XmlSpace, value);
		self
	}

	/// Set the `xml:space` attribute.
	fn set_xml_space<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(CoreAttributes::XmlSpace, value);
	}

	/// Get the `xml:space` attribute.
	fn xml_space(&self) -> Option<&dyn Value> {
		self.get_attr(CoreAttributes::XmlSpace)
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum DocumentEventAttributes {
	Onabort,
	Onerror,
	Onresize,
	Onscroll,
	Onunload,
}

impl DocumentEventAttributes {
	pub fn as_str(&self) -> &'static str {
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
	fn set_attr<V>(&mut self, attr: DocumentEventAttributes, value: V)
	where
		V: Value + 'static;
	
	fn get_attr(&self, attr: DocumentEventAttributes) -> Option<&dyn Value>;
}

pub trait TagWithDocumentEventAttributes: DocumentEventAttributesSetter + Sized {

	/// Set the `onabort` attribute.
	fn with_onabort<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(DocumentEventAttributes::Onabort, value);
		self
	}

	/// Set the `onabort` attribute.
	fn set_onabort<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(DocumentEventAttributes::Onabort, value);
	}

	/// Get the `onabort` attribute.
	fn onabort(&self) -> Option<&dyn Value> {
		self.get_attr(DocumentEventAttributes::Onabort)
	}

	/// Set the `onerror` attribute.
	fn with_onerror<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(DocumentEventAttributes::Onerror, value);
		self
	}

	/// Set the `onerror` attribute.
	fn set_onerror<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(DocumentEventAttributes::Onerror, value);
	}

	/// Get the `onerror` attribute.
	fn onerror(&self) -> Option<&dyn Value> {
		self.get_attr(DocumentEventAttributes::Onerror)
	}

	/// Set the `onresize` attribute.
	fn with_onresize<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(DocumentEventAttributes::Onresize, value);
		self
	}

	/// Set the `onresize` attribute.
	fn set_onresize<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(DocumentEventAttributes::Onresize, value);
	}

	/// Get the `onresize` attribute.
	fn onresize(&self) -> Option<&dyn Value> {
		self.get_attr(DocumentEventAttributes::Onresize)
	}

	/// Set the `onscroll` attribute.
	fn with_onscroll<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(DocumentEventAttributes::Onscroll, value);
		self
	}

	/// Set the `onscroll` attribute.
	fn set_onscroll<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(DocumentEventAttributes::Onscroll, value);
	}

	/// Get the `onscroll` attribute.
	fn onscroll(&self) -> Option<&dyn Value> {
		self.get_attr(DocumentEventAttributes::Onscroll)
	}

	/// Set the `onunload` attribute.
	fn with_onunload<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(DocumentEventAttributes::Onunload, value);
		self
	}

	/// Set the `onunload` attribute.
	fn set_onunload<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(DocumentEventAttributes::Onunload, value);
	}

	/// Get the `onunload` attribute.
	fn onunload(&self) -> Option<&dyn Value> {
		self.get_attr(DocumentEventAttributes::Onunload)
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum FilterAttributes {
	Height,
	Result,
	Width,
	X,
	Y,
}

impl FilterAttributes {
	pub fn as_str(&self) -> &'static str {
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
	fn set_attr<V>(&mut self, attr: FilterAttributes, value: V)
	where
		V: Value + 'static;
	
	fn get_attr(&self, attr: FilterAttributes) -> Option<&dyn Value>;
}

pub trait TagWithFilterAttributes: FilterAttributesSetter + Sized {

	/// Set the `height` attribute.
	fn with_height<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttributes::Height, value);
		self
	}

	/// Set the `height` attribute.
	fn set_height<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttributes::Height, value);
	}

	/// Get the `height` attribute.
	fn height(&self) -> Option<&dyn Value> {
		self.get_attr(FilterAttributes::Height)
	}

	/// Set the `result` attribute.
	fn with_result<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttributes::Result, value);
		self
	}

	/// Set the `result` attribute.
	fn set_result<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttributes::Result, value);
	}

	/// Get the `result` attribute.
	fn result(&self) -> Option<&dyn Value> {
		self.get_attr(FilterAttributes::Result)
	}

	/// Set the `width` attribute.
	fn with_width<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttributes::Width, value);
		self
	}

	/// Set the `width` attribute.
	fn set_width<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttributes::Width, value);
	}

	/// Get the `width` attribute.
	fn width(&self) -> Option<&dyn Value> {
		self.get_attr(FilterAttributes::Width)
	}

	/// Set the `x` attribute.
	fn with_x<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttributes::X, value);
		self
	}

	/// Set the `x` attribute.
	fn set_x<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttributes::X, value);
	}

	/// Get the `x` attribute.
	fn x(&self) -> Option<&dyn Value> {
		self.get_attr(FilterAttributes::X)
	}

	/// Set the `y` attribute.
	fn with_y<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttributes::Y, value);
		self
	}

	/// Set the `y` attribute.
	fn set_y<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(FilterAttributes::Y, value);
	}

	/// Get the `y` attribute.
	fn y(&self) -> Option<&dyn Value> {
		self.get_attr(FilterAttributes::Y)
	}
}

#[allow(clippy::enum_variant_names)]
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
	pub fn as_str(&self) -> &'static str {
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
	fn set_attr<V>(&mut self, attr: GlobalEventAttributes, value: V)
	where
		V: Value + 'static;
	
	fn get_attr(&self, attr: GlobalEventAttributes) -> Option<&dyn Value>;
}

pub trait TagWithGlobalEventAttributes: GlobalEventAttributesSetter + Sized {

	/// Set the `oncancel` attribute.
	fn with_oncancel<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Oncancel, value);
		self
	}

	/// Set the `oncancel` attribute.
	fn set_oncancel<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Oncancel, value);
	}

	/// Get the `oncancel` attribute.
	fn oncancel(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Oncancel)
	}

	/// Set the `oncanplay` attribute.
	fn with_oncanplay<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Oncanplay, value);
		self
	}

	/// Set the `oncanplay` attribute.
	fn set_oncanplay<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Oncanplay, value);
	}

	/// Get the `oncanplay` attribute.
	fn oncanplay(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Oncanplay)
	}

	/// Set the `oncanplaythrough` attribute.
	fn with_oncanplaythrough<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Oncanplaythrough, value);
		self
	}

	/// Set the `oncanplaythrough` attribute.
	fn set_oncanplaythrough<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Oncanplaythrough, value);
	}

	/// Get the `oncanplaythrough` attribute.
	fn oncanplaythrough(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Oncanplaythrough)
	}

	/// Set the `onchange` attribute.
	fn with_onchange<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onchange, value);
		self
	}

	/// Set the `onchange` attribute.
	fn set_onchange<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onchange, value);
	}

	/// Get the `onchange` attribute.
	fn onchange(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onchange)
	}

	/// Set the `onclick` attribute.
	fn with_onclick<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onclick, value);
		self
	}

	/// Set the `onclick` attribute.
	fn set_onclick<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onclick, value);
	}

	/// Get the `onclick` attribute.
	fn onclick(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onclick)
	}

	/// Set the `onclose` attribute.
	fn with_onclose<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onclose, value);
		self
	}

	/// Set the `onclose` attribute.
	fn set_onclose<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onclose, value);
	}

	/// Get the `onclose` attribute.
	fn onclose(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onclose)
	}

	/// Set the `oncuechange` attribute.
	fn with_oncuechange<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Oncuechange, value);
		self
	}

	/// Set the `oncuechange` attribute.
	fn set_oncuechange<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Oncuechange, value);
	}

	/// Get the `oncuechange` attribute.
	fn oncuechange(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Oncuechange)
	}

	/// Set the `ondblclick` attribute.
	fn with_ondblclick<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Ondblclick, value);
		self
	}

	/// Set the `ondblclick` attribute.
	fn set_ondblclick<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Ondblclick, value);
	}

	/// Get the `ondblclick` attribute.
	fn ondblclick(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Ondblclick)
	}

	/// Set the `ondrag` attribute.
	fn with_ondrag<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Ondrag, value);
		self
	}

	/// Set the `ondrag` attribute.
	fn set_ondrag<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Ondrag, value);
	}

	/// Get the `ondrag` attribute.
	fn ondrag(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Ondrag)
	}

	/// Set the `ondragend` attribute.
	fn with_ondragend<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Ondragend, value);
		self
	}

	/// Set the `ondragend` attribute.
	fn set_ondragend<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Ondragend, value);
	}

	/// Get the `ondragend` attribute.
	fn ondragend(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Ondragend)
	}

	/// Set the `ondragenter` attribute.
	fn with_ondragenter<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Ondragenter, value);
		self
	}

	/// Set the `ondragenter` attribute.
	fn set_ondragenter<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Ondragenter, value);
	}

	/// Get the `ondragenter` attribute.
	fn ondragenter(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Ondragenter)
	}

	/// Set the `ondragleave` attribute.
	fn with_ondragleave<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Ondragleave, value);
		self
	}

	/// Set the `ondragleave` attribute.
	fn set_ondragleave<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Ondragleave, value);
	}

	/// Get the `ondragleave` attribute.
	fn ondragleave(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Ondragleave)
	}

	/// Set the `ondragover` attribute.
	fn with_ondragover<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Ondragover, value);
		self
	}

	/// Set the `ondragover` attribute.
	fn set_ondragover<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Ondragover, value);
	}

	/// Get the `ondragover` attribute.
	fn ondragover(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Ondragover)
	}

	/// Set the `ondragstart` attribute.
	fn with_ondragstart<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Ondragstart, value);
		self
	}

	/// Set the `ondragstart` attribute.
	fn set_ondragstart<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Ondragstart, value);
	}

	/// Get the `ondragstart` attribute.
	fn ondragstart(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Ondragstart)
	}

	/// Set the `ondrop` attribute.
	fn with_ondrop<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Ondrop, value);
		self
	}

	/// Set the `ondrop` attribute.
	fn set_ondrop<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Ondrop, value);
	}

	/// Get the `ondrop` attribute.
	fn ondrop(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Ondrop)
	}

	/// Set the `ondurationchange` attribute.
	fn with_ondurationchange<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Ondurationchange, value);
		self
	}

	/// Set the `ondurationchange` attribute.
	fn set_ondurationchange<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Ondurationchange, value);
	}

	/// Get the `ondurationchange` attribute.
	fn ondurationchange(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Ondurationchange)
	}

	/// Set the `onemptied` attribute.
	fn with_onemptied<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onemptied, value);
		self
	}

	/// Set the `onemptied` attribute.
	fn set_onemptied<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onemptied, value);
	}

	/// Get the `onemptied` attribute.
	fn onemptied(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onemptied)
	}

	/// Set the `onended` attribute.
	fn with_onended<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onended, value);
		self
	}

	/// Set the `onended` attribute.
	fn set_onended<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onended, value);
	}

	/// Get the `onended` attribute.
	fn onended(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onended)
	}

	/// Set the `onerror` attribute.
	fn with_onerror<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onerror, value);
		self
	}

	/// Set the `onerror` attribute.
	fn set_onerror<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onerror, value);
	}

	/// Get the `onerror` attribute.
	fn onerror(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onerror)
	}

	/// Set the `onfocus` attribute.
	fn with_onfocus<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onfocus, value);
		self
	}

	/// Set the `onfocus` attribute.
	fn set_onfocus<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onfocus, value);
	}

	/// Get the `onfocus` attribute.
	fn onfocus(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onfocus)
	}

	/// Set the `oninput` attribute.
	fn with_oninput<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Oninput, value);
		self
	}

	/// Set the `oninput` attribute.
	fn set_oninput<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Oninput, value);
	}

	/// Get the `oninput` attribute.
	fn oninput(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Oninput)
	}

	/// Set the `oninvalid` attribute.
	fn with_oninvalid<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Oninvalid, value);
		self
	}

	/// Set the `oninvalid` attribute.
	fn set_oninvalid<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Oninvalid, value);
	}

	/// Get the `oninvalid` attribute.
	fn oninvalid(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Oninvalid)
	}

	/// Set the `onkeydown` attribute.
	fn with_onkeydown<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onkeydown, value);
		self
	}

	/// Set the `onkeydown` attribute.
	fn set_onkeydown<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onkeydown, value);
	}

	/// Get the `onkeydown` attribute.
	fn onkeydown(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onkeydown)
	}

	/// Set the `onkeypress` attribute.
	fn with_onkeypress<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onkeypress, value);
		self
	}

	/// Set the `onkeypress` attribute.
	fn set_onkeypress<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onkeypress, value);
	}

	/// Get the `onkeypress` attribute.
	fn onkeypress(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onkeypress)
	}

	/// Set the `onkeyup` attribute.
	fn with_onkeyup<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onkeyup, value);
		self
	}

	/// Set the `onkeyup` attribute.
	fn set_onkeyup<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onkeyup, value);
	}

	/// Get the `onkeyup` attribute.
	fn onkeyup(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onkeyup)
	}

	/// Set the `onload` attribute.
	fn with_onload<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onload, value);
		self
	}

	/// Set the `onload` attribute.
	fn set_onload<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onload, value);
	}

	/// Get the `onload` attribute.
	fn onload(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onload)
	}

	/// Set the `onloadeddata` attribute.
	fn with_onloadeddata<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onloadeddata, value);
		self
	}

	/// Set the `onloadeddata` attribute.
	fn set_onloadeddata<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onloadeddata, value);
	}

	/// Get the `onloadeddata` attribute.
	fn onloadeddata(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onloadeddata)
	}

	/// Set the `onloadedmetadata` attribute.
	fn with_onloadedmetadata<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onloadedmetadata, value);
		self
	}

	/// Set the `onloadedmetadata` attribute.
	fn set_onloadedmetadata<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onloadedmetadata, value);
	}

	/// Get the `onloadedmetadata` attribute.
	fn onloadedmetadata(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onloadedmetadata)
	}

	/// Set the `onloadstart` attribute.
	fn with_onloadstart<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onloadstart, value);
		self
	}

	/// Set the `onloadstart` attribute.
	fn set_onloadstart<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onloadstart, value);
	}

	/// Get the `onloadstart` attribute.
	fn onloadstart(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onloadstart)
	}

	/// Set the `onmousedown` attribute.
	fn with_onmousedown<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onmousedown, value);
		self
	}

	/// Set the `onmousedown` attribute.
	fn set_onmousedown<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onmousedown, value);
	}

	/// Get the `onmousedown` attribute.
	fn onmousedown(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onmousedown)
	}

	/// Set the `onmouseenter` attribute.
	fn with_onmouseenter<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onmouseenter, value);
		self
	}

	/// Set the `onmouseenter` attribute.
	fn set_onmouseenter<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onmouseenter, value);
	}

	/// Get the `onmouseenter` attribute.
	fn onmouseenter(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onmouseenter)
	}

	/// Set the `onmouseleave` attribute.
	fn with_onmouseleave<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onmouseleave, value);
		self
	}

	/// Set the `onmouseleave` attribute.
	fn set_onmouseleave<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onmouseleave, value);
	}

	/// Get the `onmouseleave` attribute.
	fn onmouseleave(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onmouseleave)
	}

	/// Set the `onmousemove` attribute.
	fn with_onmousemove<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onmousemove, value);
		self
	}

	/// Set the `onmousemove` attribute.
	fn set_onmousemove<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onmousemove, value);
	}

	/// Get the `onmousemove` attribute.
	fn onmousemove(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onmousemove)
	}

	/// Set the `onmouseout` attribute.
	fn with_onmouseout<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onmouseout, value);
		self
	}

	/// Set the `onmouseout` attribute.
	fn set_onmouseout<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onmouseout, value);
	}

	/// Get the `onmouseout` attribute.
	fn onmouseout(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onmouseout)
	}

	/// Set the `onmouseover` attribute.
	fn with_onmouseover<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onmouseover, value);
		self
	}

	/// Set the `onmouseover` attribute.
	fn set_onmouseover<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onmouseover, value);
	}

	/// Get the `onmouseover` attribute.
	fn onmouseover(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onmouseover)
	}

	/// Set the `onmouseup` attribute.
	fn with_onmouseup<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onmouseup, value);
		self
	}

	/// Set the `onmouseup` attribute.
	fn set_onmouseup<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onmouseup, value);
	}

	/// Get the `onmouseup` attribute.
	fn onmouseup(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onmouseup)
	}

	/// Set the `onmousewheel` attribute.
	fn with_onmousewheel<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onmousewheel, value);
		self
	}

	/// Set the `onmousewheel` attribute.
	fn set_onmousewheel<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onmousewheel, value);
	}

	/// Get the `onmousewheel` attribute.
	fn onmousewheel(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onmousewheel)
	}

	/// Set the `onpause` attribute.
	fn with_onpause<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onpause, value);
		self
	}

	/// Set the `onpause` attribute.
	fn set_onpause<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onpause, value);
	}

	/// Get the `onpause` attribute.
	fn onpause(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onpause)
	}

	/// Set the `onplay` attribute.
	fn with_onplay<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onplay, value);
		self
	}

	/// Set the `onplay` attribute.
	fn set_onplay<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onplay, value);
	}

	/// Get the `onplay` attribute.
	fn onplay(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onplay)
	}

	/// Set the `onplaying` attribute.
	fn with_onplaying<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onplaying, value);
		self
	}

	/// Set the `onplaying` attribute.
	fn set_onplaying<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onplaying, value);
	}

	/// Get the `onplaying` attribute.
	fn onplaying(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onplaying)
	}

	/// Set the `onprogress` attribute.
	fn with_onprogress<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onprogress, value);
		self
	}

	/// Set the `onprogress` attribute.
	fn set_onprogress<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onprogress, value);
	}

	/// Get the `onprogress` attribute.
	fn onprogress(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onprogress)
	}

	/// Set the `onratechange` attribute.
	fn with_onratechange<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onratechange, value);
		self
	}

	/// Set the `onratechange` attribute.
	fn set_onratechange<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onratechange, value);
	}

	/// Get the `onratechange` attribute.
	fn onratechange(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onratechange)
	}

	/// Set the `onreset` attribute.
	fn with_onreset<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onreset, value);
		self
	}

	/// Set the `onreset` attribute.
	fn set_onreset<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onreset, value);
	}

	/// Get the `onreset` attribute.
	fn onreset(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onreset)
	}

	/// Set the `onresize` attribute.
	fn with_onresize<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onresize, value);
		self
	}

	/// Set the `onresize` attribute.
	fn set_onresize<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onresize, value);
	}

	/// Get the `onresize` attribute.
	fn onresize(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onresize)
	}

	/// Set the `onscroll` attribute.
	fn with_onscroll<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onscroll, value);
		self
	}

	/// Set the `onscroll` attribute.
	fn set_onscroll<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onscroll, value);
	}

	/// Get the `onscroll` attribute.
	fn onscroll(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onscroll)
	}

	/// Set the `onseeked` attribute.
	fn with_onseeked<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onseeked, value);
		self
	}

	/// Set the `onseeked` attribute.
	fn set_onseeked<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onseeked, value);
	}

	/// Get the `onseeked` attribute.
	fn onseeked(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onseeked)
	}

	/// Set the `onseeking` attribute.
	fn with_onseeking<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onseeking, value);
		self
	}

	/// Set the `onseeking` attribute.
	fn set_onseeking<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onseeking, value);
	}

	/// Get the `onseeking` attribute.
	fn onseeking(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onseeking)
	}

	/// Set the `onselect` attribute.
	fn with_onselect<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onselect, value);
		self
	}

	/// Set the `onselect` attribute.
	fn set_onselect<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onselect, value);
	}

	/// Get the `onselect` attribute.
	fn onselect(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onselect)
	}

	/// Set the `onshow` attribute.
	fn with_onshow<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onshow, value);
		self
	}

	/// Set the `onshow` attribute.
	fn set_onshow<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onshow, value);
	}

	/// Get the `onshow` attribute.
	fn onshow(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onshow)
	}

	/// Set the `onstalled` attribute.
	fn with_onstalled<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onstalled, value);
		self
	}

	/// Set the `onstalled` attribute.
	fn set_onstalled<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onstalled, value);
	}

	/// Get the `onstalled` attribute.
	fn onstalled(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onstalled)
	}

	/// Set the `onsubmit` attribute.
	fn with_onsubmit<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onsubmit, value);
		self
	}

	/// Set the `onsubmit` attribute.
	fn set_onsubmit<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onsubmit, value);
	}

	/// Get the `onsubmit` attribute.
	fn onsubmit(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onsubmit)
	}

	/// Set the `onsuspend` attribute.
	fn with_onsuspend<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onsuspend, value);
		self
	}

	/// Set the `onsuspend` attribute.
	fn set_onsuspend<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onsuspend, value);
	}

	/// Get the `onsuspend` attribute.
	fn onsuspend(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onsuspend)
	}

	/// Set the `ontimeupdate` attribute.
	fn with_ontimeupdate<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Ontimeupdate, value);
		self
	}

	/// Set the `ontimeupdate` attribute.
	fn set_ontimeupdate<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Ontimeupdate, value);
	}

	/// Get the `ontimeupdate` attribute.
	fn ontimeupdate(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Ontimeupdate)
	}

	/// Set the `ontoggle` attribute.
	fn with_ontoggle<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Ontoggle, value);
		self
	}

	/// Set the `ontoggle` attribute.
	fn set_ontoggle<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Ontoggle, value);
	}

	/// Get the `ontoggle` attribute.
	fn ontoggle(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Ontoggle)
	}

	/// Set the `onvolumechange` attribute.
	fn with_onvolumechange<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onvolumechange, value);
		self
	}

	/// Set the `onvolumechange` attribute.
	fn set_onvolumechange<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onvolumechange, value);
	}

	/// Get the `onvolumechange` attribute.
	fn onvolumechange(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onvolumechange)
	}

	/// Set the `onwaiting` attribute.
	fn with_onwaiting<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onwaiting, value);
		self
	}

	/// Set the `onwaiting` attribute.
	fn set_onwaiting<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GlobalEventAttributes::Onwaiting, value);
	}

	/// Get the `onwaiting` attribute.
	fn onwaiting(&self) -> Option<&dyn Value> {
		self.get_attr(GlobalEventAttributes::Onwaiting)
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum GraphicalEventAttributes {
	Onactivate,
	Onfocusin,
	Onfocusout,
}

impl GraphicalEventAttributes {
	pub fn as_str(&self) -> &'static str {
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
	fn set_attr<V>(&mut self, attr: GraphicalEventAttributes, value: V)
	where
		V: Value + 'static;
	
	fn get_attr(&self, attr: GraphicalEventAttributes) -> Option<&dyn Value>;
}

pub trait TagWithGraphicalEventAttributes: GraphicalEventAttributesSetter + Sized {

	/// Set the `onactivate` attribute.
	fn with_onactivate<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GraphicalEventAttributes::Onactivate, value);
		self
	}

	/// Set the `onactivate` attribute.
	fn set_onactivate<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GraphicalEventAttributes::Onactivate, value);
	}

	/// Get the `onactivate` attribute.
	fn onactivate(&self) -> Option<&dyn Value> {
		self.get_attr(GraphicalEventAttributes::Onactivate)
	}

	/// Set the `onfocusin` attribute.
	fn with_onfocusin<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GraphicalEventAttributes::Onfocusin, value);
		self
	}

	/// Set the `onfocusin` attribute.
	fn set_onfocusin<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GraphicalEventAttributes::Onfocusin, value);
	}

	/// Get the `onfocusin` attribute.
	fn onfocusin(&self) -> Option<&dyn Value> {
		self.get_attr(GraphicalEventAttributes::Onfocusin)
	}

	/// Set the `onfocusout` attribute.
	fn with_onfocusout<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(GraphicalEventAttributes::Onfocusout, value);
		self
	}

	/// Set the `onfocusout` attribute.
	fn set_onfocusout<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(GraphicalEventAttributes::Onfocusout, value);
	}

	/// Get the `onfocusout` attribute.
	fn onfocusout(&self) -> Option<&dyn Value> {
		self.get_attr(GraphicalEventAttributes::Onfocusout)
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum PresentationAttributes {
	AlignmentBaseline,
	BaselineShift,
	Clip,
	ClipPath,
	ClipRule,
	Color,
	ColorInterpolation,
	ColorInterpolationFilters,
	ColorProfile,
	ColorRendering,
	Cursor,
	Direction,
	Display,
	DominantBaseline,
	EnableBackground,
	Fill,
	FillOpacity,
	FillRule,
	Filter,
	FloodColor,
	FloodOpacity,
	FontFamily,
	FontSize,
	FontSizeAdjust,
	FontStretch,
	FontStyle,
	FontVariant,
	FontWeight,
	GlyphOrientationHorizontal,
	GlyphOrientationVertical,
	ImageRendering,
	Kerning,
	LetterSpacing,
	LightingColor,
	MarkerEnd,
	MarkerMid,
	MarkerStart,
	Mask,
	Opacity,
	Overflow,
	PointerEvents,
	ShapeRendering,
	StopColor,
	StopOpacity,
	Stroke,
	StrokeDasharray,
	StrokeDashoffset,
	StrokeLinecap,
	StrokeLinejoin,
	StrokeMiterlimit,
	StrokeOpacity,
	StrokeWidth,
	TextAnchor,
	TextDecoration,
	TextRendering,
	Transform,
	TransformOrigin,
	UnicodeBidi,
	VectorEffect,
	Visibility,
	WordSpacing,
	WritingMode,
}

impl PresentationAttributes {
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::AlignmentBaseline => "alignment-baseline",
			Self::BaselineShift => "baseline-shift",
			Self::Clip => "clip",
			Self::ClipPath => "clip-path",
			Self::ClipRule => "clip-rule",
			Self::Color => "color",
			Self::ColorInterpolation => "color-interpolation",
			Self::ColorInterpolationFilters => "color-interpolation-filters",
			Self::ColorProfile => "color-profile",
			Self::ColorRendering => "color-rendering",
			Self::Cursor => "cursor",
			Self::Direction => "direction",
			Self::Display => "display",
			Self::DominantBaseline => "dominant-baseline",
			Self::EnableBackground => "enable-background",
			Self::Fill => "fill",
			Self::FillOpacity => "fill-opacity",
			Self::FillRule => "fill-rule",
			Self::Filter => "filter",
			Self::FloodColor => "flood-color",
			Self::FloodOpacity => "flood-opacity",
			Self::FontFamily => "font-family",
			Self::FontSize => "font-size",
			Self::FontSizeAdjust => "font-size-adjust",
			Self::FontStretch => "font-stretch",
			Self::FontStyle => "font-style",
			Self::FontVariant => "font-variant",
			Self::FontWeight => "font-weight",
			Self::GlyphOrientationHorizontal => "glyph-orientation-horizontal",
			Self::GlyphOrientationVertical => "glyph-orientation-vertical",
			Self::ImageRendering => "image-rendering",
			Self::Kerning => "kerning",
			Self::LetterSpacing => "letter-spacing",
			Self::LightingColor => "lighting-color",
			Self::MarkerEnd => "marker-end",
			Self::MarkerMid => "marker-mid",
			Self::MarkerStart => "marker-start",
			Self::Mask => "mask",
			Self::Opacity => "opacity",
			Self::Overflow => "overflow",
			Self::PointerEvents => "pointer-events",
			Self::ShapeRendering => "shape-rendering",
			Self::StopColor => "stop-color",
			Self::StopOpacity => "stop-opacity",
			Self::Stroke => "stroke",
			Self::StrokeDasharray => "stroke-dasharray",
			Self::StrokeDashoffset => "stroke-dashoffset",
			Self::StrokeLinecap => "stroke-linecap",
			Self::StrokeLinejoin => "stroke-linejoin",
			Self::StrokeMiterlimit => "stroke-miterlimit",
			Self::StrokeOpacity => "stroke-opacity",
			Self::StrokeWidth => "stroke-width",
			Self::TextAnchor => "text-anchor",
			Self::TextDecoration => "text-decoration",
			Self::TextRendering => "text-rendering",
			Self::Transform => "transform",
			Self::TransformOrigin => "transform-origin",
			Self::UnicodeBidi => "unicode-bidi",
			Self::VectorEffect => "vector-effect",
			Self::Visibility => "visibility",
			Self::WordSpacing => "word-spacing",
			Self::WritingMode => "writing-mode",
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
	fn set_attr<V>(&mut self, attr: PresentationAttributes, value: V)
	where
		V: Value + 'static;
	
	fn get_attr(&self, attr: PresentationAttributes) -> Option<&dyn Value>;
}

pub trait TagWithPresentationAttributes: PresentationAttributesSetter + Sized {

	/// Set the `alignment-baseline` attribute.
	fn with_alignment_baseline<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::AlignmentBaseline, value);
		self
	}

	/// Set the `alignment-baseline` attribute.
	fn set_alignment_baseline<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::AlignmentBaseline, value);
	}

	/// Get the `alignment-baseline` attribute.
	fn alignment_baseline(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::AlignmentBaseline)
	}

	/// Set the `baseline-shift` attribute.
	fn with_baseline_shift<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::BaselineShift, value);
		self
	}

	/// Set the `baseline-shift` attribute.
	fn set_baseline_shift<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::BaselineShift, value);
	}

	/// Get the `baseline-shift` attribute.
	fn baseline_shift(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::BaselineShift)
	}

	/// Set the `clip` attribute.
	fn with_clip<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::Clip, value);
		self
	}

	/// Set the `clip` attribute.
	fn set_clip<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::Clip, value);
	}

	/// Get the `clip` attribute.
	fn clip(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::Clip)
	}

	/// Set the `clip-path` attribute.
	fn with_clip_path<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::ClipPath, value);
		self
	}

	/// Set the `clip-path` attribute.
	fn set_clip_path<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::ClipPath, value);
	}

	/// Get the `clip-path` attribute.
	fn clip_path(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::ClipPath)
	}

	/// Set the `clip-rule` attribute.
	fn with_clip_rule<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::ClipRule, value);
		self
	}

	/// Set the `clip-rule` attribute.
	fn set_clip_rule<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::ClipRule, value);
	}

	/// Get the `clip-rule` attribute.
	fn clip_rule(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::ClipRule)
	}

	/// Set the `color` attribute.
	fn with_color<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::Color, value);
		self
	}

	/// Set the `color` attribute.
	fn set_color<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::Color, value);
	}

	/// Get the `color` attribute.
	fn color(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::Color)
	}

	/// Set the `color-interpolation` attribute.
	fn with_color_interpolation<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::ColorInterpolation, value);
		self
	}

	/// Set the `color-interpolation` attribute.
	fn set_color_interpolation<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::ColorInterpolation, value);
	}

	/// Get the `color-interpolation` attribute.
	fn color_interpolation(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::ColorInterpolation)
	}

	/// Set the `color-interpolation-filters` attribute.
	fn with_color_interpolation_filters<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::ColorInterpolationFilters, value);
		self
	}

	/// Set the `color-interpolation-filters` attribute.
	fn set_color_interpolation_filters<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::ColorInterpolationFilters, value);
	}

	/// Get the `color-interpolation-filters` attribute.
	fn color_interpolation_filters(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::ColorInterpolationFilters)
	}

	/// Set the `color-profile` attribute.
	fn with_color_profile<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::ColorProfile, value);
		self
	}

	/// Set the `color-profile` attribute.
	fn set_color_profile<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::ColorProfile, value);
	}

	/// Get the `color-profile` attribute.
	fn color_profile(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::ColorProfile)
	}

	/// Set the `color-rendering` attribute.
	fn with_color_rendering<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::ColorRendering, value);
		self
	}

	/// Set the `color-rendering` attribute.
	fn set_color_rendering<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::ColorRendering, value);
	}

	/// Get the `color-rendering` attribute.
	fn color_rendering(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::ColorRendering)
	}

	/// Set the `cursor` attribute.
	fn with_cursor<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::Cursor, value);
		self
	}

	/// Set the `cursor` attribute.
	fn set_cursor<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::Cursor, value);
	}

	/// Get the `cursor` attribute.
	fn cursor(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::Cursor)
	}

	/// Set the `direction` attribute.
	fn with_direction<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::Direction, value);
		self
	}

	/// Set the `direction` attribute.
	fn set_direction<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::Direction, value);
	}

	/// Get the `direction` attribute.
	fn direction(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::Direction)
	}

	/// Set the `display` attribute.
	fn with_display<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::Display, value);
		self
	}

	/// Set the `display` attribute.
	fn set_display<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::Display, value);
	}

	/// Get the `display` attribute.
	fn display(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::Display)
	}

	/// Set the `dominant-baseline` attribute.
	fn with_dominant_baseline<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::DominantBaseline, value);
		self
	}

	/// Set the `dominant-baseline` attribute.
	fn set_dominant_baseline<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::DominantBaseline, value);
	}

	/// Get the `dominant-baseline` attribute.
	fn dominant_baseline(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::DominantBaseline)
	}

	/// Set the `enable-background` attribute.
	fn with_enable_background<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::EnableBackground, value);
		self
	}

	/// Set the `enable-background` attribute.
	fn set_enable_background<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::EnableBackground, value);
	}

	/// Get the `enable-background` attribute.
	fn enable_background(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::EnableBackground)
	}

	/// Set the `fill` attribute.
	fn with_fill<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::Fill, value);
		self
	}

	/// Set the `fill` attribute.
	fn set_fill<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::Fill, value);
	}

	/// Get the `fill` attribute.
	fn fill(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::Fill)
	}

	/// Set the `fill-opacity` attribute.
	fn with_fill_opacity<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::FillOpacity, value);
		self
	}

	/// Set the `fill-opacity` attribute.
	fn set_fill_opacity<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::FillOpacity, value);
	}

	/// Get the `fill-opacity` attribute.
	fn fill_opacity(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::FillOpacity)
	}

	/// Set the `fill-rule` attribute.
	fn with_fill_rule<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::FillRule, value);
		self
	}

	/// Set the `fill-rule` attribute.
	fn set_fill_rule<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::FillRule, value);
	}

	/// Get the `fill-rule` attribute.
	fn fill_rule(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::FillRule)
	}

	/// Set the `filter` attribute.
	fn with_filter<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::Filter, value);
		self
	}

	/// Set the `filter` attribute.
	fn set_filter<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::Filter, value);
	}

	/// Get the `filter` attribute.
	fn filter(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::Filter)
	}

	/// Set the `flood-color` attribute.
	fn with_flood_color<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::FloodColor, value);
		self
	}

	/// Set the `flood-color` attribute.
	fn set_flood_color<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::FloodColor, value);
	}

	/// Get the `flood-color` attribute.
	fn flood_color(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::FloodColor)
	}

	/// Set the `flood-opacity` attribute.
	fn with_flood_opacity<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::FloodOpacity, value);
		self
	}

	/// Set the `flood-opacity` attribute.
	fn set_flood_opacity<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::FloodOpacity, value);
	}

	/// Get the `flood-opacity` attribute.
	fn flood_opacity(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::FloodOpacity)
	}

	/// Set the `font-family` attribute.
	fn with_font_family<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::FontFamily, value);
		self
	}

	/// Set the `font-family` attribute.
	fn set_font_family<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::FontFamily, value);
	}

	/// Get the `font-family` attribute.
	fn font_family(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::FontFamily)
	}

	/// Set the `font-size` attribute.
	fn with_font_size<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::FontSize, value);
		self
	}

	/// Set the `font-size` attribute.
	fn set_font_size<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::FontSize, value);
	}

	/// Get the `font-size` attribute.
	fn font_size(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::FontSize)
	}

	/// Set the `font-size-adjust` attribute.
	fn with_font_size_adjust<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::FontSizeAdjust, value);
		self
	}

	/// Set the `font-size-adjust` attribute.
	fn set_font_size_adjust<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::FontSizeAdjust, value);
	}

	/// Get the `font-size-adjust` attribute.
	fn font_size_adjust(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::FontSizeAdjust)
	}

	/// Set the `font-stretch` attribute.
	fn with_font_stretch<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::FontStretch, value);
		self
	}

	/// Set the `font-stretch` attribute.
	fn set_font_stretch<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::FontStretch, value);
	}

	/// Get the `font-stretch` attribute.
	fn font_stretch(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::FontStretch)
	}

	/// Set the `font-style` attribute.
	fn with_font_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::FontStyle, value);
		self
	}

	/// Set the `font-style` attribute.
	fn set_font_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::FontStyle, value);
	}

	/// Get the `font-style` attribute.
	fn font_style(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::FontStyle)
	}

	/// Set the `font-variant` attribute.
	fn with_font_variant<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::FontVariant, value);
		self
	}

	/// Set the `font-variant` attribute.
	fn set_font_variant<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::FontVariant, value);
	}

	/// Get the `font-variant` attribute.
	fn font_variant(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::FontVariant)
	}

	/// Set the `font-weight` attribute.
	fn with_font_weight<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::FontWeight, value);
		self
	}

	/// Set the `font-weight` attribute.
	fn set_font_weight<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::FontWeight, value);
	}

	/// Get the `font-weight` attribute.
	fn font_weight(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::FontWeight)
	}

	/// Set the `glyph-orientation-horizontal` attribute.
	fn with_glyph_orientation_horizontal<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::GlyphOrientationHorizontal, value);
		self
	}

	/// Set the `glyph-orientation-horizontal` attribute.
	fn set_glyph_orientation_horizontal<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::GlyphOrientationHorizontal, value);
	}

	/// Get the `glyph-orientation-horizontal` attribute.
	fn glyph_orientation_horizontal(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::GlyphOrientationHorizontal)
	}

	/// Set the `glyph-orientation-vertical` attribute.
	fn with_glyph_orientation_vertical<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::GlyphOrientationVertical, value);
		self
	}

	/// Set the `glyph-orientation-vertical` attribute.
	fn set_glyph_orientation_vertical<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::GlyphOrientationVertical, value);
	}

	/// Get the `glyph-orientation-vertical` attribute.
	fn glyph_orientation_vertical(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::GlyphOrientationVertical)
	}

	/// Set the `image-rendering` attribute.
	fn with_image_rendering<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::ImageRendering, value);
		self
	}

	/// Set the `image-rendering` attribute.
	fn set_image_rendering<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::ImageRendering, value);
	}

	/// Get the `image-rendering` attribute.
	fn image_rendering(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::ImageRendering)
	}

	/// Set the `kerning` attribute.
	fn with_kerning<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::Kerning, value);
		self
	}

	/// Set the `kerning` attribute.
	fn set_kerning<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::Kerning, value);
	}

	/// Get the `kerning` attribute.
	fn kerning(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::Kerning)
	}

	/// Set the `letter-spacing` attribute.
	fn with_letter_spacing<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::LetterSpacing, value);
		self
	}

	/// Set the `letter-spacing` attribute.
	fn set_letter_spacing<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::LetterSpacing, value);
	}

	/// Get the `letter-spacing` attribute.
	fn letter_spacing(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::LetterSpacing)
	}

	/// Set the `lighting-color` attribute.
	fn with_lighting_color<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::LightingColor, value);
		self
	}

	/// Set the `lighting-color` attribute.
	fn set_lighting_color<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::LightingColor, value);
	}

	/// Get the `lighting-color` attribute.
	fn lighting_color(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::LightingColor)
	}

	/// Set the `marker-end` attribute.
	fn with_marker_end<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::MarkerEnd, value);
		self
	}

	/// Set the `marker-end` attribute.
	fn set_marker_end<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::MarkerEnd, value);
	}

	/// Get the `marker-end` attribute.
	fn marker_end(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::MarkerEnd)
	}

	/// Set the `marker-mid` attribute.
	fn with_marker_mid<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::MarkerMid, value);
		self
	}

	/// Set the `marker-mid` attribute.
	fn set_marker_mid<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::MarkerMid, value);
	}

	/// Get the `marker-mid` attribute.
	fn marker_mid(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::MarkerMid)
	}

	/// Set the `marker-start` attribute.
	fn with_marker_start<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::MarkerStart, value);
		self
	}

	/// Set the `marker-start` attribute.
	fn set_marker_start<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::MarkerStart, value);
	}

	/// Get the `marker-start` attribute.
	fn marker_start(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::MarkerStart)
	}

	/// Set the `mask` attribute.
	fn with_mask<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::Mask, value);
		self
	}

	/// Set the `mask` attribute.
	fn set_mask<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::Mask, value);
	}

	/// Get the `mask` attribute.
	fn mask(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::Mask)
	}

	/// Set the `opacity` attribute.
	fn with_opacity<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::Opacity, value);
		self
	}

	/// Set the `opacity` attribute.
	fn set_opacity<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::Opacity, value);
	}

	/// Get the `opacity` attribute.
	fn opacity(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::Opacity)
	}

	/// Set the `overflow` attribute.
	fn with_overflow<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::Overflow, value);
		self
	}

	/// Set the `overflow` attribute.
	fn set_overflow<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::Overflow, value);
	}

	/// Get the `overflow` attribute.
	fn overflow(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::Overflow)
	}

	/// Set the `pointer-events` attribute.
	fn with_pointer_events<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::PointerEvents, value);
		self
	}

	/// Set the `pointer-events` attribute.
	fn set_pointer_events<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::PointerEvents, value);
	}

	/// Get the `pointer-events` attribute.
	fn pointer_events(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::PointerEvents)
	}

	/// Set the `shape-rendering` attribute.
	fn with_shape_rendering<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::ShapeRendering, value);
		self
	}

	/// Set the `shape-rendering` attribute.
	fn set_shape_rendering<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::ShapeRendering, value);
	}

	/// Get the `shape-rendering` attribute.
	fn shape_rendering(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::ShapeRendering)
	}

	/// Set the `stop-color` attribute.
	fn with_stop_color<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::StopColor, value);
		self
	}

	/// Set the `stop-color` attribute.
	fn set_stop_color<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::StopColor, value);
	}

	/// Get the `stop-color` attribute.
	fn stop_color(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::StopColor)
	}

	/// Set the `stop-opacity` attribute.
	fn with_stop_opacity<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::StopOpacity, value);
		self
	}

	/// Set the `stop-opacity` attribute.
	fn set_stop_opacity<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::StopOpacity, value);
	}

	/// Get the `stop-opacity` attribute.
	fn stop_opacity(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::StopOpacity)
	}

	/// Set the `stroke` attribute.
	fn with_stroke<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::Stroke, value);
		self
	}

	/// Set the `stroke` attribute.
	fn set_stroke<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::Stroke, value);
	}

	/// Get the `stroke` attribute.
	fn stroke(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::Stroke)
	}

	/// Set the `stroke-dasharray` attribute.
	fn with_stroke_dasharray<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::StrokeDasharray, value);
		self
	}

	/// Set the `stroke-dasharray` attribute.
	fn set_stroke_dasharray<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::StrokeDasharray, value);
	}

	/// Get the `stroke-dasharray` attribute.
	fn stroke_dasharray(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::StrokeDasharray)
	}

	/// Set the `stroke-dashoffset` attribute.
	fn with_stroke_dashoffset<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::StrokeDashoffset, value);
		self
	}

	/// Set the `stroke-dashoffset` attribute.
	fn set_stroke_dashoffset<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::StrokeDashoffset, value);
	}

	/// Get the `stroke-dashoffset` attribute.
	fn stroke_dashoffset(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::StrokeDashoffset)
	}

	/// Set the `stroke-linecap` attribute.
	fn with_stroke_linecap<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::StrokeLinecap, value);
		self
	}

	/// Set the `stroke-linecap` attribute.
	fn set_stroke_linecap<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::StrokeLinecap, value);
	}

	/// Get the `stroke-linecap` attribute.
	fn stroke_linecap(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::StrokeLinecap)
	}

	/// Set the `stroke-linejoin` attribute.
	fn with_stroke_linejoin<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::StrokeLinejoin, value);
		self
	}

	/// Set the `stroke-linejoin` attribute.
	fn set_stroke_linejoin<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::StrokeLinejoin, value);
	}

	/// Get the `stroke-linejoin` attribute.
	fn stroke_linejoin(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::StrokeLinejoin)
	}

	/// Set the `stroke-miterlimit` attribute.
	fn with_stroke_miterlimit<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::StrokeMiterlimit, value);
		self
	}

	/// Set the `stroke-miterlimit` attribute.
	fn set_stroke_miterlimit<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::StrokeMiterlimit, value);
	}

	/// Get the `stroke-miterlimit` attribute.
	fn stroke_miterlimit(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::StrokeMiterlimit)
	}

	/// Set the `stroke-opacity` attribute.
	fn with_stroke_opacity<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::StrokeOpacity, value);
		self
	}

	/// Set the `stroke-opacity` attribute.
	fn set_stroke_opacity<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::StrokeOpacity, value);
	}

	/// Get the `stroke-opacity` attribute.
	fn stroke_opacity(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::StrokeOpacity)
	}

	/// Set the `stroke-width` attribute.
	fn with_stroke_width<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::StrokeWidth, value);
		self
	}

	/// Set the `stroke-width` attribute.
	fn set_stroke_width<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::StrokeWidth, value);
	}

	/// Get the `stroke-width` attribute.
	fn stroke_width(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::StrokeWidth)
	}

	/// Set the `text-anchor` attribute.
	fn with_text_anchor<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::TextAnchor, value);
		self
	}

	/// Set the `text-anchor` attribute.
	fn set_text_anchor<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::TextAnchor, value);
	}

	/// Get the `text-anchor` attribute.
	fn text_anchor(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::TextAnchor)
	}

	/// Set the `text-decoration` attribute.
	fn with_text_decoration<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::TextDecoration, value);
		self
	}

	/// Set the `text-decoration` attribute.
	fn set_text_decoration<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::TextDecoration, value);
	}

	/// Get the `text-decoration` attribute.
	fn text_decoration(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::TextDecoration)
	}

	/// Set the `text-rendering` attribute.
	fn with_text_rendering<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::TextRendering, value);
		self
	}

	/// Set the `text-rendering` attribute.
	fn set_text_rendering<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::TextRendering, value);
	}

	/// Get the `text-rendering` attribute.
	fn text_rendering(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::TextRendering)
	}

	/// Set the `transform` attribute.
	fn with_transform<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::Transform, value);
		self
	}

	/// Set the `transform` attribute.
	fn set_transform<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::Transform, value);
	}

	/// Get the `transform` attribute.
	fn transform(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::Transform)
	}

	/// Set the `transform-origin` attribute.
	fn with_transform_origin<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::TransformOrigin, value);
		self
	}

	/// Set the `transform-origin` attribute.
	fn set_transform_origin<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::TransformOrigin, value);
	}

	/// Get the `transform-origin` attribute.
	fn transform_origin(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::TransformOrigin)
	}

	/// Set the `unicode-bidi` attribute.
	fn with_unicode_bidi<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::UnicodeBidi, value);
		self
	}

	/// Set the `unicode-bidi` attribute.
	fn set_unicode_bidi<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::UnicodeBidi, value);
	}

	/// Get the `unicode-bidi` attribute.
	fn unicode_bidi(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::UnicodeBidi)
	}

	/// Set the `vector-effect` attribute.
	fn with_vector_effect<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::VectorEffect, value);
		self
	}

	/// Set the `vector-effect` attribute.
	fn set_vector_effect<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::VectorEffect, value);
	}

	/// Get the `vector-effect` attribute.
	fn vector_effect(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::VectorEffect)
	}

	/// Set the `visibility` attribute.
	fn with_visibility<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::Visibility, value);
		self
	}

	/// Set the `visibility` attribute.
	fn set_visibility<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::Visibility, value);
	}

	/// Get the `visibility` attribute.
	fn visibility(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::Visibility)
	}

	/// Set the `word-spacing` attribute.
	fn with_word_spacing<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::WordSpacing, value);
		self
	}

	/// Set the `word-spacing` attribute.
	fn set_word_spacing<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::WordSpacing, value);
	}

	/// Get the `word-spacing` attribute.
	fn word_spacing(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::WordSpacing)
	}

	/// Set the `writing-mode` attribute.
	fn with_writing_mode<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::WritingMode, value);
		self
	}

	/// Set the `writing-mode` attribute.
	fn set_writing_mode<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(PresentationAttributes::WritingMode, value);
	}

	/// Get the `writing-mode` attribute.
	fn writing_mode(&self) -> Option<&dyn Value> {
		self.get_attr(PresentationAttributes::WritingMode)
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum StyleAttributes {
	Class,
	Style,
}

impl StyleAttributes {
	pub fn as_str(&self) -> &'static str {
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
	fn set_attr<V>(&mut self, attr: StyleAttributes, value: V)
	where
		V: Value + 'static;
	
	fn get_attr(&self, attr: StyleAttributes) -> Option<&dyn Value>;
}

pub trait TagWithStyleAttributes: StyleAttributesSetter + Sized {

	/// Set the `class` attribute.
	fn with_class<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(StyleAttributes::Class, value);
		self
	}

	/// Set the `class` attribute.
	fn set_class<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(StyleAttributes::Class, value);
	}

	/// Get the `class` attribute.
	fn class(&self) -> Option<&dyn Value> {
		self.get_attr(StyleAttributes::Class)
	}

	/// Set the `style` attribute.
	fn with_style<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(StyleAttributes::Style, value);
		self
	}

	/// Set the `style` attribute.
	fn set_style<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(StyleAttributes::Style, value);
	}

	/// Get the `style` attribute.
	fn style(&self) -> Option<&dyn Value> {
		self.get_attr(StyleAttributes::Style)
	}
}

#[allow(clippy::enum_variant_names)]
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
	pub fn as_str(&self) -> &'static str {
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
	fn set_attr<V>(&mut self, attr: TransferFunctionAttributes, value: V)
	where
		V: Value + 'static;
	
	fn get_attr(&self, attr: TransferFunctionAttributes) -> Option<&dyn Value>;
}

pub trait TagWithTransferFunctionAttributes: TransferFunctionAttributesSetter + Sized {

	/// Set the `amplitude` attribute.
	fn with_amplitude<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TransferFunctionAttributes::Amplitude, value);
		self
	}

	/// Set the `amplitude` attribute.
	fn set_amplitude<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TransferFunctionAttributes::Amplitude, value);
	}

	/// Get the `amplitude` attribute.
	fn amplitude(&self) -> Option<&dyn Value> {
		self.get_attr(TransferFunctionAttributes::Amplitude)
	}

	/// Set the `exponent` attribute.
	fn with_exponent<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TransferFunctionAttributes::Exponent, value);
		self
	}

	/// Set the `exponent` attribute.
	fn set_exponent<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TransferFunctionAttributes::Exponent, value);
	}

	/// Get the `exponent` attribute.
	fn exponent(&self) -> Option<&dyn Value> {
		self.get_attr(TransferFunctionAttributes::Exponent)
	}

	/// Set the `intercept` attribute.
	fn with_intercept<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TransferFunctionAttributes::Intercept, value);
		self
	}

	/// Set the `intercept` attribute.
	fn set_intercept<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TransferFunctionAttributes::Intercept, value);
	}

	/// Get the `intercept` attribute.
	fn intercept(&self) -> Option<&dyn Value> {
		self.get_attr(TransferFunctionAttributes::Intercept)
	}

	/// Set the `offset` attribute.
	fn with_offset<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TransferFunctionAttributes::Offset, value);
		self
	}

	/// Set the `offset` attribute.
	fn set_offset<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TransferFunctionAttributes::Offset, value);
	}

	/// Get the `offset` attribute.
	fn offset(&self) -> Option<&dyn Value> {
		self.get_attr(TransferFunctionAttributes::Offset)
	}

	/// Set the `slope` attribute.
	fn with_slope<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TransferFunctionAttributes::Slope, value);
		self
	}

	/// Set the `slope` attribute.
	fn set_slope<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TransferFunctionAttributes::Slope, value);
	}

	/// Get the `slope` attribute.
	fn slope(&self) -> Option<&dyn Value> {
		self.get_attr(TransferFunctionAttributes::Slope)
	}

	/// Set the `tableValues` attribute.
	fn with_table_values<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TransferFunctionAttributes::TableValues, value);
		self
	}

	/// Set the `tableValues` attribute.
	fn set_table_values<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TransferFunctionAttributes::TableValues, value);
	}

	/// Get the `tableValues` attribute.
	fn table_values(&self) -> Option<&dyn Value> {
		self.get_attr(TransferFunctionAttributes::TableValues)
	}

	/// Set the `type` attribute.
	fn with_ty<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(TransferFunctionAttributes::Type, value);
		self
	}

	/// Set the `type` attribute.
	fn set_ty<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(TransferFunctionAttributes::Type, value);
	}

	/// Get the `type` attribute.
	fn ty(&self) -> Option<&dyn Value> {
		self.get_attr(TransferFunctionAttributes::Type)
	}
}

#[allow(clippy::enum_variant_names)]
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
	pub fn as_str(&self) -> &'static str {
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
	fn set_attr<V>(&mut self, attr: XLinkAttributes, value: V)
	where
		V: Value + 'static;
	
	fn get_attr(&self, attr: XLinkAttributes) -> Option<&dyn Value>;
}

pub trait TagWithXLinkAttributes: XLinkAttributesSetter + Sized {

	/// Set the `xlink:actuate` attribute.
	fn with_xlink_actuate<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(XLinkAttributes::XlinkActuate, value);
		self
	}

	/// Set the `xlink:actuate` attribute.
	fn set_xlink_actuate<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(XLinkAttributes::XlinkActuate, value);
	}

	/// Get the `xlink:actuate` attribute.
	fn xlink_actuate(&self) -> Option<&dyn Value> {
		self.get_attr(XLinkAttributes::XlinkActuate)
	}

	/// Set the `xlink:arcrole` attribute.
	fn with_xlink_arcrole<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(XLinkAttributes::XlinkArcrole, value);
		self
	}

	/// Set the `xlink:arcrole` attribute.
	fn set_xlink_arcrole<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(XLinkAttributes::XlinkArcrole, value);
	}

	/// Get the `xlink:arcrole` attribute.
	fn xlink_arcrole(&self) -> Option<&dyn Value> {
		self.get_attr(XLinkAttributes::XlinkArcrole)
	}

	/// Set the `xlink:href` attribute.
	fn with_xlink_href<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(XLinkAttributes::XlinkHref, value);
		self
	}

	/// Set the `xlink:href` attribute.
	fn set_xlink_href<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(XLinkAttributes::XlinkHref, value);
	}

	/// Get the `xlink:href` attribute.
	fn xlink_href(&self) -> Option<&dyn Value> {
		self.get_attr(XLinkAttributes::XlinkHref)
	}

	/// Set the `xlink:role` attribute.
	fn with_xlink_role<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(XLinkAttributes::XlinkRole, value);
		self
	}

	/// Set the `xlink:role` attribute.
	fn set_xlink_role<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(XLinkAttributes::XlinkRole, value);
	}

	/// Get the `xlink:role` attribute.
	fn xlink_role(&self) -> Option<&dyn Value> {
		self.get_attr(XLinkAttributes::XlinkRole)
	}

	/// Set the `xlink:show` attribute.
	fn with_xlink_show<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(XLinkAttributes::XlinkShow, value);
		self
	}

	/// Set the `xlink:show` attribute.
	fn set_xlink_show<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(XLinkAttributes::XlinkShow, value);
	}

	/// Get the `xlink:show` attribute.
	fn xlink_show(&self) -> Option<&dyn Value> {
		self.get_attr(XLinkAttributes::XlinkShow)
	}

	/// Set the `xlink:title` attribute.
	fn with_xlink_title<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(XLinkAttributes::XlinkTitle, value);
		self
	}

	/// Set the `xlink:title` attribute.
	fn set_xlink_title<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(XLinkAttributes::XlinkTitle, value);
	}

	/// Get the `xlink:title` attribute.
	fn xlink_title(&self) -> Option<&dyn Value> {
		self.get_attr(XLinkAttributes::XlinkTitle)
	}

	/// Set the `xlink:type` attribute.
	fn with_xlink_type<V>(mut self, value: V) -> Self
	where
		V: Value + 'static
	{
		self.set_attr(XLinkAttributes::XlinkType, value);
		self
	}

	/// Set the `xlink:type` attribute.
	fn set_xlink_type<V>(&mut self, value: V)
	where
		V: Value + 'static
	{
		self.set_attr(XLinkAttributes::XlinkType, value);
	}

	/// Get the `xlink:type` attribute.
	fn xlink_type(&self) -> Option<&dyn Value> {
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
