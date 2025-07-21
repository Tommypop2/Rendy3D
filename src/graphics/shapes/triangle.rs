use crate::graphics::{
	screen::{Point, Screen},
	viewport::Viewport,
};
pub trait Draw {
	fn draw(&self, viewport: &mut Viewport, screen: &mut Screen);
}
pub struct Triangle2D {
	vertex1: Point,
	vertex2: Point,
	vertex3: Point,
}
pub struct BoundingArea {
	pub min_x: usize,
	pub max_x: usize,
	pub min_y: usize,
	pub max_y: usize,
}
impl BoundingArea {
	pub fn new(min_x: usize, max_x: usize, min_y: usize, max_y: usize) -> Self {
		Self {
			min_x,
			max_x,
			min_y,
			max_y,
		}
	}
}
impl Triangle2D {
	pub fn new(vertex1: Point, vertex2: Point, vertex3: Point) -> Self {
		Self {
			vertex1,
			vertex2,
			vertex3,
		}
	}
	pub fn doubled_area(&self) -> usize {
		let (x1, y1) = self.vertex1.as_tuple();
		let (x2, y2) = self.vertex2.as_tuple();
		let (x3, y3) = self.vertex3.as_tuple();

		i32::abs(
			x1 as i32 * (y2 as i32 - y3 as i32)
				+ x2 as i32 * (y3 as i32 - y1 as i32)
				+ x3 as i32 * (y1 as i32 - y2 as i32),
		) as usize
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
impl Draw for Triangle2D {
	fn draw(&self, viewport: &mut Viewport, screen: &mut Screen) {
		viewport.draw_line(screen, self.vertex1, self.vertex2);
		viewport.draw_line(screen, self.vertex2, self.vertex3);
		viewport.draw_line(screen, self.vertex3, self.vertex1);
		// Now need to fill in the triangle
		let bounding_area = self.bounding_area();
		// Iterate over all pixels that could possible contain the triangle
		let abc = self.doubled_area();
		for y in bounding_area.min_y..=bounding_area.max_y {
			for x in bounding_area.min_x..=bounding_area.max_x {
				let p = Point::new(x, y);
				let abp = Triangle2D::new(self.vertex1, self.vertex2, p).doubled_area();
				let bcp = Triangle2D::new(self.vertex2, self.vertex3, p).doubled_area();
				let acp = Triangle2D::new(self.vertex1, self.vertex3, p).doubled_area();

				if abc == abp + bcp + acp {
					// Point inside triangle, so draw
					viewport.draw_point(screen, p);
				}
			}
		}
	}
}
