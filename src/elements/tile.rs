extern crate piston_window;

use color; 
use traits;
use self::piston_window::*;

pub struct Tile {
    position: (u8, u8),
    color: [f32; 4]
}

impl Tile {
    pub const DIMENSIONS: (f64, f64) = (100.0, 100.0); 
    pub const MARGIN: f64 = 15.0;

    pub fn new(col: u8, row: u8) -> Tile {
        Tile {
            position: (col, row),
            color: color::rgba(238, 228, 218, 0.35)
        }
    }

    pub fn coordinates(&self) -> [f64; 4] {
        let x = (50.0 + Tile::MARGIN) + (Tile::DIMENSIONS.0 + Tile::MARGIN) * self.position.0 as f64;
        let y = (50.0 + Tile::MARGIN) + (Tile::DIMENSIONS.1 + Tile::MARGIN) * self.position.1 as f64;

        [ x, y, Tile::DIMENSIONS.0, Tile::DIMENSIONS.1 ]
    }

    pub fn as_rectangle(&self) -> Rectangle {
        Rectangle { 
            color: self.color, 
            shape: rectangle::Shape::Round(5.0, 5), 
            border: None
        }
    }
}

impl traits::render::Render for Tile {
    fn draw<G>(&self, c: piston_window::Context, g: &mut G) where G: piston_window::Graphics {
        self.as_rectangle().draw(self.coordinates(), &c.draw_state, c.transform, g)
    }
}