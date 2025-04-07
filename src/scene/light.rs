use bevy::prelude::*;
pub fn spawn_light(mut commands: Commands) {
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(3.0, 8.0, 5.0),
    ));
}
