use std::f32::consts::PI;

use crate::{
    components::{BuildWidgetTarget, Wheel}, load_assets_in, model_display::{DisplayModel, DisplayOption}, picking::components::{PickCollector, PickSelected}, placing::components::Placer, prelude::{WeaponsFolder, WheelsFolder}, resources::{BuildMenuTarget, BuildToolMode, BuildWidgetMode, HullsFolder}
};
use bevy::{asset::LoadedFolder, gltf::{GltfMesh, GltfNode, GltfPrimitive}, math::Affine3A, prelude::*, window::PrimaryWindow};
use bevy_egui::EguiContext;
use bevy_mod_outline::{bundles::InheritOutlineBundle, AsyncSceneInheritOutline, ComputedOutline, InheritOutline, OutlineVolume};
use bevy_rapier3d::prelude::Sensor;
use bevy_serialization_assemble::{components::{DisassembleAssetRequest, Maybe, RollDown}, gltf::{gltf_collider_request, GltfNodeColliderVisualChilds, GltfNodeMeshOne, GltfNodeVisuals, GltfPhysicsMeshPrimitive, GltfPhysicsModel}, traits::{Disassemble, Split, Structure}};
use bevy_serialization_extras::prelude::{colliders::ColliderFlag, RequestCollider, RequestColliderFromChildren, RigidBodyFlag};
use combat::components::Pistol;
use derive_more::From;
use egui::{Align2, Color32, RichText, Sense};
use shader_core::shaders::neon::NeonMaterial;
use strum::IntoEnumIterator;

#[derive(Component)]
pub struct ModifyTransformGltf;

/// list all placeable models
pub fn build_menu_ui(
    folders: Res<Assets<LoadedFolder>>,
    hulls_folder: Res<HullsFolder>,
    weapons_folder: Res<WeaponsFolder>,
    wheels_folder: Res<WheelsFolder>,
    mut display_model: ResMut<DisplayModel>,
    mut tool_mode: ResMut<NextState<BuildToolMode>>,
    mut placer_materials: ResMut<Assets<NeonMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    //display_models: Query<(Entity, &Mesh3d), With<DisplayModel>>,
    mut build_menu_target: ResMut<BuildMenuTarget>,
    mut build_widget_mode: ResMut<NextState<BuildWidgetMode>>,
    mut commands: Commands,
) {
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

        for handle in handles {
            //let mesh = meshes.get(mesh_handle.clone()).expect("not loaded");
            if let Some(path) = handle.path() {
                
                let str_path = path.path().to_string_lossy();
                
                let model_name = str_path.split('/').last().unwrap_or_default().to_owned();


                let spawn_button = ui
                    .button(model_name.clone())
                    .interact(Sense::click_and_drag());

                let model_path = path.to_string() + "#Node0";
                if spawn_button.drag_started() {

                    println!("spawning model");
                    let model = commands.spawn(
                        (
                            DisassembleAssetRequest::<GltfPhysicsModel>::path(model_path.clone(), None),
                            Sensor,
                            Transform::default(),
                            RigidBodyFlag::Fixed,
                            Name::new("pistol"),
                            Placer::from_path(&str_path),
                            RollDown(PickingBehavior {
                                should_block_lower: false,
                                is_hoverable: true
                            }, vec![]),
                            OutlineVolume {
                                visible: true,
                                width: 3.0,
                                colour: Color::srgb(1.0, 0.0, 0.0),
                            },
                            RollDown(InheritOutline, vec![]),
                            BuildWidgetTarget,
                            PickSelected(true),
                            RollDown(PickCollector, vec![]),
                            //RollDown(ComputedOutline, vec![]),

                            //AsyncSceneInheritOutline::default()
                        )
                    ).id();
                    match model_king {
                        BuildMenuTarget::Hulls => {}
                        BuildMenuTarget::Weapons => {
                            commands.entity(model).insert(Pistol);
                        }
                        //TODO: Wheel should NOT have "left-right" quality. This should be user defined/face defined/relativistic to other wheels.
                        BuildMenuTarget::Wheels => {
                            commands.entity(model).insert(Wheel::Right);
                        }
                    }
                    tool_mode.set(BuildToolMode::PlacerMode);
                    build_widget_mode.set(BuildWidgetMode::Pointer);
                }
                //spawn display model for hovered over spawnables
                let mut new_display_model = None;
                if spawn_button.contains_pointer() {
                    

                    new_display_model = Some(DisplayOption::Path(model_path.clone()))
                } 
                if display_model.0 != new_display_model {
                    display_model.0 = new_display_model
                }
            }
        }
        // if model_hovered == false {
        //     for (e, ..) in display_models.iter() {
        //         commands.entity(e).despawn()
        //     }
        // }
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
    // mut mouse_over_window: ResMut<MouseOverWindow>,
) {
    // for mut window in windows.iter_mut() {
    //     if window.get_mut().is_pointer_over_area() {
    //         //println!("mouse is over window");
    //         **mouse_over_window = true
    //     } else {
    //         **mouse_over_window = false
    //     }
    // }
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

pub fn make_window_not_block_picking(
    windows: Query<(Entity, &Window), Without<PickingBehavior>>,
    mut commands: Commands
) {
    for (e, window) in windows.iter() {
        commands.entity(e).insert(PickingBehavior {
            should_block_lower: false,
            is_hoverable: true,
        });
    }
}
// #[derive(Resource, Deref, Default)]
// pub struct DisplayModelImage(pub Handle<Image>);
