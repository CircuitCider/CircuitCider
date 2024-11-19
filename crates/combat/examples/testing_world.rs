//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{prelude::*, scene};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use combat::{components::{Health, Pistol}, ui::health_ui, weapons::plugins::CollisionPlugin, weapon_attacks::plugins::BulletPlugin, despawn::DespawnPlugin, asset_loader::{AssetLoaderPlugin, SceneAssets}};
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(CollisionPlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, health_ui)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    scene_assets: Res<SceneAssets>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    commands.spawn((
        SceneBundle {
            scene: scene_assets.pistol.clone(),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Health::default(),
        Pistol,
        Name::new("Player"),
    ));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
