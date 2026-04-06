use rand::seq::SliceRandom;

use crate::core::resources::Board;

pub fn place_mines_with_safe_zone(board: &mut Board, safe_row: u32, safe_col: u32) {
    for row in 0..board.rows {
        for col in 0..board.cols {
            if let Some(cell) = board.cell_mut(row, col) {
                cell.is_mine = false;
                cell.adjacent_mines = 0;
            }
        }
    }

    let mut candidates = Vec::with_capacity(board.cell_count());
    let mut forbidden = board.neighbors(safe_row, safe_col);
    forbidden.push((safe_row, safe_col));

    for row in 0..board.rows {
        for col in 0..board.cols {
            if !forbidden.contains(&(row, col)) {
                candidates.push((row, col));
            }
        }
    }

    if (board.total_mines as usize) > candidates.len() {
        // Fallback to only exclude the clicked cell when the board is too dense.
        candidates.clear();
        for row in 0..board.rows {
            for col in 0..board.cols {
                if !(row == safe_row && col == safe_col) {
                    candidates.push((row, col));
                }
            }
        }
    }

    let mut rng = rand::rng();
    candidates.shuffle(&mut rng);

    for &(row, col) in candidates.iter().take(board.total_mines as usize) {
        if let Some(cell) = board.cell_mut(row, col) {
            cell.is_mine = true;
        }
    }

    recalculate_adjacent_counts(board);
    board.mines_placed = true;
}

pub fn recalculate_adjacent_counts(board: &mut Board) {
    for row in 0..board.rows {
        for col in 0..board.cols {
            let mine_neighbors = board
                .neighbors(row, col)
                .iter()
                .filter(|&&(nr, nc)| board.cell(nr, nc).is_some_and(|c| c.is_mine))
                .count() as u8;

            if let Some(cell) = board.cell_mut(row, col) {
                cell.adjacent_mines = mine_neighbors;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::core::resources::DifficultyPreset;

    use super::*;

    #[test]
    fn places_expected_mine_count_without_duplicates() {
        let mut board = Board::new(DifficultyPreset::Beginner);
        place_mines_with_safe_zone(&mut board, 4, 4);

        let mines: Vec<_> = (0..board.rows)
            .flat_map(|r| (0..board.cols).map(move |c| (r, c)))
            .filter(|&(r, c)| board.cell(r, c).is_some_and(|cell| cell.is_mine))
            .collect();

        assert_eq!(mines.len(), board.total_mines as usize);
        let unique: HashSet<_> = mines.iter().copied().collect();
        assert_eq!(unique.len(), mines.len());
    }

    #[test]
    fn keeps_first_click_3x3_safe() {
        let mut board = Board::new(DifficultyPreset::Beginner);
        let center = (4, 4);
        place_mines_with_safe_zone(&mut board, center.0, center.1);

        let mut safe_zone = board.neighbors(center.0, center.1);
        safe_zone.push(center);
        for (r, c) in safe_zone {
            assert!(!board.cell(r, c).unwrap().is_mine);
        }
    }

    #[test]
    fn computes_adjacent_mines_for_corner_and_center() {
        let mut board = Board::new(DifficultyPreset::Beginner);
        board.clear_cells();
        board.cell_mut(0, 1).unwrap().is_mine = true;
        board.cell_mut(1, 0).unwrap().is_mine = true;
        board.cell_mut(1, 1).unwrap().is_mine = true;
        recalculate_adjacent_counts(&mut board);

        assert_eq!(board.cell(0, 0).unwrap().adjacent_mines, 3);
        assert_eq!(board.cell(2, 2).unwrap().adjacent_mines, 1);
    }
}
