use maths::{matrices::matrix2::Matrix2, vector::vector2::Vector2};

use crate::graphics::{
	colour::Colour,
	draw::Draw,
	screen::Screen,
	shapes_2d::{bounding_area::BoundingArea2D, point::AbsoluteScreenCoordinate},
	viewport::Viewport,
};

pub struct Triangle2D {
	vertex1: AbsoluteScreenCoordinate,
	vertex2: AbsoluteScreenCoordinate,
	vertex3: AbsoluteScreenCoordinate,
}

impl Triangle2D {
	pub fn new(
		vertex1: AbsoluteScreenCoordinate,
		vertex2: AbsoluteScreenCoordinate,
		vertex3: AbsoluteScreenCoordinate,
	) -> Self {
		Self {
			vertex1,
			vertex2,
			vertex3,
		}
	}
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
fn absolute_screen_coordinate_to_2d_vec(p: AbsoluteScreenCoordinate) -> Vector2<i16> {
	Vector2::new(p.x as i16, p.y as i16)
}
fn is_between_0_and_1(x: f32) -> bool {
	x >= 0.0 && x <= 1.0
}
impl Draw for Triangle2D {
	fn draw(&self, viewport: &mut Viewport, screen: &mut Screen) {
		// println!("1");
		// Optimisation: If all vertices aren't visible, don't draw
		if !(viewport.contains_point(self.vertex1)
			|| viewport.contains_point(self.vertex2)
			|| viewport.contains_point(self.vertex3))
		{
			return;
		}
		// println!("2");
		// If all vertices are below the current pixels in the Z buffer, also don't draw
		// Can't do this optimisation as some triangles may still be visible, even with 
		// all vertices below the Z buffer
		// if viewport.point_below_z_buffer(screen, self.vertex1)
		// 	&& viewport.point_below_z_buffer(screen, self.vertex2)
		// 	&& viewport.point_below_z_buffer(screen, self.vertex3)
		// {
		// 	return;
		// }
		// println!("3");
		// viewport.draw_line(screen, self.vertex1, self.vertex2);
		// viewport.draw_line(screen, self.vertex2, self.vertex3);
		// viewport.draw_line(screen, self.vertex3, self.vertex1);
		// unsafe { TRIANGLE_RENDER_COUNT += 1 };
		// Line::new(self.vertex1, self.vertex2).draw(viewport, screen);
		// Line::new(self.vertex2, self.vertex3).draw(viewport, screen);
		// Line::new(self.vertex3, self.vertex1).draw(viewport, screen);
		// println!("4");
		// Now need to fill in the triangle
		let bounding_area = self.bounding_area();
		let abc = self.signed_doubled_area();
		if abc == 0 {
			return;
		}
		let v0 = absolute_screen_coordinate_to_2d_vec(self.vertex1);
		let v1 = absolute_screen_coordinate_to_2d_vec(self.vertex2);
		let v2 = absolute_screen_coordinate_to_2d_vec(self.vertex3);
		let mat = Matrix2::new(v1 - v0, v2 - v0).adjugate();
		let denom = abc as f32;
		// Iterate over all pixels that could possibly contain the triangle
		for y in bounding_area.min_y..=bounding_area.max_y {
			for x in bounding_area.min_x..=bounding_area.max_x {
				let Vector2 { x: l0, y: l1 } = mat * (Vector2::new(x as i16, y as i16) - v0);
				let l0 = l0 as f32 / denom;
				let l1 = l1 as f32 / denom;
				let l2 = 1.0 - l0 - l1;

				if is_between_0_and_1(l0) && is_between_0_and_1(l1) && is_between_0_and_1(l2) {
					let z = self.vertex1.z * l0 + self.vertex2.z * l1 + self.vertex3.z * l2;
					let p = AbsoluteScreenCoordinate::new(x, y, z);
					// Point inside triangle, so draw
					// viewport.draw_point(screen, p);
					screen.set_draw_colour(Colour::new(
						(255.0 * l0) as u8,
						(255.0 * l1) as u8,
						(255.0 * l2) as u8,
						0xff,
					));
					p.draw(viewport, screen);
				}
			}
		}
	}
}
