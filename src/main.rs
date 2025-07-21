use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
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
	let mut world = World::new();

	let res = event_loop.run(|event, elwt| {
		if let Event::WindowEvent {
			event: WindowEvent::RedrawRequested,
			..
		} = event
		{
			world.draw(pixels.frame_mut());
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

			if let Some(size) = input.window_resized() {
				if let Err(err) = pixels.resize_surface(size.width, size.height) {
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
#[repr(C)]
#[derive(Clone, Debug)]
struct Colour {
	red: u8,
	green: u8,
	blue: u8,
	alpha: u8,
}
impl Colour {
	pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
		Self {
			red,
			green,
			blue,
			alpha,
		}
	}
}
impl Default for Colour {
	fn default() -> Self {
		Self::new(255, 255, 255, 255)
	}
}

impl World {
	fn new() -> Self {
		Self {}
	}

	fn update(&mut self) {}

	fn draw(&self, frame: &mut [u8]) {
		let frame = frame_pixels(frame);
		frame
			.as_flattened_mut()
			.fill(Colour::new(0x48, 0xb2, 0xe8, 255));
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
