use crate::graphics::shapes_2d::{point::AbsoluteScreenCoordinate, triangle::Triangle};

pub struct Polygon<'a> {
	points: &'a [AbsoluteScreenCoordinate],
}
impl<'a> Polygon<'a> {
	pub const fn new(points: &'a [AbsoluteScreenCoordinate]) -> Self {
		Self { points }
	}
	#[cfg(feature = "std")]
	pub fn to_triangles(&self) -> Vec<Triangle> {
		let mut triangles: Vec<Triangle> = Vec::with_capacity(self.points.len() - 2);
		let first = self.points[0];
		let len = self.points.len();
		for i in 1..(len - 1) {
			let current = self.points[i];
			let next = self.points[i + 1];
			triangles.push(Triangle::new(first, current, next));
		}
		triangles
	}
}
