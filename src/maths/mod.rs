pub mod vector3;
pub mod vector2;

pub trait SqrtAcos {
	fn sqrt(self) -> Self;
	fn acos(self) -> Self;
}
impl SqrtAcos for f32 {
	fn sqrt(self) -> Self {
		f32::sqrt(self)
	}
	fn acos(self) -> Self {
		f32::acos(self)
	}
}
impl SqrtAcos for f64 {
	fn sqrt(self) -> Self {
		f64::sqrt(self)
	}
	fn acos(self) -> Self {
		f64::acos(self)
	}
}
