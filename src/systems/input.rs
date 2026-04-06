use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::core::components::MainCamera;
use crate::core::events::CellActionEvent;
use crate::core::resources::{Board, DifficultyPreset, GameConfig};
use crate::utils::helpers::world_to_cell;

pub fn emit_player_actions(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    board: Res<Board>,
    config: Res<GameConfig>,
    mut event_writer: MessageWriter<CellActionEvent>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        event_writer.write(CellActionEvent::restart());
    }

    if keyboard.just_pressed(KeyCode::Digit1) {
        event_writer.write(CellActionEvent::change_difficulty(
            DifficultyPreset::Beginner,
        ));
    }
    if keyboard.just_pressed(KeyCode::Digit2) {
        event_writer.write(CellActionEvent::change_difficulty(
            DifficultyPreset::Intermediate,
        ));
    }
    if keyboard.just_pressed(KeyCode::Digit3) {
        event_writer.write(CellActionEvent::change_difficulty(DifficultyPreset::Expert));
    }

    let is_left = mouse_buttons.just_pressed(MouseButton::Left);
    let is_right = mouse_buttons.just_pressed(MouseButton::Right);
    if !is_left && !is_right {
        return;
    }

    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let Ok(world) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };

    if let Some((row, col)) = world_to_cell(world, &board, &config) {
        if is_left {
            event_writer.write(CellActionEvent::reveal(row, col));
        }
        if is_right {
            event_writer.write(CellActionEvent::toggle_flag(row, col));
        }
    }
}
