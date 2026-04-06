//! 游戏规则模块
//!
//! 实现了扫雷游戏的核心规则判定逻辑，包括：
//! - 失败条件检测
//! - 胜利条件检测
//! - 已揭开安全单元格计数

use crate::core::resources::{Board, CellData, CellVisibility};

/// 检查是否失败
///
/// 当玩家揭开了一个地雷单元格时，判定为失败。
///
/// # 参数
///
/// * `cell` - 被揭开的单元格数据
///
/// # 返回值
///
/// 如果该单元格是地雷且已被揭开，返回 `true`。
pub fn check_defeat(cell: &CellData) -> bool {
    cell.is_mine && cell.visibility == CellVisibility::Revealed
}

/// 检查是否胜利
///
/// 当所有非地雷单元格都被揭开时，判定为胜利。
///
/// # 参数
///
/// * `board` - 游戏棋盘
/// * `revealed_safe_cells` - 当前已揭开的安全单元格数量
///
/// # 返回值
///
/// 如果已揭开的安全单元格数等于棋盘上的安全单元格总数，返回 `true`。
pub fn check_victory(board: &Board, revealed_safe_cells: u32) -> bool {
    revealed_safe_cells == board.safe_cell_count()
}

/// 统计已揭开的安全单元格数量
///
/// 遍历棋盘上所有单元格，计算既不是地雷又已被揭开的单元格数量。
///
/// # 参数
///
/// * `board` - 游戏棋盘
///
/// # 返回值
///
/// 已揭开的安全单元格数量。
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
