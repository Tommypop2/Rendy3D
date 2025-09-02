use std::{fs::OpenOptions, path::Path};

use crate::graphics::{
	mesh::{IndexedMesh, vertices::Vertex},
	shapes_3d::point::Point,
};

pub fn load_file<P: AsRef<Path>>(path: P) -> IndexedMesh<Vertex, usize> {
	let mut file = OpenOptions::new().read(true).open(path).unwrap();
	let stl = stl_io::read_stl(&mut file).unwrap();
	// stl.validate().unwrap();
	let vertices = stl
		.vertices
		.iter()
		.map(|v| Vertex::new(Point::new(v.0[0] as f64, v.0[1] as f64, v.0[2] as f64)))
		.collect::<Vec<Vertex>>();
	let triangles = stl.faces;
	IndexedMesh {
		vertices,
		indices: triangles
			.iter()
			.flat_map(|t| t.vertices)
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
