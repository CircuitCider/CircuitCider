use bevy::prelude::*;
use shader_core::shaders::neon_glow::NeonGlowMaterial;

use crate::systems::intersection_colors_for;

use super::components::AttachCandidate;
use super::systems::*;
use super::ui::*;
pub struct AttachingToolingPlugin;

impl Plugin for AttachingToolingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            intersection_colors_for::<AttachCandidate, NeonGlowMaterial>,
        )
        .add_systems(Update, attach_candidate_edit_ui)
        .add_systems(Update, delete_attach_candidates);
    }
}
