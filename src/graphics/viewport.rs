use crate::{
	HEIGHT, WIDTH,
	graphics::shapes_2d::{
		point::PixelCoordinate,
		triangle::{BoundingArea, Draw},
	},
};

pub struct Viewport {
	area: BoundingArea,
}
impl Default for Viewport {
	fn default() -> Self {
		Self {
			area: BoundingArea {
				min_x: 0,
				max_x: WIDTH as usize,
				min_y: 0,
				max_y: HEIGHT as usize,
			},
		}
	}
}
#[derive(Debug)]
pub enum ViewportCreationError {
	MaxXGreaterThanScreenSize,
	MaxYGreaterThanScreenSize,
}
impl Viewport {
	pub fn new(area: BoundingArea) -> Result<Self, ViewportCreationError> {
		if area.max_x > WIDTH as usize {
			return Err(ViewportCreationError::MaxXGreaterThanScreenSize);
		}
		if area.max_y > HEIGHT as usize {
			return Err(ViewportCreationError::MaxYGreaterThanScreenSize);
		}
		Ok(Self { area })
	}
	pub fn set_area(&mut self, area: BoundingArea) {
		self.area = area;
	}
	pub fn contains_point(&self, point: PixelCoordinate) -> bool {
		let area = &self.area;
		point.x >= area.min_x && point.x < area.max_x && point.y >= area.min_y && point.y < area.max_y
	}
	pub fn draw_point(&mut self, screen: &mut super::screen::Screen, point: PixelCoordinate) {
		let offset = PixelCoordinate::new(self.area.min_x, self.area.min_y);
		let p = point + offset;
		if !self.contains_point(p) {
			return
		}
		screen.draw_point(p);
	}
	fn draw_line_low(
		&mut self,
		screen: &mut super::screen::Screen,
		start: PixelCoordinate,
		end: PixelCoordinate,
	) {
		let dx = (end.x - start.x) as i32;
		let (dy, yi) = {
			let dy = end.y as i32 - start.y as i32;
			if dy < 0 { (-dy, -1) } else { (dy, 1) }
		};
		let mut d = 2 * dy - dx;
		let mut y = start.y as i32;
		for x in start.x..=end.x {
			self.draw_point(screen, PixelCoordinate::new(x, y as usize));
			if d > 0 {
				y += yi;
				d += 2 * (dy - dx)
			} else {
				d += 2 * dy
			}
		}
	}
	fn draw_line_high(
		&mut self,
		screen: &mut super::screen::Screen,
		start: PixelCoordinate,
		end: PixelCoordinate,
	) {
		let (dx, xi) = {
			let dx = end.x as i32 - start.x as i32;
			if dx < 0 { (-dx, -1) } else { (dx, 1) }
		};
		let dy = (end.y - start.y) as i32;
		let mut d = 2 * dx - dy;
		let mut x = start.x as i32;
		for y in start.y..=end.y {
			self.draw_point(screen, PixelCoordinate::new(x as usize, y));
			if d > 0 {
				x += xi;
				d += 2 * (dx - dy)
			} else {
				d += 2 * dx
			}
		}
	}
	pub fn draw_line(
		&mut self,
		screen: &mut super::screen::Screen,
		start: PixelCoordinate,
		end: PixelCoordinate,
	) {
		if usize::abs_diff(start.y, end.y) < usize::abs_diff(start.x, end.x) {
			if end.x > start.x {
				self.draw_line_low(screen, start, end);
			} else {
				self.draw_line_low(screen, end, start);
			}
		} else if end.y > start.y {
			self.draw_line_high(screen, start, end);
		} else {
			self.draw_line_high(screen, end, start);
		}
	}
	pub fn draw_shape<T: Draw>(&mut self, screen: &mut super::screen::Screen, shape: T) {
		shape.draw(self, screen);
	}
}
