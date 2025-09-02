pub mod vertices;

use std::slice::ChunksExact;

use crate::graphics::{shapes_2d::triangle::Triangle, shapes_3d::triangle::Triangle3D};

// Non-indexed mesh
pub struct Mesh {
	/// Triangles that make up the shape
	pub triangles: Vec<Triangle3D>,
}
// #[derive(Default)]
// pub struct Dimensions {
// 	min_x: f64,
// 	min_y: f64,
// 	min_z: f64,
// 	max_x: f64,
// 	max_y: f64,
// 	max_z: f64,
// }
impl Mesh {
	pub fn new(triangles: Vec<Triangle3D>) -> Self {
		Self { triangles }
	}
	pub fn dimensions(&self) {
		todo!();
	}
}

// Indexed Mesh

pub struct IndexedMesh<T, I = u16> {
	pub vertices: Vec<T>,
	pub indices: Vec<I>,
}

pub struct IndexedMeshIter<'a, T, I> {
	mesh: &'a IndexedMesh<T, I>,
	chunks: ChunksExact<'a, I>,
}
impl<T, I> Iterator for IndexedMeshIter<'_, T, I>
where
	T: Clone,
	I: Into<usize> + Copy,
{
	type Item = Triangle<T>;
	fn next(&mut self) -> Option<Self::Item> {
		let chunk = self.chunks.next()?;
		let i1 = chunk[0];
		let i2 = chunk[1];
		let i3 = chunk[2];
		let v1 = self.mesh.vertices[i1.into()].clone();
		let v2 = self.mesh.vertices[i2.into()].clone();
		let v3 = self.mesh.vertices[i3.into()].clone();
		let triangle = Triangle::new(v1, v2, v3);
		Some(triangle)
	}
}
impl<T, I> IndexedMesh<T, I> {
	/// Returns an iterator over the triangles in this mesh
	pub fn triangles(&self) -> IndexedMeshIter<'_, T, I> {
		let chunks = self.indices.chunks_exact(3);
		IndexedMeshIter { mesh: self, chunks }
	}
}
