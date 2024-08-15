use bevy::{prelude::*, window::PrimaryWindow};
use bevy_mod_raycast::prelude::*;

use super::resources::*;
use super::systems::*;
/// adds and updates [`CursorRayHits`], a shorthand for listing things that were clicked on.
pub struct CursorRayHitsPlugin;

impl Plugin for CursorRayHitsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseOverWindow>()
            .init_resource::<CursorRayHits>()
            .insert_resource(RayCastDebugMode(true))
            .add_systems(PreUpdate, update_cursor_ray_hits);
    }
}
