use app_core::{plugins::AppSourcesPlugin};
use bevy::{
    prelude::*,
};
use bevy_mod_raycast::cursor::CursorRayPlugin;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use bevy_serialization_extras::prelude::{
    mesh::{GeometryFile, GeometryFlag}, DeserializeAssetFrom, SerializationBasePlugin, SerializationPhysicsPlugin, SerializationPlugin
};
use bevy_serialization_urdf::{
    plugin::{AssetSourcesUrdfPlugin, UrdfSerializationPlugin},
};
use bevy_ui_extras::*;
use robot_editor::{model_display::components::DisplayModel, plugins::*, resources::RobotControls, states::RobotEditorState};

pub fn main() {
    App::new()
        .add_plugins(AppSourcesPlugin::CRATE)
        .add_plugins(AssetSourcesUrdfPlugin {
            assets_folder_local_path: "../../assets".to_owned(),
        })
        .add_plugins(DefaultPlugins.set(bevy_mod_raycast::low_latency_window_plugin()))
        .insert_state(RobotEditorState::Active)
        // robot editor
        .add_plugins(RobotEditorPlugin)
        .add_plugins(UiExtrasDebug {
            ui_style: UiStyle::BlackGlass,
            ..default()
        })

        .add_plugins(SerializationPlugin)
        .add_plugins(DeserializeAssetFrom::<GeometryFlag, Mesh>::default())
        .add_plugins(DeserializeAssetFrom::<GeometryFile, Mesh>::default())

        .add_plugins(SerializationPhysicsPlugin)
        .add_plugins(UrdfSerializationPlugin)
        // // physics
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugins(RapierDebugRenderPlugin::default())

        .add_systems(Startup, setup_editor_area)
        .run();
}
