use core::ops::MulAssign;

use crate::maths::matrices::matrix4::Matrix4;

use crate::graphics::{
	draw::Draw, geometry::triangle::Triangle, interpolate::Interpolate, pipeline::Pipeline,
	target::Target,
};

/// Renders a mesh with the given shaders
pub fn render<M, P, T, U, V, F>(
	mesh: M,
	pipeline: &mut P,
	target: &mut T,
	transform: Matrix4<f64>,
	projection: Matrix4<f64>,
) where
	M: Iterator<Item = Triangle<V>>,
	P: Pipeline<VsOut = U, Fragment = F, Vertex = V>,
	T: Target<Item = F>,
	U: Interpolate,
	V: MulAssign<Matrix4<f64>> + Clone,
{
	for triangle in mesh {
		let transformed = triangle.apply(transform.clone());
		// let projected = transformed.clone().apply(projection.clone());
		transformed
			.map_vertices(|vertex| {
				let vsout = pipeline.vertex(0, vertex);
				(
					vsout
						.0
						.apply(projection.clone())
						.to_pixel_coordinate(target.area()),
					vsout.1,
				)
			})
			.draw(target, pipeline);
	}
}
