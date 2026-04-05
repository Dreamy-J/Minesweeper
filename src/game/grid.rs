// 网格系统
use bevy::prelude::*;

use crate::core::components::{AdjacentMineCount, Cell, CellState, Mine};
use crate::core::resources::GameConfig;

/// 网格组件标记
#[derive(Component)]
pub struct GridMark;

/// 创建背景网格
pub fn create_grid(commands: &mut Commands, config: &GameConfig) {
    let total_width = config.cols as f32 * config.cell_size;
    let total_height = config.rows as f32 * config.cell_size;

    let offset_x = -total_width / 2. + config.cell_size / 2.;
    let offset_y = -total_height / 2. + config.cell_size / 2.;

    // 创建网格
    for row in 0..config.rows {
        for col in 0..config.cols {
            let x = offset_x + col as f32 * config.cell_size;
            let y = offset_y + row as f32 * config.cell_size;

            commands.spawn((
                Sprite {
                    custom_size: Some(Vec2::new(config.cell_size, config.cell_size)),
                    ..Default::default()
                },
                GridMark,
                Cell::new(row, col),
                CellState::Hidden,
                Mine::new(false),
                AdjacentMineCount(0),
                Transform::from_xyz(x, y, 0.0),
            ));
        }
    }

    // 创建边框（黑色线条）
    let border_thickness = 2.;
    let grid_left = -total_width / 2.;
    let grid_bottom = -total_height / 2.;

    // 水平边框（上下）
    for row in 0..=config.rows {
        let y = grid_bottom + row as f32 * config.cell_size;
        commands.spawn((
            Sprite {
                custom_size: Some(Vec2::new(total_width, border_thickness)),
                color: Color::BLACK,
                ..Default::default()
            },
            GridMark,
            Transform::from_xyz(0., y, 1.),
        ));
    }

    // 垂直边框（左右）
    for col in 0..=config.cols {
        let x = grid_left + col as f32 * config.cell_size;
        commands.spawn((
            Sprite {
                custom_size: Some(Vec2::new(border_thickness, total_height)),
                color: Color::BLACK,
                ..Default::default()
            },
            GridMark,
            Transform::from_xyz(x, 0., 1.),
        ));
    }
}

/// 清理网格
pub fn cleanup_grid(mut commands: Commands, query: Query<Entity, With<GridMark>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
