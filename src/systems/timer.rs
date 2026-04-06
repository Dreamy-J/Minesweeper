use bevy::prelude::*;

use crate::core::resources::GameSession;
use crate::state::game_state::GameStatus;

pub fn update_timer(
    time: Res<Time>,
    current_state: Res<State<GameStatus>>,
    mut session: ResMut<GameSession>,
) {
    if *current_state.get() == GameStatus::Playing && !session.frozen {
        session.elapsed_seconds += time.delta_secs();
    }
}
