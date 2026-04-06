use bevy::{log::debug, prelude::*};

use crate::core::events::{CellAction, CellActionEvent};
use crate::core::resources::{Board, CellVisibility, GameSession};
use crate::game::flood_fill::reveal_connected_safe_area;
use crate::game::minefield::place_mines_with_safe_zone;
use crate::game::rules::{check_defeat, check_victory};
use crate::state::game_state::GameStatus;

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

fn toggle_flag(board: &mut Board, session: &mut GameSession, row: u32, col: u32) {
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

fn reveal_cell(
    board: &mut Board,
    session: &mut GameSession,
    row: u32,
    col: u32,
    next_state: &mut NextState<GameStatus>,
) {
    if session.frozen || !board.in_bounds(row as i32, col as i32) {
        return;
    }

    let Some(snapshot) = board.cell(row, col).cloned() else {
        return;
    };

    if snapshot.visibility == CellVisibility::Flagged
        || snapshot.visibility == CellVisibility::Revealed
    {
        return;
    }

    if session.first_click {
        place_mines_with_safe_zone(board, row, col);
        session.first_click = false;
    }

    let Some(cell_after_place) = board.cell(row, col).cloned() else {
        return;
    };

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

    let revealed_now = if cell_after_place.adjacent_mines == 0 {
        reveal_connected_safe_area(board, row, col)
    } else {
        if let Some(cell) = board.cell_mut(row, col) {
            cell.visibility = CellVisibility::Revealed;
            1
        } else {
            0
        }
    };

    session.revealed_safe_cells += revealed_now;

    if check_victory(board, session.revealed_safe_cells) {
        session.frozen = true;
        next_state.set(GameStatus::Victory);
    }
}
