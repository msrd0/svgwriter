use xmlwriter::XmlWriter;

mod graphic;
pub mod tags;

pub use graphic::Graphic;

pub trait Tag {
	fn write_to(&self, w: &mut XmlWriter, pretty: bool);
}

impl Tag for String {
	fn write_to(&self, w: &mut XmlWriter, _pretty: bool) {
		w.write_text(self)
	}
}

impl Tag for &'static str {
	fn write_to(&self, w: &mut XmlWriter, _pretty: bool) {
		w.write_text(self)
	}
}

impl<T> Tag for Box<T>
where
	T: ?Sized + Tag
{
	fn write_to(&self, w: &mut XmlWriter, pretty: bool) {
		self.as_ref().write_to(w, pretty)
	}
}
