use std::mem;

use crate::{
	Float,
	vector::vector3::{Vector3, VectorType},
};

#[derive(Default, PartialEq, Debug)]
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
impl<T> Matrix3<T>
where
	T: VectorType + Float,
{
	pub fn unit() -> Self {
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
		Self::unit().with_rotation_x(angle)
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
		Self::unit().with_rotation_y(angle)
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
		Self::unit().with_rotation_z(angle)
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
}
