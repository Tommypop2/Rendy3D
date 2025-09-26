use core::f32;
use std::f64::consts::PI;
use std::time::{Instant, SystemTime};

use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use rendy3d::graphics::camera::Camera;
use rendy3d::graphics::colour::Colour;
use rendy3d::graphics::geometry::bounding_area::BoundingArea2D;
use rendy3d::graphics::geometry::point::AbsoluteScreenCoordinate;
use rendy3d::graphics::geometry_3d::point::Point;
use rendy3d::graphics::interpolate::{Interpolate, PerspectiveCorrectInterpolate};
use rendy3d::graphics::mesh::indexed::IndexedMesh;
use rendy3d::graphics::mesh::vertices::TexturedVertex;
use rendy3d::graphics::pipeline::Pipeline;
use rendy3d::graphics::screen::{Screen, frame_pixels};
use rendy3d::graphics::target::Target;
use rendy3d::graphics::texture::{ImageTexture, Texture};
use rendy3d::graphics::viewport::Viewport;
use rendy3d::maths::matrices::matrix4::Matrix4;
use rendy3d::maths::vector::vector2::Vector2;
use rendy3d::maths::vector::vector3::Vector3;
use rendy3d::render::render;
use rendy3d_loaders::obj::load_obj_indexed;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
struct World {
	pub cameras: Vec<Camera>,
	pub objects: Vec<IndexedMesh<TexturedVertex>>,
}
fn main() -> Result<(), Error> {
	env_logger::init();
	let event_loop = EventLoop::new().unwrap();
	let mut input = WinitInputHelper::new();
	let window = {
		let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
		WindowBuilder::new()
			.with_title("Rasterizer")
			.with_inner_size(size)
			.with_min_inner_size(size)
			.build(&event_loop)
			.unwrap()
	};

	let mut pixels = {
		let window_size = window.inner_size();
		let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
		Pixels::new(WIDTH, HEIGHT, surface_texture)?
	};
	let pers_mat = Matrix4::new_perspective(PI / 4.0, PI / 4.0, 20.0, 0.01);
	// let pers_mat = Matrix4::identity();
	// let p = Point::new(0.0, 0.0, 20.1);
	// let res = p.apply(pers_mat.clone());
	// dbg!(res);
	let viewport =
		Viewport::new(BoundingArea2D::new(0, WIDTH as usize, 0, HEIGHT as usize)).unwrap();
	let main_camera = Camera::new(viewport, pers_mat.clone())
		.with_transformation(Matrix4::translation(Vector3::new(0.0, 0.0, 3.0)));
	// let viewport2 = Viewport::new(BoundingArea2D::new(
	// 	(WIDTH / 2) as usize,
	// 	WIDTH as usize,
	// 	0,
	// 	HEIGHT as usize,
	// ))
	// .unwrap();
	// let second_camera = Camera::new(viewport2, pers_mat.clone());
	// let object = Mesh::new(vec![
	// 	Triangle3D::new(
	// 		Point::new(0.0, 0.0, 0.0),
	// 		Point::new(0.5, 0.0, 0.0),
	// 		Point::new(0.1, 0.4, 0.0),
	// 	),
	// 	Triangle3D::new(
	// 		Point::new(0.1, 0.4, 0.0),
	// 		Point::new(0.5, 0.0, 0.0),
	// 		Point::new(0.8, 0.4, 0.0),
	// 	),
	// ]);
	let object = load_obj_indexed("../obj-tests/potted_plant_04_4k.obj").unwrap();
	// let object = Mesh::new(load_file("./F1_RB16B.stl"));
	// let guinea_pig = Mesh::new(load_file("./GatlingGuineaPig.stl"));
	let mut scene = World::new(vec![main_camera], vec![object]);
	// let mut scene2 = World::new(vec![second_camera], vec![guinea_pig]);
	let mut frame_num: usize = 0;
	let mut sum: u128 = 0;
	let mut pipeline = Test {
		light_direction: Vector3::new(0.0, 0.0, 1.0),
		texture: ImageTexture::from_path("../obj-tests/potted_plant_04_diff_4k.jpg"),
	};
	// let pers_mat = Matrix4::unit();
	let mut z_buffer = vec![f32::INFINITY; { WIDTH * HEIGHT } as usize];
	let res = event_loop.run(|event, elwt| {
		let mut screen = Screen::new(
			frame_pixels(pixels.frame_mut()),
			&mut z_buffer,
			WIDTH as usize,
			HEIGHT as usize,
		);
		if let Event::WindowEvent {
			event: WindowEvent::RedrawRequested,
			..
		} = event
		{
			// Clear buffer
			let start = Instant::now();
			screen.clear(Colour::BLACK);
			scene.draw(&mut screen, &mut pipeline);
			// scene2.draw(&mut screen);
			let time_taken = start.elapsed();
			frame_num += 1;
			sum += time_taken.as_micros();
			if frame_num.is_multiple_of(1000) {
				//
				let mean = sum as f64 / frame_num as f64;
				frame_num = 0;
				sum = 0;
				println!(
					"Mean draw time taken over most recent 1000 frames is {mean} microseconds"
				);
				println!("This is {} FPS", 1E6 / mean)
			}

			if let Err(err) = pixels.render() {
				log_error("pixels.render", err);
				elwt.exit();
				return;
			}
		}

		if input.update(&event) {
			if input.key_pressed(KeyCode::Escape) || input.close_requested() {
				elwt.exit();
				return;
			}

			if let Some(size) = input.window_resized()
				&& let Err(err) = pixels.resize_surface(size.width, size.height)
			{
				log_error("pixels.resize_surface", err);
				elwt.exit();
				return;
			}

			scene.update();
			window.request_redraw();
		}
	});
	res.map_err(|e| Error::UserDefined(Box::new(e)))
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
	error!("{method_name}() failed: {err}");
	for source in err.sources().skip(1) {
		error!("  Caused by: {source}");
	}
}
struct Test {
	light_direction: Vector3<f64>,
	texture: ImageTexture,
}
impl Pipeline for Test {
	type VsOut = (PerspectiveCorrectInterpolate<Vector2<f64>>, f64);
	type Vertex = TexturedVertex;
	type Fragment = Colour;

	fn vertex(&self, _index: usize, vertex: Self::Vertex) -> (Point, Self::VsOut) {
		let intensity = vertex.normal.dot_with(&self.light_direction);
		// dbg!(&vertex);
		let z = vertex.position.z;
		(
			vertex.position,
			(
				PerspectiveCorrectInterpolate::new(vertex.texture, z),
				intensity / 1.5,
			),
		)
		// let res = index % 3;
		// match res {
		// 	0 => Colour::RED,
		// 	1 => Colour::GREEN,
		// 	2 => Colour::BLUE,
		// 	_ => unreachable!(),
		// }
	}

	fn fragment(&self, _pos: AbsoluteScreenCoordinate, data: Self::VsOut) -> Self::Fragment {
		let texture_coordinates = data.0.get();
		let base_colour = self
			.texture
			.get_texel(texture_coordinates.x as f32, texture_coordinates.y as f32);
		let intensity = data.1;
		// let intensity = 1.0;
		// println!("Fragment");
		Colour::new(
			(base_colour.red as f64 * intensity) as u8,
			(base_colour.green as f64 * intensity) as u8,
			(base_colour.blue as f64 * intensity) as u8,
			(base_colour.alpha as f64 * intensity) as u8,
		)
		// let (r, g, b) = hsv_to_rgb(
		// 	((pos.z + 1.0) * 360.0).clamp(0.0, 360.0) as f64 * 0.75,
		// 	1.0,
		// 	1.0,
		// );
		// Colour::new(r, g, b, 255)
	}
	fn backface_culling() -> rendy3d::graphics::pipeline::back_face_culling::BackFaceCulling {
		rendy3d::graphics::pipeline::back_face_culling::BackFaceCulling::CullClockwise
	}
}
impl World {
	fn new(cameras: Vec<Camera>, objects: Vec<IndexedMesh<TexturedVertex>>) -> Self {
		Self { objects, cameras }
	}

	fn update(&mut self) {}

	fn draw<
		U: Interpolate + Clone,
		T: Pipeline<VsOut = U, Fragment = Colour, Vertex = TexturedVertex>,
	>(
		&mut self,
		screen: &mut Screen,
		pipeline: &mut T,
	) {
		let x: std::time::Duration = SystemTime::now()
			.duration_since(SystemTime::UNIX_EPOCH)
			.unwrap();
		let tu64 = x.as_secs();
		let tf64 = x.as_secs_f64();
		let fract = tf64.fract();
		let rotation = Matrix4::rotation_z(x.as_secs_f64())
			* Matrix4::rotation_y(x.as_secs_f64())
			* Matrix4::rotation_x(x.as_secs_f64());
		let m = f64::sin(fract * PI);
		let base_transform = Matrix4::translation(if (tu64 - 1).is_multiple_of(3) {
			if fract < 0.01 {
				println!("X")
			}
			// x
			Vector3::new(m, 0.0, 0.0)
		} else if (tu64 - 2).is_multiple_of(3) {
			// y
			Vector3::new(0.0, m * 1.5, 0.0)
		} else {
			// z
			Vector3::new(0.0, 0.0, m)
		}) * Matrix4::scale(3.0)
			* rotation;
		for object in &self.objects {
			for camera in &mut self.cameras {
				let transform = camera.view()
					* Matrix4::scale_x(
						camera.viewport.area.height() as f64 / camera.viewport.area.width() as f64,
					) * base_transform.clone();
				let target: &mut rendy3d::graphics::viewport::ViewportTarget<'_, Screen<'_>> =
					&mut camera.viewport.target(screen);
				render(
					object.triangles(),
					pipeline,
					target,
					transform,
					camera.projection.clone(),
				);
			}
		}
	}
}
