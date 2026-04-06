use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::core::components::HudText;
use crate::core::resources::{Board, GameConfig, GameSession};
use crate::state::game_state::GameStatus;
use crate::utils::helpers::board_dimensions;

pub fn update_hud(
    board: Res<Board>,
    config: Res<GameConfig>,
    session: Res<GameSession>,
    state: Res<State<GameStatus>>,
    mut hud_query: Query<(&mut Text2d, &mut Transform), With<HudText>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let remaining = board.total_mines as i32 - session.flags_placed as i32;
    let status_text = match state.get() {
        GameStatus::Playing => "Playing",
        GameStatus::Victory => "Victory",
        GameStatus::Defeat => "Defeat",
        GameStatus::MainMenu => "MainMenu",
        GameStatus::DifficultySelect => "DifficultySelect",
        GameStatus::Paused => "Paused",
    };

    let hud_text = format!(
        "Diff: {}  Mines: {}  Flags: {}  Remaining: {}  Time: {:03}s  Status: {}  [R restart | 1/2/3 diff]",
        session.difficulty.label(),
        board.total_mines,
        session.flags_placed,
        remaining,
        session.elapsed_seconds as u32,
        status_text,
    );

    if let Ok((mut text, mut transform)) = hud_query.single_mut() {
        *text = Text2d::new(hud_text.clone());
        let dims = board_dimensions(&board, &config);
        transform.translation = Vec3::new(0.0, dims.y / 2.0 + config.cell_size * 0.75, 5.0);
    }

    if let Ok(mut window) = windows.single_mut() {
        window.title = format!("Minesweeper | {}", hud_text);
    }
}
