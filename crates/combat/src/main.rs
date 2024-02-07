use bevy::{asset::io::{file::FileAssetReader, AssetSource}, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::{picking_core::Pickable, DefaultPickingPlugins, PickableBundle};
use bevy_rapier3d::{plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use bevy_serialization_extras::prelude::{link::{JointFlag, LinkFlag, StructureFlag}, rigidbodies::RigidBodyFlag, AssetSpawnRequest, AssetSpawnRequestQueue, PhysicsBundle, PhysicsSerializationPlugin, SerializationPlugin};
use bevy_serialization_urdf::{loaders::urdf_loader::Urdf, plugin::{AssetSourcesUrdfPlugin, UrdfSerializationPlugin}};
use bevy_transform_gizmo::TransformGizmoPlugin;
use app_core::{plugins::AppSourcesPlugin, ROOT};
use robot_editor::{plugins::RobotEditorPlugin, states::RobotEditorState, systems::WasFrozen};

pub fn main() {
    
    App::new()
    
    // app sources
    .add_plugins(AppSourcesPlugin)
    .add_plugins(AssetSourcesUrdfPlugin)

    .add_plugins(DefaultPlugins)
    .add_plugins(RobotEditorPlugin)  

    // serialization plugins
    .add_plugins(SerializationPlugin)
    .add_plugins(PhysicsSerializationPlugin)
    .add_plugins(UrdfSerializationPlugin)
    
    // Picking/selecting
    .add_plugins(
        (
            DefaultPickingPlugins,
            TransformGizmoPlugin::new(
                Quat::from_rotation_y(-0.2), 
            )
        )
    )

    // physics
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugins(RapierDebugRenderPlugin::default())


    .add_systems(Startup, setup)
    .add_systems(PostStartup, turn_on_editor)
    .add_systems(Update, make_robots_editable)
    .run()
    ;
}

pub fn freeze_spawned_robots(
    mut robots: Query<(Entity, &mut RigidBodyFlag), (With<StructureFlag>, Without<JointFlag>, Without<WasFrozen>)>,
    mut commands: Commands,
) {
    for (e, mut body) in robots.iter_mut() {
        *body = RigidBodyFlag::Fixed;
        commands.entity(e).insert(WasFrozen);
    }
}

pub fn make_robots_editable(
    unmodified_bots: Query<(Entity, &LinkFlag), Without<Pickable>>,
    mut commands: Commands,
) {
    for (e, ..) in unmodified_bots.iter() {
        commands.entity(e)
        .insert(PickableBundle::default())
        .insert(        bevy_transform_gizmo::GizmoTransformable)
        ;
    }
}

fn turn_on_editor(
    mut commands: Commands,
) {
    commands.insert_resource(NextState(Some(RobotEditorState::Active)));                

}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut urdf_load_requests: ResMut<AssetSpawnRequestQueue<Urdf>>,
) {

    // robot
    urdf_load_requests.requests.push_front(
        AssetSpawnRequest {
                source: format!("{:#}://model_pkg/urdf/diff_bot.xml", ROOT).to_owned().into(), 
                position: Transform::from_xyz(0.0, 15.0, 0.0), 
                ..Default::default()
        }
    )
    ;

    // plane
    commands.spawn(
    (
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(50.0).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            ..default()
        },
        PhysicsBundle::default()
        )
    );

    // light
    commands.spawn(
        (
        PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    },
    )
);
    // camera
    commands.spawn(
    (
        Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
        },
        bevy_transform_gizmo::GizmoPickSource::default()
    ));
}