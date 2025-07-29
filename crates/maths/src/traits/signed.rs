use std::ops::Neg;

use crate::traits::num::Num;

pub trait Signed: Num + Neg<Output = Self> {}

macro_rules! implement_signed {
	($x: ident) => {
		impl Signed for $x {}
	};
}

implement_signed!(i8);
implement_signed!(i16);
implement_signed!(i32);
implement_signed!(i64);
implement_signed!(i128);

implement_signed!(f32);
implement_signed!(f64);
