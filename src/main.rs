extern crate piston_window;

pub mod color;
pub mod traits;
pub mod elements;
pub mod window;

use piston_window::*;
use elements::tile::Tile;
use elements::grid::Grid;
use traits::render::Render;

fn main() {
    let background      = color::rgb(250, 248, 239);

    let window_dimension = (
        (Tile::DIMENSIONS.0 + Tile::MARGIN) * 4.0 + Tile::MARGIN,
        (Tile::DIMENSIONS.1 + Tile::MARGIN) * 4.0 + Tile::MARGIN,
    );

    let mut window = window::create_window(window_dimension.0 as u32, window_dimension.1 as u32);

    let grid = Grid::build();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear(background, g);
            grid.draw(c, g)
        });
    }
}