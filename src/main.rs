//! 扫雷游戏入口
//!
//! 程序入口点，创建 Bevy 应用并添加扫雷插件。

use bevy::prelude::*;

use minesweeper::MinesweeperPlugin;

/// 程序入口
///
/// 创建 Bevy 应用实例，添加默认插件和扫雷插件，
/// 然后启动游戏循环。
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MinesweeperPlugin)
        .run();
}
