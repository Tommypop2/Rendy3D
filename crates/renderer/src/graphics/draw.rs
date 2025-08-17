use crate::graphics::{interpolate::Interpolate, shaders::shaders::Pipeline, target::Target};

pub trait Draw<VsOut: Interpolate> {
	fn draw<T: Target, S: Pipeline<VsOut = VsOut, Fragment = T::Item>>(
		&self,
		target: &mut T,
		shaders: &mut S,
	);
}
