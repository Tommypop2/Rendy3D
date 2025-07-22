use crate::maths::vector::vector3::Vector3;

pub type Point = Vector3<f64>;
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
