use std::{fs::File, io::BufReader, path::Path};

use maths::vector::vector3::Vector3;
use obj::{Obj, TexturedVertex as TexturedVertex_OBJ, load_obj as load_obj_1, raw::RawObj};

#[derive(Clone, Copy)]
pub struct TexturedVertex {
	pub position: Vector3<f32>,
	pub normal: Vector3<f32>,
	pub texture: Vector3<f32>,
}
impl From<TexturedVertex_OBJ> for TexturedVertex {
	fn from(value: TexturedVertex_OBJ) -> Self {
		Self {
			position: value.position.into(),
			normal: value.normal.into(),
			texture: value.texture.into(),
		}
	}
}
pub struct Mesh {
	pub vertices: Vec<TexturedVertex>,
	pub indices: Vec<u16>,
}
impl From<Obj<TexturedVertex_OBJ, u16>> for Mesh {
	fn from(value: Obj<TexturedVertex_OBJ>) -> Self {
		Self {
			vertices: value.vertices.into_iter().map(|v| v.into()).collect(),
			indices: value.indices,
		}
	}
}
pub fn load_obj<P: AsRef<Path>>(path: P) -> Result<Mesh, Box<dyn std::error::Error>> {
	let input = BufReader::new(File::open(path)?);
	let dome: Obj<TexturedVertex_OBJ, u16> = load_obj_1(input)?;
	let mesh: Mesh = dome.into();

	Ok(mesh)
}
