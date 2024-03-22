use app_core::{plugins::AppSourcesPlugin, ExecLocation};
use bevy::{
    asset::io::{file::FileAssetReader, AssetSource},
    prelude::*,
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use ui_core::plugins::StartMenuPlugin;

pub fn main() {
    App::new()
        //asset sources
        .add_plugins(AppSourcesPlugin {
            exec_location: ExecLocation::CRATE
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(StartMenuPlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
