use crate::{geometry::bounding_area::BoundingArea2D, vector::vector2::Vector2};

#[derive(Debug, Clone)]
pub struct Triangle<Vertex> {
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
	pub fn vertices_mut(&mut self) -> [&mut Vertex; 3] {
		[&mut self.vertex1, &mut self.vertex2, &mut self.vertex3]
	}
}
impl<Vertex> Triangle<Vertex>
where
	Vertex: Copy,
{
	pub fn vertices(&self) -> [Vertex; 3] {
		[self.vertex1, self.vertex2, self.vertex3]
	}
}
impl Triangle<Vector2<i32>> {
	pub const fn signed_doubled_area(&self) -> i32 {
		let (x1, y1) = self.vertex1.as_tuple();
		let (x2, y2) = self.vertex2.as_tuple();
		let (x3, y3) = self.vertex3.as_tuple();

		x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)
	}
	pub const fn doubled_area(&self) -> usize {
		i32::abs(self.signed_doubled_area()) as usize
	}
	pub fn bounding_area(&self) -> BoundingArea2D {
		let min_x = self.vertex1.x.min(self.vertex2.x).min(self.vertex3.x) as usize;
		let max_x = self.vertex1.x.max(self.vertex2.x).max(self.vertex3.x) as usize;
		let min_y = self.vertex1.y.min(self.vertex2.y).min(self.vertex3.y) as usize;
		let max_y = self.vertex1.y.max(self.vertex2.y).max(self.vertex3.y) as usize;
		BoundingArea2D {
			min_x,
			max_x,
			min_y,
			max_y,
		}
	}
}
