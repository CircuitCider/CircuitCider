//! test for the positioning of displaying models
//!

use bevy::prelude::*;
use bevy_ui_extras::UiExtrasDebug;
use robot_editor::model_display::{plugins::ModelDisplayerPlugin, DisplayModel};
use shader_core::shaders::plugins::CustomShadersPlugin;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiExtrasDebug::default())
        .add_plugins(CustomShadersPlugin)
        .add_plugins(ModelDisplayerPlugin)
        .add_systems(Startup, display_model_test)
        .run();
}

pub fn display_model_test(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    //mut neon_materials: ResMut<Assets<NeonMaterial>>,
    mut display_model: ResMut<DisplayModel>
) {
    let cube = commands.spawn(
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0).mesh()))
    ).id();
    //let mesh_handle = meshes.add(Cuboid::new(1.0, 1.0, 1.0).mesh());
    display_model.0 = Some(cube);
}
