use crate::graphics::{interpolate::Interpolate, shaders::shaders::Shaders, target::Target};

pub trait Draw<VsOut: Interpolate> {
	fn draw<T: Target, S: Shaders<VsOut = VsOut, Fragment = T::Item> + Clone>(
		&self,
		target: &mut T,
		shaders: &mut S,
	);
}
