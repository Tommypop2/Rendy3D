use crate::graphics::{
	colour::Colour,
	screen::Point,
	shapes::triangle::{Draw, Triangle2D}, viewport::Viewport,
};

pub struct Polygon<'a> {
	points: &'a [Point],
}

impl<'a> Polygon<'a> {
	pub fn new(points: &'a [Point]) -> Self {
		Self { points }
	}
	pub fn to_triangles(&self) -> Vec<Triangle2D> {
		let mut triangles: Vec<Triangle2D> = vec![];
		triangles.reserve(self.points.len() - 2);
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
