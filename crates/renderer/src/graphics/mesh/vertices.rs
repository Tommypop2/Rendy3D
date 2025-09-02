//! Vertex Types

use std::ops::{Mul, MulAssign};

use maths::{
	matrices::matrix4::Matrix4,
	vector::{vector2::Vector2, vector3::Vector3},
};

use crate::graphics::shapes_3d::point::Point;

/// Vertex with texture coordinates
#[derive(Clone, Copy)]
pub struct TexturedVertex {
	pub position: Point,
	pub normal: Vector3<f64>,
	pub texture: Vector2<f64>,
}
impl Mul<Matrix4<f64>> for TexturedVertex {
	type Output = Self;
	fn mul(mut self, rhs: Matrix4<f64>) -> Self::Output {
		self *= rhs;
		self
	}
}
impl MulAssign<Matrix4<f64>> for TexturedVertex {
	fn mul_assign(&mut self, rhs: Matrix4<f64>) {
		self.position = Point::from_vector(Vector3::from_homogenous(
			rhs.clone() * self.position.to_homogenous(),
		));
		// TODO: technically use inverse-transpose here but just the rotation should be fine for now :)
		self.normal = rhs.extract_rotation() * self.normal;
	}
}

//

/// Vertex with only position information
#[derive(Clone, Copy)]
pub struct Vertex {
	pub position: Point,
}
impl Vertex {
	pub fn new(p: Point) -> Self {
		Self { position: p }
	}
}
impl MulAssign<Matrix4<f64>> for Vertex {
	fn mul_assign(&mut self, rhs: Matrix4<f64>) {
		self.position = self.position.apply(rhs)
	}
}
