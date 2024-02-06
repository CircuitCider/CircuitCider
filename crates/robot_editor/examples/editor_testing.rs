use bevy::{asset::io::{file::FileAssetReader, AssetSource}, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::{plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use bevy_serialization_extras::prelude::{AssetSpawnRequest, AssetSpawnRequestQueue, PhysicsBundle, PhysicsSerializationPlugin, SerializationPlugin};
use bevy_serialization_urdf::{loaders::urdf_loader::Urdf, plugin::UrdfSerializationPlugin};
use robot_editor::{plugins::RobotEditorPlugin, states::RobotEditorState};
use app_core::{plugins::AppSourcesPlugin, ROOT};

pub fn main() {
    
    App::new()
    
    // app sources
    .add_plugins(AppSourcesPlugin)

    .add_plugins(DefaultPlugins)
    .add_plugins(RobotEditorPlugin)  

    // serialization plugins
    .add_plugins(SerializationPlugin)
    .add_plugins(PhysicsSerializationPlugin)
    .add_plugins(UrdfSerializationPlugin)
    
    // physics
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugins(RapierDebugRenderPlugin::default())


    .add_systems(Startup, setup_camera)
    .add_systems(PostStartup, turn_on_editor)
    .add_systems(OnEnter(RobotEditorState::Active), spawn_robot)

    .run()
    ;
}


pub fn spawn_robot(
    mut urdf_load_requests: ResMut<AssetSpawnRequestQueue<Urdf>>,
) {
    urdf_load_requests.requests.push_front(
        AssetSpawnRequest {
                source: format!("{:#}://model_pkg/urdf/diff_bot.xml", ROOT).to_owned().into(), 
                position: Transform::from_xyz(0.0, 15.0, 0.0), 
                ..Default::default()
        }
    )
    ;
}

fn turn_on_editor(
    mut commands: Commands,
) {
    commands.insert_resource(NextState(Some(RobotEditorState::Active)));                

}

fn setup_camera(
    mut commands: Commands,
) {

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
