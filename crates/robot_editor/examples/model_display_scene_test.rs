//! test for the positioning of displaying models
//!

use app_core::plugins::AppSourcesPlugin;
use bevy::{
    gltf::{GltfMesh, GltfNode},
    prelude::*,
};
use bevy_serialization_extras::prelude::{
    SerializationAssembleBasePlugin, SerializationBasePlugin,
};
use bevy_ui_extras::UiExtrasDebug;
use robot_editor::model_display::{DisplayModel, DisplayOption, plugins::ModelDisplayerPlugin};
use shader_core::shaders::plugins::CustomShadersPlugin;

pub fn main() {
    App::new()
        .add_plugins(AppSourcesPlugin::CRATE)
        .add_plugins(DefaultPlugins)
        .add_plugins(SerializationBasePlugin)
        .add_plugins(SerializationAssembleBasePlugin)
        .add_plugins(UiExtrasDebug::default())
        .add_plugins(CustomShadersPlugin)
        .add_plugins(ModelDisplayerPlugin)
        .add_systems(Startup, display_model_test)
        .run();
}

pub fn display_model_test(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    //mut gltfs: ResMut<Assets<Gltf>>,
    mut asset_server: Res<AssetServer>,
    mut display_model: ResMut<DisplayModel>,
) {
    // let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0).mesh());
    // display_model.0 = Some(DisplayOption::Mesh(cube));

    //let gun = asset_server.load("root://models/weapons.robot_gun.glb");
    let gun = asset_server.load::<GltfNode>(
        GltfAssetLabel::Node(0).from_asset("root://models/weapons/robot_gun.glb"),
    );
    //let gun = asset_server.load("root://models/weapons/robot_gun.glb");
    display_model.0 = Some(DisplayOption::Handle(gun))
}
