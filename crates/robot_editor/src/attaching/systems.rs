use bevy::prelude::*;

use crate::resources::*;
use super::components::*;

/// gets rid of placers if current mode is not placermode
pub fn delete_attach_candidates(
    tool_mode: ResMut<BuildToolMode>,
    placers: Query<Entity, With<AttachCandidate>>,
    mut commands: Commands,
) {
    if *tool_mode != BuildToolMode::EditerMode {
        for e in placers.iter() {
            commands.entity(e).despawn()
        }
    }
}

/// manages gizmos associated with attacher
pub fn manage_attacher_position(

) {

}

/// confirms attacher target for attacher
pub fn confirm_attacher_target() {

}