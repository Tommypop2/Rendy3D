use maths::matrices::matrix4::Matrix4;

use crate::graphics::viewport::Viewport;

pub struct Camera {
	/// Viewport which displays the camera view
	pub viewport: Viewport,
	/// Perspective matrix for projecting the objects to the screen
	pub perspective: Matrix4<f64>,
	/// Camera transformation
	pub transformation: Matrix4<f64>,
}
impl Camera {
	pub fn new(viewport: Viewport, perspective: Matrix4<f64>) -> Self {
		Self {
			viewport,
			perspective,
			transformation: Matrix4::identity(),
		}
	}
	pub fn with_transformation(mut self, transformation: Matrix4<f64>) -> Self {
		self.transformation = transformation;
		self
	}
	pub fn direction() {
		todo!()
	}
}

#[cfg(test)]
mod test {}
