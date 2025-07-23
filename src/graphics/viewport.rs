use std::thread::Scope;

use crate::{
	HEIGHT, WIDTH,
	graphics::{
		screen::Screen,
		shapes_2d::{
			point::PixelCoordinate,
			triangle::{BoundingArea, Draw},
		},
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
		point.x >= area.min_x
			&& point.x < area.max_x
			&& point.y >= area.min_y
			&& point.y < area.max_y
	}
	pub fn draw_point(&mut self, screen: &mut super::screen::Screen, point: PixelCoordinate) {
		let offset = PixelCoordinate::new(self.area.min_x, self.area.min_y, 0.0);
		let p = point + offset;
		if !self.contains_point(p) {
			return;
		}
		screen.draw_point(p);
	}
	pub fn point_below_z_buffer(&self, screen: &Screen, p: PixelCoordinate) -> bool {
		if !self.contains_point(p) {
			return true;
		}
		let (_, _, z) = p.as_tuple();
		let buffered_z = screen.get_z_in_z_buffer(p);
		if z < buffered_z && f32::abs(z - buffered_z) >= 0.01 {
			true
		} else {
			false
		}
	}
	pub fn draw_shape<T: Draw>(&mut self, screen: &mut super::screen::Screen, shape: T) {
		shape.draw(self, screen);
	}
}
