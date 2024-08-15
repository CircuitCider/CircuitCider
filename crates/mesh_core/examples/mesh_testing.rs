//! A simple 3D scene with light shining over a cube sitting on a plane.

use std::fmt::Display;

use bevy::{prelude::*, render::{mesh::{self, Indices, VertexAttributeValues}, render_resource::VertexAttribute}, window::PrimaryWindow};
use bevy_inspector_egui::{bevy_egui::EguiContext, egui::{self, Ui}, quick::WorldInspectorPlugin};
use bevy_ui_extras::*;
use egui_extras::{Column, TableBody, TableBuilder};
use mesh_core::{arrow::Arrow3D, cone::Cone, ui::MeshAttributes, MeshAttr};
use mesh_core::*;
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
    let x = Vec3::new(0.0, 0.0, 0.0);
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
// pub fn mesh_attr_formatted(
//     attr_check: Option<&mut VertexAttributeValues>,
// ) -> &mut Vec<[f32; 3]>{
//     match attr_check {
//         Some(attr_type) => {
//             match attr_type {
//                 VertexAttributeValues::Float32x3(mut attr_vec) => {
//                     &mut attr_vec
//                 },
//                 //FIXME! uv uses Float32x2. Adjust code to account for this.
//                 _ => todo!("position, (oops this is Float32x2)uv, and normal all use float32x3, how did this get here?")
//             }
            
//             //return format!("{:#?}", attr_type)
//         }
//         None => {
//             todo!("implement this properly later")
//             //return Vec::<[f32; 3]>::new();
//         }
//     }
// }
/// gives a ui window displaying mesh 
pub fn display_mesh_info(
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut mesh_attr_table: ResMut<TablePick<MeshAttributes>>,
    target_meshes: Query<&Handle<Mesh>, With<MeshInfoTarget>>,
    mut meshes: ResMut<Assets<Mesh>>
) {
    for mut context in primary_window.iter_mut() {
        let ui_name = "Mesh Info";
        egui::Window::new(ui_name)
        //.collapsible(false)
        .resizable(false)
        .show(context.get_mut(), |ui| {
            for mesh_check in target_meshes.iter() {
                let Some(mesh) = meshes.get_mut(mesh_check) else {continue;};
                
                //let table_attrs = mesh_attr_table.get_table(ui);
                
                let collum_count = MeshAttributes::iter().len();
                //let collum_count = 4;//table_attrs.iter().len();

                TableTemplate::new(ui, &mut *mesh_attr_table)
                .body(|mut body| {
                    body.row(20.0, |mut row| {
                        for attr_type in MeshAttributes::iter() {
                             if mesh_attr_table.contains_key(&attr_type.to_string()) {
                                row.col(|ui| {
                                    match attr_type {
                                        MeshAttributes::POSITION => {
                                            let mut pos_vertices = attr_to_vec(mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)).unwrap_or_default();                                            
                                                for vertex in pos_vertices.iter_mut() {
                                                    ui.horizontal(|ui| {
                                                        for n in vertex.iter_mut() {
                                                            ui.add(egui::DragValue::new(*n).speed(0.1));
                                                        }
                                                    });   
                                                }     
                                        },
                                        MeshAttributes::INDICE => {
                                            let Some(indicies_type) = mesh.indices() else {return;};
                                            let mut indicies = Vec::new();
                                            match indicies_type {
                                                Indices::U32(vec) => {
                                                    for n in vec {
                                                        indicies.push(*n)
                                                    }  
                                                },
                                                Indices::U16(vec) => {
                                                    for n in vec {
                                                        indicies.push(*n as u32)
                                                    }  
                                                    //&(vec.iter().map(|n| n.to_owned() as u32).collect::<Vec<_>>())
                                                }
                                            };
                                            let grouped = indicies.chunks_exact(3);
                                            for indice in grouped.into_iter() {
                                                ui.horizontal(|ui| {
                                                    for n in indice.iter() {
                                                        ui.label(n.to_string());
                                                    }
                                                });
                                            }
                                            ui.label(
                                                format!(
                                                    "INDICES: {:#?}", 
                                                    mesh.indices_mut()
                                                )
                                            );            
                                        },
                                        MeshAttributes::NORMAL => {
                                            let mut pos_vertices = attr_to_vec(mesh.attribute_mut(Mesh::ATTRIBUTE_NORMAL)).unwrap_or_default();                                            
                                                for vertex in pos_vertices.iter_mut() {
                                                    ui.horizontal(|ui| {
                                                        for n in vertex.iter_mut() {
                                                            ui.add(egui::DragValue::new(*n).speed(0.1));
                                                        }
                                                    });   
                                                }             
                                        },
                                        MeshAttributes::UV => {
                                            let mut pos_vertices = attr_to_vec(mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0)).unwrap_or_default();                                            
                                                for vertex in pos_vertices.iter_mut() {
                                                    ui.horizontal(|ui| {
                                                        for n in vertex.iter_mut() {
                                                            ui.add(egui::DragValue::new(*n).speed(0.1));
                                                        }
                                                    });   
                                                }              
                                        },
                                        _ => {
                                            ui.label("unimplemented");
                                        }
                                    }
                                });
                             } else {
                                row.col(|ui| {ui.label("");});
                             }
                        }
                    });
                });
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