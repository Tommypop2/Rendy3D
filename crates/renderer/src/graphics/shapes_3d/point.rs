use derive_more::{Add, Deref, DerefMut, Sub};
use maths::vector::vector3::Vector3;

use crate::graphics::shapes_2d::{bounding_area::BoundingArea2D, point::AbsoluteScreenCoordinate};

/// Coordinates between -1 and 1
#[derive(Deref, DerefMut, Clone, Add, Copy, Sub, Debug)]
pub struct Point(Vector3<f64>);

impl Point {
	pub const fn new(x: f64, y: f64, z: f64) -> Self {
		Self(Vector3::new(x, y, z))
	}
	pub fn from_vector(v: Vector3<f64>) -> Self {
		Self(v)
	}
	pub fn to_vector(self) -> Vector3<f64> {
		Vector3::new(self.x, self.y, self.z)
	}
	pub fn to_pixel_coordinate(self, target_area: BoundingArea2D) -> AbsoluteScreenCoordinate {
		let width = target_area.width();
		let height = target_area.height();
		let offset = AbsoluteScreenCoordinate::new(
			target_area.min_x + (width / 2),
			target_area.min_y + (height / 2),
			0.0,
		);
		let x = (offset.x as f64 + self.x * (width as f64) / 2.0).round() as usize;
		let y = (offset.y as f64 - self.y * (height as f64) / 2.0).round() as usize;
		// println!("({}, {}, {})", x, y, self.z);
		AbsoluteScreenCoordinate::new(x, y, self.z as f32)
	}
}
