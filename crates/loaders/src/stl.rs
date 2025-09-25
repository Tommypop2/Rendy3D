use std::{fs::OpenOptions, path::Path};

use rendy3d::graphics::{
	geometry_3d::point::Point,
	mesh::{indexed::IndexedMesh, vertices::Vertex},
};

pub fn load_stl_indexed<P: AsRef<Path>>(path: P) -> IndexedMesh<Vertex, usize> {
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
}
