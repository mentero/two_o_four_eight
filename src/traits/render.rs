extern crate piston_window;

pub trait Render {
    fn draw<G>(&self, c: piston_window::Context, g: &mut G) where G: piston_window::Graphics;
}