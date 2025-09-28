use crate::maths::{matrices::matrix2::Matrix2, vector::vector2::Vector2};

use crate::graphics::interpolate::Interpolate;
use crate::graphics::pipeline::back_face_culling::BackFaceCulling;
use crate::graphics::{
	draw::Draw,
	geometry::{bounding_area::BoundingArea2D, point::AbsoluteScreenCoordinate},
	pipeline::Pipeline,
	target::Target,
};
#[derive(Debug, Clone)]
pub struct Triangle<Vertex = AbsoluteScreenCoordinate> {
	pub vertex1: Vertex,
	pub vertex2: Vertex,
	pub vertex3: Vertex,
}
impl<Vertex> Triangle<Vertex> {
	pub const fn new(vertex1: Vertex, vertex2: Vertex, vertex3: Vertex) -> Self {
		Self {
			vertex1,
			vertex2,
			vertex3,
		}
	}
	pub fn map_vertices<T, U: Fn(Vertex) -> T>(self, map_fn: U) -> Triangle<T> {
		Triangle::new(
			map_fn(self.vertex1),
			map_fn(self.vertex2),
			map_fn(self.vertex3),
		)
	}
}
impl Triangle<AbsoluteScreenCoordinate> {
	pub const fn signed_doubled_area(&self) -> i32 {
		let (x1, y1, _) = self.vertex1.as_tuple();
		let (x2, y2, _) = self.vertex2.as_tuple();
		let (x3, y3, _) = self.vertex3.as_tuple();

		x1 as i32 * (y2 as i32 - y3 as i32)
			+ x2 as i32 * (y3 as i32 - y1 as i32)
			+ x3 as i32 * (y1 as i32 - y2 as i32)
	}
	pub const fn doubled_area(&self) -> usize {
		i32::abs(self.signed_doubled_area()) as usize
	}
	pub fn bounding_area(&self) -> BoundingArea2D {
		let min_x = self.vertex1.x.min(self.vertex2.x).min(self.vertex3.x);
		let max_x = self.vertex1.x.max(self.vertex2.x).max(self.vertex3.x);
		let min_y = self.vertex1.y.min(self.vertex2.y).min(self.vertex3.y);
		let max_y = self.vertex1.y.max(self.vertex2.y).max(self.vertex3.y);
		BoundingArea2D {
			min_x,
			max_x,
			min_y,
			max_y,
		}
	}
}
