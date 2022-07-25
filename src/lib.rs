use xmlwriter::XmlWriter;

pub mod tags;

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
