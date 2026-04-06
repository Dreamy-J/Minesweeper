//! 游戏状态模块
//!
//! 定义了游戏的所有可能状态，用于状态机驱动的系统执行控制。

use bevy::prelude::*;

/// 游戏状态枚举
///
/// 表示游戏当前所处的状态。Bevy 的状态系统会根据当前状态
/// 自动决定是否执行某些系统（如计时器只在 Playing 状态下运行）。
#[derive(States, Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameStatus {
    /// 主菜单状态
    MainMenu,
    /// 难度选择状态
    DifficultySelect,
    /// 游戏中状态（默认状态）
    #[default]
    Playing,
    /// 暂停状态
    Paused,
    /// 胜利状态
    Victory,
    /// 失败状态
    Defeat,
}
