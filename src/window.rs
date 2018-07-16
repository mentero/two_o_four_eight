extern crate piston_window;

use self::piston_window::{PistonWindow, WindowSettings};

pub fn create_window(width: u32, height: u32) -> PistonWindow {
    let title  = "Two O Four Eight";
    let margin = 50;

    let size = [ width + margin * 2, height + margin * 2];

    WindowSettings::new(title, size).build().unwrap()
}