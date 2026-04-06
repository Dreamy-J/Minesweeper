use bevy::prelude::*;

use crate::core::components::MainCamera;
use crate::core::resources::{GameConfig, GridTable};
use crate::game::grid::create_grid;

//* 初始化系统

/// 设置主相机
pub fn setup(mut commands: Commands) {
    commands.spawn((MainCamera, Camera2d));
}

/// 重新开始游戏
pub fn restart_game(
    mut commands: Commands,
    config: Res<GameConfig>,
    mut grid_table: ResMut<GridTable>, 
) {
    create_grid(&mut commands, &config, &mut grid_table);
}
