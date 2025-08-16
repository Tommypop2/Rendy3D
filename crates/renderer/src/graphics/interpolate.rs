use std::ops::{Add, Mul};

use maths::vector::vector3::Vector3;

/// Represents types that can be interpolated
pub trait Interpolate {
	fn interpolate3(a: &Self, b: &Self, c: &Self, x: f32, y: f32, z: f32) -> Self;
}

impl Interpolate for f32 {
	fn interpolate3(a: &Self, b: &Self, c: &Self, x: f32, y: f32, z: f32) -> Self {
		a * x + b * y + c * z
	}
}

impl Interpolate for () {
	fn interpolate3(_: &Self, _: &Self, _: &Self, _: f32, _: f32, _: f32) -> Self {}
}

impl<T, U> Interpolate for (T, U)
where
	T: Interpolate,
	U: Interpolate,
{
	fn interpolate3(a: &Self, b: &Self, c: &Self, x: f32, y: f32, z: f32) -> Self {
		(
			T::interpolate3(&a.0, &b.0, &c.0, x, y, z),
			U::interpolate3(&a.1, &b.1, &c.1, x, y, z),
		)
	}
}

impl<T, U, V> Interpolate for (T, U, V)
where
	T: Interpolate,
	U: Interpolate,
	V: Interpolate,
{
	fn interpolate3(a: &Self, b: &Self, c: &Self, x: f32, y: f32, z: f32) -> Self {
		(
			T::interpolate3(&a.0, &b.0, &c.0, x, y, z),
			U::interpolate3(&a.1, &b.1, &c.1, x, y, z),
			V::interpolate3(&a.2, &b.2, &c.2, x, y, z),
		)
	}
}

// Implement for Vector types

impl<T> Interpolate for Vector3<T>
where
	T: From<f32> + Mul<T, Output = T> + Add<T, Output = T> + Copy,
{
	fn interpolate3(a: &Self, b: &Self, c: &Self, x: f32, y: f32, z: f32) -> Self {
		let x = x.into();
		let y = y.into();
		let z = z.into();
		Self::new(
			a.x * x + b.x * y + c.x * z,
			a.y * x + b.y * y + c.y * z,
			a.z * x + b.z * y + c.z * z,
		)
	}
}
