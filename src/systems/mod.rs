pub mod audio;
pub mod game_logic;
pub mod input;
pub mod setup;
pub mod timer;

use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameSet {
    Input,
    Logic,
    Render,
}
