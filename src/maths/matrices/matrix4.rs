use std::ops::Mul;

use crate::maths::vector::{vector3::VectorType, vector4::Vector4};

pub struct Matrix4<T> {
	// Matrix Columns
	x: Vector4<T>,
	y: Vector4<T>,
	z: Vector4<T>,
	w: Vector4<T>,
}

impl<T> Matrix4<T> {
	pub fn new(x: Vector4<T>, y: Vector4<T>, z: Vector4<T>, w: Vector4<T>) -> Self {
		Self { x, y, z, w }
	}
}

impl<T> Mul<Vector4<T>> for Matrix4<T>
where
	T: VectorType,
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
