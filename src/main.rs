use std::time::Instant;

use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use crate::graphics::colour::Colour;
use crate::graphics::screen::{Point, Screen};
use crate::maths::vector2::Vector2;
pub mod graphics;
pub mod maths;
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

	let mut pixels = {
		let window_size = window.inner_size();
		let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
		Pixels::new(WIDTH, HEIGHT, surface_texture)?
	};
	let mut screen = Screen::new(pixels);
	let mut world = World::new();
	let mut frame_num: usize = 0;
	let mut sum: u128 = 0;
	let res = event_loop.run(|event, elwt| {
		if let Event::WindowEvent {
			event: WindowEvent::RedrawRequested,
			..
		} = event
		{
			let start = Instant::now();
			world.draw(&mut screen);
			let time_taken = start.elapsed();
			frame_num += 1;
			sum += time_taken.as_micros();

			if frame_num % 1000 == 0 {
				let mean = sum as f64 / frame_num as f64;
				frame_num = 0;
				sum = 0;
				println!("Mean draw time taken over most recent 1000 frames is {mean} microseconds")
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

	fn draw(&self, screen: &mut Screen) {
		// screen.clear(Colour::new(0x48, 0xb2, 0xe8, 255));
		// screen.draw_point(Vector2::new(0, 0), Colour::new(0x48, 0xb2, 0xe8, 255));
		// screen.draw_line(Vector2::new(0, 0), Vector2::new(100, 200));
		screen.draw_triangle(Point::new(500, 300), Point::new(800, 400), Point::new(640,600));
	}
}

const fn frame_pixels(frame: &mut [u8]) -> &mut [[Colour; WIDTH as usize]] {
	// SAFETY: Format for each pixel matches the layout of the `Colour` struct (and is 4 bytes)
	// mem::transmute doesn't work here as it doesn't adjust the length of the slice, even though it is transmuted into a 2D array (so the length reduces)
	let frame = unsafe {
		let ptr = frame as *mut [u8];
		let casted = ptr as *mut [[Colour; WIDTH as usize]; HEIGHT as usize];
		&mut *casted
	};
	frame
}
