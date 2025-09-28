use core::ops::MulAssign;

use rendy3d_maths::geometry::triangle::Triangle;

use crate::maths::{matrices::matrix4::Matrix4, vector::vector3::Vector3};

use crate::graphics::geometry_3d::point::Point;
pub type Triangle3D = Triangle<Point>;
pub trait Apply {
	fn apply(self, matrix: Matrix4<f64>) -> Self;
}
impl<T> Apply for Triangle<T>
where
	T: MulAssign<Matrix4<f64>>,
{
	fn apply(mut self, matrix: Matrix4<f64>) -> Self {
		self.vertex1 *= matrix.clone();
		self.vertex2 *= matrix.clone();
		self.vertex3 *= matrix;
		self
	}
}
// This is dumb so will be fixed at some point!
trait Normal {
	fn normal(&self) -> Vector3<f64>;
}
impl Normal for Triangle<Point> {
	fn normal(&self) -> Vector3<f64> {
		let side1 = self.vertex2 - self.vertex1;
		let side2 = self.vertex3 - self.vertex1;

		side1.cross_with(&side2)
	}
}
