use crate::{
	graphics::{shapes_2d::triangle::Triangle2D, shapes_3d::point::Point},
	maths::{matrices::matrix4::Matrix4, vector::vector3::Vector3},
};
#[derive(Clone)]
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
	pub fn apply(mut self, matrix: Matrix4<f64>) -> Self {
		self.vertex1 = Point::from_vector(Vector3::from_homogenous(
			matrix.clone() * self.vertex1.to_homogenous(),
		));
		self.vertex2 = Point::from_vector(Vector3::from_homogenous(
			matrix.clone() * self.vertex2.to_homogenous(),
		));
		self.vertex3 = Point::from_vector(Vector3::from_homogenous(
			matrix * self.vertex3.to_homogenous(),
		));
		self
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
