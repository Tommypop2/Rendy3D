use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

// Base number trait
pub trait Num:
	Sized
	+ PartialEq
	+ Mul<Output = Self>
	+ MulAssign
	+ Div<Output = Self>
	+ DivAssign
	+ Add<Output = Self>
	+ AddAssign
	+ Sub<Output = Self>
	+ SubAssign
	+ Copy
	+ Clone
{
	fn one() -> Self;
	fn zero() -> Self;
}

impl Num for f32 {
	fn one() -> Self {
		1.0
	}
	fn zero() -> Self {
		0.0
	}
}
impl Num for f64 {
	fn one() -> Self {
		1.0
	}
	fn zero() -> Self {
		0.0
	}
}
impl Num for i32 {
	fn one() -> Self {
		1
	}
	fn zero() -> Self {
		0
	}
}
impl Num for i64 {
	fn one() -> Self {
		1
	}
	fn zero() -> Self {
		0
	}
}
