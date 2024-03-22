use app_core::{plugins::AppSourcesPlugin, ExecLocation, ROOT};
use bevy::{
    asset::io::{file::FileAssetReader, AssetSource},
    prelude::*,
};
use bevy_camera_extras::plugins::DefaultCameraPlugin;
use bevy_component_extras::components::{Followed, Watched};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::{
    focus::PickingInteraction, picking_core::Pickable, DefaultPickingPlugins, PickableBundle,
};
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use bevy_serialization_extras::prelude::{
    link::{JointFlag, LinkFlag, StructureFlag},
    rigidbodies::RigidBodyFlag,
    AssetSpawnRequest, AssetSpawnRequestQueue, PhysicsBundle, PhysicsSerializationPlugin,
    SerializationPlugin,
};
use bevy_serialization_urdf::{
    loaders::urdf_loader::Urdf,
    plugin::{AssetSourcesUrdfPlugin, UrdfSerializationPlugin},
};
use bevy_transform_gizmo::TransformGizmoPlugin;
use bevy_ui_extras::systems::visualize_right_sidepanel_for;
use robot_editor::{plugins::RobotEditorPlugin, states::RobotEditorState};

pub fn main() {
    App::new()
        // app sources
        .add_plugins(AppSourcesPlugin {
            exec_location: ExecLocation::CRATE
        })
        .add_plugins(AssetSourcesUrdfPlugin {
            assets_folder_local_path: "../../assets".into(),
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(RobotEditorPlugin)
        // camera
        .add_plugins(DefaultCameraPlugin)
        // Picking/selecting
        // .add_plugins((
        //     DefaultPickingPlugins,
        //     TransformGizmoPlugin::new(Quat::from_rotation_y(-0.2)),
        // ))
        // serialization plugins
        .add_plugins(SerializationPlugin)
        .add_plugins(PhysicsSerializationPlugin)
        .add_plugins(UrdfSerializationPlugin)
        // physics
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(PostStartup, turn_on_editor)
        .run();
}

fn turn_on_editor(mut commands: Commands) {
    commands.insert_resource(NextState(Some(RobotEditorState::Active)));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut urdf_load_requests: ResMut<AssetSpawnRequestQueue<Urdf>>,
) {
    // robot
    urdf_load_requests.requests.push_front(AssetSpawnRequest {
        source: format!("{:#}://model_pkg/urdf/diff_bot.xml", ROOT)
            .to_owned()
            .into(),
        position: Transform::from_xyz(0.0, 15.0, 0.0),
        ..Default::default()
    });

    // plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(
                Plane3d::new(Vec3::new(0.0, 1.0, 0.0))
                    .mesh()
                    .size(50.0, 50.0),
            ),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            ..default()
        },
        PhysicsBundle::default(),
    ));

    // light
    commands.spawn((PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    },));
    // camera
    commands.spawn((
        // Camera3dBundle {
        // transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        // ..default()
        // },
        Camera3dBundle {
            transform: Transform::from_xyz(2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        //bevy_transform_gizmo::GizmoPickSource::default(),
    ));
}
