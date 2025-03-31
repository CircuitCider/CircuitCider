use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::EguiContext;

use crate::ui::window_follow_mouse;

use super::components::*;

/// ui for editing functionality of placed part
pub fn placer_editor_ui(
    placers: Query<(&Placer, &Name)>,
    mut primary_window: Query<(&Window, &mut EguiContext), With<PrimaryWindow>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if placers.iter().len() <= 0 {
        return;
    }

    for (win, mut context) in primary_window.iter_mut() {
        let ui_name = "Model features";

        let fix_window_not_pressed = !keys.pressed(KeyCode::ControlLeft);

        let Some(window) = window_follow_mouse(win, fix_window_not_pressed, ui_name) else {
            return;
        };

        window.show(context.get_mut(), |ui| {
            for (placer, name) in placers.iter() {
                ui.label(format!("name: {:#}", name.to_string()));
                ui.label(format!("Placer type: {:#?}", placer.to_string()));
            }
        });
    }
}
