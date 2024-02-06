use bevy::{prelude::*, window::PrimaryWindow, input::common_conditions::input_toggle_active};
use bevy_camera_extras::plugins::DefaultCameraPlugin;
use bevy_egui::{EguiPlugin, EguiContext};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::{plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use bevy_serialization_extras::prelude::{link::{JointFlag, StructureFlag}, rigidbodies::RigidBodyFlag, AssetSpawnRequest, AssetSpawnRequestQueue, PhysicsBundle, PhysicsSerializationPlugin, SerializationPlugin};
use bevy_serialization_urdf::{loaders::urdf_loader::Urdf, plugin::UrdfSerializationPlugin, ui::DEBUG_FRAME_STYLE};
use bevy_camera_extras::prelude::*;
use bevy_component_extras::components::*;
use bevy_ui_extras::systems::*;

use robot_editor::plugins::RobotEditorPlugin;
use strum_macros::Display;
use ui_core::plugins::StartMenuPlugin;
//use bevy_flycam::{FlyCam, PlayerPlugin, MovementSettings, NoCameraPlayerPlugin, KeyBindings};


fn main() {
    App::new()
        .insert_resource(KeyBindings {toggle_grab_cursor: KeyCode::ControlLeft, ..default()})
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(DefaultCameraPlugin)
        .add_plugins(StartMenuPlugin)
        .add_plugins(RobotEditorPlugin)


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

        .run()
        ;
}


/// set up a simple 3D scene
fn setup_camera(
    mut commands: Commands,
) {

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
