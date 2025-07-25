use crate::graphics::{
	colour::Colour,
	draw::Draw,
	shapes_2d::{point::PixelCoordinate, triangle::Triangle2D},
	viewport::Viewport,
};

pub struct Polygon<'a> {
	points: &'a [PixelCoordinate],
}

impl<'a> Polygon<'a> {
	pub fn new(points: &'a [PixelCoordinate]) -> Self {
		Self { points }
	}
	pub fn to_triangles(&self) -> Vec<Triangle2D> {
		let mut triangles: Vec<Triangle2D> = Vec::with_capacity(self.points.len() - 2);
		let first = self.points[0];
		let len = self.points.len();
		for i in 1..(len - 1) {
			let current = self.points[i];
			let next = self.points[i + 1];
			triangles.push(Triangle2D::new(first, current, next));
		}
		triangles
	}
}

impl<'a> Draw for Polygon<'a> {
	fn draw(&self, viewport: &mut Viewport, screen: &mut crate::graphics::screen::Screen) {
		let triangles = self.to_triangles();
		for (i, triangle) in triangles.iter().enumerate() {
			screen.set_draw_colour(Colour::COLOURS[i % Colour::COLOURS.len()].clone());
			triangle.draw(viewport, screen);
		}
	}
}
