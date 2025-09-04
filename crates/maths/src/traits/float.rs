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
#[cfg(feature = "std")]
macro_rules! implement_float {
	($T: ident) => {
		impl Float for $T {
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
#[cfg(not(feature = "std"))]
macro_rules! implement_float {
	($T: ident) => {
		use micromath::F32Ext;
		impl Float for $T {
			fn sqrt(self) -> Self {
				<Self as F32Ext>::sqrt(self)
			}
			fn sin(self) -> Self {
				<Self as F32Ext>::sin(self)
			}
			fn cos(self) -> Self {
				<Self as F32Ext>::cos(self)
			}
			fn tan(self) -> Self {
				<Self as F32Ext>::tan(self)
			}
			fn asin(self) -> Self {
				<Self as F32Ext>::asin(self)
			}
			fn acos(self) -> Self {
				<Self as F32Ext>::acos(self)
			}
			fn atan(self) -> Self {
				<Self as F32Ext>::atan(self)
			}
		}
	};
}
// implement_float!(f16);
implement_float!(f32);
#[cfg(feature = "std")]
implement_float!(f64);
#[cfg(not(feature = "std"))]
impl Float for f64 {
	fn sqrt(self) -> Self {
		F32Ext::sqrt(self as f32) as f64
	}
	// Trig
	fn sin(self) -> Self {
		F32Ext::sin(self as f32) as f64
	}
	fn cos(self) -> Self {
		F32Ext::cos(self as f32) as f64
	}
	fn tan(self) -> Self {
		F32Ext::tan(self as f32) as f64
	}
	fn asin(self) -> Self {
		F32Ext::asin(self as f32) as f64
	}
	fn acos(self) -> Self {
		F32Ext::acos(self as f32) as f64
	}
	fn atan(self) -> Self {
		F32Ext::atan(self as f32) as f64
	}
}
// implement_float!(f128);
