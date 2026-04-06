use bevy::prelude::*;

pub mod core;
pub mod game;
pub mod state;
pub mod systems;
pub mod ui;
pub mod utils;

pub struct MinesweeperPlugin;

impl Plugin for MinesweeperPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(core::resources::GameConfig::new(22, 22, 10, 32.0))
            .insert_resource(core::resources::GridTable::new())
            .add_systems(Startup, systems::setup::setup)
            .add_systems(Startup, systems::setup::restart_game)
            .add_systems(Update, systems::input::handle_mouse_input);
    }
}
