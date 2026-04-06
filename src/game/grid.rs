//! 网格视觉创建模块
//!
//! 负责创建和管理游戏棋盘的视觉表现，包括单元格精灵、
//! 网格线和文本标签。

use bevy::prelude::*;

use crate::core::components::{Cell, CellLabel, GridVisual};
use crate::core::resources::{Board, CellEntityMap, GameConfig};
use crate::utils::helpers::cell_to_world_center;

/// 创建网格视觉元素
///
/// 为棋盘上的每个单元格创建对应的视觉实体，包括：
/// - 单元格背景精灵（Sprite）
/// - 单元格文本标签（Text2d）
/// - 网格线
///
/// # 参数
///
/// * `commands` - 命令队列，用于生成实体
/// * `board` - 游戏棋盘
/// * `config` - 游戏配置
/// * `entity_map` - 单元格实体映射（会被更新）
pub fn create_grid_visuals(
    commands: &mut Commands,
    board: &Board,
    config: &GameConfig,
    entity_map: &mut CellEntityMap,
) {
    entity_map.clear();

    for row in 0..board.rows {
        for col in 0..board.cols {
            let position = cell_to_world_center(row, col, board, config);

            // 创建单元格背景精灵
            let entity = commands
                .spawn((
                    GridVisual,
                    Cell::new(row, col),
                    Sprite {
                        custom_size: Some(Vec2::splat(config.cell_size - 2.0)),
                        color: Color::srgb(0.45, 0.45, 0.45),
                        ..default()
                    },
                    Transform::from_xyz(position.x, position.y, 0.0),
                ))
                .id();

            entity_map.entities.push(entity);

            // 创建单元格文本标签
            commands.spawn((
                GridVisual,
                CellLabel { row, col },
                Text2d::new(""),
                TextFont {
                    font_size: config.cell_size * 0.55,
                    ..default()
                },
                TextColor(Color::BLACK),
                Transform::from_xyz(position.x, position.y - config.cell_size * 0.05, 1.0),
            ));
        }
    }

    spawn_grid_lines(commands, board, config);
}

/// 清除所有网格视觉元素
///
/// 用于在重建棋盘时清理旧的视觉实体。
///
/// # 参数
///
/// * `commands` - 命令队列，用于删除实体
/// * `query` - 所有带有 GridVisual 组件的实体查询
pub fn despawn_grid_visuals(commands: &mut Commands, query: &Query<Entity, With<GridVisual>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

/// 创建网格线
///
/// 在棋盘周围绘制水平和垂直的线条，形成网格效果。
///
/// # 参数
///
/// * `commands` - 命令队列
/// * `board` - 游戏棋盘
/// * `config` - 游戏配置
fn spawn_grid_lines(commands: &mut Commands, board: &Board, config: &GameConfig) {
    let width = board.cols as f32 * config.cell_size;
    let height = board.rows as f32 * config.cell_size;
    let left = -width / 2.0;
    let bottom = -height / 2.0;
    let line = 1.0;

    // 创建水平线
    for row in 0..=board.rows {
        let y = bottom + row as f32 * config.cell_size;
        commands.spawn((
            GridVisual,
            Sprite {
                custom_size: Some(Vec2::new(width, line)),
                color: Color::BLACK,
                ..default()
            },
            Transform::from_xyz(0.0, y, 2.0),
        ));
    }

    // 创建垂直线
    for col in 0..=board.cols {
        let x = left + col as f32 * config.cell_size;
        commands.spawn((
            GridVisual,
            Sprite {
                custom_size: Some(Vec2::new(line, height)),
                color: Color::BLACK,
                ..default()
            },
            Transform::from_xyz(x, 0.0, 2.0),
        ));
    }
}
