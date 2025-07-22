use std::{fs::OpenOptions, path::Path};

use crate::graphics::shapes_3d::{point::Point, triangle::Triangle3D};

pub fn load_file<P: AsRef<Path>>(path: P) -> Vec<Triangle3D> {
	let mut file = OpenOptions::new().read(true).open(path).unwrap();
	let stl = stl_io::read_stl(&mut file).unwrap();
	let vertices = stl
		.vertices
		.iter()
		.map(|v| Point::new(v.0[0] as f64, v.0[1] as f64, v.0[2] as f64))
		.collect::<Vec<Point>>();
	let triangles = stl.faces;
	let triangles = triangles
		.iter()
		.map(|indexed_triangle| {
			let vertex_indices = indexed_triangle.vertices;
			Triangle3D::new(
				vertices[vertex_indices[0]],
				vertices[vertex_indices[1]],
				vertices[vertex_indices[2]],
			)
		})
		.collect::<Vec<Triangle3D>>();
	triangles
}
