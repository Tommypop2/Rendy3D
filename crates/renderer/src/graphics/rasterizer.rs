use rendy3d_maths::{
	geometry::triangle::Triangle, matrices::matrix2::Matrix2, vector::vector2::Vector2,
};

use crate::graphics::{
	draw::Draw,
	geometry::point::AbsoluteScreenCoordinate,
	interpolate::Interpolate,
	pipeline::{Pipeline, back_face_culling::BackFaceCulling},
	target::Target,
};

pub trait Rasterizer<Input, VsOut> {
	fn draw<T: Target, P: Pipeline<Fragment = T::Item, VsOut = VsOut>>(
		target: &mut T,
		pipeline: &mut P,
		item: Input,
	);
}
const fn absolute_screen_coordinate_to_2d_vec(p: AbsoluteScreenCoordinate) -> Vector2<i32> {
	Vector2::new(p.x as i32, p.y as i32)
}
#[inline]
fn is_between_0_and_1(x: f32) -> bool {
	(0.0..=1.0).contains(&x)
}

pub struct TriangleRasterizer;
impl<U> Rasterizer<Triangle<(AbsoluteScreenCoordinate, U)>, U> for TriangleRasterizer
where
	U: Interpolate,
{
	#[inline]
	fn draw<T: Target, P: Pipeline<Fragment = T::Item, VsOut = U>>(
		target: &mut T,
		pipeline: &mut P,
		item: Triangle<(AbsoluteScreenCoordinate, U)>,
	) {
		let shape = Triangle::new(item.vertex1.0, item.vertex2.0, item.vertex3.0);
		if !(target.contains_point(shape.vertex1)
			|| target.contains_point(shape.vertex2)
			|| target.contains_point(shape.vertex3))
		{
			return;
		}
		let v0 = absolute_screen_coordinate_to_2d_vec(shape.vertex1);
		let v1 = absolute_screen_coordinate_to_2d_vec(shape.vertex2);
		let v2 = absolute_screen_coordinate_to_2d_vec(shape.vertex3);
		let t2 = Triangle::new(v0, v1, v2);
		let bounding_area = t2.bounding_area();
		let abc = t2.signed_doubled_area();
		if match P::backface_culling() {
			BackFaceCulling::CullClockwise => abc >= 0,
			BackFaceCulling::CullAnticlockwise => abc <= 0,
			BackFaceCulling::None => abc == 0,
		} {
			return;
		}
		let mat = Matrix2::new(v1 - v0, v2 - v0).adjugate();
		let denom = abc as f32;
		// Iterate over all pixels that could possibly contain the triangle
		for y in bounding_area.min_y..=bounding_area.max_y {
			for x in bounding_area.min_x..=bounding_area.max_x {
				let Vector2 { x: l0, y: l1 } = mat * (Vector2::new(x as i32, y as i32) - v0);
				let l0 = l0 as f32 / denom;
				let l1 = l1 as f32 / denom;
				let l2 = 1.0 - l0 - l1;

				// Check if point is inside triangle
				if is_between_0_and_1(l0) && is_between_0_and_1(l1) && is_between_0_and_1(l2) {
					// Interpolate Z
					let z = shape.vertex1.z * l2 + shape.vertex2.z * l0 + shape.vertex3.z * l1;
					let p = AbsoluteScreenCoordinate::new(x, y, z);
					let out = P::VsOut::interpolate3(
						&item.vertex1.1,
						&item.vertex2.1,
						&item.vertex3.1,
						l2,
						l0,
						l1,
					);
					let colour = pipeline.fragment(p, out);
					target.set_draw_colour(colour);
					p.draw(target, pipeline);
				}
			}
		}
	}
}
