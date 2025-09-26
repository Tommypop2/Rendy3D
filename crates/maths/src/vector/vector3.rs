use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::{
	traits::{float::Float, signed::Signed},
	vector::vector4::Vector4,
};

#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct Vector3<T> {
	pub x: T,
	pub y: T,
	pub z: T,
}
impl<T> Vector3<T> {
	pub const fn new(x: T, y: T, z: T) -> Self {
		Self { x, y, z }
	}
	pub fn as_tuple(self) -> (T, T, T) {
		(self.x, self.y, self.z)
	}
	pub fn map_components<U>(self, function: fn(value: T) -> U) -> Vector3<U> {
		Vector3::new(function(self.x), function(self.y), function(self.z))
	}
}

impl<T, U> From<[T; 3]> for Vector3<U>
where
	T: Clone + Into<U>,
{
	fn from(value: [T; 3]) -> Self {
		Self::new(
			value[0].clone().into(),
			value[1].clone().into(),
			value[2].clone().into(),
		)
	}
}
impl<T: Signed> Vector3<T> {
	pub fn cross(a: &Self, b: &Self) -> Self {
		Self {
			x: a.y * b.z - b.y * a.z,
			y: -(a.x * b.z - b.x * a.z),
			z: a.x * b.y - b.x * a.y,
		}
	}
	pub fn dot(a: &Self, b: &Self) -> T {
		a.x * b.x + a.y * b.y + a.z * b.z
	}
	pub fn cross_with(&self, b: &Self) -> Self {
		Self::cross(self, b)
	}
	pub fn dot_with(&self, b: &Self) -> T {
		Self::dot(self, b)
	}
	pub fn magnitude_squared(&self) -> T {
		self.x * self.x + self.y * self.y + self.z * self.z
	}
}
impl<T: Float> Vector3<T> {
	pub fn to_homogenous(self) -> Vector4<T> {
		Vector4::new(self.x, self.y, self.z, T::one())
	}
	pub fn from_homogenous(c: Vector4<T>) -> Self {
		Self::new(c.x / c.w, c.y / c.w, c.z / c.w)
	}
	pub fn magnitude(&self) -> T {
		self.magnitude_squared().sqrt()
	}
	pub fn angle(&self, b: &Self) -> T {
		let cos_theta = Self::dot(self, b) / (self.magnitude() * b.magnitude());
		T::acos(cos_theta)
	}
	pub fn normalize(&mut self) {
		let m = self.magnitude();
		self.x /= m;
		self.y /= m;
		self.z /= m;
	}
	pub fn normalized(mut self) -> Self {
		self.normalize();
		self
	}
}
// Add
impl<T: AddAssign + Copy> Add<Self> for Vector3<T> {
	type Output = Self;

	fn add(mut self, rhs: Self) -> Self::Output {
		self += rhs;
		self
	}
}
impl<T: AddAssign + Copy> AddAssign<Self> for Vector3<T> {
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
		self.z += rhs.z;
	}
}

// Subtract
impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Copy> Sub<Self>
	for Vector3<T>
{
	type Output = Self;

	fn sub(mut self, rhs: Self) -> Self::Output {
		self -= rhs;
		self
	}
}
impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Copy>
	SubAssign<Self> for Vector3<T>
{
	fn sub_assign(&mut self, rhs: Self) {
		*self = Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z,
		}
	}
}

// Multiply
impl<T: Mul<Output = T> + Copy> MulAssign<T> for Vector3<T> {
	fn mul_assign(&mut self, rhs: T) {
		*self = Self {
			x: self.x * rhs,
			y: self.y * rhs,
			z: self.z * rhs,
		}
	}
}
impl<T: Mul<Output = T> + Copy> Mul<T> for Vector3<T> {
	type Output = Self;
	fn mul(mut self, rhs: T) -> Self {
		self *= rhs;
		self
	}
}
#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn addition() {
		let vec1 = Vector3::new(1, 2, 3);
		let vec2 = Vector3::new(3, 2, 1);
		assert_eq!(vec1 + vec2, Vector3::new(4, 4, 4))
	}
	#[test]
	fn subtraction() {
		let vec1 = Vector3::new(1, 2, 3);
		let vec2 = Vector3::new(3, 2, 1);
		assert_eq!(vec1 - vec2, Vector3::new(-2, 0, 2))
	}
	#[test]
	fn multiplication() {
		let vec1 = Vector3::new(1, 2, 3);
		assert_eq!(vec1 * 5, Vector3::new(5, 10, 15))
	}
	#[test]
	fn dot() {
		let vec1 = Vector3::new(1, 2, 3);
		let vec2 = Vector3::new(3, 2, 1);
		assert_eq!(vec1.dot_with(&vec2), 3 + 4 + 3)
	}
	#[test]
	fn cross() {
		let vec1 = Vector3::new(1, 2, 3);
		let vec2 = Vector3::new(3, 2, 1);
		assert_eq!(
			vec1.cross_with(&vec2),
			Vector3::new(2 - 2 * 3, -(1 - 3 * 3), 2 - 2 * 3)
		)
	}
	#[test]
	fn magnitude() {
		let vec1 = Vector3::new(1.0, 2.0, 3.0);
		assert_eq!(vec1.magnitude_squared(), 14.0);
		assert_eq!(vec1.magnitude(), f64::sqrt(14.0))
	}
	#[test]
	fn angle() {
		let vec1 = Vector3::new(1.0, 2.0, 3.0);
		let vec2 = Vector3::new(3.0, 2.0, 1.0);
		assert_eq!(vec1.angle(&vec2), 0.7751933733103613);
	}
}
