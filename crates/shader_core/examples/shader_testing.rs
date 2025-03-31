//! A simple 3D scene with light shining over a cube sitting on a plane.

use app_core::plugins::AppSourcesPlugin;

use bevy::prelude::*;
use bevy_camera_extras::{CameraController, CameraExtrasPlugin, CameraMode, CameraRestrained};
use bevy_pbr::MeshLayouts;
use bevy_render::renderer::RenderDevice;
// use bevy_ui_extras::UiExtrasDebug;
use shader_core::{
    plugins::{ShaderCorePlugin, ShaderDebugPlugin},
    shaders::{
        flow_wireframe::FlowWireframeMaterial, glow::GlowMaterial, grid::GridMaterial,
        neon::NeonMaterial,
    },
};

fn main() {
    App::new()
        .add_plugins(AppSourcesPlugin::CRATE)
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Tell the asset server to watch for asset changes on disk:
            watch_for_changes_override: Some(true),
            ..default()
        }))
        .add_plugins(ShaderCorePlugin)
        .add_plugins(CameraExtrasPlugin {
            cursor_grabbed_by_default: false,
            keybinds_override: None,
            movement_settings_override: None,
        })
        // .add_plugins(UiExtrasDebug::default())
        //.add_plugins(ShaderDebugPlugin)
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
    mut glow_materials: ResMut<Assets<GlowMaterial>>,
    mut flowing_wireframe_mat: ResMut<Assets<FlowWireframeMaterial>>,
    mut grid_mat: ResMut<Assets<GridMaterial>>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(glow_materials.add(GlowMaterial { heat: 0.0 })),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Name::new("Cube"),
    ));
    // commands.spawn((
    //     MaterialMeshBundle {
    //         mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
    //         material: glow_materials.add(GlowMaterial {heat: 0.0}),
    //         transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //         ..default()
    //     },
    //     Name::new("Cube"),
    //     // flowing_wireframe_mat.add(FlowWireframeMaterial {
    //     //     color: LinearRgba::WHITE
    //     // })
    //     grid_mat.add(GridMaterial {})
    //     // FlowWireframeMaterial {
    //     //     color: LinearRgba::WHITE
    //     // },
    //     // Wireframe,
    //     // // This lets you configure the wireframe color of this entity.
    //     // // If not set, this will use the color in `WireframeConfig`
    //     // WireframeColor { color: LIME.into() },
    // ));
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
        CameraController {
            restrained: CameraRestrained(false),
            camera_mode: CameraMode::Free,
        },
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
