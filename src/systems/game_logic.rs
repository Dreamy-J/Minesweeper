//! 游戏逻辑系统模块
//!
//! 处理所有游戏逻辑相关的操作，包括：
//! - 处理玩家动作事件
//! - 揭开单元格
//! - 切换旗帜标记
//! - 重置游戏
//! - 判定胜负

use bevy::{log::debug, prelude::*};

use crate::core::events::{CellAction, CellActionEvent};
use crate::core::resources::{Board, CellVisibility, GameSession};
use crate::game::flood_fill::reveal_connected_safe_area;
use crate::game::minefield::place_mines_with_safe_zone;
use crate::game::rules::{check_defeat, check_victory};
use crate::state::game_state::GameStatus;

/// 处理单元格动作事件
///
/// 从事件队列中读取所有 [`CellActionEvent`] 事件，
/// 并根据动作类型执行相应的游戏逻辑。
///
/// # 系统参数
///
/// * `action_reader` - 事件读取器
/// * `board` - 游戏棋盘（可变引用）
/// * `session` - 游戏会话（可变引用）
/// * `next_state` - 下一状态资源（用于切换游戏状态）
pub fn process_cell_actions(
    mut action_reader: MessageReader<CellActionEvent>,
    mut board: ResMut<Board>,
    mut session: ResMut<GameSession>,
    mut next_state: ResMut<NextState<GameStatus>>,
) {
    for event in action_reader.read() {
        match event.action {
            CellAction::Restart => {
                let difficulty = session.difficulty;
                reset_game(&mut board, &mut session, difficulty, &mut next_state);
            }
            CellAction::ChangeDifficulty(difficulty) => {
                reset_game(&mut board, &mut session, difficulty, &mut next_state);
            }
            CellAction::ToggleFlag { row, col } => {
                toggle_flag(&mut board, &mut session, row, col);
            }
            CellAction::Reveal { row, col } => {
                reveal_cell(&mut board, &mut session, row, col, &mut next_state);
            }
        }
    }
}

/// 重置游戏
///
/// 将棋盘和会话状态重置为初始状态，并切换到 Playing 状态。
///
/// # 参数
///
/// * `board` - 游戏棋盘
/// * `session` - 游戏会话
/// * `difficulty` - 目标难度
/// * `next_state` - 下一状态资源
fn reset_game(
    board: &mut Board,
    session: &mut GameSession,
    difficulty: crate::core::resources::DifficultyPreset,
    next_state: &mut NextState<GameStatus>,
) {
    board.reset(difficulty);
    session.difficulty = difficulty;
    session.reset_runtime();
    next_state.set(GameStatus::Playing);
    debug!("game reset: {}", difficulty.label());
}

/// 切换旗帜标记
///
/// 在指定位置放置或移除旗帜。如果单元格已揭开则不执行任何操作。
///
/// # 参数
///
/// * `board` - 游戏棋盘
/// * `session` - 游戏会话
/// * `row` - 行号
/// * `col` - 列号
fn toggle_flag(board: &mut Board, session: &mut GameSession, row: u32, col: u32) {
    // 游戏冻结或越界时不处理
    if session.frozen || !board.in_bounds(row as i32, col as i32) {
        return;
    }

    let Some(cell) = board.cell_mut(row, col) else {
        return;
    };

    match cell.visibility {
        CellVisibility::Hidden => {
            cell.visibility = CellVisibility::Flagged;
            session.flags_placed += 1;
        }
        CellVisibility::Flagged => {
            cell.visibility = CellVisibility::Hidden;
            session.flags_placed = session.flags_placed.saturating_sub(1);
        }
        CellVisibility::Revealed => {}
    }
}

/// 揭开单元格
///
/// 处理玩家揭开单元格的动作。包括：
/// 1. 首次点击时布置地雷（保证安全区）
/// 2. 点击到地雷时判定失败
/// 3. 点击到空白区域时展开连通安全区
/// 4. 检查是否达到胜利条件
///
/// # 参数
///
/// * `board` - 游戏棋盘
/// * `session` - 游戏会话
/// * `row` - 行号
/// * `col` - 列号
/// * `next_state` - 下一状态资源
fn reveal_cell(
    board: &mut Board,
    session: &mut GameSession,
    row: u32,
    col: u32,
    next_state: &mut NextState<GameStatus>,
) {
    // 游戏冻结或越界时不处理
    if session.frozen || !board.in_bounds(row as i32, col as i32) {
        return;
    }

    // 获取单元格快照
    let Some(snapshot) = board.cell(row, col).cloned() else {
        return;
    };

    // 已标记或已揭开的单元格不处理
    if snapshot.visibility == CellVisibility::Flagged
        || snapshot.visibility == CellVisibility::Revealed
    {
        return;
    }

    // 首次点击时布置地雷
    if session.first_click {
        place_mines_with_safe_zone(board, row, col);
        session.first_click = false;
    }

    // 获取布置地雷后的单元格状态
    let Some(cell_after_place) = board.cell(row, col).cloned() else {
        return;
    };

    // 点击到地雷
    if cell_after_place.is_mine {
        if let Some(cell) = board.cell_mut(row, col) {
            cell.visibility = CellVisibility::Revealed;
            if check_defeat(cell) {
                board.reveal_all_mines();
                session.frozen = true;
                next_state.set(GameStatus::Defeat);
            }
        }
        return;
    }

    // 揭开安全单元格
    let revealed_now = if cell_after_place.adjacent_mines == 0 {
        // 空白区域：展开连通安全区
        reveal_connected_safe_area(board, row, col)
    } else {
        // 数字区域：只揭开当前单元格
        if let Some(cell) = board.cell_mut(row, col) {
            cell.visibility = CellVisibility::Revealed;
            1
        } else {
            0
        }
    };

    session.revealed_safe_cells += revealed_now;

    // 检查胜利条件
    if check_victory(board, session.revealed_safe_cells) {
        session.frozen = true;
        next_state.set(GameStatus::Victory);
    }
}
