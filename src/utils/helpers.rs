//! 辅助函数模块
//!
//! 提供坐标转换和尺寸计算等工具函数，
//! 用于在网格坐标和世界坐标之间进行转换。

use bevy::prelude::*;

use crate::core::resources::{Board, GameConfig};

/// 计算棋盘的像素尺寸
///
/// # 参数
///
/// * `board` - 游戏棋盘
/// * `config` - 游戏配置
///
/// # 返回值
///
/// 返回棋盘的宽度和高度（Vec2）。
pub fn board_dimensions(board: &Board, config: &GameConfig) -> Vec2 {
    Vec2::new(
        board.cols as f32 * config.cell_size,
        board.rows as f32 * config.cell_size,
    )
}

/// 将网格坐标转换为世界空间中心坐标
///
/// 用于在创建单元格视觉元素时确定其位置。
/// 坐标原点位于棋盘中心。
///
/// # 参数
///
/// * `row` - 行号
/// * `col` - 列号
/// * `board` - 游戏棋盘
/// * `config` - 游戏配置
///
/// # 返回值
///
/// 返回单元格中心的世界坐标（Vec2）。
pub fn cell_to_world_center(row: u32, col: u32, board: &Board, config: &GameConfig) -> Vec2 {
    let dims = board_dimensions(board, config);
    let left = -dims.x / 2.0;
    let top = dims.y / 2.0;

    let x = left + col as f32 * config.cell_size + config.cell_size / 2.0;
    let y = top - row as f32 * config.cell_size - config.cell_size / 2.0;
    Vec2::new(x, y)
}

/// 将世界空间坐标转换为网格坐标
///
/// 用于将鼠标点击位置转换为对应的单元格行列号。
///
/// # 参数
///
/// * `world` - 世界坐标
/// * `board` - 游戏棋盘
/// * `config` - 游戏配置
///
/// # 返回值
///
/// 如果坐标在棋盘范围内，返回 `Some((row, col))`；
/// 否则返回 `None`。
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
