use derive_more::{Add, Deref, DerefMut};

use crate::{HEIGHT, WIDTH, graphics::shapes_3d::point::Point, maths::vector::vector2::Vector2};

#[derive(Deref, DerefMut, Clone, Add, Copy)]
pub struct PixelCoordinate(Vector2<usize>);
impl PixelCoordinate {
	pub fn new(x: usize, y: usize) -> Self {
		Self(Vector2::new(x, y))
	}
}

impl From<Point> for PixelCoordinate {
	fn from(value: Point) -> Self {
		let offset = PixelCoordinate::new((WIDTH / 2) as usize, (HEIGHT / 2) as usize);
		let x = (offset.x as f64 + value.x * WIDTH as f64).round() as usize;
		let y = (offset.y as f64 + value.y * WIDTH as f64).round() as usize;
		Self::new(x, y)
	}
}
