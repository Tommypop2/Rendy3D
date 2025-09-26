use core::ops::MulAssign;

use derive_more::{Add, Mul, Sub};

use crate::{traits::signed::Signed, vector::vector3::Vector3};
#[derive(Default, Clone, Debug, Mul, Sub, Add, Copy)]
pub struct Vector4<T> {
	pub x: T,
	pub y: T,
	pub z: T,
	pub w: T,
}

impl<T> Vector4<T> {
	pub const fn new(x: T, y: T, z: T, w: T) -> Self {
		Self { x, y, z, w }
	}
	pub fn as_tuple(self) -> (T, T, T, T) {
		(self.x, self.y, self.z, self.w)
	}
}
impl<T: Signed> Vector4<T> {
	pub fn dot(a: Vector4<T>, b: Vector4<T>) -> T {
		a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
	}
	pub fn dot_with(self, other: Self) -> T {
		Self::dot(self, other)
	}
}
impl<T> From<Vector3<T>> for Vector4<T>
where
	T: Default,
{
	fn from(value: Vector3<T>) -> Self {
		Self::new(value.x, value.y, value.z, Default::default())
	}
}
impl<T> MulAssign<T> for Vector4<T>
where
	T: MulAssign + Clone,
{
	fn mul_assign(&mut self, rhs: T) {
		self.x *= rhs.clone();
		self.y *= rhs.clone();
		self.z *= rhs.clone();
		self.w *= rhs;
	}
}
