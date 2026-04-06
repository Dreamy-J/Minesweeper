use std::collections::VecDeque;

use crate::core::resources::{Board, CellVisibility};

pub fn reveal_connected_safe_area(board: &mut Board, start_row: u32, start_col: u32) -> u32 {
    let mut revealed_safe = 0;
    let mut queue = VecDeque::new();
    queue.push_back((start_row, start_col));

    while let Some((row, col)) = queue.pop_front() {
        let (is_mine, adjacent, visibility) = match board.cell(row, col) {
            Some(cell) => (cell.is_mine, cell.adjacent_mines, cell.visibility),
            None => continue,
        };

        if is_mine || visibility != CellVisibility::Hidden {
            continue;
        }

        if let Some(cell) = board.cell_mut(row, col) {
            cell.visibility = CellVisibility::Revealed;
            revealed_safe += 1;
        }

        if adjacent == 0 {
            for (nr, nc) in board.neighbors(row, col) {
                if board
                    .cell(nr, nc)
                    .is_some_and(|neighbor| neighbor.visibility == CellVisibility::Hidden)
                {
                    queue.push_back((nr, nc));
                }
            }
        }
    }

    revealed_safe
}

#[cfg(test)]
mod tests {
    use crate::core::resources::{CellData, DifficultyPreset};
    use crate::game::minefield::recalculate_adjacent_counts;

    use super::*;

    #[test]
    fn flood_fill_reveals_empty_area_without_touching_flags() {
        let mut board = Board::new(DifficultyPreset::Beginner);
        board.clear_cells();
        board.cell_mut(0, 0).unwrap().is_mine = true;
        recalculate_adjacent_counts(&mut board);

        board.cell_mut(4, 4).unwrap().visibility = CellVisibility::Flagged;
        let revealed = reveal_connected_safe_area(&mut board, 8, 8);

        assert!(revealed > 1);
        assert_eq!(
            board.cell(4, 4).unwrap().visibility,
            CellVisibility::Flagged
        );
        assert_eq!(board.cell(0, 0).unwrap().visibility, CellVisibility::Hidden);
    }

    #[test]
    fn flood_fill_handles_pre_revealed_start() {
        let mut board = Board::new(DifficultyPreset::Beginner);
        board.clear_cells();
        board.cell_mut(3, 3).unwrap().visibility = CellVisibility::Revealed;

        let revealed = reveal_connected_safe_area(&mut board, 3, 3);
        assert_eq!(revealed, 0);
    }

    #[test]
    fn flood_fill_stops_at_number_boundary() {
        let mut board = Board::new(DifficultyPreset::Beginner);
        board.clear_cells();
        board.cell_mut(3, 3).unwrap().is_mine = true;
        recalculate_adjacent_counts(&mut board);

        let revealed = reveal_connected_safe_area(&mut board, 0, 0);
        assert!(revealed > 0);
        assert_eq!(
            board.cell(2, 2).unwrap().visibility,
            CellVisibility::Revealed
        );
        assert_eq!(board.cell(3, 3).unwrap().visibility, CellVisibility::Hidden);
    }

    #[test]
    fn keeps_board_shape_intact() {
        let mut board = Board::new(DifficultyPreset::Intermediate);
        let original_len = board.cell_count();
        board.clear_cells();
        reveal_connected_safe_area(&mut board, 0, 0);

        assert_eq!(board.cell_count(), original_len);
        assert!(board.iter_cells().all(|c| matches!(c, CellData { .. })));
    }
}
