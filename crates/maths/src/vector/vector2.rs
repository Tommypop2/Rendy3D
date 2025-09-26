use core::ops::{Add, AddAssign, Mul, MulAssign};

use derive_more::{Add, Mul, Sub};

use crate::{
	traits::{float::Float, num::Num},
	vector::vector3::Vector3,
};

#[derive(Debug, Clone, Copy, PartialEq, Sub, Add, Mul)]
pub struct Vector2<T> {
	pub x: T,
	pub y: T,
}
impl<T> Vector2<T> {
	pub const fn new(x: T, y: T) -> Self {
		Self { x, y }
	}
}
impl<T: Mul<Output = T> + Add<Output = T> + Copy> Vector2<T> {
	pub fn magnitude_squared(&self) -> T {
		self.x * self.x + self.y * self.y
	}
	pub fn as_tuple(&self) -> (T, T) {
		(self.x, self.y)
	}
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy + Float> Vector2<T> {
	pub fn magnitude(&self) -> T {
		self.magnitude_squared().sqrt()
	}
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> AddAssign<Vector2<T>> for Vector2<T> {
	fn add_assign(&mut self, rhs: Vector2<T>) {
		*self = Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl<T> From<Vector3<T>> for Vector2<T> {
	fn from(value: Vector3<T>) -> Self {
		Self::new(value.x, value.y)
	}
}

impl<T> MulAssign<T> for Vector2<T>
where
	T: Num,
{
	fn mul_assign(&mut self, rhs: T) {
		self.x *= rhs;
		self.y *= rhs;
	}
}
// impl<T> Mul<T> for Vector2<T>
// where
// 	T: Num,
// {
// 	type Output = Vector2<T>;
// 	fn mul(mut self, rhs: T) -> Self::Output {
// 		self *= rhs;
// 		self
// 	}
// }
