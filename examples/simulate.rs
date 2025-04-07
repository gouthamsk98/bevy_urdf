use bevy::{
    color::palettes::css::WHITE,
    input::common_conditions::input_toggle_active,
    prelude::*,
};
use bevy_flycam::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use bevy_stl::StlPlugin;
use rapier3d::prelude::InteractionGroups;

use bevy_urdf::events::{ ControlMotors, LoadRobot, RobotLoaded };
use bevy_urdf::events::{ SensorsRead, SpawnRobot };
use bevy_urdf::plugin::UrdfPlugin;
use bevy_urdf::urdf_asset_loader::UrdfAsset;
use bevy_urdf::scene::ScenePlugin;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            UrdfPlugin,
            StlPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            // WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        ))
        .init_state::<AppState>()
        .insert_resource(MovementSettings {
            speed: 1.0,
            ..default()
        })
        .insert_resource(UrdfRobotHandle(None))
        .add_systems(Startup, setup)
        // .add_systems(Update, control_motors)
        .add_systems(Update, start_simulation.run_if(in_state(AppState::Loading)))
        .add_plugins(ScenePlugin)
        .run();
}

#[derive(Resource)]
struct UrdfRobotHandle(Option<Handle<UrdfAsset>>);

fn start_simulation(
    mut commands: Commands,
    mut er_robot_loaded: EventReader<RobotLoaded>,
    mut ew_spawn_robot: EventWriter<SpawnRobot>,
    mut state: ResMut<NextState<AppState>>
) {
    for event in er_robot_loaded.read() {
        ew_spawn_robot.send(SpawnRobot {
            handle: event.handle.clone(),
            mesh_dir: event.mesh_dir.clone(),
            parent_entity: None,
        });
        state.set(AppState::Simulation);
        commands.insert_resource(UrdfRobotHandle(Some(event.handle.clone())));
    }
}

fn print_sensor_values(mut er_read_sensors: EventReader<SensorsRead>) {
    for event in er_read_sensors.read() {
        continue;
        println!("Robot: {:?}", event.handle.id());
        println!("\transforms:");
        for transform in &event.transforms {
            let trans = transform.translation;
            let rot = transform.rotation;
            println!(
                "\t{} {} {} {} {} {} {}",
                trans.x,
                trans.y,
                trans.z,
                rot.x,
                rot.y,
                rot.z,
                rot.w
            );
        }

        let joint_angles_string: Vec<String> = event.joint_angles
            .iter()
            .map(|a| a.to_string())
            .collect();
        println!("\tjoint_angles:");
        println!("\t{}", joint_angles_string.join(" "));
    }
}

fn control_motors(
    robot_handle: Res<UrdfRobotHandle>,
    mut ew_control_motors: EventWriter<ControlMotors>
) {
    if let Some(handle) = robot_handle.0.clone() {
        let mut rng = rand::rng();
        let mut velocities: Vec<f32> = Vec::new();

        for _ in 0..50 {
            velocities.push(1.0);
        }

        ew_control_motors.send(ControlMotors { handle, velocities });
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    Simulation,
}

#[allow(deprecated)]
fn setup(mut ew_load_robot: EventWriter<LoadRobot>) {
    ew_load_robot.send(LoadRobot {
        urdf_path: "robots/mycobot_pro_600/mycobot_pro_600.urdf".to_string(),
        mesh_dir: "assets/robots/mycobot_pro_600/".to_string(),
        interaction_groups: None,
        marker: None,
    });
}
