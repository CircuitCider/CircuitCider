//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;
use bevy::animation::{animate_targets, RepeatAnimation};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use combat::weapons::plugins::MacePlugin;
use combat::{components::{Health, Weapon, Animations}, ui::health_ui};
use std::time::Duration;
use bevy_serialization_assemble::{components::DisassembleAssetRequest, gltf::GltfPhysicsModel};

fn main() {
    App::new()
        // .add_plugins(AppSourcesPlugin::CRATE)
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(MacePlugin)
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    asset_server: Res<AssetServer>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    //mace
    commands.spawn((
        DisassembleAssetRequest::<GltfPhysicsModel>::path(
            "root://models/weapons/mace.glb#Node0".to_string(),
            None,
        ),
        Transform::from_xyz(1.0, 10.0, 1.0),
        Weapon
    ));
    //animation graph
    let mut graph = AnimationGraph::new();
    let animations = graph
        .add_clips(
            [
                GltfAssetLabel::Animation(0).from_asset("mace.glb#Scene0"),
            ]
            .into_iter()
            .map(|path| asset_server.load(path)),
            1.0,
            graph.root,
        )
        .collect();
    //mace animation
    let graph = graphs.add(graph);
    commands.insert_resource(Animations {
        animations,
        graph: graph.clone(),
    });
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}