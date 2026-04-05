use bevy::prelude::*;

use crate::core::components::MainCamera;
use crate::core::resources::GameConfig;
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
) {
    create_grid(&mut commands, &config);
}
