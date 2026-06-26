use crate::graphics::{pipeline::Pipeline, target::Target};

pub trait Rasterizer<Input, VsOut> {
	fn draw<T: Target, P: Pipeline<Fragment = T::Item, VsOut = VsOut>>(
		target: &mut T,
		pipeline: &mut P,
		item: Input,
	);
}

pub mod triangle_rasterizer;
