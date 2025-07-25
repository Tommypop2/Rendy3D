use crate::graphics::{
	draw::Draw, screen::Screen, shapes_2d::point::PixelCoordinate, viewport::Viewport,
};

pub struct Line {
	start: PixelCoordinate,
	end: PixelCoordinate,
}
impl Line {
	pub fn new(start: PixelCoordinate, end: PixelCoordinate) -> Self {
		Self { start, end }
	}
	fn draw_line_low(
		viewport: &mut Viewport,
		screen: &mut Screen,
		start: PixelCoordinate,
		end: PixelCoordinate,
	) {
		let min_z = start.z.min(end.z);
		let dx = (end.x - start.x) as i32;
		let (dy, yi) = {
			let dy = end.y as i32 - start.y as i32;
			if dy < 0 { (-dy, -1) } else { (dy, 1) }
		};
		let mut d = 2 * dy - dx;
		let mut y = start.y as i32;
		for x in start.x..=end.x {
			PixelCoordinate::new(x, y as usize, min_z).draw(viewport, screen);
			if d > 0 {
				y += yi;
				d += 2 * (dy - dx)
			} else {
				d += 2 * dy
			}
		}
	}
	fn draw_line_high(
		viewport: &mut Viewport,
		screen: &mut Screen,
		start: PixelCoordinate,
		end: PixelCoordinate,
	) {
		let min_z = start.z.min(end.z);
		let (dx, xi) = {
			let dx = end.x as i32 - start.x as i32;
			if dx < 0 { (-dx, -1) } else { (dx, 1) }
		};
		let dy = (end.y - start.y) as i32;
		let mut d = 2 * dx - dy;
		let mut x = start.x as i32;
		for y in start.y..=end.y {
			PixelCoordinate::new(x as usize, y, min_z).draw(viewport, screen);
			if d > 0 {
				x += xi;
				d += 2 * (dx - dy)
			} else {
				d += 2 * dx
			}
		}
	}
	pub fn draw_line(
		&self,
		viewport: &mut Viewport,
		screen: &mut Screen,
		start: PixelCoordinate,
		end: PixelCoordinate,
	) {
		if usize::abs_diff(start.y, end.y) < usize::abs_diff(start.x, end.x) {
			if end.x > start.x {
				Self::draw_line_low(viewport, screen, start, end);
			} else {
				Self::draw_line_low(viewport, screen, end, start);
			}
		} else if end.y > start.y {
			Self::draw_line_high(viewport, screen, start, end);
		} else {
			Self::draw_line_high(viewport, screen, end, start);
		}
	}
}
impl Draw for Line {
	fn draw(
		&self,
		viewport: &mut crate::graphics::viewport::Viewport,
		screen: &mut crate::graphics::screen::Screen,
	) {
		if viewport.point_below_z_buffer(screen, self.start)
			&& viewport.point_below_z_buffer(screen, self.end)
		{
			return;
		}
		self.draw_line(viewport, screen, self.start, self.end);
	}
}
