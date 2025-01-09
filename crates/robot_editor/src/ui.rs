use crate::{
    components::Wheel,
    load_assets_in,
    model_display::{components::DisplayModel, systems::display_model},
    placing::components::Placer,
    prelude::{WeaponsFolder, WheelsFolder},
    raycast_utils::resources::MouseOverWindow,
    resources::{BuildMenuTarget, BuildToolMode, HullsFolder},
};
use bevy::{asset::LoadedFolder, gltf::{GltfMesh, GltfPrimitive}, prelude::*, window::PrimaryWindow};
use bevy_egui::EguiContext;
use bevy_rapier3d::prelude::Sensor;
use bevy_serialization_extras::prelude::colliders::ColliderFlag;
use combat::components::Pistol;
use egui::{Align2, Color32, RichText, Sense};
use shader_core::shaders::neon::NeonMaterial;
use strum::IntoEnumIterator;

/// list all placeable models
pub fn build_menu_ui(
    folders: Res<Assets<LoadedFolder>>,
    hulls_folder: Res<HullsFolder>,
    weapons_folder: Res<WeaponsFolder>,
    wheels_folder: Res<WheelsFolder>,
    gltfs: Res<Assets<Gltf>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    meshes: Res<Assets<Mesh>>,
    mut tool_mode: ResMut<NextState<BuildToolMode>>,
    mut placer_materials: ResMut<Assets<NeonMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    display_models: Query<(Entity, &Mesh3d), With<DisplayModel>>,
    mut build_menu_target: ResMut<BuildMenuTarget>,
    mut commands: Commands,
) {
    let mut model_hovered = false;

    let Ok(mut context) = primary_window
        .get_single_mut()
        .inspect_err(|err| println!("issue spawning build menu: {:#}", err))
    else {
        return;
    };

    let ui_name = "build menu";
    egui::SidePanel::left(ui_name).show(context.get_mut(), |ui| {
        ui.heading(ui_name);
        ui.horizontal(|ui| {
            for item in BuildMenuTarget::iter() {
                let color = match *build_menu_target == item {
                    true => Color32::WHITE,
                    false => Color32::GRAY,
                };
                if ui
                    .button(RichText::new(item.to_string()).color(color))
                    .clicked()
                {
                    *build_menu_target = item
                }
            }
        });
        let model_king = build_menu_target.clone();
        let Some(handles) = (match model_king {
            BuildMenuTarget::Hulls => load_assets_in::<Gltf>(&folders, &hulls_folder.0),
            BuildMenuTarget::Weapons => load_assets_in::<Gltf>(&folders, &weapons_folder.0),
            BuildMenuTarget::Wheels => load_assets_in::<Gltf>(&folders, &wheels_folder.0),
        }) else {
            ui.label("could not load folder..");
            return;
        };

        // let Some(handles) = load_assets_in::<Mesh>(&folders, &hulls_folder.0) else {
        //     ui.label("could not load folder...");
        //     return;
        // };

        for handle in handles {
            //let mesh = meshes.get(mesh_handle.clone()).expect("not loaded");
            if let Some(path) = handle.path() {
                
                let str_path = path.path().to_string_lossy();

                let Some(gltf) = gltfs.get(&handle) else {
                    ui.label("loading gltf");
                    continue
                };
                
                let model_name = str_path.split('/').last().unwrap_or_default().to_owned();
                

                let Some(handle) = gltf.meshes.first() else {
                    ui.label(RichText::new(
                        format!("{:#} [INVALID]: Contains no mesh]", model_name)
                    ).color(Color32::RED));
                    continue
                };

                let Some(gltf_mesh) = gltf_meshes.get(handle) else {
                    ui.label("loading gltf mesh");
                    continue
                };
                let mesh_handle = {
                    if gltf_mesh.primitives.len() > 1 {
                        ui.label(RichText::new(
                            format!("{:#} [UNIMPLEMENTED]: multi-primitive .gltfs unimplemented]", model_name)
                        ).color(Color32::RED));
                        continue
                    }
                    let Some(primitive) = gltf_mesh.primitives.first() else {
                        ui.label(RichText::new(
                            format!("{:#} [INVALID]: Contains no primitive", model_name)
                        ).color(Color32::RED));
                        continue
                    };
                    primitive.mesh.clone()
                };

                let spawn_button = ui
                    .button(model_name.clone())
                    .interact(Sense::click_and_drag());

                if spawn_button.drag_started() {
                    println!("spawning model");
                    //TODO! put raycasting code here
                    let mut model = commands.spawn((
                        Mesh3d(mesh_handle.clone()),
                        MeshMaterial3d(materials.add(Color::WHITE)),
                        ColliderFlag::Convex,
                        Sensor,
                        RayCastPickable::default(),
                        //GizmoTarget::default(),
                        Name::new(model_name),
                        Placer::from_path(&str_path),
                        // MaterialMeshBundle {
                        //     mesh: mesh_handle.clone(),
                        //     material: placer_materials.add(NeonMaterial {
                        //         color: Color::Srgba(Srgba::RED).into(),
                        //     }),
                        //     ..default()
                        // },
                        // Placer::from_path(str_path),
                        // ColliderFlag::Convex,
                        // Sensor,
                        // Name::new(model_name.clone()),
                    ));
                    match model_king {
                        BuildMenuTarget::Hulls => {}
                        BuildMenuTarget::Weapons => {
                            model.insert(Pistol);
                        }
                        //TODO: Wheel should NOT have "left-right" quality. This should be user defined/face defined/relativistic to other wheels.
                        BuildMenuTarget::Wheels => {
                            model.insert(Wheel::Right);
                        }
                    }
                    tool_mode.set(BuildToolMode::PlacerMode)
                }
                //spawn display model for hovered over spawnables

                if spawn_button.contains_pointer() {
                    model_hovered = true;
                    ui.label("show display model here!");
                    for (e, display_handle) in display_models.iter() {
                        if mesh_handle.path() != display_handle.0.path() {
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

// #[derive(Component, Default)]
// pub struct Edited;

// /// editor mode for editing attached
// pub fn editor_mode_ui

pub fn save_load_model_ui(
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    //mut commands: Commands,
) {
    let Ok(context) = primary_window.get_single() else {
        return;
    };
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
