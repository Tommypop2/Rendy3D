use std::ops::{Div, Mul, Neg, Sub};

use crate::{
	traits::{num::Num, signed::Signed},
	vector::vector2::Vector2,
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Matrix2<T> {
	x: Vector2<T>,
	y: Vector2<T>,
}

impl<T> Matrix2<T> {
	pub const fn new(x: Vector2<T>, y: Vector2<T>) -> Self {
		Self { x, y }
	}
}
impl<T> Matrix2<T>
where
	T: Num,
{
	pub fn identity() -> Self {
		Self::new(
			Vector2::new(T::one(), T::zero()),
			Vector2::new(T::zero(), T::one()),
		)
	}
}
impl<T> Matrix2<T>
where
	T: Mul<Output = T> + Sub<Output = T> + Clone + Copy,
{
	pub fn determinant(&self) -> T {
		// ad - bc
		(self.x.x * self.y.y) - (self.y.x * self.x.y)
	}
}
impl<T> Matrix2<T>
where
	T: Signed,
{
	pub fn adjugate(&self) -> Self {
		Self::new(
			Vector2::new(self.y.y, -self.x.y),
			Vector2::new(-self.y.x, self.x.x),
		)
	}
	pub fn inverse(&self) -> Self {
		let inverse_det = T::one() / self.determinant();
		let adj = self.adjugate();
		adj * inverse_det
	}
}

impl<T> Mul<T> for Matrix2<T>
where
	T: Num,
{
	type Output = Matrix2<T>;
	fn mul(mut self, rhs: T) -> Self::Output {
		self.x *= rhs;
		self.y *= rhs;
		self
	}
}
impl<T> Mul<Vector2<T>> for Matrix2<T>
where
	T: Num,
{
	type Output = Vector2<T>;
	fn mul(self, rhs: Vector2<T>) -> Self::Output {
		let x = self.x.x * rhs.x + self.y.x * rhs.y;
		let y = self.x.y * rhs.x + self.y.y * rhs.y;
		Vector2::new(x, y)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn determinant() {
		let mat = Matrix2::new(Vector2::new(1.0, 3.0), Vector2::new(2.0, 4.0));
		let det = mat.determinant();
		let expected_det = -2.0;
		assert_eq!(expected_det, det)
	}
	#[test]
	fn inverse() {
		let mat = Matrix2::new(Vector2::new(1.0, 3.0), Vector2::new(2.0, 4.0));
		let expected_inverse = Matrix2::new(Vector2::new(-2.0, 1.5), Vector2::new(1.0, -0.5));
		assert_eq!(mat.inverse(), expected_inverse)
	}

	#[test]
	fn adjugate() {}
}
