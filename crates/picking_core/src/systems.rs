use bevy_ecs::prelude::*;
use bevy_input::{mouse::MouseButton, ButtonInput};
use bevy_picking::pointer::PointerInteraction;
use bevy_hierarchy::prelude::*;

use crate::{components::{PickCollector, PickSelected}, get_top_pickable_entity};

pub fn pick_self_select_air_deselect(
    pointer: Single<&PointerInteraction>,
    pick_collectors: Query<(Entity, &Parent), With<PickCollector>>,
    mut interactables: Query<&mut PickSelected>,
    mouse: ResMut<ButtonInput<MouseButton>>,
) {
    let mouse_pressed = mouse.just_pressed(MouseButton::Left);
    let Some((e, hit)) = pointer.first() else {
        // remove selection if air is clicked
        if mouse_pressed {
            for mut selected in &mut interactables {
                selected.0 = false;
            }
         }
        return;
    };


    if mouse_pressed {
        // remove all preivous selection if hiting something that is in the world(not a window)
        // TODO: If multi-select is added at some point, this will need to be changed to check for that.
        if hit.position.is_some() {
            if mouse_pressed {
                for mut selected in &mut interactables {
                    selected.0 = false;
                }
             }
        }
        let Ok(mut picked) = interactables.get_mut(get_top_pickable_entity(*e, pick_collectors)) else {
            return
        };

        picked.0 = true;
    }
}

/// behaviour for what happens when stuff is clicked on
pub fn pick_self_select_deselect(
    pick_collectors: Query<(Entity, &Parent), With<PickCollector>>,
    mut interactables: Query<&mut PickSelected>,
    pointer: Single<&PointerInteraction>,
    mouse: ResMut<ButtonInput<MouseButton>>,
) {

    let Some((e, _hit)) = pointer.first() else {
        return;
    };
    let Ok(mut picked) = interactables.get_mut(get_top_pickable_entity(*e, pick_collectors)) else {
        return
    };

    if mouse.just_pressed(MouseButton::Left) {
        if picked.0 {
            picked.0 = false;
        } else {
            picked.0 = true;
        }
    }

}