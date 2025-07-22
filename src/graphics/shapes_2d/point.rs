use derive_more::{Add, Deref, DerefMut};

use crate::{
	graphics::{shapes_2d::triangle::Draw, shapes_3d::point::Point}, maths::vector::{vector2::Vector2, vector3::Vector3}, HEIGHT, WIDTH
};

#[derive(Deref, DerefMut, Clone, Add, Copy)]
pub struct PixelCoordinate(Vector3<usize>);
impl PixelCoordinate {
	pub fn new(x: usize, y: usize, z: usize) -> Self {
		Self(Vector3::new(x, y, z))
	}
}

impl From<Point> for PixelCoordinate {
	fn from(value: Point) -> Self {
		let offset = PixelCoordinate::new((WIDTH / 2) as usize, (HEIGHT / 2) as usize, 0);
		let x = (offset.x as f64 + value.x * (WIDTH as f64) / 100.0).round() as usize;
		let y = (offset.y as f64 + value.y * (HEIGHT as f64) / 100.0).round() as usize;
		Self::new(x, y, value.z as usize)
	}
}

impl Draw for PixelCoordinate {
	fn draw(
		&self,
		viewport: &mut crate::graphics::viewport::Viewport,
		screen: &mut crate::graphics::screen::Screen,
	) {
		// Record Z in Z buffer
		screen.z_buffer[self.y][self.x] = self.z as u16;
		screen.frame()[self.y][self.x] = screen.draw_colour.clone();
	}
}
