use bevy::prelude::*;
use shader_core::shaders::neon::NeonMaterial;

use crate::systems::build_tool_control_util_for;
use crate::systems::intersection_colors_for;
use crate::systems::move_to_cursor;

use super::components::AttachCandidate;
use super::systems::*;
use super::ui::*;
pub struct AttachingToolingPlugin;

impl Plugin for AttachingToolingPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<AttachCandidate>()
        // .add_systems(Update,intersection_colors_for::<AttachCandidate, NeonMaterial>)
        .add_systems(Update, move_to_cursor::<AttachCandidate>)
        .add_systems(Update, attach_candidate_edit_ui)
        .add_systems(Update, build_tool_control_util_for::<AttachCandidate>)
        // .add_systems(Update, delete_attach_candidates);
        ;
    }
}
