use rendy3d_maths::geometry::triangle::Triangle;
use rendy3d_maths::vector::vector3::Vector3;
use rendy3d_maths::vector::vector4::Vector4;

use crate::graphics::geometry::clipping::TriangleClipper;
use crate::graphics::geometry_3d::point::Point;
use crate::graphics::rasterizer::Rasterizer;
use crate::graphics::rasterizer::triangle_rasterizer::TriangleRasterizer;

use crate::graphics::{interpolate::Interpolate, pipeline::Pipeline, target::Target};

/// Tests a given point for whether it's within the view frustum
///
/// Returns `true` if the point is within the view frustum, and `false` if it isn't
fn test_point(p: Vector4<f64>) -> bool {
	let (x, y, z, w) = p.as_tuple();
	(-w <= x && x <= w) && (-w <= y && y <= w) && (-w <= z && z <= w)
}

pub fn render<M, P, T, U, V, F, W>(mesh: M, pipeline: &mut P, target: &mut T, state: W)
where
	M: IntoIterator<Item = Triangle<V>>,
	P: Pipeline<VsOut = U, Fragment = F, Vertex = V, VsIn = W>,
	T: Target<Item = F>,
	U: Interpolate + Clone,
	W: Clone,
{
	for triangle in mesh {
		// Convert triangle to clip space
		let clip_space = triangle.map_vertices(|vertex| {
			let vsout = pipeline.vertex(0, vertex, state.clone());
			// Triangle now in clip space after vertex shader application
			let clip_space = vsout.0.to_homogenous();
			(clip_space, vsout.1)
		});
		// Test each vertex for clipping
		let vertex_test = [
			test_point(clip_space.vertex1.0),
			test_point(clip_space.vertex2.0),
			test_point(clip_space.vertex3.0),
		];
		// If all vertices are outside of the view frustum, discard triangle entirely
		if !(vertex_test[0] || vertex_test[1] || vertex_test[2]) {
			continue;
		}
		// If any vertex is outside the view frustum, clip
		else if !(vertex_test[0] && vertex_test[1] && vertex_test[2]) {
			// Clip!
			for t in P::ClippingStrategy::clip(clip_space) {
				TriangleRasterizer::draw(
					target,
					pipeline,
					t.map_vertices(|(p, a)| {
						(
							Point::from_vector(Vector3::from_homogenous(p))
								.to_pixel_coordinate(target.area()),
							a,
						)
					}),
				);
			}
		}
		// If all vertices are inside the viewing frustum, render
		else {
			// Convert to screen space
			TriangleRasterizer::draw(
				target,
				pipeline,
				clip_space.map_vertices(|(p, a)| {
					(
						Point::from_vector(Vector3::from_homogenous(p))
							.to_pixel_coordinate(target.area()),
						a,
					)
				}),
			);
		}
	}
}
