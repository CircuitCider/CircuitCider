//! A simple 3D scene with light shining over a cube sitting on a plane.

use app_core::plugins::AppSourcesPlugin;
use bevy::prelude::*;
use bevy_obj::ObjPlugin;
use bevy_ui_extras::{states::DebugMenuState, UiExtrasDebug};
use combat::{components::{Health, Pistol}, ui::health_ui, weapons::plugins::CollisionPlugin, weapon_attacks::plugins::BulletPlugin, despawn::DespawnPlugin};
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

fn main() {
    App::new()
    .add_plugins(AppSourcesPlugin::CRATE)
    .add_plugins(DefaultPlugins)
    .add_plugins(ObjPlugin)
    //.add_plugins(WorldInspectorPlugin::default())
    .add_plugins(UiExtrasDebug {
        menu_mode: DebugMenuState::Open,
        ..default()
    })
    .add_plugins(CollisionPlugin)
    .add_plugins(BulletPlugin)
    .add_plugins(DespawnPlugin)
    // .add_plugins(AssetLoaderPlugin)
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugins(RapierDebugRenderPlugin::default())
    .add_systems(Startup, setup)
    .add_systems(Update, health_ui)
    .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // scene_assets: Res<SceneAssets>,
    asset_server: Res<AssetServer>,
) {
    let mesh: Handle<Mesh> = asset_server.load("root://models/weapons/pistol.obj");

    // circular base
    commands.spawn(
        (
            Mesh3d(meshes.add(Circle::new(4.0))),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        )
    );
    // cube
    commands.spawn((
        Mesh3d(mesh),
        Transform::from_xyz(0.0, 0.5, 0.0),
        MeshMaterial3d(materials.add(Color::WHITE)),
        // SceneBundle {
        //     scene: scene_assets.pistol.clone(),
        //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
        //     ..default()
        // },
        Health::default(),
        Pistol,
        Name::new("Player"),
    ));
    // light
    commands.spawn(
        (
            PointLight {
                shadows_enabled: true,
                ..default()
            },
            Transform::from_xyz(4.0, 8.0, 4.0),
        )
    );
    // camera
    commands.spawn(
        (
            Camera3d::default(),
            Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        )
    );
}
