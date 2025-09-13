use crate::maths::{matrices::matrix2::Matrix2, vector::vector2::Vector2};

use crate::graphics::interpolate::Interpolate;
use crate::graphics::pipeline::back_face_culling::BackFaceCulling;
use crate::graphics::{
	draw::Draw,
	pipeline::Pipeline,
	shapes_2d::{bounding_area::BoundingArea2D, point::AbsoluteScreenCoordinate},
	target::Target,
};
#[derive(Debug, Clone)]
pub struct Triangle<Vertex = AbsoluteScreenCoordinate> {
	pub vertex1: Vertex,
	pub vertex2: Vertex,
	pub vertex3: Vertex,
}
impl<Vertex> Triangle<Vertex> {
	pub const fn new(vertex1: Vertex, vertex2: Vertex, vertex3: Vertex) -> Self {
		Self {
			vertex1,
			vertex2,
			vertex3,
		}
	}
	pub fn map_vertices<T, U: Fn(Vertex) -> T>(self, map_fn: U) -> Triangle<T> {
		Triangle::new(
			map_fn(self.vertex1),
			map_fn(self.vertex2),
			map_fn(self.vertex3),
		)
	}
}
impl Triangle<AbsoluteScreenCoordinate> {
	pub fn signed_doubled_area(&self) -> i32 {
		let (x1, y1, _) = self.vertex1.as_tuple();
		let (x2, y2, _) = self.vertex2.as_tuple();
		let (x3, y3, _) = self.vertex3.as_tuple();

		x1 as i32 * (y2 as i32 - y3 as i32)
			+ x2 as i32 * (y3 as i32 - y1 as i32)
			+ x3 as i32 * (y1 as i32 - y2 as i32)
	}
	pub fn doubled_area(&self) -> usize {
		i32::abs(self.signed_doubled_area()) as usize
	}
	fn bounding_area(&self) -> BoundingArea2D {
		let min_x = self.vertex1.x.min(self.vertex2.x).min(self.vertex3.x);
		let max_x = self.vertex1.x.max(self.vertex2.x).max(self.vertex3.x);
		let min_y = self.vertex1.y.min(self.vertex2.y).min(self.vertex3.y);
		let max_y = self.vertex1.y.max(self.vertex2.y).max(self.vertex3.y);
		BoundingArea2D {
			min_x,
			max_x,
			min_y,
			max_y,
		}
	}
}
pub static mut TRIANGLE_RENDER_COUNT: usize = 0;

fn absolute_screen_coordinate_to_2d_vec(p: AbsoluteScreenCoordinate) -> Vector2<i32> {
	Vector2::new(p.x as i32, p.y as i32)
}
#[inline]
fn is_between_0_and_1(x: f32) -> bool {
	(0.0..=1.0).contains(&x)
}

impl<W> Draw<W> for Triangle<(AbsoluteScreenCoordinate, W)>
where
	W: Interpolate,
{
	fn draw<T: Target, P: Pipeline<VsOut = W, Fragment = T::Item>>(
		&self,
		target: &mut T,
		pipeline: &mut P,
	) {
		// println!("1");
		// Optimisation: If all vertices aren't visible, don't draw
		let shape = Triangle::new(self.vertex1.0, self.vertex2.0, self.vertex3.0);
		if !(target.contains_point(shape.vertex1)
			|| target.contains_point(shape.vertex2)
			|| target.contains_point(shape.vertex3))
		{
			return;
		}
		let bounding_area = shape.bounding_area();
		let abc = shape.signed_doubled_area();
		if match P::backface_culling() {
			BackFaceCulling::CullClockwise => abc >= 0,
			BackFaceCulling::CullAnticlockwise => abc <= 0,
			BackFaceCulling::None => abc == 0,
		} {
			return;
		}
		let v0 = absolute_screen_coordinate_to_2d_vec(shape.vertex1);
		let v1 = absolute_screen_coordinate_to_2d_vec(shape.vertex2);
		let v2 = absolute_screen_coordinate_to_2d_vec(shape.vertex3);
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
						&self.vertex1.1,
						&self.vertex2.1,
						&self.vertex3.1,
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
