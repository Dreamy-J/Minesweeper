//! 扫雷游戏库
//!
//! 这是一个使用 Bevy 游戏引擎开发的经典扫雷游戏。
//! 项目采用 ECS（Entity-Component-System）架构模式。
//!
//! # 模块结构
//!
//! - `core`: 核心模块，包含组件、事件、资源定义
//! - `game`: 游戏逻辑模块，包含地雷布置、规则判定等
//! - `state`: 状态管理模块，包含游戏状态机
//! - `systems`: 系统模块，包含输入处理、游戏逻辑等
//! - `ui`: 用户界面模块，包含网格渲染、HUD 等
//! - `utils`: 工具模块，包含坐标转换辅助函数

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

/// 扫雷游戏插件
///
/// 主插件，负责注册所有游戏系统、资源和事件。
/// 将此插件添加到 Bevy 应用中即可运行游戏。
pub struct MinesweeperPlugin;

impl Plugin for MinesweeperPlugin {
    /// 构建插件
    ///
    /// 注册所有资源、事件、系统集合和系统。
    ///
    /// # 系统执行顺序
    ///
    /// 1. **Startup**: `setup_scene` - 初始化场景
    /// 2. **Update - Input**: `emit_player_actions` - 处理输入
    /// 3. **Update - Logic**: `process_cell_actions` - 处理游戏逻辑
    /// 4. **Update - Logic**: `update_timer` - 更新计时器（仅 Playing 状态）
    /// 5. **Update - Render**: `sync_grid_visuals` - 同步网格实体
    /// 6. **Update - Render**: `update_cell_sprites` - 更新精灵颜色
    /// 7. **Update - Render**: `update_cell_labels` - 更新文本标签
    /// 8. **Update - Render**: `update_hud` - 更新 HUD
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
