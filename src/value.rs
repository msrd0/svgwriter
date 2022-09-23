use std::{
	borrow::Cow,
	fmt::Debug,
	num::{
		NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize,
		NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize
	}
};

/// This trait implements an SVG attribute value.
pub trait Value: Debug {
	/// Convert this value to a string.
	fn value_to_string(&self, pretty: bool) -> String;
}

impl<V: Value + ?Sized> Value for &V {
	fn value_to_string(&self, pretty: bool) -> String {
		V::value_to_string(self, pretty)
	}
}

macro_rules! impl_str_to_owned {
	($($ty:ty),*) => {
		$(impl Value for $ty {
			fn value_to_string(&self, _pretty: bool) -> String {
				let s: &str = self.as_ref();
				s.to_owned()
			}
		})*
	};
}

impl_str_to_owned!(String, str, Cow<'_, str>);

macro_rules! impl_display {
	($($ty:ty),*) => {
		$(impl Value for $ty {
			fn value_to_string(&self, _pretty: bool) -> String {
				format!("{self}")
			}
		})*
	};
}

impl_display! {
	u8, u16, u32, u64, u128, usize,
	i8, i16, i32, i64, i128, isize,
	NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
	NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
}

macro_rules! impl_float {
	($($ty:ty),*) => {
		$(impl Value for $ty {
			fn value_to_string(&self, _pretty: bool) -> String {
				let mut d = self % 1e-6;
				if d >= 5e-7 {
					d -= 1e-6;
				} else if d <= -5e-7 {
					d += 1e-6;
				}
				format!("{}", self - d)
			}
		})*
	}
}

impl_float!(f32, f64);
