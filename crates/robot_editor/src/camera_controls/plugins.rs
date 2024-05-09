use bevy::{prelude::*, transform::commands, window::PrimaryWindow};
use bevy_camera_extras::{components::{FlyCam, Viewer, Watched}, plugins::DefaultCameraPlugin};
use bevy_egui::EguiContext;
use bevy_mod_raycast::{immediate::Raycast, CursorRay};
use bevy_ui_extras::systems::visualize_window_for;

use crate::raycast_utils::resources::CursorRayHits;

/// camera controls for robot editor camera
pub struct RobotEditorCameraPlugin;

impl Plugin for RobotEditorCameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(DefaultCameraPlugin)
        .add_systems(Update, set_cam_to_watch)
        //.add_systems(Update, visualize_window_for::<Watched>)
        .add_systems(Update, click_camera_focus_target)
        //.add_systems(PreUpdate, click_camera_focus_target)
        ;
    }
}

pub fn set_cam_to_watch(
    flycams: Query<(Entity, &FlyCam), Without<Viewer>>,
    mut commands: Commands,
) {
    for (e, _) in flycams.iter() {
        commands.entity(e).insert(Viewer::default())
        ;
    }
}

///click a target to focus camera on
pub fn click_camera_focus_target(
    cursor_ray_hits: Res<CursorRayHits>,
    mesh_query: Query<(Entity, &Handle<Mesh>)>,
    watched_bodies: Query<&Watched>,
    mouse: ResMut<ButtonInput<MouseButton>>,
    keys: ResMut<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    if mouse.just_pressed(MouseButton::Right) && keys.pressed(KeyCode::ShiftLeft){
        let Some((_, _, (e, _))) = cursor_ray_hits.first_with(&mesh_query) else {return;};
        // get_first_hit_with(
        //     &**cursor_ray_hits
        //     , &mesh_query
        // ) else {return;};
        if watched_bodies.contains(e) {
            commands.entity(e).remove::<Watched>();
        } else {
            commands.entity(e).insert(Watched);
    
        }
    }
}
