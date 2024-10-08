use bevy::render::camera;
use bevy::window::PrimaryWindow;
use bevy::{prelude::*};
use bevy_mod_raycast::cursor::CursorRay;
use bevy_mod_raycast::prelude::ray_from_screenspace;

use super::resources::*;
use super::systems::*;
/// adds and updates [`CursorRayHits`], a shorthand for listing things that were clicked on.
pub struct CursorRayHitsPlugin {
    pub debug_mode: bool
}

impl Default for CursorRayHitsPlugin {
    fn default() -> Self {
        Self {
            debug_mode: false
        }
    }
}


impl Plugin for CursorRayHitsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(CustomCursorRayPlugin)
        .init_resource::<MouseOverWindow>()
        .init_resource::<CursorRayHits>()
        .register_type::<CursorRayHits>()
        .insert_resource(RayCastDebugMode(self.debug_mode))
        .register_type::<RayCastDebugMode>()
        .add_systems(PreUpdate, update_cursor_ray_hits);
    }
}

/// cam used for [`CursorRay`]
#[derive(Component)]
pub struct CursorRayCam;

/// Automatically generates a ray in world space corresponding to the mouse cursor, and stores it in
/// [`CursorRay`].
/// modified to older use cams marked iwth `[CursorRayCam]` 
#[derive(Default)]
pub struct CustomCursorRayPlugin;
impl Plugin for CustomCursorRayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(First, update_cursor_ray)
            .add_systems(
                PostUpdate,
                update_cursor_ray.after(TransformSystem::TransformPropagate),
            )
            .init_resource::<CursorRay>();
    }
}

/// Updates the [`CursorRay`] every frame.
pub fn update_cursor_ray(
    primary_window: Query<Entity, With<PrimaryWindow>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform), With<CursorRayCam>>,
    mut cursor_ray: ResMut<CursorRay>,
) {
    cursor_ray.0 = cameras
        .iter()
        .filter_map(|(camera, transform)| {
            if let camera::RenderTarget::Window(window_ref) = camera.target {
                Some(((camera, transform), window_ref))
            } else {
                None
            }
        })
        .filter_map(|(cam, window_ref)| {
            window_ref
                .normalize(primary_window.get_single().ok())
                .map(|window_ref| (cam, window_ref.entity()))
        })
        .filter_map(|(cam, window_entity)| windows.get(window_entity).ok().map(|w| (cam, w)))
        .filter_map(|(cam, window)| window.cursor_position().map(|pos| (cam, window, pos)))
        .filter_map(|((camera, transform), window, cursor)| {
            ray_from_screenspace(cursor, camera, transform, window)
        })
        .next();
}