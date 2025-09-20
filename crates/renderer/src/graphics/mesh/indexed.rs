//! Indexed mesh

use core::slice::ChunksExact;

use crate::graphics::geometry::triangle::Triangle;

/// Indexed mesh type
///
/// The vertices are stored in a separate [`Vec`] to the indices. This means that no vertex is duplicated, even if it appears in multiple triangles.
/// This approach saves memory, at the expensive of adding some extra indirection on each triangle access
#[derive(Debug)]
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
