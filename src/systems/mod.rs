//! 系统模块
//!
//! 包含所有游戏系统的定义和注册：
//! - `audio`: 音频系统（待实现）
//! - `game_logic`: 游戏逻辑系统
//! - `input`: 输入处理系统
//! - `setup`: 初始化系统
//! - `timer`: 计时器系统

pub mod audio;
pub mod game_logic;
pub mod input;
pub mod setup;
pub mod timer;

use bevy::prelude::*;

/// 游戏系统集合
///
/// 用于对系统进行分组和排序，确保系统按正确的顺序执行。
/// 执行顺序为：Input → Logic → Render
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameSet {
    /// 输入处理阶段
    Input,
    /// 游戏逻辑处理阶段
    Logic,
    /// 渲染更新阶段
    Render,
}
