use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;
pub fn spawn_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((
        Collider::cuboid(200.0, 0.05, 200.0),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(200.0)))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.3, 0.3))),
        Transform::from_xyz(0.0, -20.0, 0.0),
    ));
}
pub fn add_grid(mut gizmos: Gizmos) {
    gizmos.grid(
        Quat::from_rotation_x(PI / 2.0),
        UVec2::splat(200),
        Vec2::new(2.0, 2.0),
        // Light gray
        LinearRgba::gray(0.05)
    );
}
