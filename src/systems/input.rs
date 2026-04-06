use bevy::prelude::*;

use crate::core::components::MainCamera;
use crate::core::resources::GameConfig;
use crate::core::resources::GridTable;
use crate::utils::helpers::world_to_vec;

// 输入处理

pub fn handle_mouse_input(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    game_config: Res<GameConfig>,
    grid_table: ResMut<GridTable>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let window = windows.single().unwrap();
        let (camera, camera_transform) = cameras.single().unwrap();

        if let Some(cursor_pos) = window.cursor_position()
            && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos)
        {
            let (i, j) = world_to_vec(world_pos.x, world_pos.y, &game_config);
            println!("Clicked at: ({}, {})", i, j);
            if let Some(entity) = grid_table.get_entity(i, j, &game_config) {
                println!("Entity: {:?}", entity);
            }
        }
    }
}
