use super::{data::Number, Value};
use paste::paste;
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Transform(Vec<TransformFunction>);

macro_rules! transform {
	($($ident:ident($($arg:ident),+);)+) => {
		paste! {
			#[derive(Clone, Copy, Debug, PartialEq)]
			enum TransformFunction {
				$([<$ident:camel>] { $($arg: Number),+ }),+
			}

			impl Transform {
				$(
					pub fn [<$ident:lower>]<$([<$arg:upper>]),+>(
						mut self, $($arg: [<$arg:upper>]),+
					) -> Self
					where
						$([<$arg:upper>]: Into<Number>),+
					{
						self.0.push(TransformFunction::[<$ident:camel>] {
							$($arg: $arg.into()),+
						});
						self
					}
				)+
			}

			impl Display for TransformFunction {
				#[allow(unused_assignments)]
				fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
					match self {
						$(
							Self::[<$ident:camel>] { $($arg),+ } => {
								write!(f, concat!(stringify!($ident), "("))?;
								let mut first = true;
								$(
									if !first {
										write!(f, ",")?;
									}
									first = false;
									write!(f, "{}", $arg.value_to_string(false))?;
								)+
								write!(f, ")")?;
							}
						),+
					};
					Ok(())
				}
			}
		}
	};
}

transform! {
	translate(x, y);
	scale(x, y);
	skewX(x);
	skewY(y);
	rotate(deg);
}

impl Transform {
	pub const fn new() -> Self {
		Self(Vec::new())
	}
}

pub(crate) struct DisplayTransform<'a>(&'a [TransformFunction], bool);

impl Display for DisplayTransform<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		for (i, tf) in self.0.into_iter().enumerate() {
			if self.1 && i > 0 {
				write!(f, " ")?;
			}
			write!(f, "{tf}")?;
		}
		Ok(())
	}
}

impl Value for Transform {
	fn value_to_string(&self, pretty: bool) -> String {
		DisplayTransform(&self.0, pretty).to_string()
	}
}
