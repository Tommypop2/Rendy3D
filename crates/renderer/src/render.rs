use core::ops::MulAssign;

use maths::matrices::matrix4::Matrix4;

use crate::graphics::{
	colour::Colour, draw::Draw, interpolate::Interpolate, pipeline::Pipeline,
	shapes_2d::triangle::Triangle, target::Target,
};

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
