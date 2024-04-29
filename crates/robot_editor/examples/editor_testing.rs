use app_core::{plugins::AppSourcesPlugin, ExecLocation, ROOT};
use bevy::{
    prelude::*,
    render::{
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    transform::commands,
};
use bevy_camera_extras::{plugins::DefaultCameraPlugin, FlyCameraSystems};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::{
    backends::raycast::RaycastBackend,
    debug::{DebugPickingMode, DebugPickingPlugin},
    focus::PickingInteraction,
    highlight::PickHighlight,
    picking_core::Pickable,
    selection::PickSelection,
    DefaultPickingPlugins, PickableBundle,
};
use bevy_obj::ObjPlugin;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use bevy_serialization_extras::prelude::{
    AssetSpawnRequest, AssetSpawnRequestQueue, PhysicsBundle, PhysicsSerializationPlugin,
    SerializationPlugin,
};
use bevy_serialization_urdf::{
    loaders::urdf_loader::Urdf,
    plugin::{AssetSourcesUrdfPlugin, UrdfSerializationPlugin},
};
use bevy_ui_extras::systems::{visualize_right_sidepanel_for, visualize_window_for};
use robot_editor::{
    model_display::components::DisplayModel, plugins::*, states::RobotEditorState,
    systems::shape::Cube,
};
pub fn main() {
    App::new()
        .insert_state(RobotEditorState::Active)
        // app sources
        .add_plugins(AppSourcesPlugin::CRATE)
        .add_plugins(AssetSourcesUrdfPlugin {
            assets_folder_local_path: "../../assets".to_owned(),
        })
        .add_plugins(DefaultPlugins)
        //.add_plugins(WorldInspectorPlugin::default())
        // robot editor
        .add_plugins(RobotEditorPlugin)
        // // serialization plugins
        .add_plugins(SerializationPlugin)
        .add_plugins(PhysicsSerializationPlugin)
        .add_plugins(UrdfSerializationPlugin)
        // // physics
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        // world setup
        .add_systems(Update, visualize_window_for::<DisplayModel>)
        .add_systems(Startup, setup_editor_area)
        //.add_systems(Update, display_model_image_to_file)
        .run();
}
