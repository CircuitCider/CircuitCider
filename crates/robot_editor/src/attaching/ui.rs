use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::EguiContext;
use bevy_rapier3d::{plugin::RapierContext};
use bevy_serialization_extras::prelude::colliders::ColliderFlag;
use egui::{Color32, RichText};

use crate::ui::window_follow_mouse;

use super::components::AttachCandidate;

/// ui for editing attach candidates to fine tune and confirm their placement.
pub fn attach_candidate_edit_ui(
    rapier_context: Res<RapierContext>,
    mut primary_window: Query<(&Window, &mut EguiContext), With<PrimaryWindow>>,
    attach_candidates: Query<(Entity, &mut Transform, &ColliderFlag, &AttachCandidate)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    //don't render this ui if there is nothing its focusing on.
    if attach_candidates.iter().len() <= 0 {
        return;
    }

    let valid_button_color = Color32::GREEN;
    let invalid_button_color = Color32::RED;

    let mut placement_conditions = Vec::new();

    let mut no_intersections = false;

    // check attach canidates to confirm nothing is wrong with them before attaching
    for (e, ..) in attach_candidates.iter() {
        if rapier_context
            .intersection_pairs_with(e)
            .collect::<Vec<_>>()
            .len()
            <= 0
        {
            no_intersections = true;
        } else {
            no_intersections = false;
        }
    }
    placement_conditions.push(no_intersections);

    for (win, mut context) in primary_window.iter_mut() {
        let ui_name = "edit attachemnt";

        let fix_window_not_pressed = !keys.pressed(KeyCode::ControlLeft);

        let Some(window) = window_follow_mouse(win, fix_window_not_pressed, ui_name) else {
            return;
        };
        window.show(context.get_mut(), |ui| {
            if ui
                .button(RichText::new("Confirm").color(valid_button_color))
                .clicked()
            {
                println!("attaching candidate")
            }
            ui.horizontal(|ui| {
                ui.label(format!("translation: "));
                for (_, trans, ..) in attach_candidates.iter() {
                    ui.label(format!("{:#}", trans.translation));
                }
            });

            let mut color = invalid_button_color;
            if no_intersections {
                color = valid_button_color;
            }
            ui.label(RichText::new("no intersections?").color(color));
        });
    }
}
