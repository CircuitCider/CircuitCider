use bevy::prelude::*;
use shader_core::shaders::neon::NeonMaterial;

use crate::resources::BuildToolMode;
use crate::states::RobotEditorState;
use crate::systems::intersection_colors_for;
use crate::systems::move_to_cursor;

use super::components::Placer;
use super::resources::*;
use super::systems::*;
use super::ui::*;

pub struct CachePrefabsPlugin;

impl Plugin for CachePrefabsPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_state::<BuildToolMode>()
            //.init_resource::<DisplayModelImage>()
            .insert_resource(ModelFolder::default())
            .add_systems(Startup, cache_initial_folders)
            //.add_systems(Update, select_build_tool)
            
            ;
    }
}

/// stuff required to run individual tools of robot editor
pub struct PlacingToolingPlugin;

impl Plugin for PlacingToolingPlugin {
    fn build(&self, app: &mut App) {
        // placers
        app
        .register_type::<Placer>()
        .add_systems(Update, intersection_colors_for::<Placer, NeonMaterial>)
        .add_systems(Update, move_to_cursor::<Placer>)
        .add_systems(Update, attach_placer)
        //.add_systems(Update, delete_placers)
        .add_systems(Update, placer_editor_ui)
        .add_systems(Update, placer_spawner_ui.run_if(in_state(RobotEditorState::Active)));
    }
}
