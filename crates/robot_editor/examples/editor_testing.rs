use app_core::{plugins::AppSourcesPlugin, ExecLocation, ROOT};
use bevy::prelude::*;
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
    components::GizmoFocused, plugins::{setup_editor_area, RobotEditorPlugin}, prelude::{raycast_utils::debug::debug_mouse_info, selection_behaviour::components::Grabbed}, raycast_utils::debug::shoot_ray_down_to_target, states::RobotEditorState, systems::{delete_attach_candidates, delete_placers, move_placer_to_cursor}, ui::attach_placer
};




pub fn main() {
    App::new()
        .insert_state(RobotEditorState::Active)
        // app sources
        .add_plugins(AppSourcesPlugin {
            exec_location: ExecLocation::CRATE
        })
        .add_plugins(AssetSourcesUrdfPlugin {
            assets_folder_local_path: "../../assets".to_owned(),
        })
        .add_plugins(DefaultPlugins)
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
        //.add_systems(Update, visualize_window_for::<GizmoFocused>)
        .add_systems(Update, visualize_window_for::<Grabbed>)
        .add_systems(Update, visualize_window_for::<Camera>)
        //.add_systems(First, turn_on_editor)
        .add_systems(Update, debug_mouse_info)
        .add_systems(Update, shoot_ray_down_to_target)
        .add_systems(Startup, setup_editor_area)
        .run();
}

// fn turn_on_editor(mut commands: Commands) {
//     commands.insert_resource(NextState(Some(RobotEditorState::Active)));
// }

