use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::EguiContext;
use bevy_mod_raycast::{immediate::Raycast, CursorRay};

use super::resources::{CursorRayHits, MouseOverWindow};
use super::systems::*;
/// adds and updates [`CursorRayHits`], a shorthand for listing things that were clicked on.
pub struct CursorRayHitsPlugin;

impl Plugin for CursorRayHitsPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<CursorRayHits>()
        .add_systems(PreUpdate, update_cursor_ray_hits)
        ;
    }
}
///
pub fn update_cursor_ray_hits(
    cursor_ray: Res<CursorRay>,
    mut cursor_ray_hits: ResMut<CursorRayHits>,
    mut raycast: Raycast,
    mouse_over_window: Res<MouseOverWindow>,
) {
    let hit_iter = cursor_ray_hititer(&cursor_ray, &mut raycast, &mouse_over_window)
    .unwrap_or_default()
    .map(|n| n.clone())
    .collect::<Vec<_>>();

    **cursor_ray_hits = hit_iter

}