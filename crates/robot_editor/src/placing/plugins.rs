use bevy::prelude::*;
use shader_core::shaders::neon::NeonMaterial;

use crate::systems::intersection_colors_for;
use crate::systems::move_to_cursor;

use super::components::Placer;
use super::systems::*;
use super::ui::*;

/// stuff required to run individual tools of robot editor
pub struct PlacingToolingPlugin;

impl Plugin for PlacingToolingPlugin {
    fn build(&self, app: &mut App) {
        // placers
        app
        .register_type::<Placer>()
        .add_systems(Update, intersection_colors_for::<Placer, NeonMaterial>)
        // .add_systems(Update, build_tool_control_util_for::<Placer>)
        .add_systems(Update, move_to_cursor::<Placer>)
        .add_systems(Update, attach_placer)
        .add_systems(Update, delete_placers)
        .add_systems(Update, placer_editor_ui)
        
        // .add_systems(Update, placer_spawner_ui.run_if(in_state(RobotEditorState::Active)));
        ;
    }
}
