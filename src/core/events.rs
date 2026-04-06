use bevy::prelude::*;

use crate::core::resources::DifficultyPreset;

#[derive(Debug, Clone, Copy)]
pub enum CellAction {
    Reveal { row: u32, col: u32 },
    ToggleFlag { row: u32, col: u32 },
    Restart,
    ChangeDifficulty(DifficultyPreset),
}

#[derive(Message, Debug, Clone, Copy)]
pub struct CellActionEvent {
    pub action: CellAction,
}

impl CellActionEvent {
    pub fn reveal(row: u32, col: u32) -> Self {
        Self {
            action: CellAction::Reveal { row, col },
        }
    }

    pub fn toggle_flag(row: u32, col: u32) -> Self {
        Self {
            action: CellAction::ToggleFlag { row, col },
        }
    }

    pub fn restart() -> Self {
        Self {
            action: CellAction::Restart,
        }
    }

    pub fn change_difficulty(difficulty: DifficultyPreset) -> Self {
        Self {
            action: CellAction::ChangeDifficulty(difficulty),
        }
    }
}
