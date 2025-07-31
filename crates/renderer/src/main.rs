use std::time::{Instant, SystemTime};

use error_iter::ErrorIter as _;
use log::error;
use maths::matrices::matrix4::Matrix4;
use maths::vector::vector3::Vector3;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use crate::graphics::camera::Camera;
use crate::graphics::colour::Colour;
use crate::graphics::mesh::Mesh;
use crate::graphics::perspective::perspective_matrix;
use crate::graphics::screen::Screen;
use crate::graphics::shapes_2d::bounding_area::BoundingArea2D;
use crate::graphics::shapes_3d::triangle::Triangle3D;
use crate::graphics::viewport::Viewport;
use crate::loaders::stl::load_file;
pub mod graphics;
pub mod loaders;
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

struct World {
	pub cameras: Vec<Camera>,
	pub objects: Vec<Mesh>,
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

	let pixels = {
		let window_size = window.inner_size();
		let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
		Pixels::new(WIDTH, HEIGHT, surface_texture)?
	};
	let pers_mat = perspective_matrix(1.0, 1.0, -20.0, 1.0);
	let mut screen = Screen::new(pixels);
	let viewport = Viewport::new(BoundingArea2D::new(
		0,
		(WIDTH as usize) / 2,
		0,
		HEIGHT as usize,
	))
	.unwrap();
	let main_camera = Camera::new(viewport, pers_mat.clone()).with_transformation(Matrix4::rotation_z(0.5));
	let viewport2 = Viewport::new(BoundingArea2D::new(
		(WIDTH / 2) as usize,
		WIDTH as usize,
		0,
		HEIGHT as usize,
	))
	.unwrap();
	let second_camera = Camera::new(viewport2, pers_mat.clone());
	let f1_car = Mesh::new(load_file("./F1_RB16B.stl"));
	let guinea_pig = Mesh::new(load_file("./GatlingGuineaPig.stl"));
	let mut scene = World::new(vec![main_camera], vec![f1_car]);
	let mut scene2 = World::new(vec![second_camera], vec![guinea_pig]);
	let mut frame_num: usize = 0;
	let mut sum: u128 = 0;

	// let pers_mat = Matrix4::unit();
	let res = event_loop.run(|event, elwt| {
		if let Event::WindowEvent {
			event: WindowEvent::RedrawRequested,
			..
		} = event
		{
			// Clear buffer
			screen.clear(Colour::BLACK);
			let start = Instant::now();
			scene.draw(&mut screen);
			scene2.draw(&mut screen);
			let time_taken = start.elapsed();
			frame_num += 1;
			sum += time_taken.as_micros();
			if frame_num % 1000 == 0 {
				//
				let mean = sum as f64 / frame_num as f64;
				frame_num = 0;
				sum = 0;
				println!(
					"Mean draw time taken over most recent 1000 frames is {mean} microseconds"
				);
				println!("This is {} FPS", 1E6 / mean)
			}
			if let Err(err) = screen.pixels.render() {
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

			if let Some(size) = input.window_resized() {
				if let Err(err) = screen.pixels.resize_surface(size.width, size.height) {
					log_error("pixels.resize_surface", err);
					elwt.exit();
					return;
				}
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

impl World {
	fn new(cameras: Vec<Camera>, objects: Vec<Mesh>) -> Self {
		Self { objects, cameras }
	}

	fn update(&mut self) {}

	fn draw(&mut self, screen: &mut Screen) {
		let x: std::time::Duration = SystemTime::now()
			.duration_since(SystemTime::UNIX_EPOCH)
			.unwrap();
		let base_transform = Matrix4::translation(Vector3::new(0.0, 0.0, -1.0))
			// * Matrix4::rotation_z(x.as_secs_f64())
			// * Matrix4::rotation_y(x.as_secs_f64())
			// * Matrix4::rotation_x(x.as_secs_f64())
			* Matrix4::scale(0.01);
		for mesh in &self.objects {
			for camera in &mut self.cameras {
				let transform = camera.perspective.clone()
					* camera.transformation.reverse_rotation_translation()
					* Matrix4::scale_x(
						camera.viewport.area.height() as f64 / camera.viewport.area.width() as f64,
					) * base_transform.clone();
				render_mesh(&mut camera.viewport, screen, &mesh.triangles, transform);
			}
		}
	}
}
fn render_mesh(
	viewport: &mut Viewport,
	screen: &mut Screen,
	mesh: &[Triangle3D],
	transform: Matrix4<f64>,
) {
	let light_dir = Vector3::new(0.0, 0.0, 1.0);
	for (i, triangle) in mesh.iter().enumerate() {
		let transformed = triangle.clone().apply(transform.clone());
		let n = transformed.normal();
		let intensity = n.normalized().dot_with(&light_dir);
		// Back-face culling :)
		if intensity < 0.0 {
			continue;
		}
		let val = (255.0 * intensity) as u8;
		screen.set_draw_colour(Colour::new(val, val, val, 0xff));
		// let mut colour = Colour::COLOURS[(i) % Colour::COLOURS.len()].clone();
		// colour.alpha = val;
		// screen.set_draw_colour(colour);
		// let perspectified = transformed.apply(perspective_matrix.clone());
		// println!("{:?}", perspectified);
		viewport.draw_shape(screen, transformed)
	}
}
const fn frame_pixels(frame: &mut [u8]) -> &mut [[Colour; WIDTH as usize]] {
	// SAFETY: Format for each pixel matches the layout of the `Colour` struct (and is 4 bytes)
	// mem::transmute doesn't work here as it doesn't adjust the length of the slice, even though it is transmuted into a 2D array (so the length reduces)

	(unsafe {
		let ptr = frame as *mut [u8];
		let casted = ptr as *mut [[Colour; WIDTH as usize]; HEIGHT as usize];
		&mut *casted
	}) as _
}
