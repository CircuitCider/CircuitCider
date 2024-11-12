use bevy::{prelude::Query, window::PrimaryWindow};
use bevy_egui::EguiContext;
use bevy::prelude::*;

use super::components::AssemblingTarget;


pub fn assembler_ui(
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut assembling_target: Query<(Entity, &AssemblingTarget)>,
    //mut commands: Commands,
) {
    let Ok(mut context) = primary_window.get_single_mut() else {return;};
    // let Some(target) = assembling_target.target else {return;};

    
    egui::Window::new(format!("Assembling Targets"))
    .show(context.get_mut(), |ui| {
        ui.label(format!("Assembling: {:#?}", assembling_target.iter().collect::<Vec<_>>()))
    });
    // for mut context in primary_window.iter_mut() {
    //     let ui_name = "Save Load Model";
    //     egui::Window::new(ui_name)
    //         .anchor(Align2::RIGHT_TOP, [0.0, 0.0])
    //         .collapsible(false)
    //         .resizable(false)
    //         .show(context.get_mut(), |ui| {
    //             ui.label("save conditions");

    //             ui.horizontal(|ui| {
    //                 ui.button("save");
    //             });
    //         });
    // }
}
