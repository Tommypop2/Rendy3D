use std::time::{Duration, Instant};

pub struct FrameTimeCounter {
	time_of_last_frame: Instant,
}
impl Default for FrameTimeCounter {
	fn default() -> Self {
		Self::new()
	}
}

impl FrameTimeCounter {
	pub fn new() -> Self {
		Self {
			time_of_last_frame: Instant::now(),
		}
	}
	/// Must be called on every frame
	pub fn frame_time(&mut self) -> Duration {
		let now = Instant::now();
		let frame_time = now - self.time_of_last_frame;
		self.time_of_last_frame = now;
		frame_time
	}
	pub fn fps(frame_time: Duration) -> f32 {
		1.0 / frame_time.as_secs_f32()
	}
}
