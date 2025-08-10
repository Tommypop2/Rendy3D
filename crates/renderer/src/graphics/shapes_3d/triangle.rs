use maths::{matrices::matrix4::Matrix4, vector::vector3::Vector3};

use crate::graphics::{
	shaders::shaders::Shaders,
	shapes_2d::{point::AbsoluteScreenCoordinate, triangle::Triangle2D},
	shapes_3d::point::Point,
	target::Target,
};
#[derive(Clone, Debug)]
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
		let side1 = self.vertex2 - self.vertex1;
		let side2 = self.vertex3 - self.vertex1;

		side1.cross_with(&side2)
	}
	pub fn to_triangle_2d<T: Target, S: Shaders>(
		self,
		target: &T,
		shaders: &mut S,
		normal: Vector3<f64>,
	) -> Triangle2D<(AbsoluteScreenCoordinate, S::VsOut)> {
		Triangle2D::new(
			(
				self.vertex1.to_pixel_coordinate(target.area()),
				shaders.vertex(0, self.vertex1, normal),
			),
			(
				self.vertex2.to_pixel_coordinate(target.area()),
				shaders.vertex(1, self.vertex2, normal),
			),
			(
				self.vertex3.to_pixel_coordinate(target.area()),
				shaders.vertex(2, self.vertex3, normal),
			),
		)
	}
}

// impl<VsOut: Interpolate> Draw<VsOut> for Triangle3D {
// 	fn draw<T: Target, S: Shaders<VsOut = VsOut, Pixel = T::Item> + Clone>(
// 		&self,
// 		target: &mut T,
// 		shaders: S,
// 	) {
// 		let t = self.clone().to_triangle_2d(target, shaders.clone(),);
// 		t.draw(target, shaders);
// 	}
// }
