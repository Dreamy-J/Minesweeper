use bevy::prelude::*;

pub mod core;
pub mod game;
pub mod state;
pub mod systems;
pub mod ui;
pub mod utils;

use crate::core::events::CellActionEvent;
use crate::core::resources::{Board, CellEntityMap, DifficultyPreset, GameConfig, GameSession};
use crate::state::game_state::GameStatus;
use crate::systems::GameSet;

pub struct MinesweeperPlugin;

impl Plugin for MinesweeperPlugin {
    fn build(&self, app: &mut App) {
        let initial_difficulty = DifficultyPreset::Beginner;

        app.init_state::<GameStatus>()
            .insert_resource(GameConfig::default())
            .insert_resource(Board::new(initial_difficulty))
            .insert_resource(GameSession::new(initial_difficulty))
            .insert_resource(CellEntityMap::default())
            .add_message::<CellActionEvent>()
            .configure_sets(
                Update,
                (GameSet::Input, GameSet::Logic, GameSet::Render).chain(),
            )
            .add_systems(Startup, systems::setup::setup_scene)
            .add_systems(
                Update,
                systems::input::emit_player_actions.in_set(GameSet::Input),
            )
            .add_systems(
                Update,
                systems::game_logic::process_cell_actions.in_set(GameSet::Logic),
            )
            .add_systems(
                Update,
                systems::timer::update_timer
                    .run_if(in_state(GameStatus::Playing))
                    .in_set(GameSet::Logic),
            )
            .add_systems(
                Update,
                (
                    ui::grid_renderer::sync_grid_visuals,
                    ui::grid_renderer::update_cell_sprites,
                    ui::grid_renderer::update_cell_labels,
                    ui::hud::update_hud,
                )
                    .chain()
                    .in_set(GameSet::Render),
            );
    }
}
