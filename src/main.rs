use bevy::prelude::*;

use minesweeper::MinesweeperPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MinesweeperPlugin)
        .run();
}
