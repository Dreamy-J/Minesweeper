use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
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
pub struct MainCamera;

#[derive(Component)]
pub struct GridVisual;

#[derive(Component)]
pub struct CellLabel {
    pub row: u32,
    pub col: u32,
}

#[derive(Component)]
pub struct HudText;
