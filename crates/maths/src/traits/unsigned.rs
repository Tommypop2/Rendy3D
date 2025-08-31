use crate::traits::num::Num;

pub trait Unsigned: Num {}

macro_rules! implement_unsigned {
	($x: ident) => {
		impl Unsigned for $x {}
	};
}

implement_unsigned!(usize);
