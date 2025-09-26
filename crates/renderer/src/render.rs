use core::ops::MulAssign;

use rendy3d_maths::vector::vector3::Vector3;
use rendy3d_maths::vector::vector4::Vector4;

use crate::graphics::geometry::clipping::{ClippingPlane, SutherlandHodgman, TriangleClipper};
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
	U: Interpolate + Clone,
	V: MulAssign<Matrix4<f64>> + Clone,
{
	for triangle in mesh {
		let transformed = triangle.apply(transform.clone());

		// Convert triangle to clip space
		let clip_space = transformed.map_vertices(|vertex| {
			let vsout = pipeline.vertex(0, vertex);
			let clip_space = projection.clone() * vsout.0.to_homogenous();
			(clip_space, vsout.1)
		});
		// Test each vertex for clipping
		if !(test_point(clip_space.vertex1.0)
			&& test_point(clip_space.vertex2.0)
			&& test_point(clip_space.vertex3.0))
		{
			// Clip!
			for t in SutherlandHodgman::clip(clip_space) {
				t.map_vertices(|(p, a)| {
					(
						Point::from_vector(Vector3::from_homogenous(p))
							.to_pixel_coordinate(target.area()),
						a,
					)
				})
				.draw(target, pipeline);
			}
		} else {
			// Convert to screen space
			clip_space
				.map_vertices(|(p, a)| {
					(
						Point::from_vector(Vector3::from_homogenous(p))
							.to_pixel_coordinate(target.area()),
						a,
					)
				})
				.draw(target, pipeline);
		}
	}
}
