use app_core::{plugins::AppSourcesPlugin, ExecLocation, ROOT};
use bevy::prelude::*;
use bevy_camera_extras::{plugins::DefaultCameraPlugin, FlyCameraSystems};
use bevy_mod_picking::{backends::raycast::RaycastBackend, debug::DebugPickingMode, focus::PickingInteraction, highlight::PickHighlight, picking_core::Pickable, selection::PickSelection, DefaultPickingPlugins, PickableBundle};
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
use bevy_transform_gizmo::{GizmoTransformable, TransformGizmoPlugin};
use bevy_ui_extras::systems::{visualize_right_sidepanel_for, visualize_window_for};
use robot_editor::plugins::*;
use robot_editor::states::*;




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

        .add_plugins(DefaultCameraPlugin)

        // Picking
        .add_plugins(
            (
                DefaultPickingPlugins.build(),
                TransformGizmoPlugin::new(
                    Quat::from_rotation_y(-0.2), // Align the gizmo to a different coordinate system.
                                                 // Use TransformGizmoPlugin::default() to align to the
                                                 // scene's coordinate system.
                ),
            )
        )
        .insert_resource(DebugPickingMode::Normal)
        // .insert_resource(RapierBackendSettings {
        //     require_markers: true, // Optional: only needed when you want fine-grained control over which cameras and entities should be used with the rapier picking backend. This is disabled by default, and no marker components are required on cameras or colliders. This resource is inserted by default, you only need to add it if you want to override the default settings.
        // })

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
        //.add_systems(Update, visualize_window_for::<Grabbed>)
        .add_systems(Update, visualize_window_for::<Camera>)
        //.add_systems(First, turn_on_editor)
        //.add_systems(Update, debug_mouse_info)
        //.add_systems(Update, shoot_ray_down_to_target)
        .add_systems(Startup, setup_editor_area)
        .add_systems(Update, make_models_pickable)
        .run();
}

pub fn make_models_pickable(
    mut commands: Commands,
    models_query: Query<Entity, (With<Handle<Mesh>>, Without<Pickable>)>,
) {
    for e in models_query.iter() {
        commands.entity(e).insert(
            (
                PickableBundle {
                    pickable: Pickable::default(),
                    interaction: PickingInteraction::default(),
                    selection: PickSelection::default(),
                    highlight: PickHighlight::default(),
                },
                GizmoTransformable,
            )
        );
    }
}

// fn turn_on_editor(mut commands: Commands) {
//     commands.insert_resource(NextState(Some(RobotEditorState::Active)));
// }

