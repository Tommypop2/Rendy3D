use std::{fs::OpenOptions, ops::MulAssign, path::Path};

use maths::{matrices::matrix4::Matrix4, vector::vector3::Vector3};

use crate::{
	graphics::shapes_3d::{point::Point, triangle::Triangle3D},
	loaders::obj::Mesh,
};
#[derive(Clone, Copy)]
pub struct Vertex {
	pub position: Point,
}
impl Vertex {
	pub fn new(p: Point) -> Self {
		Self { position: p }
	}
}
impl MulAssign<Matrix4<f64>> for Vertex {
	fn mul_assign(&mut self, rhs: Matrix4<f64>) {
		self.position = self.position.apply(rhs)
	}
}
pub fn load_file<P: AsRef<Path>>(path: P) -> Mesh<Vertex, usize> {
	let mut file = OpenOptions::new().read(true).open(path).unwrap();
	let stl = stl_io::read_stl(&mut file).unwrap();
	// stl.validate().unwrap();
	let vertices = stl
		.vertices
		.iter()
		.map(|v| Vertex::new(Point::new(v.0[0] as f64, v.0[1] as f64, v.0[2] as f64)))
		.collect::<Vec<Vertex>>();
	let triangles = stl.faces;
	Mesh {
		vertices: vertices,
		indices: triangles
			.iter()
			.map(|t| t.vertices)
			.flatten()
			.collect::<Vec<usize>>(),
	}
	// triangles
	// 	.iter()
	// 	.map(|indexed_triangle| {
	// 		let vertex_indices = indexed_triangle.vertices;
	// 		Triangle3D::new(
	// 			vertices[vertex_indices[0]],
	// 			vertices[vertex_indices[1]],
	// 			vertices[vertex_indices[2]],
	// 		)
	// 	})
	// 	.collect::<Vec<Triangle3D>>()
}
