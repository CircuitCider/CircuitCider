use std::any::TypeId;

use bevy::{asset::LoadedFolder, prelude::*, window::PrimaryWindow};
use bevy_egui::EguiContext;
use bevy_rapier3d::geometry::Sensor;
use bevy_rapier3d::plugin::RapierContext;
use bevy_serialization_extras::prelude::colliders::ColliderFlag;
use egui::Color32;
use egui::RichText;

use crate::model_display::components::DisplayModel;
use crate::model_display::systems::display_model;
use crate::resources::BuildToolMode;
use crate::shaders::neon_glow::NeonGlowMaterial;
use crate::ui::window_follow_mouse;

use super::components::*;
use super::resources::*;



/// ui for editing functionality of placed part
pub fn placer_editor_ui(
    placers: Query<(&Placer, &Name)>,
    mut primary_window: Query<(&Window, &mut EguiContext), With<PrimaryWindow>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if placers.iter().len() <= 0 {return;}

    for (win, mut context) in primary_window.iter_mut() {
        let ui_name = "Model features";

        let fix_window_not_pressed = !keys.pressed(KeyCode::ControlLeft);
        
        let Some(window) = window_follow_mouse(win, fix_window_not_pressed, ui_name) else {return};

        window
        .show(context.get_mut(), |ui| {
            for (placer, name) in placers.iter() {
                ui.label(format!("name: {:#}", name.to_string()));
                ui.label(format!("Placer type: {:#?}", placer.to_string()));
            }
        });
    }
}





/// list all placeable models
pub fn placer_spawner_ui(
    folders: Res<Assets<LoadedFolder>>,
    model_folder: Res<ModelFolder>,
    mut tool_mode: ResMut<BuildToolMode>,
    mut placer_materials: ResMut<Assets<NeonGlowMaterial>>,
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    display_models: Query<(Entity, &Handle<Mesh>), With<DisplayModel>>,

    mut commands: Commands,
) {
    //if tool_mode.into_inner() == &BuildToolMode::PlacerMode {

    let typeid = TypeId::of::<Mesh>();
    //println!("PREPARING TO ADD STUFF TO PLACE MODE UI");
    //info!("PRIMARY WINDOW COUNT: {:#?}", primary_window.iter().len());
    let mut model_hovered = false;

    for mut context in primary_window.iter_mut() {
        let ui_name = "prefab meshes";
        egui::SidePanel::left(ui_name).show(context.get_mut(), |ui| {
            ui.heading(ui_name);
            if let Some(folder) = folders.get(&model_folder.0) {
                let handles: Vec<Handle<Mesh>> = folder
                    .handles
                    .clone()
                    .into_iter()
                    .filter(|handle| handle.type_id() == typeid)
                    .map(|handle| handle.typed::<Mesh>())
                    .collect::<Vec<_>>();

                for mesh_handle in handles {
                    //let mesh = meshes.get(mesh_handle.clone()).expect("not loaded");
                    if let Some(path) = mesh_handle.path() {
                        let str_path = path.path().to_str().unwrap();

                        let model_name = str_path.split('/').last().unwrap_or_default().to_owned();
                        let spawn_button = ui.button(model_name.clone());

 
                        if spawn_button.clicked() {
                            //TODO! put raycasting code here
                            commands.spawn((
                                MaterialMeshBundle {
                                    mesh: mesh_handle.clone(),
                                    material: placer_materials.add(NeonGlowMaterial {
                                        color: Color::RED.into(),
                                    }),
                                    ..default()
                                },
                                Placer::from_path(str_path),
                                ColliderFlag::Convex,
                                Sensor,
                                Name::new(model_name.clone()),
                            ));
                            *tool_mode = BuildToolMode::PlacerMode
                        }
                        //spawn display model for hovered over spawnables

                        if spawn_button.hovered() {
                            model_hovered = true;
                            ui.label("show display model here!");
                            for (e, display_handle) in display_models.iter() {
                                if mesh_handle.path() != display_handle.path() {
                                    commands.entity(e).despawn()
                                }
                            }
                            if display_models.iter().len() < 1 {
                                display_model(&mut commands, &mut placer_materials, mesh_handle)
                            }
                        } 
                    }

                }
                if model_hovered == false {
                    for (e, ..) in display_models.iter() {
                        commands.entity(e).despawn()
                    }
                }
                //println!("model hover status: {:#?}", model_hovered);

            } else {
                ui.label("could not load folder...");
            }
        });
    }
    //}
}
