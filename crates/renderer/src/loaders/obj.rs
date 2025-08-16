use std::{
	fs::File,
	io::BufReader,
	ops::{Mul, MulAssign},
	path::Path,
};

use maths::{matrices::matrix4::Matrix4, vector::vector3::Vector3};
use obj::{Obj, TexturedVertex as TexturedVertex_OBJ, load_obj as load_obj_1};

use crate::graphics::shapes_3d::point::Point;

#[derive(Clone, Copy)]
pub struct TexturedVertex {
	pub position: Point,
	pub normal: Vector3<f64>,
	pub texture: Vector3<f64>,
}
impl Mul<Matrix4<f64>> for TexturedVertex {
	type Output = Self;
	fn mul(mut self, rhs: Matrix4<f64>) -> Self::Output {
		self *= rhs;
		self
	}
}
impl MulAssign<Matrix4<f64>> for TexturedVertex {
	fn mul_assign(&mut self, rhs: Matrix4<f64>) {
		self.position = Point::from_vector(Vector3::from_homogenous(
			rhs.clone() * self.position.to_homogenous(),
		));
		// TODO: correctly map normals
		self.normal = self.normal;
		self.texture = self.texture;
	}
}
impl From<TexturedVertex_OBJ> for TexturedVertex {
	fn from(value: TexturedVertex_OBJ) -> Self {
		Self {
			position: Point::from_vector(value.position.into()),
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
