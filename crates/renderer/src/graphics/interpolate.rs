//! Trait for types which can be interpolated across a triangle
use std::ops::{Add, Mul};

use maths::vector::{vector2::Vector2, vector3::Vector3};

/// Represents types that can be interpolated
pub trait Interpolate {
	fn interpolate3(a: &Self, b: &Self, c: &Self, x: f32, y: f32, z: f32) -> Self;
}

impl Interpolate for f32 {
	fn interpolate3(a: &Self, b: &Self, c: &Self, x: f32, y: f32, z: f32) -> Self {
		a * x + b * y + c * z
	}
}
impl Interpolate for f64 {
	fn interpolate3(a: &Self, b: &Self, c: &Self, x: f32, y: f32, z: f32) -> Self {
		a * x as f64 + b * y as f64 + c * z as f64
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

impl<T> Interpolate for Vector2<T>
where
	T: From<f32> + Mul<T, Output = T> + Add<T, Output = T> + Copy,
{
	fn interpolate3(a: &Self, b: &Self, c: &Self, x: f32, y: f32, z: f32) -> Self {
		let x = x.into();
		let y = y.into();
		let z = z.into();
		Self::new(a.x * x + b.x * y + c.x * z, a.y * x + b.y * y + c.y * z)
	}
}

//
#[derive(Clone)]
pub struct PerspectiveCorrectInterpolate<T: Interpolate + Mul<f64, Output = T>> {
	data: T,
	z_reciprocal: f64,
}
impl<T> PerspectiveCorrectInterpolate<T>
where
	T: Interpolate + Mul<f64, Output = T>,
{
	pub fn new(data: T, z: f64) -> Self {
		let z_reciprocal = 1.0 / z;
		Self {
			data: data * z_reciprocal,
			z_reciprocal,
		}
	}
	pub fn get(self) -> T {
		let z = 1.0 / self.z_reciprocal;
		self.data * z
	}
}

impl<T> Interpolate for PerspectiveCorrectInterpolate<T>
where
	T: Interpolate + Mul<f64, Output = T> + Copy,
{
	fn interpolate3(a: &Self, b: &Self, c: &Self, x: f32, y: f32, z: f32) -> Self {
		let data = T::interpolate3(&a.data, &b.data, &c.data, x, y, z);
		let z_reciprocal =
			f64::interpolate3(&a.z_reciprocal, &b.z_reciprocal, &c.z_reciprocal, x, y, z);
		Self { data, z_reciprocal }
	}
}
