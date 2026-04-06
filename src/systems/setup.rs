//! 初始化设置系统模块
//!
//! 负责在游戏启动时创建初始场景，包括相机和 HUD 文本实体。

use bevy::prelude::*;

use crate::core::components::{HudText, MainCamera};

/// 设置初始场景
///
/// 在 Startup 阶段执行，创建游戏所需的基础实体：
/// - 2D 主相机（带有 [`MainCamera`] 标记）
/// - HUD 文本实体（带有 [`HudText`] 标记）
///
/// # 系统参数
///
/// * `commands` - 命令队列
pub fn setup_scene(mut commands: Commands) {
    // 创建 2D 主相机
    commands.spawn((MainCamera, Camera2d));

    // 创建 HUD 文本实体
    commands.spawn((
        HudText,
        Text2d::new(""),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 0.0, 5.0),
    ));
}
