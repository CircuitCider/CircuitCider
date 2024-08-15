use bevy::{
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
};
use bevy_camera_extras::*;

use bevy_egui::EguiPlugin;

use app_core::{plugins::AppSourcesPlugin, ExecLocation};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use bevy_serialization_extras::prelude::{PhysicsSerializationPlugin, SerializationPlugin};
use bevy_serialization_urdf::plugin::{AssetSourcesUrdfPlugin, UrdfSerializationPlugin};
use robot_editor::{
    plugins::{setup_editor_area, RobotEditorPlugin},
    states::RobotEditorState,
};
use ui_core::plugins::StartMenuPlugin;
//use bevy_flycam::{FlyCam, PlayerPlugin, MovementSettings, NoCameraPlayerPlugin, KeyBindings};

fn main() {
    App::new()
        .add_plugins(AppSourcesPlugin::MAIN)
        .add_plugins(AssetSourcesUrdfPlugin {
            assets_folder_local_path: "assets".to_owned(),
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(StartMenuPlugin)
        .add_plugins(RobotEditorPlugin)
        //.add_plugins(WorldInspectorPlugin::default())
        // serialization plugins
        .add_plugins(SerializationPlugin)
        .add_plugins(PhysicsSerializationPlugin)
        .add_plugins(UrdfSerializationPlugin)
        // physics
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        // setup systems
        .add_systems(Startup, setup_camera)
        //.add_systems(Update, visualize_right_sidepanel_for::<Name>.run_if(in_state(AppState::Editor)))
        .add_systems(OnEnter(RobotEditorState::Active), setup_editor_area)
        .run();
}

// fn turn_on_editor(mut commands: Commands) {
//     commands.insert_resource(NextState(Some(RobotEditorState::Active)));
// }

/// set up a simple 3D scene
fn setup_camera(mut commands: Commands) {
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraController {
            restrained: CameraRestrained(true),
            camera_mode: CameraMode::Observer(ObserverCam::Orbit),
        },
        //bevy_transform_gizmo::GizmoPickSource::default(),
    ));
}
