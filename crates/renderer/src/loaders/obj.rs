use std::{
	fs::File,
	io::BufReader,
	ops::{Mul, MulAssign},
	path::Path,
	slice::ChunksExact,
};

use maths::{
	matrices::matrix4::Matrix4,
	vector::{vector2::Vector2, vector3::Vector3},
};
use obj::{Obj, TexturedVertex as TexturedVertex_OBJ, load_obj as load_obj_1};

use crate::graphics::{
	colour::Colour, draw::Draw, interpolate::Interpolate, pipeline::pipeline::Pipeline,
	shapes_2d::triangle::Triangle, shapes_3d::point::Point, target::Target,
};

#[derive(Clone, Copy)]
pub struct TexturedVertex {
	pub position: Point,
	pub normal: Vector3<f64>,
	pub texture: Vector2<f64>,
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
		// TODO: technically use inverse-transpose here but just the rotation should be fine for now :)
		self.normal = rhs.extract_rotation() * self.normal;
	}
}
impl From<TexturedVertex_OBJ> for TexturedVertex {
	fn from(value: TexturedVertex_OBJ) -> Self {
		let t = value.texture;
		// Z value should always be 0!
		debug_assert_eq!(t[2], 0.0);
		Self {
			position: Point::from_vector(value.position.into()),
			normal: value.normal.into(),
			texture: Vector2::new(t[0] as f64, t[1] as f64),
		}
	}
}
pub struct Mesh<T> {
	pub vertices: Vec<T>,
	pub indices: Vec<u16>,
}
impl<ObjVertex, V> From<Obj<ObjVertex, u16>> for Mesh<V>
where
	ObjVertex: Into<V>,
{
	fn from(value: Obj<ObjVertex>) -> Self {
		Self {
			vertices: value.vertices.into_iter().map(|v| v.into()).collect(),
			indices: value.indices,
		}
	}
}
pub struct MeshIter<'a, T> {
	mesh: &'a Mesh<T>,
	chunks: ChunksExact<'a, u16>,
}
impl<T> Iterator for MeshIter<'_, T>
where
	T: Clone,
{
	type Item = Triangle<T>;
	fn next(&mut self) -> Option<Self::Item> {
		let chunk = self.chunks.next()?;
		let i1 = chunk[0];
		let i2 = chunk[1];
		let i3 = chunk[2];
		let v1 = self.mesh.vertices[i1 as usize].clone();
		let v2 = self.mesh.vertices[i2 as usize].clone();
		let v3 = self.mesh.vertices[i3 as usize].clone();
		let triangle = Triangle::new(v1, v2, v3);
		Some(triangle)
	}
}
impl<T> Mesh<T> {
	/// Returns an iterator over the triangles in this mesh
	pub fn triangles(&self) -> MeshIter<'_, T> {
		let chunks = self.indices.chunks_exact(3);
		MeshIter { mesh: self, chunks }
	}
}

pub fn load_obj<P: AsRef<Path>>(
	path: P,
) -> Result<Mesh<TexturedVertex>, Box<dyn std::error::Error>> {
	let input = BufReader::new(File::open(path)?);
	let dome: Obj<TexturedVertex_OBJ, u16> = load_obj_1(input)?;
	let mesh: Mesh<TexturedVertex> = dome.into();

	Ok(mesh)
}

impl Mesh<TexturedVertex> {
	pub fn render<P, T, U>(
		&self,
		pipeline: &mut P,
		target: &mut T,
		transform: Matrix4<f64>,
		projection: Matrix4<f64>,
	) where
		P: Pipeline<VsOut = U, Fragment = Colour, Vertex = TexturedVertex>,
		T: Target<Item = Colour>,
		U: Interpolate,
	{
		for triangle in self.triangles() {
			let transformed = triangle.apply(transform.clone());
			// let projected = transformed.clone().apply(projection.clone());
			Triangle::new(
				{
					let vsout = pipeline.vertex(0, transformed.vertex1);
					(
						vsout
							.0
							.apply(projection.clone())
							.to_pixel_coordinate(target.area()),
						vsout.1,
					)
				},
				{
					let vsout = pipeline.vertex(0, transformed.vertex2);
					(
						vsout
							.0
							.apply(projection.clone())
							.to_pixel_coordinate(target.area()),
						vsout.1,
					)
				},
				{
					let vsout = pipeline.vertex(0, transformed.vertex3);
					(
						vsout
							.0
							.apply(projection.clone())
							.to_pixel_coordinate(target.area()),
						vsout.1,
					)
				},
			)
			.draw(target, pipeline);
		}
	}
}
