use core::ops::MulAssign;

use crate::maths::{matrices::matrix4::Matrix4, vector::vector3::Vector3};
use derive_more::{Add, Sub};
use rendy3d_maths::vector::vector4::Vector4;

use crate::graphics::geometry::{bounding_area::BoundingArea2D, point::AbsoluteScreenCoordinate};

/// Coordinates between -1 and 1
#[derive(Clone, Add, Copy, Sub, Debug)]
pub struct Point(Vector4<f64>);

impl Point {
	pub const fn new(x: f64, y: f64, z: f64) -> Self {
		Self(Vector4::new(x, y, z, 1.0))
	}
	pub fn from_vector3(v: Vector3<f64>) -> Self {
		Self(Vector4::new(v.x, v.y, v.z, 1.0))
	}
	pub fn from_vector4(v: Vector4<f64>) -> Self {
		Self(v)
	}
	pub fn to_vector3(self) -> Vector3<f64> {
		Vector3::from_homogenous(self.0)
	}
	pub fn to_vector4(self) -> Vector4<f64> {
		self.0
	}
	pub fn to_pixel_coordinate(self, target_area: BoundingArea2D) -> AbsoluteScreenCoordinate {
		let width = target_area.width();
		let height = target_area.height();
		let offset = AbsoluteScreenCoordinate::new(
			target_area.min_x + (width / 2),
			target_area.min_y + (height / 2),
			0.0,
		);
		let v3 = self.to_vector3();
		let x = (offset.x as f64 + v3.x * (width as f64) / 2.0) as usize;
		let y = (offset.y as f64 - v3.y * (height as f64) / 2.0) as usize;
		// println!("({}, {}, {})", x, y, self.z);
		AbsoluteScreenCoordinate::new(x, y, v3.z as f32)
	}
	pub fn apply(self, transformation: Matrix4<f64>) -> Point {
		// Point::from_vector3(Vector3::from_homogenous(
		// 	transformation * self.to_homogenous(),
		// ))
		Point::from_vector4(transformation * self.0)
	}
}

impl MulAssign<Matrix4<f64>> for Point {
	fn mul_assign(&mut self, rhs: Matrix4<f64>) {
		*self = Point::from_vector3(Vector3::from_homogenous(rhs * self.0));
	}
}
