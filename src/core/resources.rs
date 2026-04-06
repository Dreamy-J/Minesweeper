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
    pub total_width: f32,
    pub total_height: f32,
    pub half_width: f32,
    pub half_height: f32,
}

impl GameConfig {
    pub fn new(rows: u32, cols: u32, total_mines: u32, cell_size: f32) -> Self {
        Self {
            rows,
            cols,
            total_mines,
            cell_size,
            total_width: cols as f32 * cell_size,
            total_height: rows as f32 * cell_size,
            half_width: cols as f32 * cell_size / 2.,
            half_height: rows as f32 * cell_size / 2.,
        }
    }
}

/// 游戏状态
#[derive(Resource)]
pub struct GameState {
    pub status: GameStatus,
    pub elapsed_time: f32,
    pub flags_placed: u32,
}

#[derive(Resource, Default)]
pub struct GridTable(pub Vec<Entity>);

impl GridTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_entity(&self, i: i32, j: i32, game_config: &GameConfig) -> Option<Entity> {
        if i >= game_config.rows as i32 || j >= game_config.cols as i32 || i < 0 || j < 0 {
            return None;
        }
        self.0
            .get((i as u32 * game_config.cols + j as u32) as usize)
            .cloned()
    }
}
