use crate::Value;
use paste::paste;
use std::{
	borrow::Cow,
	fmt::{self, Display, Formatter}
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Number {
	Int(i64),
	UInt(u64),
	Float(f64)
}

impl Number {
	pub fn is_zero(self) -> bool {
		match self {
			Self::Int(i) => i == 0,
			Self::UInt(u) => u == 0,
			Self::Float(f) => f == 0.0
		}
	}
}

macro_rules! number_from {
	($($ty:ty),+ => $variant:ident) => {
		$(
			impl From<$ty> for Number {
				fn from(this: $ty) -> Self {
					Self::$variant(this.into())
				}
			}
		)+
	}
}

number_from!(i8, i16, i32, i64 => Int);
number_from!(u8, u16, u32, u64 => UInt);
number_from!(f32, f64 => Float);

impl Value for Number {
	fn value_to_string(&self, pretty: bool) -> String {
		match self {
			Self::Int(i) => i.value_to_string(pretty),
			Self::UInt(u) => u.value_to_string(pretty),
			Self::Float(f) => f.value_to_string(pretty)
		}
	}
}

trait Payload {
	fn format<'a>(&self, itoa: &'a mut itoa::Buffer) -> Cow<'a, str>;
}

impl Payload for Number {
	fn format<'a>(&self, itoa: &'a mut itoa::Buffer) -> Cow<'a, str> {
		match self {
			Self::Int(i) => itoa.format(*i).into(),
			Self::UInt(u) => itoa.format(*u).into(),
			Self::Float(f) => f.to_string().into()
		}
	}
}

impl Payload for bool {
	fn format<'a>(&self, _: &'a mut itoa::Buffer) -> Cow<'static, str> {
		match *self {
			true => "1",
			false => "0"
		}
		.into()
	}
}

macro_rules! commands {
	($(
		$ident:ident($($arg:ident: $arg_ty:ty),*) = $command:literal;
	)+) => {
		paste! {
			#[derive(Clone, Debug, PartialEq)]
			enum Command {
				$([<$ident:camel>] {$($arg: $arg_ty),*}),+
			}

			impl Display for Command {
				#[allow(unused_assignments, unused_mut, unused_variables)]
				fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
					let mut buf = itoa::Buffer::new();
					match self {
						$(Self::[<$ident:camel>] {$($arg),*} => {
							write!(f, "{}", $command)?;
							let mut first = true;
							$(
								if first {
									write!(f, ",")?;
								}
								write!(f, "{}", Payload::format($arg, &mut buf))?;
								first = false;
							)*
						}),+
					};
					Ok(())
				}
			}

			impl Data {
				$(
					#[allow(clippy::too_many_arguments)]
					pub fn $ident<$([<$arg:upper>]),*>(
						&mut self $(, $arg: [<$arg:upper>])*
					) -> &mut Self
					where
						$([<$arg:upper>]: Into<$arg_ty>),*
					{
						self.0.push(Command::[<$ident:camel>] {$($arg: $arg.into()),*});
						self
					}
				)*
			}
		}
	};
}

commands! {
	move_to(x: Number, y: Number) = 'M';
	move_by(dx: Number, dy: Number) = 'm';

	line_to(x: Number, y: Number) = 'L';
	line_by(x: Number, y: Number) = 'l';
	horiz_line_to(x: Number) = 'H';
	horiz_line_by(dx: Number) = 'h';
	vert_line_to(y: Number) = 'V';
	vert_line_by(dy: Number) = 'v';

	cubic_to(x1: Number, y1: Number, x2: Number, y2: Number, x: Number, y: Number) = 'C';
	cubic_by(dx1: Number, dy1: Number, dx2: Number, dy2: Number, dx: Number, dy: Number) = 'c';
	smooth_cubic_to(x2: Number, y2: Number, x: Number, y: Number) = 'S';
	smooth_cubic_by(dx2: Number, dy2: Number, dx: Number, dy: Number) = 's';

	quad_to(x1: Number, y1: Number, x: Number, y: Number) = 'Q';
	quad_by(dx1: Number, dy1: Number, dx: Number, dy: Number) = 'q';
	smooth_quad_to(x: Number, y: Number) = 'T';
	smooth_quad_by(dx: Number, dy: Number) = 't';

	arc_to(rx: Number, ry: Number, angle: bool, large: bool, sweep: bool, x: Number, y: Number) = 'A';
	arc_by(rx: Number, ry: Number, angle: bool, large: bool, sweep: bool, dx: Number, dy: Number) = 'a';

	close() = 'z';
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Data(Vec<Command>);

pub struct DisplayData<'a>(&'a Vec<Command>, bool);

impl Display for DisplayData<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let mut first = true;
		for cmd in self.0 {
			if first {
				first = false;
			} else if self.1 {
				f.write_str(" ")?;
			}
			Display::fmt(cmd, f)?;
		}
		Ok(())
	}
}

impl Value for Data {
	fn value_to_string(&self, pretty: bool) -> String {
		DisplayData(&self.0, pretty).to_string()
	}
}

impl Data {
	pub fn new() -> Self {
		Self::default()
	}
}
