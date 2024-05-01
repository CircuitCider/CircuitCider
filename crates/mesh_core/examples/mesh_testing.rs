//! A simple 3D scene with light shining over a cube sitting on a plane.

use std::fmt::Display;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::{bevy_egui::EguiContext, egui, quick::WorldInspectorPlugin};
use mesh_core::{arrow::Arrow3D, cone::Cone, ui::{MeshAttributes, TablePick}};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

fn main() {
    App::new()
        .init_resource::<TablePick<MeshAttributes>>()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::default())
        
        // demo mesh
        .add_systems(Startup, spawn_mesh_for::<Arrow3D>)
        .add_systems(Update, display_mesh_info)
        .add_systems(Startup, setup)
        .run();
}
#[derive(Component)]
pub struct MeshInfoTarget;

fn spawn_mesh_for<T: Into<Mesh> + Default>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(
        (
        PbrBundle {
            mesh: meshes.add(T::default()),
            material: materials.add(Color::rgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        MeshInfoTarget
    ));
}

/// gives a ui window displaying mesh 
pub fn display_mesh_info(
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut mesh_attr_table: ResMut<TablePick<MeshAttributes>>,
    target_meshes: Query<&Handle<Mesh>, With<MeshInfoTarget>>,
    meshes: ResMut<Assets<Mesh>>
) {
    for mut context in primary_window.iter_mut() {
        let ui_name = "Mesh Info";
        egui::Window::new(ui_name)
        //.collapsible(false)
        .resizable(false)
        .show(context.get_mut(), |ui| {
            for mesh_check in target_meshes.iter() {
                let Some(mesh) = meshes.get(mesh_check) else {continue;};
                
                let attr = mesh_attr_table.table(ui);
                
                match attr {
                    MeshAttributes::POSITION => {
                        ui.label(
                            format!(
                                "positions: {:#?}", 
                                mesh.attribute(Mesh::ATTRIBUTE_POSITION)
                            )
                        );                        
                    },
                    MeshAttributes::INDICE => {
                        ui.label(
                            format!(
                                "IN: {:#?}", 
                                mesh.indices()
                            )
                        );            
                    },
                    _ => {
                        ui.label("unimplemented");
                    }
                }
            }
        });
    }
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