use bevy::prelude::*;

use crate::core::components::{Cell, CellLabel, GridVisual};
use crate::core::resources::{Board, CellEntityMap, CellVisibility, GameConfig};
use crate::game::grid::{create_grid_visuals, despawn_grid_visuals};

pub fn sync_grid_visuals(
    mut commands: Commands,
    board: Res<Board>,
    config: Res<GameConfig>,
    mut entity_map: ResMut<CellEntityMap>,
    visuals: Query<Entity, With<GridVisual>>,
    cell_visuals: Query<Entity, With<Cell>>,
) {
    if entity_map.entities.len() == board.cell_count()
        && cell_visuals.iter().count() == board.cell_count()
    {
        return;
    }

    despawn_grid_visuals(&mut commands, &visuals);
    create_grid_visuals(&mut commands, &board, &config, &mut entity_map);
}

pub fn update_cell_sprites(board: Res<Board>, mut cells: Query<(&Cell, &mut Sprite)>) {
    for (cell_ref, mut sprite) in &mut cells {
        let Some(cell) = board.cell(cell_ref.row, cell_ref.col) else {
            continue;
        };

        sprite.color = match cell.visibility {
            CellVisibility::Hidden => Color::srgb(0.45, 0.45, 0.45),
            CellVisibility::Flagged => Color::srgb(0.95, 0.6, 0.15),
            CellVisibility::Revealed => {
                if cell.is_mine {
                    Color::srgb(0.85, 0.1, 0.1)
                } else if cell.adjacent_mines == 0 {
                    Color::srgb(0.85, 0.85, 0.85)
                } else {
                    Color::srgb(0.75, 0.75, 0.75)
                }
            }
        };
    }
}

pub fn update_cell_labels(
    board: Res<Board>,
    mut labels: Query<(&CellLabel, &mut Text2d, &mut TextColor)>,
) {
    for (label, mut text, mut text_color) in &mut labels {
        let Some(cell) = board.cell(label.row, label.col) else {
            continue;
        };

        let (value, color) = match cell.visibility {
            CellVisibility::Hidden => (String::new(), Color::BLACK),
            CellVisibility::Flagged => ("F".to_string(), Color::BLACK),
            CellVisibility::Revealed => {
                if cell.is_mine {
                    ("*".to_string(), Color::BLACK)
                } else if cell.adjacent_mines == 0 {
                    (String::new(), Color::BLACK)
                } else {
                    (
                        cell.adjacent_mines.to_string(),
                        number_color(cell.adjacent_mines),
                    )
                }
            }
        };

        *text = Text2d::new(value);
        *text_color = TextColor(color);
    }
}

fn number_color(adjacent: u8) -> Color {
    match adjacent {
        1 => Color::srgb(0.1, 0.2, 0.8),
        2 => Color::srgb(0.0, 0.55, 0.15),
        3 => Color::srgb(0.75, 0.1, 0.1),
        4 => Color::srgb(0.4, 0.1, 0.6),
        _ => Color::srgb(0.1, 0.1, 0.1),
    }
}
