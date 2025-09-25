use core::ops::MulAssign;

use rendy3d_maths::vector::vector3::Vector3;
use rendy3d_maths::vector::vector4::Vector4;

use crate::graphics::geometry_3d::point::Point;
use crate::maths::matrices::matrix4::Matrix4;

use crate::graphics::{
	draw::Draw, geometry::triangle::Triangle, interpolate::Interpolate, pipeline::Pipeline,
	target::Target,
};
fn test_point(p: Vector4<f64>) -> bool {
	let (x, y, z, w) = p.as_tuple();
	// println!("{x}, {y}, {z}, {w}");
	(-w <= x && x <= w) && (-w <= y && y <= w) && (-w <= z && z <= w)
}
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
		// transformed
		// 	.map_vertices(|vertex| {
		// 		let vsout = pipeline.vertex(0, vertex);
		// 		(
		// 			vsout
		// 				.0
		// 				.apply(projection.clone())
		// 				.to_pixel_coordinate(target.area()),
		// 			vsout.1,
		// 		)
		// 	})
		// 	.draw(target, pipeline);
		Triangle::new(
			{
				let vsout = pipeline.vertex(0, transformed.vertex1);
				let clip_space = projection.clone() * vsout.0.to_homogenous();
				if !test_point(clip_space.clone()) {
					continue;
				}
				(
					Point::from_vector(Vector3::from_homogenous(clip_space))
						.to_pixel_coordinate(target.area()),
					vsout.1,
				)
			},
			{
				let vsout = pipeline.vertex(0, transformed.vertex2);
				let clip_space = projection.clone() * vsout.0.to_homogenous();
				if !test_point(clip_space.clone()) {
					continue;
				}
				(
					Point::from_vector(Vector3::from_homogenous(clip_space))
						.to_pixel_coordinate(target.area()),
					vsout.1,
				)
			},
			{
				let vsout = pipeline.vertex(0, transformed.vertex3);
				let clip_space = projection.clone() * vsout.0.to_homogenous();
				if !test_point(clip_space.clone()) {
					continue;
				}
				(
					Point::from_vector(Vector3::from_homogenous(clip_space))
						.to_pixel_coordinate(target.area()),
					vsout.1,
				)
			},
		)
		.draw(target, pipeline);
	}
}
