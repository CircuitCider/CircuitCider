//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{
    pbr::MeshLayouts,
    prelude::*,
    render::{
        renderer::RenderDevice,
    },
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Tell the asset server to watch for asset changes on disk:
            watch_for_changes_override: Some(true),
            ..default()
        }))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        //.add_systems(Startup, display_mesh_bindgroup_info)
        .run();
}
/// print relevant information about mesh_bindgroups
pub fn display_mesh_bindgroup_info(render_device: Res<RenderDevice>) {
    println!("bind group layout for mesh:");

    println!("{:#?}", MeshLayouts::new(&*render_device).model_only)
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
        MaterialMeshBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            // material: fresnel_mats.add(
            //     NormalVisualizerMaterial {
            //         selection: Vec4::new(0.0, 0.0, 0.0, 1.0)
            //     }

            // ),
            material: materials.add(Color::LinearRgba(LinearRgba::BLUE)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Name::new("Cube"),
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
