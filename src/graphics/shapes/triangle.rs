use crate::graphics::screen::{Point, Screen};
pub trait Draw {
	fn draw(&self, screen: &mut Screen);
}
pub struct Triangle {
	vertex1: Point,
	vertex2: Point,
	vertex3: Point,
}
pub struct BoundingArea {
	min_x: usize,
	max_x: usize,
	min_y: usize,
	max_y: usize,
}
impl Triangle {
	pub fn new(vertex1: Point, vertex2: Point, vertex3: Point) -> Self {
		Self {
			vertex1,
			vertex2,
			vertex3,
		}
	}
	fn bounding_area(&self) -> BoundingArea {
		let min_x = self.vertex1.x.min(self.vertex2.x).min(self.vertex3.x);
		let max_x = self.vertex1.x.max(self.vertex2.x).max(self.vertex3.x);
		let min_y = self.vertex1.y.min(self.vertex2.y).min(self.vertex3.y);
		let max_y = self.vertex1.y.max(self.vertex2.y).max(self.vertex3.y);
		BoundingArea {
			min_x,
			max_x,
			min_y,
			max_y,
		}
	}
}
impl Draw for Triangle {
	fn draw(&self, screen: &mut Screen) {
		screen.draw_line(self.vertex1, self.vertex2);
		screen.draw_line(self.vertex2, self.vertex3);
		screen.draw_line(self.vertex3, self.vertex1);
	}
}
