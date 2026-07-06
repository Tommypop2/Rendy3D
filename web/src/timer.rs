use std::time::SystemTime;

use web_sys::Performance;

/// Standardises getting the current time between wasm and std environments

///
pub trait Timer {
	fn secs(&self) -> f64;
}
pub struct StdTime {}
impl Timer for StdTime {
	fn secs(&self) -> f64 {
		SystemTime::now()
			.duration_since(SystemTime::UNIX_EPOCH)
			.unwrap()
			.as_secs_f64()
	}
}

#[allow(dead_code)]
pub struct WasmTime {
	pub performance: Performance,
}
impl Timer for WasmTime {
	fn secs(&self) -> f64 {
		self.performance.now() / 1000.0
	}
}
