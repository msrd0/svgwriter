use crate::xmlwriter::XmlWriter;
use std::borrow::Cow;

pub trait Tag {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool);
}

impl<T> Tag for Box<T>
where
	T: ?Sized + Tag
{
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		self.as_ref().write_to(w, pretty)
	}
}

macro_rules! impl_as_ref_str {
	($($ty:ty),*) => {
		$(impl Tag for $ty {
			fn write_to(&self, w: &mut XmlWriter, _pretty: bool) {
				let s: &str = self.as_ref();
				w.write_text(s);
			}
		})*
	};
}

impl_as_ref_str!(String, str, &'static str, Cow<'static, str>);
