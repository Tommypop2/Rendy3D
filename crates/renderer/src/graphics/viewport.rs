use crate::{
	HEIGHT, WIDTH,
	graphics::{
		draw::Draw,
		screen::Screen,
		shapes_2d::{bounding_area::BoundingArea2D, point::AbsoluteScreenCoordinate},
	},
};

pub struct Viewport {
	pub area: BoundingArea2D,
}
impl Default for Viewport {
	fn default() -> Self {
		Self {
			area: BoundingArea2D {
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
	pub fn new(area: BoundingArea2D) -> Result<Self, ViewportCreationError> {
		if area.max_x > WIDTH as usize {
			return Err(ViewportCreationError::MaxXGreaterThanScreenSize);
		}
		if area.max_y > HEIGHT as usize {
			return Err(ViewportCreationError::MaxYGreaterThanScreenSize);
		}
		Ok(Self { area })
	}
	pub fn set_area(&mut self, area: BoundingArea2D) {
		self.area = area;
	}
	pub fn contains_point(&self, point: AbsoluteScreenCoordinate) -> bool {
		let area = &self.area;
		point.x >= area.min_x
			&& point.x < area.max_x
			&& point.y >= area.min_y
			&& point.y < area.max_y
	}
	pub fn draw_point(&mut self, screen: &mut super::screen::Screen, point: AbsoluteScreenCoordinate) {
		// Don't draw if absolute point is outside of the viewport
		if !self.contains_point(point) {
			return;
		}
		screen.draw_point(point);
	}
	pub fn point_below_z_buffer(&self, screen: &Screen, p: AbsoluteScreenCoordinate) -> bool {
		if !self.contains_point(p) {
			return true;
		}
		let (_, _, z) = p.as_tuple();
		let buffered_z = screen.get_z_in_z_buffer(p);
		z < buffered_z && f32::abs(z - buffered_z) >= 0.001
	}
	pub fn draw_shape<T: Draw>(&mut self, screen: &mut super::screen::Screen, shape: T) {
		shape.draw(self, screen);
	}
}
