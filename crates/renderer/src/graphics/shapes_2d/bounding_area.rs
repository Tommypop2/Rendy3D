pub struct BoundingArea2D {
	pub min_x: usize,
	pub max_x: usize,
	pub min_y: usize,
	pub max_y: usize,
}
impl BoundingArea2D {
	pub fn new(min_x: usize, max_x: usize, min_y: usize, max_y: usize) -> Self {
		Self {
			min_x,
			max_x,
			min_y,
			max_y,
		}
	}
}
