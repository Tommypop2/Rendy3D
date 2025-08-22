//! Implements the generation of a 4x4 perspective projection matrix

use maths::{matrices::matrix4::Matrix4, vector::vector4::Vector4};

pub fn perspective_matrix(fov_x: f64, fov_y: f64, far: f64, near: f64) -> Matrix4<f64> {
	Matrix4::new(
		Vector4::new(1.0 / f64::tan(fov_x / 2.0), 0.0, 0.0, 0.0),
		Vector4::new(0.0, 1.0 / f64::tan(fov_y / 2.0), 0.0, 0.0),
		// Should be -1.0 here for w but 1.0 seems to make things work for some reason
		// TODO: Look into why this is the case
		Vector4::new(0.0, 0.0, -((far + near) / (far - near)), -1.0),
		Vector4::new(0.0, 0.0, -2.0 * (far * near) / (far - near), 0.0),
	)
}
