use crate::maths::{
	Float,
	vector::vector3::{Vector3, VectorType},
};
#[derive(Default)]
pub struct Vector4<T> {
	pub x: T,
	pub y: T,
	pub z: T,
	pub w: T,
}

impl<T> Vector4<T> {
	pub fn new(x: T, y: T, z: T, w: T) -> Self {
		Self { x, y, z, w }
	}
}

impl<T> From<Vector3<T>> for Vector4<T>
where
	T: VectorType + Default,
{
	fn from(value: Vector3<T>) -> Self {
		Self::new(value.x, value.y, value.z, Default::default())
	}
}
