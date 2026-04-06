//! 网格渲染器模块
//!
//! 负责同步游戏棋盘数据与视觉表现，包括：
//! - 同步网格实体
//! - 更新单元格精灵颜色
//! - 更新单元格文本标签

use bevy::prelude::*;

use crate::core::components::{Cell, CellLabel, GridVisual};
use crate::core::resources::{Board, CellEntityMap, CellVisibility, GameConfig};
use crate::game::grid::{create_grid_visuals, despawn_grid_visuals};

/// 同步网格视觉元素
///
/// 检查当前视觉实体数量是否与棋盘单元格数量匹配。
/// 如果不匹配（如游戏重置后），则清除旧实体并重新创建。
///
/// # 系统参数
///
/// * `commands` - 命令队列
/// * `board` - 游戏棋盘
/// * `config` - 游戏配置
/// * `entity_map` - 单元格实体映射
/// * `visuals` - 所有网格视觉实体查询
/// * `cell_visuals` - 单元格视觉实体查询
pub fn sync_grid_visuals(
    mut commands: Commands,
    board: Res<Board>,
    config: Res<GameConfig>,
    mut entity_map: ResMut<CellEntityMap>,
    visuals: Query<Entity, With<GridVisual>>,
    cell_visuals: Query<Entity, With<Cell>>,
) {
    // 如果实体数量匹配则跳过
    if entity_map.entities.len() == board.cell_count()
        && cell_visuals.iter().count() == board.cell_count()
    {
        return;
    }

    despawn_grid_visuals(&mut commands, &visuals);
    create_grid_visuals(&mut commands, &board, &config, &mut entity_map);
}

/// 更新单元格精灵颜色
///
/// 根据单元格的当前状态更新其背景颜色：
/// - 隐藏：灰色
/// - 旗帜：橙色
/// - 已揭开-地雷：红色
/// - 已揭开-空白：浅灰
/// - 已揭开-数字：中灰
///
/// # 系统参数
///
/// * `board` - 游戏棋盘
/// * `cells` - 所有单元格组件和精灵查询
pub fn update_cell_sprites(board: Res<Board>, mut cells: Query<(&Cell, &mut Sprite)>) {
    for (cell_ref, mut sprite) in &mut cells {
        let Some(cell) = board.cell(cell_ref.row, cell_ref.col) else {
            continue;
        };

        sprite.color = match cell.visibility {
            CellVisibility::Hidden => Color::srgb(0.45, 0.45, 0.45),
            CellVisibility::Flagged => Color::srgb(0.95, 0.6, 0.15),
            CellVisibility::Revealed => {
                if cell.is_mine {
                    Color::srgb(0.85, 0.1, 0.1)
                } else if cell.adjacent_mines == 0 {
                    Color::srgb(0.85, 0.85, 0.85)
                } else {
                    Color::srgb(0.75, 0.75, 0.75)
                }
            }
        };
    }
}

/// 更新单元格文本标签
///
/// 根据单元格状态更新显示的文本：
/// - 隐藏：空字符串
/// - 旗帜："F"
/// - 地雷："*"
/// - 数字：显示相邻地雷数
///
/// 数字颜色根据值不同而变化。
///
/// # 系统参数
///
/// * `board` - 游戏棋盘
/// * `labels` - 所有标签组件查询
pub fn update_cell_labels(
    board: Res<Board>,
    mut labels: Query<(&CellLabel, &mut Text2d, &mut TextColor)>,
) {
    for (label, mut text, mut text_color) in &mut labels {
        let Some(cell) = board.cell(label.row, label.col) else {
            continue;
        };

        let (value, color) = match cell.visibility {
            CellVisibility::Hidden => (String::new(), Color::BLACK),
            CellVisibility::Flagged => ("F".to_string(), Color::BLACK),
            CellVisibility::Revealed => {
                if cell.is_mine {
                    ("*".to_string(), Color::BLACK)
                } else if cell.adjacent_mines == 0 {
                    (String::new(), Color::BLACK)
                } else {
                    (
                        cell.adjacent_mines.to_string(),
                        number_color(cell.adjacent_mines),
                    )
                }
            }
        };

        *text = Text2d::new(value);
        *text_color = TextColor(color);
    }
}

/// 获取数字对应的颜色
///
/// 不同数字使用不同颜色，便于玩家区分：
/// - 1: 蓝色
/// - 2: 绿色
/// - 3: 红色
/// - 4: 紫色
/// - 其他: 黑色
fn number_color(adjacent: u8) -> Color {
    match adjacent {
        1 => Color::srgb(0.1, 0.2, 0.8),
        2 => Color::srgb(0.0, 0.55, 0.15),
        3 => Color::srgb(0.75, 0.1, 0.1),
        4 => Color::srgb(0.4, 0.1, 0.6),
        _ => Color::srgb(0.1, 0.1, 0.1),
    }
}
