extern crate piston_window;

use elements::tile::{Tile};
use color;
use traits;
use self::piston_window::*;

pub struct Grid {
    background_tiles: [Tile; 16],
    tiles: Vec<Tile>,
} 

impl Grid {
    pub fn build() -> Grid {
        let background_tiles = [
            Tile::new(0, 0),
            Tile::new(0, 1),
            Tile::new(0, 2),
            Tile::new(0, 3),
            Tile::new(1, 0),
            Tile::new(1, 1),
            Tile::new(1, 2),
            Tile::new(1, 3),
            Tile::new(2, 0),
            Tile::new(2, 1),
            Tile::new(2, 2),
            Tile::new(2, 3),
            Tile::new(3, 0),
            Tile::new(3, 1),
            Tile::new(3, 2),
            Tile::new(3, 3),
        ];

        Grid { background_tiles: background_tiles, tiles: Vec::new() }
    }
}

mod render_helpers {
    use super::*;

    pub fn background() -> Rectangle {
        Rectangle {
            color: color::rgb(187, 173, 160),
            shape: rectangle::Shape::Round(5.0, 5),
            border: None
        }
    }

    pub fn size() -> [f64; 4] {
        let width  = Tile::MARGIN + (Tile::MARGIN + Tile::DIMENSIONS.1) * 4.0;
        let height = Tile::MARGIN + (Tile::MARGIN + Tile::DIMENSIONS.1) * 4.0;
        [0.0, 0.0, width, height]
    }

    pub fn offset(position: [f64; 4], x: f64, y: f64) -> [f64; 4] {
        [
            position[0] + x,
            position[1] + y,
            position[2],
            position[3]
        ]
    }
}

impl traits::render::Render for Grid {
    fn draw<G>(&self, c: piston_window::Context, g: &mut G) where G: piston_window::Graphics {
        let background_position = render_helpers::offset(render_helpers::size(), 50.0, 50.0);

        render_helpers::background().draw(background_position, &c.draw_state, c.transform, g);
        self.background_tiles.iter().for_each(|tile| tile.draw(c, g));
        self.tiles.iter().for_each(|tile| tile.draw(c,g));
    }
}