use crate::geometry::triangle::Triangle;
#[derive(Clone, Copy)]
pub struct Quad<T> {
	v0: T,
	v1: T,
	v2: T,
	v3: T,
}

impl<T> Quad<T> {
	pub const fn new(v0: T, v1: T, v2: T, v3: T) -> Self {
		Self { v0, v1, v2, v3 }
	}
	pub fn map_vertices<V, U: Fn(T) -> V>(self, map_fn: U) -> Quad<V> {
		Quad::new(
			map_fn(self.v0),
			map_fn(self.v1),
			map_fn(self.v2),
			map_fn(self.v3),
		)
	}
}
impl<T> Quad<T>
where
	T: Clone,
{
	pub fn triangulate(self) -> [Triangle<T>; 2] {
		let t0 = Triangle::new(self.v0.clone(), self.v1, self.v2.clone());
		let t1 = Triangle::new(self.v0, self.v2, self.v3);
		[t0, t1]
	}
}
