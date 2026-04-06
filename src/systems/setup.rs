use bevy::prelude::*;

use crate::core::components::{HudText, MainCamera};

pub fn setup_scene(mut commands: Commands) {
    commands.spawn((MainCamera, Camera2d));

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
