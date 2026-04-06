use bevy::prelude::*;

// ECS组件定义

//* 网格相关组件

/// 网络单元格
#[derive(Component)]
pub struct Cell {
    pub row: u32,
    pub col: u32,
}

impl Cell {
    pub fn new(row: u32, col: u32) -> Self {
        Self { row, col }
    }
}

/// 是否存在地雷
#[derive(Component)]
pub struct Mine {
    pub is_mine: bool,
}

impl Mine {
    pub fn new(is_mine: bool) -> Self {
        Self { is_mine }
    }
}

/// 单元格状态
#[derive(Component, Default)]
pub enum CellState {
    #[default]
    Hidden,
    Revealed,
    Flagged,
    Questioned,
}

/// 相邻地雷数量（0-8）
#[derive(Component, Default)]
pub struct AdjacentMineCount(pub u8);

/// 单元格视觉表现
#[derive(Component)]
pub struct CellSprite {
    pub texture_index: u32,
}

impl CellSprite {
    pub fn new(texture_index: u32) -> Self {
        Self { texture_index }
    }
}

//* 游戏控制组件

/// 标记主相机
#[derive(Component)]
pub struct MainCamera;

/// 标记UI相机
#[derive(Component)]
pub struct UICamera;

/// 游戏难度
#[derive(Component)]
pub struct DifficultyConfig {
    pub rows: u32,
    pub cols: u32,
    pub mine_count: u32,
}
