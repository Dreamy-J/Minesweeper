//! 计时器系统模块
//!
//! 负责更新游戏计时器，记录玩家的游戏时间。

use bevy::prelude::*;

use crate::core::resources::GameSession;
use crate::state::game_state::GameStatus;

/// 更新计时器
///
/// 在游戏进行中（Playing 状态且未冻结）累加已用时间。
///
/// # 运行条件
///
/// 仅在 `GameStatus::Playing` 状态下执行。
///
/// # 系统参数
///
/// * `time` - 时间资源
/// * `current_state` - 当前游戏状态
/// * `session` - 游戏会话（可变引用）
pub fn update_timer(
    time: Res<Time>,
    current_state: Res<State<GameStatus>>,
    mut session: ResMut<GameSession>,
) {
    if *current_state.get() == GameStatus::Playing && !session.frozen {
        session.elapsed_seconds += time.delta_secs();
    }
}
