use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

use crate::coord::Coord;
use crate::field::Cell;

const BLOCK_SIZE: f64 = 25.0;

const WALL_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
pub const EMPTY_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
const SNAKE_COLOR: Color = [0.0, 0.8, 0.0, 1.0];
const SNAKE_EATING_COLOR: Color = [0.0, 0.5, 0.0, 1.0];
const COLLISION_COLOR: Color = [0.8, 0.8, 0.0, 1.0];
const FOOD_COLOR: Color = [0.8, 0.0, 0.0, 1.0];

pub fn to_coord(coord: Coord) -> f64 {
    coord * BLOCK_SIZE
}

pub fn to_coord_u32(coord: Coord) -> u32 {
    to_coord(coord) as u32
}

pub fn draw_block(color: Color, (x, y): &Cell, ctx: &Context, graphics: &mut G2d) {
    let gui_x = to_coord(x.clone());
    let gui_y = to_coord(y.clone());

    let decrease = 4.0;

    rectangle(
        color,
        [
            gui_x + (decrease / 2.0),
            gui_y + (decrease / 2.0),
            BLOCK_SIZE - decrease,
            BLOCK_SIZE - decrease,
        ],
        ctx.transform,
        graphics,
    )
}

pub fn draw_large_block(color: Color, (x, y): &Cell, ctx: &Context, graphics: &mut G2d) {
    let gui_x = to_coord(x.clone());
    let gui_y = to_coord(y.clone());

    let decrease = 0.0;

    rectangle(
        color,
        [
            gui_x + (decrease / 2.0),
            gui_y + (decrease / 2.0),
            BLOCK_SIZE - decrease,
            BLOCK_SIZE - decrease,
        ],
        ctx.transform,
        graphics,
    )
}

pub fn draw_wall(cell: &Cell, ctx: &Context, graphics: &mut G2d) {
    draw_block(WALL_COLOR, cell, ctx, graphics)
}

pub fn draw_empty(cell: &Cell, ctx: &Context, graphics: &mut G2d) {
    draw_block(EMPTY_COLOR, cell, ctx, graphics)
}

pub fn draw_snake(cell: &Cell, ctx: &Context, graphics: &mut G2d) {
    draw_block(SNAKE_COLOR, cell, ctx, graphics);
}

pub fn draw_snake_eating(cell: &Cell, ctx: &Context, graphics: &mut G2d) {
    draw_large_block(SNAKE_EATING_COLOR, cell, ctx, graphics);
}

pub fn draw_collision(cell: &Cell, ctx: &Context, graphics: &mut G2d) {
    draw_block(COLLISION_COLOR, cell, ctx, graphics);
}

pub fn draw_food(cell: &Cell, ctx: &Context, graphics: &mut G2d) {
    draw_block(FOOD_COLOR, cell, ctx, graphics);
}
