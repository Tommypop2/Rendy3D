use std::ops::MulAssign;

use maths::{matrices::matrix4::Matrix4, vector::vector3::Vector3};

use crate::graphics::{shapes_2d::triangle::Triangle, shapes_3d::point::Point};
pub type Triangle3D = Triangle<Point>;
impl<T> Triangle<T>
where
	T: MulAssign<Matrix4<f64>>,
{
	pub fn apply(mut self, matrix: Matrix4<f64>) -> Self {
		self.vertex1 *= matrix.clone();
		self.vertex2 *= matrix.clone();
		self.vertex3 *= matrix;
		self
	}
}
impl Triangle3D {
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
}
