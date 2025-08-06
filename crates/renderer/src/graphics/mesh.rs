use maths::{matrices::matrix4::Matrix4, vector::vector3::Vector3};

use crate::graphics::{screen::Screen, shaders::vertex::VertexShader, shapes_3d::triangle::Triangle3D, viewport::Viewport};

pub struct Mesh {
	/// Triangles that make up the shape
	pub triangles: Vec<Triangle3D>,
}
#[derive(Default)]
pub struct Dimensions {
	min_x: f64,
	min_y: f64,
	min_z: f64,
	max_x: f64,
	max_y: f64,
	max_z: f64,
}
impl Mesh {
	pub fn new(triangles: Vec<Triangle3D>) -> Self {
		Self { triangles }
	}
	pub fn dimensions(&self) {
		todo!();
		let first = self.triangles[0].clone();
		// let initial = Dimensions {
		// 	min_x: first
		// }
		self.triangles
			.iter()
			.fold((0.0, 0.0, 0.0), |dimensions, element| {
				let (width_x, width_y, width_z) = dimensions;

				dimensions
			});
	}
}

pub fn render_mesh(
	viewport: &mut Viewport,
	screen: &mut Screen,
	mesh: &[Triangle3D],
	transform: Matrix4<f64>,
	perspective: Matrix4<f64>,
	shader: &mut VertexShader<Vector3<f64>>,
) {
	let camera_dir = Vector3::new(0.0, 0.0, 1.0);
	for (i, triangle) in mesh.iter().enumerate() {
		let transformed = triangle.clone().apply(transform.clone());
		let n = transformed.normal().normalized();
		let intensity = n.dot_with(&camera_dir);
		// Back-face culling :)
		if intensity < 0.0 {
			continue;
		}
		// Only apply shader to single vertex (as all normals are the same)
		screen.set_draw_colour(shader.execute(i, triangle.vertex1, n));
		viewport.draw_shape(screen, transformed.apply(perspective.clone()))
	}
}
