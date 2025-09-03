use core::ops::{Mul, MulAssign, Neg};

use crate::{
	matrices::matrix3::Matrix3,
	traits::{float::Float, num::Num, signed::Signed},
	vector::{vector3::Vector3, vector4::Vector4},
};

#[derive(Default, Clone, Debug)]
pub struct Matrix4<T> {
	// Matrix Columns
	x: Vector4<T>,
	y: Vector4<T>,
	z: Vector4<T>,
	w: Vector4<T>,
}
impl Matrix4<f64> {
	pub fn new_perspective(fov_x: f64, fov_y: f64, far: f64, near: f64) -> Matrix4<f64> {
		Matrix4::new(
			Vector4::new(1.0 / f64::tan(fov_x / 2.0), 0.0, 0.0, 0.0),
			Vector4::new(0.0, 1.0 / f64::tan(fov_y / 2.0), 0.0, 0.0),
			// Should be -1.0 here for w but 1.0 seems to make things work for some reason
			// TODO: Look into why this is the case
			Vector4::new(0.0, 0.0, -((far + near) / (far - near)), 1.0),
			Vector4::new(0.0, 0.0, -2.0 * (far * near) / (far - near), 0.0),
		)
	}
}
impl<T> Matrix4<T> {
	pub const fn new(x: Vector4<T>, y: Vector4<T>, z: Vector4<T>, w: Vector4<T>) -> Self {
		Self { x, y, z, w }
	}
}
impl<T> Mul<Matrix4<T>> for Matrix4<T>
where
	T: Num,
{
	type Output = Matrix4<T>;
	fn mul(self, rhs: Matrix4<T>) -> Self::Output {
		let x = self.clone() * rhs.x;
		let y = self.clone() * rhs.y;
		let z = self.clone() * rhs.z;
		let w = self * rhs.w;
		Self::new(x, y, z, w)
	}
}
impl<T> Mul<Vector4<T>> for Matrix4<T>
where
	T: Num,
{
	type Output = Vector4<T>;
	fn mul(self, rhs: Vector4<T>) -> Self::Output {
		let x = self.x.x * rhs.x + self.y.x * rhs.y + self.z.x * rhs.z + self.w.x * rhs.w;
		let y = self.x.y * rhs.x + self.y.y * rhs.y + self.z.y * rhs.z + self.w.y * rhs.w;
		let z = self.x.z * rhs.x + self.y.z * rhs.y + self.z.z * rhs.z + self.w.z * rhs.w;
		let w = self.x.w * rhs.x + self.y.w * rhs.y + self.z.w * rhs.z + self.w.w * rhs.w;
		Vector4::new(x, y, z, w)
	}
}
impl<T> Matrix4<T>
where
	T: Num,
{
	pub fn identity() -> Self {
		Self::new(
			Vector4::new(T::one(), T::zero(), T::zero(), T::zero()),
			Vector4::new(T::zero(), T::one(), T::zero(), T::zero()),
			Vector4::new(T::zero(), T::zero(), T::one(), T::zero()),
			Vector4::new(T::zero(), T::zero(), T::zero(), T::one()),
		)
	}
}
impl<T> Matrix4<T>
where
	T: Signed,
{
	pub fn determinant(&self) -> T {
		self.x.x
			* Matrix3::new(
				Vector3::new(self.y.y, self.y.z, self.y.w),
				Vector3::new(self.z.y, self.z.z, self.z.w),
				Vector3::new(self.w.y, self.w.z, self.w.w),
			)
			.determinant()
			- self.x.y
				* Matrix3::new(
					Vector3::new(self.y.x, self.y.z, self.y.w),
					Vector3::new(self.z.x, self.z.z, self.z.w),
					Vector3::new(self.w.x, self.w.z, self.w.w),
				)
				.determinant()
			+ self.x.z
				* Matrix3::new(
					Vector3::new(self.y.x, self.y.y, self.y.w),
					Vector3::new(self.z.x, self.z.y, self.z.w),
					Vector3::new(self.w.x, self.w.y, self.w.w),
				)
				.determinant()
			- self.x.w
				* Matrix3::new(
					Vector3::new(self.y.x, self.y.y, self.y.z),
					Vector3::new(self.z.x, self.z.y, self.z.z),
					Vector3::new(self.w.x, self.w.y, self.w.z),
				)
				.determinant()
	}
}
impl<T> Matrix4<T>
where
	T: Default + Float + Neg<Output = T>,
{
	pub fn with_translation(mut self, vector: Vector3<T>) -> Self {
		let mut vector4: Vector4<T> = vector.into();
		vector4.w = T::one();
		self.w = vector4;
		self
	}
	pub fn translation(vector: Vector3<T>) -> Self {
		Self::identity().with_translation(vector)
	}
	// Scale
	pub fn with_scale_x(mut self, scale: T) -> Self {
		self.x.x *= scale;
		self
	}
	pub fn scale_x(scale: T) -> Self {
		Self::identity().with_scale_x(scale)
	}
	pub fn with_scale_y(mut self, scale: T) -> Self {
		self.y.y *= scale;
		self
	}
	pub fn scale_y(scale: T) -> Self {
		Self::identity().with_scale_y(scale)
	}
	pub fn with_scale_z(mut self, scale: T) -> Self {
		self.z.z *= scale;
		self
	}
	pub fn scale_z(scale: T) -> Self {
		Self::identity().with_scale_z(scale)
	}
	pub fn with_scale(self, scale: T) -> Self {
		self.with_scale_x(scale)
			.with_scale_y(scale)
			.with_scale_z(scale)
	}
	pub fn scale(scale: T) -> Self {
		Self::identity().with_scale(scale)
	}
	pub fn rotation_x(angle: T) -> Self {
		Matrix3::rotate_x(angle).into()
	}
	pub fn rotation_y(angle: T) -> Self {
		Matrix3::rotate_y(angle).into()
	}
	pub fn rotation_z(angle: T) -> Self {
		Matrix3::rotate_z(angle).into()
	}
	/// Returns a 3x3 matrix representing the rotation component of this matrix
	pub fn extract_rotation(&self) -> Matrix3<T> {
		Matrix3::new(
			Vector3::new(self.x.x, self.x.y, self.x.z),
			Vector3::new(self.y.x, self.y.y, self.y.z),
			Vector3::new(self.z.x, self.z.y, self.z.z),
		)
	}
	/// Returns a vector representing the translation component
	pub fn extract_translation(&self) -> Vector3<T> {
		Vector3::new(self.w.x, self.w.y, self.w.z)
	}

	/// Computes a matrix that reverses the combination of translation and rotation represented by this Matrix
	pub fn reverse_rotation_translation(&self) -> Self {
		// A combined rotation-translation matrix applies the rotation first, then the translation
		// So, the inverse needs to undo the translation first, then undo the rotation
		let reverse_translation = Matrix4::translation(self.extract_translation() * -T::one());
		let reverse_rotation: Matrix4<T> = self.extract_rotation().transposed().into();
		reverse_rotation * reverse_translation
	}
}

impl<T> From<Matrix3<T>> for Matrix4<T>
where
	T: Float + Default,
{
	fn from(value: Matrix3<T>) -> Self {
		Self::new(
			value.x.into(),
			value.y.into(),
			value.z.into(),
			Vector4::new(T::zero(), T::zero(), T::zero(), T::one()),
		)
	}
}

impl<T> MulAssign<T> for Matrix4<T>
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
impl<T> Mul<T> for Matrix4<T>
where
	T: MulAssign + Clone,
{
	type Output = Self;
	fn mul(mut self, rhs: T) -> Self::Output {
		self *= rhs;
		self
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn determinant() {
		let mat = Matrix4::new(
			Vector4::new(1, 5, 9, 13),
			Vector4::new(2, 6, 10, 14),
			Vector4::new(3, 7, 11, 15),
			Vector4::new(4, 8, 12, 16),
		);
		assert_eq!(mat.determinant(), 0);
	}
}
