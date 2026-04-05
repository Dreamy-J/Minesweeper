// 网格相关组件
use bevy::prelude::*;

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

#[derive(Component)]
pub struct Mine {
    pub is_mine: bool,
}

impl Mine {
    pub fn new(is_mine: bool) -> Self {
        Self { is_mine }
    }
}

#[derive(Component, Default)]
pub enum CellState {
    #[default]
    Hidden,
    Revealed,
    Flagged,
    Questioned,
}

#[derive(Component, Default)]
pub struct AdjacentMineCount(pub u8);
