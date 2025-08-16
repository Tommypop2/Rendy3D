use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader};

pub struct Texture {
	base: DynamicImage,
}
fn yes() {
	let img = ImageReader::open("asd").unwrap().decode().unwrap();
	let px = img.get_pixel(0, 0);
}
