
use crate::{
    load_assets_in, model_display::{components::DisplayModel, systems::display_model}, placing::components::Placer, raycast_utils::resources::MouseOverWindow, resources::{BuildMenuTarget, BuildToolMode, ModelFolder}
};
use bevy::{
    asset::LoadedFolder, prelude::*, window::PrimaryWindow
};
use bevy_egui::EguiContext;
use bevy_rapier3d::prelude::Sensor;
use bevy_serialization_extras::prelude::colliders::ColliderFlag;
use egui::{Align2, UiBuilder};
use shader_core::shaders::neon::NeonMaterial;
use strum::IntoEnumIterator;



/// list all placeable models
pub fn build_menu_ui(
    folders: Res<Assets<LoadedFolder>>,
    model_folder: Res<ModelFolder>,
    mut tool_mode: ResMut<NextState<BuildToolMode>>,
    mut placer_materials: ResMut<Assets<NeonMaterial>>,
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    display_models: Query<(Entity, &Handle<Mesh>), With<DisplayModel>>,
    build_menu_taget: ResMut<BuildMenuTarget>,
    mut commands: Commands,
) {
    let mut model_hovered = false;

    for mut context in primary_window.iter_mut() {
        let ui_name = "build menu";
        egui::SidePanel::left(ui_name).show(context.get_mut(), |ui| {
            ui.heading(ui_name);



            let Some(handles) = load_assets_in::<Mesh>(&folders, &model_folder.0) else {
                ui.label("could not load folder...");
                return;
            };
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
                                material: placer_materials.add(NeonMaterial {
                                    color: Color::Srgba(Srgba::RED).into(),
                                }),
                                ..default()
                            },
                            Placer::from_path(str_path),
                            ColliderFlag::Convex,
                            Sensor,
                            Name::new(model_name.clone()),
                        ));
                        tool_mode.set(BuildToolMode::PlacerMode)
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

            // } else {
            //     ui.label("could not load folder...");
            // }
        });
    }
    //}
}


// /// ui for build menu
// pub fn build_menu_ui(
//     mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
//     mut build_menu_target: ResMut<BuildMenuTarget>,
//     mut build_menu_ui_child: ResMut<BuildMenuUi>,
// ) {
//     let Ok(mut context) = primary_window.get_single_mut()
//     .inspect_err(|err| warn!("issue displaying build menu: {:#}", err))
//     else {return;};

//     let ui_name = "Build Menu";

//     egui::SidePanel::left(ui_name)
//     .show(context.get_mut(), |ui| {
//         ui.heading(ui_name);

//         ui.horizontal(|ui| {
//             if ui.button("Weapons").clicked() {
//                 *build_menu_target = BuildMenuTarget::Weapons
//             };
//             if ui.button("Hulls").clicked() {
//                 *build_menu_target = BuildMenuTarget::Hulls
//             };
//             if ui.button("Wheels").clicked() {
//                 *build_menu_target = BuildMenuTarget::Wheels
//             }
//         });

//         build_menu_ui_child.0 = Some(ui.new_child(UiBuilder::new()))
//     });


// }

/// creates a egui window that follows mouse when its given condition is satisfied.
///
/// if mouse is not in window, this will not create a iwndow
pub fn window_follow_mouse(
    window: &Window,
    condition: bool,
    ui_name: &str,
) -> Option<egui::Window<'static>> {
    let offset = 10.0;

    let cursor_pos = window.cursor_position()?;

    let window = egui::Window::new(ui_name);
    if condition {
        return Some(window.fixed_pos((cursor_pos.x + offset, cursor_pos.y + offset)));
    } else {
        return Some(window);
    }
}

pub fn select_build_tool(
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut tool_mode: ResMut<NextState<BuildToolMode>>,
) {
    for mut context in primary_window.iter_mut() {
        egui::Window::new("BuildToolMode debug").show(context.get_mut(), |ui| {
            ui.heading("select mode");
            ui.label(format!("Current mode: {:#?}", *tool_mode));
            for tool in BuildToolMode::iter() {
                if ui.button(tool.to_string()).clicked() {
                    tool_mode.set(tool);
                }
            }
        });
    }
}

/// Sets mouse over window resource to true/false depending on mouse state.
pub fn check_if_mouse_over_ui(
    mut windows: Query<&mut EguiContext>,
    mut mouse_over_window: ResMut<MouseOverWindow>,
) {
    for mut window in windows.iter_mut() {
        if window.get_mut().is_pointer_over_area() {
            //println!("mouse is over window");
            **mouse_over_window = true
        } else {
            **mouse_over_window = false
        }
    }
    //**mouse_over_window = false
}

#[derive(Component, Default)]
pub struct Edited;

// /// editor mode for editing attached
// pub fn editor_mode_ui

pub fn save_load_model_ui(
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    //mut commands: Commands,
) {
    let Ok(context) = primary_window.get_single() else {return;};
    for mut context in primary_window.iter_mut() {
        let ui_name = "Save Load Model";
        egui::Window::new(ui_name)
            .anchor(Align2::RIGHT_TOP, [0.0, 0.0])
            .collapsible(false)
            .resizable(false)
            .show(context.get_mut(), |ui| {
                ui.label("save conditions");

                ui.horizontal(|ui| {
                    ui.button("save");
                });
            });
    }
}

// #[derive(Resource, Deref, Default)]
// pub struct DisplayModelImage(pub Handle<Image>);
