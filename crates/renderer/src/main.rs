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

use crate::graphics::colour::Colour;
use crate::graphics::screen::Screen;
use crate::graphics::shapes_2d::triangle::BoundingArea;
use crate::graphics::shapes_3d::triangle::Triangle3D;
use crate::graphics::viewport::Viewport;
use crate::loaders::stl::load_file;
pub mod graphics;
pub mod loaders;
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

struct World {}

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
	let mut screen = Screen::new(pixels);
	let mut viewport =
		Viewport::new(BoundingArea::new(0, WIDTH as usize, 0, HEIGHT as usize)).unwrap();
	let mut world = World::new();
	let mut frame_num: usize = 0;
	let mut sum: u128 = 0;
	let mesh = load_file("./cube.stl");
	let res = event_loop.run(|event, elwt| {
		if let Event::WindowEvent {
			event: WindowEvent::RedrawRequested,
			..
		} = event
		{
			// Clear buffer
			screen.clear(Colour::BLACK);
			let start = Instant::now();
			world.draw(&mut viewport, &mut screen, &mesh);
			let time_taken = start.elapsed();
			frame_num += 1;
			sum += time_taken.as_micros();
			// viewport.set_area(BoundingArea::new(
			// 	frame_num % 40,
			// 	WIDTH as usize,
			// 	0,
			// 	HEIGHT as usize,
			// ));
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

			world.update();
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
	fn new() -> Self {
		Self {}
	}

	fn update(&mut self) {}

	fn draw(&self, viewport: &mut Viewport, screen: &mut Screen, mesh: &[Triangle3D]) {
		// screen.clear(Colour::new(0x48, 0xb2, 0xe8, 255));
		// screen.draw_point(Vector2::new(0, 0), Colour::new(0x48, 0xb2, 0xe8, 255));
		// screen.draw_line(Vector2::new(0, 0), Vector2::new(100, 200));
		// for (i, x) in (40..(WIDTH - 40)).step_by(100).enumerate() {
		// 	for (w, y) in (40..(HEIGHT - 100)).step_by(100).enumerate() {
		// 		screen.set_draw_colour(Colour::COLOURS[(w + i) % Colour::COLOURS.len()].clone());
		// 		viewport.draw_shape(
		// 			screen,
		// 			Triangle2D::new(
		// 				PixelCoordinate::new(x as usize + 10, y as usize),
		// 				PixelCoordinate::new(100 + x as usize, y as usize),
		// 				PixelCoordinate::new(100 + x as usize, y as usize + 100),
		// 			),
		// 		);
		// 	}
		// }
		let x: std::time::Duration = SystemTime::now()
			.duration_since(SystemTime::UNIX_EPOCH)
			.unwrap();
		// let triangle_3d = Triangle3D::new(
		// 	Point::new(0.0, -0.1, 0.0),
		// 	Point::new(0.3, 0.1, 0.0),
		// 	Point::new(-0.2, 0.1, 0.0),
		// )
		// .apply(Matrix4::rotation(x.as_secs_f64()));
		// println!("{:?}", Matrix4::scale(100.0));
		// screen.set_draw_colour(Colour::RED);
		// viewport.draw_shape(
		// 	screen,
		// 	Triangle3D::new(
		// 		Point::new(0.0, -0.1, 0.0),
		// 		Point::new(0.3, 0.1, 0.0),
		// 		Point::new(-0.2, 0.1, 0.0),
		// 	)
		// 	.apply(Matrix4::scale(100.0)),
		// );
		// screen.set_draw_colour(Colour::PURPLE);
		// viewport.draw_shape(
		// 	screen,
		// 	Triangle3D::new(
		// 		Point::new(0.2, -0.1, -0.2),
		// 		Point::new(0.3, 0.1, -0.2),
		// 		Point::new(-0.2, 0.1, -0.2),
		// 	)
		// 	.apply(Matrix4::scale(100.0)),
		// );
		let transform = Matrix4::scale_x(HEIGHT as f64 / WIDTH as f64)
			* Matrix4::rotation_z(x.as_secs_f64())
			* Matrix4::rotation_y(x.as_secs_f64())
			* Matrix4::rotation_x(x.as_secs_f64())
			* Matrix4::scale(5.0);
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
			// screen.set_draw_colour(Colour::COLOURS[(i) % Colour::COLOURS.len()].clone());

			viewport.draw_shape(screen, transformed)
		}
		// println!("Actual # of triangles drawn: {}", unsafe {
		// 	TRIANGLE_RENDER_COUNT
		// });
		// unsafe { TRIANGLE_RENDER_COUNT = 0 }
		// println!("MAX Z: {}", unsafe { MAX_Z })
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
