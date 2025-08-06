use rendy3d::graphics::{camera::Camera, object::Object};

struct World {
	pub cameras: Vec<Camera>,
	pub objects: Vec<Object>,
}
fn main() {
	println!("Hello, world!");
}
