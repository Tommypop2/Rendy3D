use crate::graphics::{
	draw::Draw,
	screen::Screen,
	shapes_2d::{bounding_area::BoundingArea2D, line::Line, point::AbsoluteScreenCoordinate},
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
	pub fn doubled_area(&self) -> usize {
		let (x1, y1, _) = self.vertex1.as_tuple();
		let (x2, y2, _) = self.vertex2.as_tuple();
		let (x3, y3, _) = self.vertex3.as_tuple();

		i32::abs(
			x1 as i32 * (y2 as i32 - y3 as i32)
				+ x2 as i32 * (y3 as i32 - y1 as i32)
				+ x3 as i32 * (y1 as i32 - y2 as i32),
		) as usize
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
		if viewport.point_below_z_buffer(screen, self.vertex1)
			&& viewport.point_below_z_buffer(screen, self.vertex2)
			&& viewport.point_below_z_buffer(screen, self.vertex3)
		{
			return;
		}
		// println!("3");
		// viewport.draw_line(screen, self.vertex1, self.vertex2);
		// viewport.draw_line(screen, self.vertex2, self.vertex3);
		// viewport.draw_line(screen, self.vertex3, self.vertex1);
		// unsafe { TRIANGLE_RENDER_COUNT += 1 };
		Line::new(self.vertex1, self.vertex2).draw(viewport, screen);
		Line::new(self.vertex2, self.vertex3).draw(viewport, screen);
		Line::new(self.vertex3, self.vertex1).draw(viewport, screen);
		// println!("4");
		// Now need to fill in the triangle
		let bounding_area = self.bounding_area();
		// Iterate over all pixels that could possibly contain the triangle
		let abc = self.doubled_area();
		if abc == 0 {
			return;
		}
		for y in bounding_area.min_y..=bounding_area.max_y {
			for x in bounding_area.min_x..=bounding_area.max_x {
				let p = AbsoluteScreenCoordinate::new(
					x,
					y,
					self.vertex1.z.min(self.vertex2.z).min(self.vertex3.z),
				);
				let abp = Triangle2D::new(self.vertex1, self.vertex2, p).doubled_area();
				let bcp = Triangle2D::new(self.vertex2, self.vertex3, p).doubled_area();
				let acp = Triangle2D::new(self.vertex1, self.vertex3, p).doubled_area();

				if abc == abp + bcp + acp {
					// Point inside triangle, so draw
					// viewport.draw_point(screen, p);
					p.draw(viewport, screen);
				}
			}
		}
	}
}
