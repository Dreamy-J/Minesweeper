use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameStatus {
    MainMenu,
    DifficultySelect,
    #[default]
    Playing,
    Paused,
    Victory,
    Defeat,
}
