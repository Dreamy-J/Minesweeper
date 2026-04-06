use bevy::prelude::*;

use crate::core::components::{Cell, CellLabel, GridVisual};
use crate::core::resources::{Board, CellEntityMap, GameConfig};
use crate::utils::helpers::cell_to_world_center;

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

pub fn despawn_grid_visuals(commands: &mut Commands, query: &Query<Entity, With<GridVisual>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn spawn_grid_lines(commands: &mut Commands, board: &Board, config: &GameConfig) {
    let width = board.cols as f32 * config.cell_size;
    let height = board.rows as f32 * config.cell_size;
    let left = -width / 2.0;
    let bottom = -height / 2.0;
    let line = 1.0;

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
