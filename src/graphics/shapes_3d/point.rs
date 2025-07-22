use derive_more::{Add, Deref, DerefMut};

use crate::maths::vector::vector3::Vector3;

/// Coordinates between -1 and 1
#[derive(Deref, DerefMut, Clone, Add, Copy)]
pub struct Point(Vector3<f64>);

impl Point {
	pub fn new(x: f64, y: f64, z: f64) -> Self {
		Self(Vector3::new(x, y, z))
	}
	pub fn from_vector(v: Vector3<f64>) -> Self {
		Self(v)
	}
}
