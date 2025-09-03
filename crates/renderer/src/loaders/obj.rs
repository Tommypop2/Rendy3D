use std::{fs::File, io::BufReader, ops::MulAssign, path::Path};

use maths::{matrices::matrix4::Matrix4, vector::vector2::Vector2};
use obj::{Obj, TexturedVertex as TexturedVertex_OBJ, load_obj as load_obj_1};

use crate::graphics::{
	colour::Colour, draw::Draw, interpolate::Interpolate, mesh::{indexed::IndexedMesh, vertices::TexturedVertex},
	pipeline::Pipeline, shapes_2d::triangle::Triangle, shapes_3d::point::Point, target::Target,
};

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
impl<ObjVertex, V> From<Obj<ObjVertex, u16>> for IndexedMesh<V>
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
pub fn load_obj<P: AsRef<Path>>(
	path: P,
) -> Result<IndexedMesh<TexturedVertex>, Box<dyn std::error::Error>> {
	let input = BufReader::new(File::open(path)?);
	let dome: Obj<TexturedVertex_OBJ, u16> = load_obj_1(input)?;
	let mesh: IndexedMesh<TexturedVertex> = dome.into();

	Ok(mesh)
}

pub fn render<M, P, T, U, V>(
	mesh: M,
	pipeline: &mut P,
	target: &mut T,
	transform: Matrix4<f64>,
	projection: Matrix4<f64>,
) where
	M: Iterator<Item = Triangle<V>>,
	P: Pipeline<VsOut = U, Fragment = Colour, Vertex = V>,
	T: Target<Item = Colour>,
	U: Interpolate,
	V: MulAssign<Matrix4<f64>> + Clone,
{
	for triangle in mesh {
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
