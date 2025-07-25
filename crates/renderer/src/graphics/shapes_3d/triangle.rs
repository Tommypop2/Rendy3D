use maths::{matrices::matrix4::Matrix4, vector::vector3::Vector3};

use crate::graphics::{draw::Draw, shapes_2d::triangle::Triangle2D, shapes_3d::point::Point};
#[derive(Clone)]
pub struct Triangle3D {
	pub vertex1: Point,
	pub vertex2: Point,
	pub vertex3: Point,
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
	pub fn vertices(&self) -> [Point; 3] {
		[self.vertex1, self.vertex2, self.vertex3]
	}
	pub fn vertices_mut(&mut self) -> [&mut Point; 3] {
		[&mut self.vertex1, &mut self.vertex2, &mut self.vertex3]
	}
	pub fn normal(&self) -> Vector3<f64> {
		let side1 = self.vertex1 - self.vertex2;
		let side2 = self.vertex1 - self.vertex3;

		side1.cross_with(&side2)
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

impl Draw for Triangle3D {
	fn draw(
		&self,
		viewport: &mut crate::graphics::viewport::Viewport,
		screen: &mut crate::graphics::screen::Screen,
	) {
		let t: Triangle2D = self.clone().into();
		t.draw(viewport, screen);
	}
}
