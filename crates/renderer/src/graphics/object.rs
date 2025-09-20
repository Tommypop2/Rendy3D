use crate::maths::matrices::matrix4::Matrix4;

use crate::graphics::mesh::{indexed::IndexedMesh, vertices::Vertex};

/// Contains a mesh, and its world-space transformation
pub struct Object {
	pub mesh: IndexedMesh<Vertex, usize>,
	pub transformation: Matrix4<f64>,
}
impl Object {
	pub const fn new(mesh: IndexedMesh<Vertex, usize>, transformation: Matrix4<f64>) -> Self {
		Self {
			mesh,
			transformation,
		}
	}
	pub fn set_transformation(&mut self, transformation: Matrix4<f64>) {
		self.transformation = transformation
	}
}
