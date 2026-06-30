use criterion::{BatchSize::SmallInput, Criterion, criterion_group, criterion_main};
use rendy3d::{
	graphics::{
		colour::Colour,
		geometry::{clipping::SutherlandHodgman, point::AbsoluteScreenCoordinate},
		geometry_3d::{cube::Cube, point::Point},
		pipeline::Pipeline,
		screen::Screen,
	},
	render::render,
};
use rendy3d_maths::{
	matrices::matrix4::Matrix4, vector::vector3::Vector3,
};
use std::hint::black_box;
struct Bench {
	c: Colour,
}
impl Pipeline for Bench {
	type VsOut = Colour;
	type Vertex = Point;
	type Fragment = Colour;
	type ClippingStrategy = SutherlandHodgman;
	fn vertex(&self, _index: usize, vertex: Self::Vertex) -> (Point, Self::VsOut) {
		(vertex, self.c)
	}

	fn fragment(&self, _pos: AbsoluteScreenCoordinate, data: Self::VsOut) -> Self::Fragment {
		data
	}
	fn backface_culling() -> rendy3d::graphics::pipeline::back_face_culling::BackFaceCulling {
		rendy3d::graphics::pipeline::back_face_culling::BackFaceCulling::CullClockwise
	}
}
fn draw_triangle(c: Colour, target: &mut Screen, transform: Matrix4<f64>) -> u64 {
	// Draw the same triangle lots of times
	render(
		Cube::new(2.0),
		&mut Bench { c },
		target,
		transform,
		Matrix4::new_perspective(1.0, 1.0, 20.0, 0.1),
	);
	2
}

fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("draw cube in viewport", |b| {
		b.iter_batched_ref(
			|| {
				let width = 640;
				let height = 480;
				(
					vec![Colour::BLACK; width * height],
					vec![f32::INFINITY; width * height],
					width,
					height,
				)
			},
			|(display_buffer, z_buffer, width, height)| {
				draw_triangle(
					black_box(Colour::RED),
					&mut Screen::new(display_buffer, z_buffer, *width, *height),
					{
						Matrix4::scale_x(*height as f64 / *width as f64)
							* Matrix4::translation(Vector3::new(0.0, 0.0, -2.0))
							* Matrix4::scale(0.4)
						// Matrix4::identity()
					},
				)
			},
			SmallInput,
		)
	});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
