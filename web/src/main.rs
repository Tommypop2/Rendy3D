#[cfg(not(target_os = "android"))]
fn main() {
	use web::entry;
	use winit::event_loop::EventLoop;

	entry(EventLoop::new().unwrap())
}
