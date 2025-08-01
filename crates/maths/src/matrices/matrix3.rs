use std::{
	mem,
	ops::{Add, Mul, Neg, Sub},
};

use derive_more::Mul;

use crate::{
	matrices::matrix2::Matrix2,
	traits::float::Float,
	vector::{vector2::Vector2, vector3::Vector3},
};

#[derive(Default, PartialEq, Debug, Mul)]
pub struct Matrix3<T> {
	// Matrix Columns
	pub x: Vector3<T>,
	pub y: Vector3<T>,
	pub z: Vector3<T>,
}

impl<T> Matrix3<T> {
	pub fn new(x: Vector3<T>, y: Vector3<T>, z: Vector3<T>) -> Self {
		Self { x, y, z }
	}
}
#[derive(Debug)]
pub enum MatrixInversionError {
	SingularMatrix,
}
impl<T> Matrix3<T>
where
	T: Float + Neg<Output = T>,
{
	pub fn identity() -> Self {
		Self::new(
			Vector3::new(T::one(), T::zero(), T::zero()),
			Vector3::new(T::zero(), T::one(), T::zero()),
			Vector3::new(T::zero(), T::zero(), T::one()),
		)
	}
	/// Rotate about X
	pub fn with_rotation_x(mut self, angle: T) -> Self {
		let sin = angle.sin();
		let cos = angle.cos();
		self.x.x = cos;
		self.x.y = sin;
		self.y.x = -sin;
		self.y.y = cos;
		self
	}
	pub fn rotate_x(angle: T) -> Self {
		Self::identity().with_rotation_x(angle)
	}
	/// Rotate about Y
	pub fn with_rotation_y(mut self, angle: T) -> Self {
		let sin = angle.sin();
		let cos = angle.cos();
		self.y.y = cos;
		self.y.z = sin;
		self.z.y = -sin;
		self.z.z = cos;
		self
	}
	pub fn rotate_y(angle: T) -> Self {
		Self::identity().with_rotation_y(angle)
	}
	/// Rotate about Z
	pub fn with_rotation_z(mut self, angle: T) -> Self {
		let sin = angle.sin();
		let cos = angle.cos();
		self.x.x = cos;
		self.x.z = -sin;
		self.z.x = sin;
		self.z.z = cos;
		self
	}
	pub fn rotate_z(angle: T) -> Self {
		Self::identity().with_rotation_z(angle)
	}
	pub fn invert(&self) -> Result<Self, MatrixInversionError> {
		let det = self.determinant();
		if det == T::zero() {
			Err(MatrixInversionError::SingularMatrix)
		} else {
			let cofactors = self.cofactors();
			let inverted = cofactors.transposed() * (T::one() / det);
			Ok(inverted)
		}
	}
}
impl<T> Matrix3<T> {
	pub fn transpose(&mut self) {
		mem::swap(&mut self.x.z, &mut self.z.x);
		mem::swap(&mut self.x.y, &mut self.y.x);
		mem::swap(&mut self.y.z, &mut self.z.y);
	}
	pub fn transposed(mut self) -> Self {
		self.transpose();
		self
	}
}
impl<T> Matrix3<T>
where
	T: Mul<Output = T> + Sub<Output = T> + Clone + Copy + Add<Output = T>,
{
	pub fn determinant(&self) -> T {
		self.x.x
			* Matrix2::new(
				Vector2::new(self.y.y, self.y.z),
				Vector2::new(self.z.y, self.z.z),
			)
			.determinant()
			- self.y.x
				* Matrix2::new(
					Vector2::new(self.x.y, self.x.z),
					Vector2::new(self.z.y, self.z.z),
				)
				.determinant()
			+ self.z.x
				* Matrix2::new(
					Vector2::new(self.x.y, self.x.z),
					Vector2::new(self.y.y, self.y.z),
				)
				.determinant()
	}
	pub fn minors(&self) -> Self {
		Self::new(
			Vector3::new(
				Matrix2::new(
					Vector2::new(self.y.y, self.y.z),
					Vector2::new(self.z.y, self.z.z),
				)
				.determinant(),
				Matrix2::new(
					Vector2::new(self.y.x, self.z.x),
					Vector2::new(self.y.z, self.z.z),
				)
				.determinant(),
				Matrix2::new(
					Vector2::new(self.y.x, self.y.y),
					Vector2::new(self.z.x, self.z.y),
				)
				.determinant(),
			),
			Vector3::new(
				Matrix2::new(
					Vector2::new(self.x.y, self.x.z),
					Vector2::new(self.z.y, self.z.z),
				)
				.determinant(),
				Matrix2::new(
					Vector2::new(self.x.x, self.x.z),
					Vector2::new(self.z.x, self.z.z),
				)
				.determinant(),
				Matrix2::new(
					Vector2::new(self.x.x, self.x.y),
					Vector2::new(self.z.x, self.z.y),
				)
				.determinant(),
			),
			Vector3::new(
				Matrix2::new(
					Vector2::new(self.x.y, self.x.z),
					Vector2::new(self.y.y, self.y.z),
				)
				.determinant(),
				Matrix2::new(
					Vector2::new(self.x.x, self.x.z),
					Vector2::new(self.y.x, self.y.z),
				)
				.determinant(),
				Matrix2::new(
					Vector2::new(self.x.x, self.x.y),
					Vector2::new(self.y.x, self.y.y),
				)
				.determinant(),
			),
		)
	}
}
impl<T> Matrix3<T>
where
	T: Mul<Output = T> + Sub<Output = T> + Clone + Copy + Neg<Output = T> + Add<Output = T>,
{
	pub fn cofactors(&self) -> Self {
		let mut minors = self.minors();
		minors.x.y = -minors.x.y;
		minors.y.x = -minors.y.x;
		minors.y.z = -minors.y.z;
		minors.z.y = -minors.z.y;
		minors
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn transpose() {
		let mat = Matrix3::new(
			Vector3::new(1, 2, 3),
			Vector3::new(4, 5, 6),
			Vector3::new(7, 8, 9),
		);
		let transposed = mat.transposed();
		let expected = Matrix3::new(
			Vector3::new(1, 4, 7),
			Vector3::new(2, 5, 8),
			Vector3::new(3, 6, 9),
		);
		assert_eq!(transposed, expected)
	}

	#[test]
	fn minors() {
		let mat = Matrix3::new(
			Vector3::new(1, 2, 3),
			Vector3::new(4, 5, 6),
			Vector3::new(7, 8, 9),
		);
		let minors = mat.minors();
		let expected = Matrix3::new(
			Vector3::new(-3, -6, -3),
			Vector3::new(-6, -12, -6),
			Vector3::new(-3, -6, -3),
		);
		assert_eq!(minors, expected)
	}
	#[test]
	fn cofactors() {
		let mat = Matrix3::new(
			Vector3::new(1, 2, 3),
			Vector3::new(4, 5, 6),
			Vector3::new(7, 8, 9),
		);
		let cofactors = mat.cofactors();
		let expected = Matrix3::new(
			Vector3::new(-3, 6, -3),
			Vector3::new(6, -12, 6),
			Vector3::new(-3, 6, -3),
		);
		assert_eq!(cofactors, expected)
	}
	#[test]
	fn determinant() {
		let mat = Matrix3::new(
			Vector3::new(1, 2, 3),
			Vector3::new(4, 5, 6),
			Vector3::new(7, 8, 9),
		);
		let det = mat.determinant();
		assert_eq!(det, 0)
	}
	#[test]
	fn invert_singular_matrix() {
		let res = Matrix3::new(
			Vector3::new(1.0, 2.0, 3.0),
			Vector3::new(4.0, 5.0, 6.0),
			Vector3::new(7.0, 8.0, 9.0),
		)
		.invert();
		assert!(res.is_err())
	}
}
