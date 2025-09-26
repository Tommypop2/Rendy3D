//! Implementation of a triangle clipping algorithm

use rendy3d_maths::vector::vector4::Vector4;

use crate::graphics::{geometry::triangle::Triangle, interpolate::Interpolate};

pub trait TriangleClipper<T>
where
	T: Interpolate,
{
	type ClipResult: IntoIterator<Item = Triangle<Vertex<T>>>;
	fn clip(triangle: Triangle<Vertex<T>>) -> Self::ClipResult;
}

pub struct SutherlandHodgman;
type Vertex<T> = (Vector4<f64>, T);
impl SutherlandHodgman {
	/// Compute intersection point with plane
	fn intersection<T: Interpolate + Default>(
		v0: Vertex<T>,
		v1: Vertex<T>,
		value0: f64,
		value1: f64,
	) -> Vertex<T> {
		let t = value0 / (value0 - value1);
		let u = 1.0 - t;
		// Interpolate vertices
		let p = v0.0 + (v1.0 - v0.0) * t;

		// Interpolate attributes
		let a = T::interpolate3(&v0.1, &v1.1, &Default::default(), t as f32, u as f32, 0.0);

		(p, a)
	}
}
impl<T> TriangleClipper<T> for SutherlandHodgman
where
	T: Interpolate,
{
	type ClipResult = Vec<Triangle<Vertex<T>>>;
	fn clip(triangle: Triangle<Vertex<T>>) -> Self::ClipResult {
		vec![]
	}
}
