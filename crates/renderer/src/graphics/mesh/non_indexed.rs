//! Non-indexed mesh

use crate::graphics::geometry_3d::triangle::Triangle3D;

/// Non-indexed mesh type
///
/// Triangles are stored in a single [`Vec`], so vertices may be duplicated if they are shared between multiple triangles
pub struct Mesh {
	/// Triangles that make up the shape
	pub triangles: Vec<Triangle3D>,
}
// #[derive(Default)]
// pub struct Dimensions {
// 	min_x: f64,
// 	min_y: f64,
// 	min_z: f64,
// 	max_x: f64,
// 	max_y: f64,
// 	max_z: f64,
// }
impl Mesh {
	pub fn new(triangles: Vec<Triangle3D>) -> Self {
		Self { triangles }
	}
	pub fn dimensions(&self) {
		todo!();
	}
}
