//! 地雷布置算法模块
//!
//! 实现了随机布置地雷的逻辑，确保首次点击位置及其周围
//! 区域安全（无地雷）。

use rand::seq::SliceRandom;

use crate::core::resources::Board;

/// 布置地雷（带安全区）
///
/// 在棋盘上随机布置指定数量的地雷，同时确保指定的安全位置
/// 及其周围 3x3 区域内不会有地雷。
///
/// 如果棋盘过于密集（地雷数 > 可用位置数），则回退到
/// 仅排除点击位置本身。
///
/// # 参数
///
/// * `board` - 游戏棋盘的可变引用
/// * `safe_row` - 安全区中心行号
/// * `safe_col` - 安全区中心列号
pub fn place_mines_with_safe_zone(board: &mut Board, safe_row: u32, safe_col: u32) {
    // 先清空所有地雷状态
    for row in 0..board.rows {
        for col in 0..board.cols {
            if let Some(cell) = board.cell_mut(row, col) {
                cell.is_mine = false;
                cell.adjacent_mines = 0;
            }
        }
    }

    // 收集所有候选位置（排除安全区）
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

    // 如果候选位置不够，回退到仅排除点击位置
    if (board.total_mines as usize) > candidates.len() {
        candidates.clear();
        for row in 0..board.rows {
            for col in 0..board.cols {
                if !(row == safe_row && col == safe_col) {
                    candidates.push((row, col));
                }
            }
        }
    }

    // 使用 Fisher-Yates 洗牌算法随机选择地雷位置
    let mut rng = rand::rng();
    candidates.shuffle(&mut rng);

    // 布置地雷
    for &(row, col) in candidates.iter().take(board.total_mines as usize) {
        if let Some(cell) = board.cell_mut(row, col) {
            cell.is_mine = true;
        }
    }

    // 重新计算每个单元格周围的地雷数
    recalculate_adjacent_counts(board);
    board.mines_placed = true;
}

/// 重新计算所有单元格周围的地雷数量
///
/// 遍历棋盘上的每个单元格，统计其 8 个邻居中的地雷数量，
/// 并更新到 `adjacent_mines` 字段。
///
/// # 参数
///
/// * `board` - 游戏棋盘的可变引用
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
