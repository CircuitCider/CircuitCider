use bevy::prelude::*;

use super::ui::*;
use super::systems::*;
pub struct AttachingToolingPlugin;

impl Plugin for AttachingToolingPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, attach_candidate_edit_ui)
        .add_systems(Update, delete_attach_candidates)

        ;
    }
}