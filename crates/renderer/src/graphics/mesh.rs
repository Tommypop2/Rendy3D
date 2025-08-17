use maths::{matrices::matrix4::Matrix4, vector::vector3::Vector3};

use crate::graphics::{
	draw::Draw, interpolate::Interpolate, pipeline::pipeline::Pipeline, shapes_2d::triangle::Triangle, shapes_3d::{point::Point, triangle::Triangle3D}, target::Target
};

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
		// let first = self.triangles[0].clone();
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

pub fn render_mesh<
	T: Target,
	S: Pipeline<VsOut = T::Item, Fragment = T::Item, Vertex = Point> + Clone,
>(
	target: &mut T,
	mesh: &[Triangle3D],
	transform: Matrix4<f64>,
	perspective: Matrix4<f64>,
	shaders: &mut S,
) where
	<T as Target>::Item: Interpolate,
{
	let camera_dir = Vector3::new(0.0, 0.0, 1.0);
	for (i, triangle) in mesh.iter().enumerate() {
		let transformed = triangle.clone().apply(transform.clone());
		let n = transformed.normal().normalized();
		let intensity = n.dot_with(&camera_dir);
		// Back-face culling :)
		if intensity < 0.0 {
			continue;
		}
		let projected = transformed.clone().apply(perspective.clone());
		let shaded = projected
			.map_vertices(|p| (p.to_pixel_coordinate(target.area()), shaders.vertex(i, p)));
		shaded.draw(target, shaders);
		// transformed
		// 	.apply(perspective.clone())
		// 	.to_triangle_2d(target, shaders, n)
		// 	.draw(target, shaders);
	}
}
