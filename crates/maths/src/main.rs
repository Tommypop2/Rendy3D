use std::hint::black_box;

use maths::matrices::matrix3::Matrix3;

#[inline(never)]
pub fn yes() -> f64 {
	let mut det = black_box(5.0);
	det += Matrix3::<f64>::identity().determinant();
	det
}
pub fn main() {
	println!("{}", yes())
}
