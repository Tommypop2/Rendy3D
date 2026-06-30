use std::array::IntoIter;

use rendy3d_maths::geometry::triangle::Triangle;

use crate::graphics::geometry_3d::{point::Point, triangle::Triangle3D};
pub struct Cube {
	triangles: [Triangle3D; 12],
}
impl Cube {
	pub const fn new(side_length: f64) -> Self {
		let v = side_length / 2.0;
		let a = Point::new(v, v, v);
		let b = Point::new(-v, v, v);
		let c = Point::new(v, -v, v);
		let d = Point::new(v, -v, -v);
		let e = Point::new(-v, -v, v);
		let f = Point::new(-v, -v, -v);
		let g = Point::new(-v, v, -v);
		let h = Point::new(v, v, -v);
		Self {
			triangles: [
				Triangle::new(a, b, c),
				Triangle::new(c, b, e),
				Triangle::new(a, h, g),
				Triangle::new(b, a, g),
				Triangle::new(f, b, g),
				Triangle::new(e, b, f),
				Triangle::new(a, c, d),
				Triangle::new(h, a, d),
				Triangle::new(c, e, d),
				Triangle::new(e, f, d),
				Triangle::new(g, h, d),
				Triangle::new(f, g, d),
			],
		}
	}
}
impl IntoIterator for Cube {
	type Item = Triangle3D;
	type IntoIter = IntoIter<Triangle3D, 12>;
	fn into_iter(self) -> Self::IntoIter {
		self.triangles.into_iter()
	}
}
