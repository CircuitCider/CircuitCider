//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{
    pbr::MeshLayouts,
    prelude::*,
    render::{
        render_resource::ShaderRef, renderer::RenderDevice
    },
};
use bevy_camera_extras::{CameraController, CameraExtrasPlugin, CameraRestrained};
use bevy_ui_extras::UiExtrasDebug;
use shader_core::{plugins::ShaderDebugPlugin, shaders::{neon::NeonMaterial, plugins::CustomShadersPlugin}};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Tell the asset server to watch for asset changes on disk:
            watch_for_changes_override: Some(true),
            ..default()
        }))
        .add_plugins(CameraExtrasPlugin {
            cursor_grabbed_by_default: false,
            keybinds_override: None,
            movement_settings_override: None
        })
        .add_plugins(UiExtrasDebug::default())
        .add_plugins(ShaderDebugPlugin)
        .add_plugins(CustomShadersPlugin)
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
    mut neon_glow_materials: ResMut<Assets<NeonMaterial>>,
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
            material: neon_glow_materials.add(NeonMaterial {
                color: LinearRgba::BLUE
            }),
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
    commands.spawn(
        (
        Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },
    CameraController {
        restrained: CameraRestrained(false),
        camera_mode: bevy_camera_extras::CameraMode::Free
    }
    )
);
}
