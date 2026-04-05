use bevy::prelude::*;

// 游戏状态定义

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameStatus {
    #[default]
    MainMenu,
    DifficultySelect,
    Playing,
    Paused,
    Victory,
    Defeat,
}
