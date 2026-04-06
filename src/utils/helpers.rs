use bevy::prelude::*;

use crate::core::resources::{Board, GameConfig};

pub fn board_dimensions(board: &Board, config: &GameConfig) -> Vec2 {
    Vec2::new(
        board.cols as f32 * config.cell_size,
        board.rows as f32 * config.cell_size,
    )
}

pub fn cell_to_world_center(row: u32, col: u32, board: &Board, config: &GameConfig) -> Vec2 {
    let dims = board_dimensions(board, config);
    let left = -dims.x / 2.0;
    let top = dims.y / 2.0;

    let x = left + col as f32 * config.cell_size + config.cell_size / 2.0;
    let y = top - row as f32 * config.cell_size - config.cell_size / 2.0;
    Vec2::new(x, y)
}

pub fn world_to_cell(world: Vec2, board: &Board, config: &GameConfig) -> Option<(u32, u32)> {
    let dims = board_dimensions(board, config);
    let left = -dims.x / 2.0;
    let top = dims.y / 2.0;

    let col = ((world.x - left) / config.cell_size).floor() as i32;
    let row = ((top - world.y) / config.cell_size).floor() as i32;

    if !board.in_bounds(row, col) {
        return None;
    }

    Some((row as u32, col as u32))
}
