use crate::graphics::{shapes_2d::triangle::Triangle2D, shapes_3d::point::Point};

pub struct Triangle3D {
	vertex1: Point,
	vertex2: Point,
	vertex3: Point,
}

impl Triangle3D {
	pub fn new(vertex1: Point, vertex2: Point, vertex3: Point) -> Self {
		Self {
			vertex1,
			vertex2,
			vertex3,
		}
	}
}

impl From<Triangle3D> for Triangle2D {
	fn from(value: Triangle3D) -> Self {
		Self::new(
			value.vertex1.into(),
			value.vertex2.into(),
			value.vertex3.into(),
		)
	}
}
