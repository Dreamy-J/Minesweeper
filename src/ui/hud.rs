//! HUD（平视显示器）模块
//!
//! 负责在屏幕上方显示游戏状态信息，
//! 包括难度、地雷数、旗帜数、剩余雷数、游戏时间和当前状态。

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::core::components::HudText;
use crate::core::resources::{Board, GameConfig, GameSession};
use crate::state::game_state::GameStatus;
use crate::utils::helpers::board_dimensions;

/// 更新 HUD 显示内容
///
/// 每帧更新 HUD 文本，显示当前游戏状态信息。
/// 同时更新窗口标题以便在任务栏中查看。
///
/// 显示格式：
/// `Diff: {难度}  Mines: {总雷数}  Flags: {旗帜数}  Remaining: {剩余}  Time: {时间}s  Status: {状态}`
///
/// # 系统参数
///
/// * `board` - 游戏棋盘
/// * `config` - 游戏配置
/// * `session` - 游戏会话
/// * `state` - 当前游戏状态
/// * `hud_query` - HUD 文本实体查询
/// * `windows` - 主窗口查询
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
