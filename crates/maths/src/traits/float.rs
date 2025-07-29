use crate::traits::num::Num;

pub trait Float: Num {
	fn sqrt(self) -> Self;
	// Trig
	fn sin(self) -> Self;
	fn cos(self) -> Self;
	fn tan(self) -> Self;
	fn asin(self) -> Self;
	fn acos(self) -> Self;
	fn atan(self) -> Self;
}
impl Float for f32 {
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
impl Float for f64 {
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
