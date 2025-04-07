use bevy::prelude::*;
mod plane;
mod light;
mod camera;
// mod props;
// pub mod gui;
pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            plane::spawn_plane,
            light::spawn_light,
            camera::spawn_camera,
            // props::spwan_obj,
            // props::spwan_obj,
        ));
        app.add_systems(Update, (
            // plane::add_grid,
            camera::update_camera,
            // props::attach_collider_after_load,
        ));
    }
}
