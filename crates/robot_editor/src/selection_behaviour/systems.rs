use bevy::prelude::*;
use bevy_mod_raycast::{immediate::Raycast, CursorRay};

use crate::raycast_utils::{resources::MouseOverWindow, systems::cursor_ray_hititer};

use super::components::Grabbed;

pub fn check_grabs(
    grabing: Query<Entity, With<Grabbed>>,
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if mouse.pressed(MouseButton::Left) == false {
        for e in grabing.iter() {
            commands.entity(e).remove::<Grabbed>();
        }
    }
}

pub fn grab_clicked(
    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,
    mouse_over_window: Res<MouseOverWindow>,
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Some(mut hits) = cursor_ray_hititer(&cursor_ray, &mut raycast, &mouse_over_window) {
            if let Some((e, ..)) = hits.next() {
                commands.entity(*e).insert(Grabbed);    
            }
        }
    }
}