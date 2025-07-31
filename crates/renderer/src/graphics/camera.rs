use maths::matrices::matrix4::Matrix4;

use crate::graphics::{
	draw::Draw, screen::Screen, shapes_3d::triangle::Triangle3D, viewport::Viewport,
};

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
			transformation: Matrix4::unit(),
		}
	}
}
