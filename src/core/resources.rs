use bevy::prelude::*;

use crate::state::game_state::GameStatus;
// 全局资源定义

/// 游戏配置资源
#[derive(Resource)]
pub struct GameConfig {
    pub rows: u32,
    pub cols: u32,
    pub total_mines: u32,
    pub cell_size: f32,
}

impl GameConfig {
    pub fn new(rows: u32, cols: u32, total_mines: u32, cell_size: f32) -> Self {
        Self {
            rows,
            cols,
            total_mines,
            cell_size,
        }
    }
}

/// 游戏状态
#[derive(Resource)]
pub struct GameState {
    pub status: GameStatus,
    pub elapsed_time: f32,
    pub flags_placed: u32
}
