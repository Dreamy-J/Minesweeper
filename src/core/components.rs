//! 组件定义模块
//!
//! 定义了扫雷游戏中使用的所有 ECS 组件。组件是纯数据容器，
//! 用于附加到实体上以描述实体的属性和行为。

use bevy::prelude::*;

/// 网格单元格组件
///
/// 标记一个实体为扫雷游戏中的单元格，并存储其行列位置信息。
/// 每个单元格实体都包含此组件，用于在输入处理和渲染时定位单元格。
#[derive(Component, Debug, Clone, Copy)]
pub struct Cell {
    /// 单元格所在的行号（从 0 开始）
    pub row: u32,
    /// 单元格所在的列号（从 0 开始）
    pub col: u32,
}

impl Cell {
    /// 创建一个新的单元格组件
    ///
    /// # 参数
    ///
    /// * `row` - 行号
    /// * `col` - 列号
    pub fn new(row: u32, col: u32) -> Self {
        Self { row, col }
    }
}

/// 主相机标记组件
///
/// 用于标记场景中的主 2D 相机实体。输入系统使用此组件
/// 来获取相机位置和变换，以便将屏幕坐标转换为世界坐标。
#[derive(Component)]
pub struct MainCamera;

/// 网格视觉元素标记组件
///
/// 标记所有与网格视觉相关的实体，包括单元格精灵、网格线和标签。
/// 用于在重建网格时批量查询和清理旧的视觉元素。
#[derive(Component)]
pub struct GridVisual;

/// 单元格文本标签组件
///
/// 附加到单元格旁边的 Text2d 实体上，用于显示数字或旗帜标记。
/// 包含行列信息以便与对应的单元格数据同步。
#[derive(Component)]
pub struct CellLabel {
    /// 对应的单元格行号
    pub row: u32,
    /// 对应的单元格列号
    pub col: u32,
}

/// HUD 文本标记组件
///
/// 用于标记 HUD（平视显示器）文本实体，
/// 该实体显示游戏状态信息如难度、时间、剩余雷数等。
#[derive(Component)]
pub struct HudText;
