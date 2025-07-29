use crate::traits::{num::Num, signed::Signed};

pub trait Float: Num + Signed {
	fn sqrt(self) -> Self;
	// Trig
	fn sin(self) -> Self;
	fn cos(self) -> Self;
	fn tan(self) -> Self;
	fn asin(self) -> Self;
	fn acos(self) -> Self;
	fn atan(self) -> Self;
}
macro_rules! implement_float {
	($x: ident) => {
		impl Float for $x {
			fn sqrt(self) -> Self {
				Self::sqrt(self)
			}
			fn sin(self) -> Self {
				Self::sin(self)
			}
			fn cos(self) -> Self {
				Self::cos(self)
			}
			fn tan(self) -> Self {
				Self::tan(self)
			}
			fn asin(self) -> Self {
				Self::asin(self)
			}
			fn acos(self) -> Self {
				Self::acos(self)
			}
			fn atan(self) -> Self {
				Self::atan(self)
			}
		}
	};
}

// implement_float!(f16);
implement_float!(f32);
implement_float!(f64);
// implement_float!(f128);
