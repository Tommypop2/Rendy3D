/// Represents types that can be interpolated
pub trait Interpolate {
	fn interpolate3(a: &Self, b: &Self, c: &Self, x: f32, y: f32, z: f32) -> Self;
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
