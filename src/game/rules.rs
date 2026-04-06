use crate::core::resources::{Board, CellData, CellVisibility};

pub fn check_defeat(cell: &CellData) -> bool {
    cell.is_mine && cell.visibility == CellVisibility::Revealed
}

pub fn check_victory(board: &Board, revealed_safe_cells: u32) -> bool {
    revealed_safe_cells == board.safe_cell_count()
}

pub fn count_revealed_safe_cells(board: &Board) -> u32 {
    board
        .iter_cells()
        .filter(|cell| !cell.is_mine && cell.visibility == CellVisibility::Revealed)
        .count() as u32
}

#[cfg(test)]
mod tests {
    use crate::core::resources::DifficultyPreset;

    use super::*;

    #[test]
    fn defeat_requires_revealed_mine() {
        let mut mine = CellData {
            is_mine: true,
            adjacent_mines: 0,
            visibility: CellVisibility::Hidden,
        };
        assert!(!check_defeat(&mine));

        mine.visibility = CellVisibility::Revealed;
        assert!(check_defeat(&mine));
    }

    #[test]
    fn victory_matches_safe_cell_target() {
        let board = Board::new(DifficultyPreset::Beginner);
        assert!(check_victory(&board, board.safe_cell_count()));
        assert!(!check_victory(&board, board.safe_cell_count() - 1));
    }

    #[test]
    fn counts_revealed_safe_cells_correctly() {
        let mut board = Board::new(DifficultyPreset::Beginner);
        board.clear_cells();

        board.cell_mut(0, 0).unwrap().visibility = CellVisibility::Revealed;
        board.cell_mut(0, 1).unwrap().is_mine = true;
        board.cell_mut(0, 1).unwrap().visibility = CellVisibility::Revealed;

        assert_eq!(count_revealed_safe_cells(&board), 1);
    }
}
