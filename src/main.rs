use app_internal::AppDefaultPlugins;
use bevy::prelude::*;
use bevy_camera_extras::*;

use app_core::plugins::AppSourcesPlugin;
use bevy_serialization_extras::prelude::AssetSourcesUrdfPlugin;
use ui_core::plugins::StartMenuPlugin;

fn main() {
    App::new()
        .add_plugins(AppSourcesPlugin::MAIN)
        .add_plugins(AssetSourcesUrdfPlugin {
            assets_folder_local_path: "assets".to_owned(),
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(AppDefaultPlugins)
        .add_plugins(StartMenuPlugin)
        // setup systems
        .add_systems(Startup, setup_camera)
        .run();
}

/// set up a simple 3D scene
fn setup_camera(mut commands: Commands) {
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        CameraController {
            restrained: CameraRestrained(true),
            camera_mode: CameraMode::Observer(ObserverCam::Orbit),
        },
    ));
}
