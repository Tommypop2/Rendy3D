use crate::graphics::shapes_2d::{bounding_area::BoundingArea2D, point::AbsoluteScreenCoordinate};

/// Target that we can render to
/// Includes basic functions for manipulating a buffer
pub trait Target {
	type Item: Clone + Default;
	fn set(&mut self, x: usize, y: usize, value: Self::Item);
	fn get(&self, x: usize, y: usize) -> Self::Item;
	fn set_depth(&mut self, x: usize, y: usize, value: f32);
	fn get_depth(&self, x: usize, y: usize) -> f32;
	fn clear(&mut self, fill: Self::Item);
	fn clear_depth(&mut self);
	fn area(&self) -> BoundingArea2D;

	// Default implementations

	fn draw_point(&mut self, p: AbsoluteScreenCoordinate, colour: Self::Item) {
		self.set(p.x, p.y, colour);
	}
	fn contains_point(&self, point: AbsoluteScreenCoordinate) -> bool {
		let area = &self.area();
		point.x >= area.min_x
			&& point.x < area.max_x
			&& point.y >= area.min_y
			&& point.y < area.max_y
	}
	fn point_below_z_buffer(&self, p: AbsoluteScreenCoordinate) -> bool {
		if !self.contains_point(p) {
			return true;
		}
		let (_, _, z) = p.as_tuple();
		let buffered_z = self.get_depth(p.x, p.y);
		z < buffered_z && f32::abs(z - buffered_z) >= 0.00001
	}
	fn set_z_in_z_buffer(&mut self, p: AbsoluteScreenCoordinate) {
		self.set_depth(p.x, p.y, p.z);
	}
	fn draw_colour(&self) -> Self::Item;
	fn set_draw_colour(&mut self, v: Self::Item);
}
