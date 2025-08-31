use maths::matrices::matrix4::Matrix4;

use crate::loaders::{obj::Mesh, stl::Vertex};

/// Contains a mesh, and its world-space transformation
pub struct Object {
	pub mesh: Mesh<Vertex, usize>,
	pub transformation: Matrix4<f64>,
}
impl Object {
	pub fn new(mesh: Mesh<Vertex, usize>, transformation: Matrix4<f64>) -> Self {
		Self {
			mesh,
			transformation,
		}
	}
	pub fn set_transformation(&mut self, transformation: Matrix4<f64>) {
		self.transformation = transformation
	}
}
