//! 事件定义模块
//!
//! 定义了扫雷游戏中使用的事件类型。事件用于在系统之间
//! 传递消息，实现输入系统与游戏逻辑系统的解耦。

use bevy::prelude::*;

use crate::core::resources::DifficultyPreset;

/// 单元格动作枚举
///
/// 表示玩家可以执行的各种游戏动作。这些动作由输入系统
/// 生成并通过事件队列传递给游戏逻辑系统处理。
#[derive(Debug, Clone, Copy)]
pub enum CellAction {
    /// 揭开指定位置的单元格
    Reveal {
        /// 行号
        row: u32,
        /// 列号
        col: u32,
    },
    /// 切换指定位置的旗帜标记
    ToggleFlag {
        /// 行号
        row: u32,
        /// 列号
        col: u32,
    },
    /// 重新开始当前游戏
    Restart,
    /// 更改游戏难度（会自动重新开始）
    ChangeDifficulty(DifficultyPreset),
}

/// 单元格动作事件
///
/// 封装 [`CellAction`] 的 Bevy Message 类型。输入系统写入此事件，
/// 游戏逻辑系统读取并处理这些事件。
#[derive(Message, Debug, Clone, Copy)]
pub struct CellActionEvent {
    /// 具体的动作类型
    pub action: CellAction,
}

impl CellActionEvent {
    /// 创建一个揭开单元格的事件
    ///
    /// # 参数
    ///
    /// * `row` - 行号
    /// * `col` - 列号
    pub fn reveal(row: u32, col: u32) -> Self {
        Self {
            action: CellAction::Reveal { row, col },
        }
    }

    /// 创建一个切换旗帜标记的事件
    ///
    /// # 参数
    ///
    /// * `row` - 行号
    /// * `col` - 列号
    pub fn toggle_flag(row: u32, col: u32) -> Self {
        Self {
            action: CellAction::ToggleFlag { row, col },
        }
    }

    /// 创建一个重新开始游戏的事件
    pub fn restart() -> Self {
        Self {
            action: CellAction::Restart,
        }
    }

    /// 创建一个更改难度的事件
    ///
    /// # 参数
    ///
    /// * `difficulty` - 目标难度级别
    pub fn change_difficulty(difficulty: DifficultyPreset) -> Self {
        Self {
            action: CellAction::ChangeDifficulty(difficulty),
        }
    }
}
