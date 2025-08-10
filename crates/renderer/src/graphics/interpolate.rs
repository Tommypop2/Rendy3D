/// Represents types that can be interpolated
pub trait Interpolate {
	fn interpolate3(a: &Self, b: &Self, c: &Self, x: f32, y: f32, z: f32) -> Self;
}