use crate::{
	Float,
	vector::vector3::{Vector3, VectorType},
};

#[derive(Default)]
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
