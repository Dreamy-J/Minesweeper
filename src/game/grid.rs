use bevy::ecs::relationship::OrderedRelationshipSourceCollection;

// 网格系统
use bevy::prelude::*;

use crate::core::components::{AdjacentMineCount, Cell, CellState, Mine};
use crate::core::resources::{GameConfig, GridTable};
use crate::utils::helpers::grid_to_world;

/// 网格组件标记
#[derive(Component)]
pub struct GridMark;

/// 创建背景网格
pub fn create_grid(
    commands: &mut Commands,
    config: &GameConfig,
    grid_table: &mut ResMut<GridTable>,
) {
    // 创建网格
    for row in 0..config.rows {
        for col in 0..config.cols {
            let (x, y) = grid_to_world(row, col, config);
            let grid_entity = commands.spawn((
                Sprite {
                    custom_size: Some(Vec2::new(config.cell_size, config.cell_size)),
                    ..Default::default()
                },
                GridMark,
                Cell::new(row, col),
                CellState::Hidden,
                Mine::new(false),
                AdjacentMineCount(0),
                Transform::from_xyz(x - config.cell_size / 2., y - config.cell_size / 2., 0.0),
            )).id();
            grid_table.0.push_back(grid_entity);
        }
    }

    // 创建边框（黑色线条）
    let border_thickness = 2.;
    let grid_left = -config.half_width;
    let grid_bottom = -config.half_height;

    // 水平边框（上下）
    for row in 0..=config.rows {
        let y = grid_bottom + row as f32 * config.cell_size;
        commands
            .spawn((
                Sprite {
                    custom_size: Some(Vec2::new(config.total_width, border_thickness)),
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
                custom_size: Some(Vec2::new(border_thickness, config.total_height)),
                color: Color::BLACK,
                ..Default::default()
            },
            GridMark,
            Transform::from_xyz(x, 0., 1.),
        ));
    }
}

/// 清理网格
pub fn cleanup_grid(mut commands: Commands, query: Query<Entity, With<GridMark>>, grid_table: &mut ResMut<GridTable>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    grid_table.0.clear();
}
