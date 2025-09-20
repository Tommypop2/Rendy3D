use crate::graphics::{shapes_2d::bounding_area::BoundingArea2D, target::Target};

pub struct Viewport {
	pub area: BoundingArea2D,
}

#[derive(Debug)]
pub enum ViewportCreationError {
	MaxXGreaterThanScreenSize,
	MaxYGreaterThanScreenSize,
}
impl Viewport {
	pub fn new(area: BoundingArea2D) -> Result<Self, ViewportCreationError> {
		// if area.max_x > WIDTH as usize {
		// 	return Err(ViewportCreationError::MaxXGreaterThanScreenSize);
		// }
		// if area.max_y > HEIGHT as usize {
		// 	return Err(ViewportCreationError::MaxYGreaterThanScreenSize);
		// }
		Ok(Self { area })
	}
	pub fn set_area(&mut self, area: BoundingArea2D) {
		self.area = area;
	}
	pub fn target<'a, T: Target>(&'a mut self, parent: &'a mut T) -> ViewportTarget<'a, T> {
		ViewportTarget::new(parent, &self.area)
	}
}

pub struct ViewportTarget<'a, T: Target> {
	area: &'a BoundingArea2D,
	parent_target: &'a mut T,
}
impl<'a, T> ViewportTarget<'a, T>
where
	T: Target,
{
	pub fn new(parent: &'a mut T, area: &'a BoundingArea2D) -> Self {
		Self {
			area,
			parent_target: parent,
		}
	}
}
impl<'a, T> Target for ViewportTarget<'a, T>
where
	T: Target,
{
	type Item = T::Item;

	fn set(&mut self, x: usize, y: usize, value: Self::Item) {
		self.parent_target.set(x, y, value);
	}

	fn get(&self, x: usize, y: usize) -> Self::Item {
		self.parent_target.get(x, y)
	}

	fn set_depth(&mut self, x: usize, y: usize, value: f32) {
		self.parent_target.set_depth(x, y, value);
	}

	fn get_depth(&self, x: usize, y: usize) -> f32 {
		self.parent_target.get_depth(x, y)
	}

	fn clear(&mut self, fill: Self::Item) {
		self.parent_target.clear(fill);
	}

	fn clear_depth(&mut self) {
		self.parent_target.clear_depth();
	}

	fn area(&self) -> BoundingArea2D {
		self.area.clone()
	}

	fn draw_colour(&self) -> Self::Item {
		self.parent_target.draw_colour()
	}
	fn set_draw_colour(&mut self, v: Self::Item) {
		self.parent_target.set_draw_colour(v);
	}
}
