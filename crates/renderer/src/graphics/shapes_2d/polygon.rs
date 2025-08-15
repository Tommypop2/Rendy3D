use crate::graphics::shapes_2d::{point::AbsoluteScreenCoordinate, triangle::Triangle2D};

pub struct Polygon<'a> {
	points: &'a [AbsoluteScreenCoordinate],
}

impl<'a> Polygon<'a> {
	pub fn new(points: &'a [AbsoluteScreenCoordinate]) -> Self {
		Self { points }
	}
	pub fn to_triangles(&self) -> Vec<Triangle2D> {
		let mut triangles: Vec<Triangle2D> = Vec::with_capacity(self.points.len() - 2);
		let first = self.points[0];
		let len = self.points.len();
		for i in 1..(len - 1) {
			let current = self.points[i];
			let next = self.points[i + 1];
			triangles.push(Triangle2D::new(first, current, next));
		}
		triangles
	}
}
