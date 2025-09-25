use std::{fs::File, io::BufReader, path::Path};

use obj::{Obj, TexturedVertex as TexturedVertex_OBJ, load_obj as load_obj_1};
use rendy3d::maths::vector::vector2::Vector2;

use rendy3d::graphics::{
	mesh::{indexed::IndexedMesh, vertices::TexturedVertex},
	geometry_3d::point::Point,
};

// impl From<TexturedVertex_OBJ> for TexturedVertex {
// 	fn from(value: TexturedVertex_OBJ) -> Self {
// 		let t = value.texture;
// 		// Z value should always be 0!
// 		debug_assert_eq!(t[2], 0.0);
// 		Self {
// 			position: Point::from_vector(value.position.into()),
// 			normal: value.normal.into(),
// 			texture: Vector2::new(t[0] as f64, t[1] as f64),
// 		}
// 	}
// }
// impl<ObjVertex, V> From<Obj<ObjVertex, u16>> for IndexedMesh<V>
// where
// 	ObjVertex: Into<V>,
// {
// 	fn from(value: Obj<ObjVertex>) -> Self {
// 		Self {
// 			vertices: value.vertices.into_iter().map(|v| v.into()).collect(),
// 			indices: value.indices,
// 		}
// 	}
// }
fn convert_textured_vertex(vertex: TexturedVertex_OBJ) -> TexturedVertex {
	let t = vertex.texture;
	// Z value should always be 0!
	debug_assert_eq!(t[2], 0.0);
	TexturedVertex {
		position: Point::from_vector(vertex.position.into()),
		normal: vertex.normal.into(),
		texture: Vector2::new(t[0] as f64, t[1] as f64),
	}
}
pub fn load_obj_indexed<P: AsRef<Path>>(
	path: P,
) -> Result<IndexedMesh<TexturedVertex>, Box<dyn std::error::Error>> {
	let input = BufReader::new(File::open(path)?);
	let dome: Obj<TexturedVertex_OBJ, u16> = load_obj_1(input)?;
	let mesh: IndexedMesh<TexturedVertex> = IndexedMesh {
		vertices: dome
			.vertices
			.into_iter()
			.map(|v| convert_textured_vertex(v))
			.collect(),
		indices: dome.indices,
	};

	Ok(mesh)
}
