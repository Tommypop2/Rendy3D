use std::ops::{Add, Mul};

pub struct Vector3<T: Mul<Output = T> + Add<Output = T>> {
	x: T,
	y: T,
	z: T,
}
impl<T: Mul<Output = T> + Add<Output = T>> Vector3<T> {
	pub fn new(x: T, y: T, z: T) -> Self {
		Self { x, y, z }
	}
	fn cross(a: Self, b: Self) -> Self {
		todo!();
	}
	fn dot(a: Self, b: Self) -> T {
		a.x * b.x + a.y * b.y + a.z * b.z
	}
}
