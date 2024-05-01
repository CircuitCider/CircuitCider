//! A simple 3D scene with light shining over a cube sitting on a plane.

use std::fmt::Display;

use bevy::{prelude::*, render::{mesh::{self, VertexAttributeValues}, render_resource::VertexAttribute}, window::PrimaryWindow};
use bevy_inspector_egui::{bevy_egui::EguiContext, egui::{self, Ui}, quick::WorldInspectorPlugin};
use egui_extras::{Column, TableBody, TableBuilder};
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
pub fn mesh_attr_formatted(
    attr_check: Option<&VertexAttributeValues>,
) -> &Vec<[f32; 3]>{
    match attr_check {
        Some(attr_type) => {
            match attr_type {
                VertexAttributeValues::Float32x3(attr_vec) => {
                    &attr_vec
                },
                //FIXME! uv uses Float32x2. Adjust code to account for this.
                _ => todo!("position, (oops this is Float32x2)uv, and normal all use float32x3, how did this get here?")
            }
            
            //return format!("{:#?}", attr_type)
        }
        None => {
            todo!("implement this properly later")
            //return Vec::<[f32; 3]>::new();
        }
    }
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
                
                //let table_attrs = mesh_attr_table.get_table(ui);
                

                let collum_count = 4;//table_attrs.iter().len();

                let table = TableBuilder::new(ui)
                .columns(Column::auto()
                    .resizable(true)
                    .clip(false)
                    .at_least(150.0)
                    , 
                    collum_count
                )
                .min_scrolled_height(0.0)
                .auto_shrink(true)
                .resizable(true)
                ;
                // //println!("collum count: {:#?}", collum_count);
                table
                .header(20.0, |mut header| {
                    mesh_attr_table.layout_headers(&mut header)
                })
                .body(|mut body| {
                    body.row(20.0, |mut row| {
                        for attr_type in MeshAttributes::iter() {
                             if mesh_attr_table.contains_key(&attr_type.to_string()) {
                                row.col(|ui| {
                                    match attr_type {
                                        MeshAttributes::POSITION => {
                                            let attr_vec = mesh_attr_formatted(mesh.attribute(Mesh::ATTRIBUTE_POSITION));

                                            for attr in attr_vec.iter() {
                                                //let attr_str = attr.iter().map(|mut n| &mut n);
                                                let attr_str = attr.iter().map(|n| n.to_string()).collect::<Vec<String>>();
                                                ui.horizontal(|ui| {
                                                    for mut str in attr_str {
                                                        ui.text_edit_singleline(&mut str);
    
                                                    }
                                                });   

                                            }     
              
                                        },
                                        MeshAttributes::INDICE => {
                                            ui.label(
                                                format!(
                                                    "INDICES: {:#?}", 
                                                    mesh.indices()
                                                )
                                            );            
                                        },
                                        MeshAttributes::NORMAL => {
                                            let attr_vec = mesh_attr_formatted(mesh.attribute(Mesh::ATTRIBUTE_NORMAL));

                                            for attr in attr_vec.iter() {
                                                //let attr_str = attr.iter().map(|mut n| &mut n);
                                                let attr_str = attr.iter().map(|n| n.to_string()).collect::<Vec<String>>();
                                                ui.horizontal(|ui| {
                                                    for mut str in attr_str {
                                                        ui.text_edit_singleline(&mut str);
    
                                                    }
                                                });   

                                            }             
                                        },
                                        MeshAttributes::UV => {
                                            let attr_vec = mesh_attr_formatted(mesh.attribute(Mesh::ATTRIBUTE_UV_0));

                                            for attr in attr_vec.iter() {
                                                //let attr_str = attr.iter().map(|mut n| &mut n);
                                                let attr_str = attr.iter().map(|n| n.to_string()).collect::<Vec<String>>();
                                                ui.horizontal(|ui| {
                                                    for mut str in attr_str {
                                                        ui.text_edit_singleline(&mut str);
    
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