

use bevy::{
    prelude::*,
};
use bevy_camera_extras::*;

use bevy_egui::EguiPlugin;

use app_core::{plugins::AppSourcesPlugin};
use bevy_mod_raycast::cursor::CursorRayPlugin;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use bevy_serialization_urdf::plugin::AssetSourcesUrdfPlugin;
use bevy_ui_extras::{UiExtrasDebug, UiStyle};
use robot_editor::plugins::RobotEditorPlugin;
use ui_core::plugins::StartMenuPlugin;

fn main() {
    App::new()
        .add_plugins(AppSourcesPlugin::MAIN)
        .add_plugins(AssetSourcesUrdfPlugin {
            assets_folder_local_path: "assets".to_owned(),
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(UiExtrasDebug {
            ui_style: UiStyle::BlackGlass,
            ..default()
        })        
        .add_plugins(StartMenuPlugin)
        .add_plugins(RobotEditorPlugin)
        
        // setup systems
        .add_systems(Startup, setup_camera)
        .run();
}

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
