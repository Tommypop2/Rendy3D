#[cfg(not(target_os = "android"))]
fn main() {
	use web::start;

	start();
}
