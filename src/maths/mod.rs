pub mod vector;

pub trait Float {
	fn sqrt(self) -> Self;
	fn acos(self) -> Self;
}
impl Float for f32 {
	fn sqrt(self) -> Self {
		f32::sqrt(self)
	}
	fn acos(self) -> Self {
		f32::acos(self)
	}
}
impl Float for f64 {
	fn sqrt(self) -> Self {
		f64::sqrt(self)
	}
	fn acos(self) -> Self {
		f64::acos(self)
	}
}
