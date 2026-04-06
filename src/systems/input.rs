//! 输入处理系统模块
//!
//! 负责监听玩家的键盘和鼠标输入，并将其转换为游戏动作事件。

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::core::components::MainCamera;
use crate::core::events::CellActionEvent;
use crate::core::resources::{Board, DifficultyPreset, GameConfig};
use crate::utils::helpers::world_to_cell;

/// 发射玩家动作事件
///
/// 监听键盘和鼠标输入，根据输入类型写入相应的 [`CellActionEvent`] 事件。
///
/// # 键盘快捷键
///
/// | 按键 | 动作 |
/// |------|------|
/// | R | 重新开始游戏 |
/// | 1 | 切换到初级难度 |
/// | 2 | 切换到中级难度 |
/// | 3 | 切换到专家难度 |
///
/// # 鼠标操作
///
/// | 按键 | 动作 |
/// |------|------|
/// | 左键 | 揭开单元格 |
/// | 右键 | 切换旗帜标记 |
///
/// # 系统参数
///
/// * `mouse_buttons` - 鼠标按钮输入状态
/// * `keyboard` - 键盘输入状态
/// * `windows` - 主窗口查询
/// * `camera_query` - 主相机查询
/// * `board` - 游戏棋盘
/// * `config` - 游戏配置
/// * `event_writer` - 事件写入器
pub fn emit_player_actions(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    board: Res<Board>,
    config: Res<GameConfig>,
    mut event_writer: MessageWriter<CellActionEvent>,
) {
    // 键盘快捷键处理
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

    // 鼠标输入处理
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

    // 将屏幕坐标转换为世界坐标
    let Ok(world) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };

    // 将世界坐标转换为网格坐标并写入事件
    if let Some((row, col)) = world_to_cell(world, &board, &config) {
        if is_left {
            event_writer.write(CellActionEvent::reveal(row, col));
        }
        if is_right {
            event_writer.write(CellActionEvent::toggle_flag(row, col));
        }
    }
}
