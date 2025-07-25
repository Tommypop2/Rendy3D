use crate::graphics::{screen::Screen, viewport::Viewport};

pub trait Draw {
	fn draw(&self, viewport: &mut Viewport, screen: &mut Screen);
}
