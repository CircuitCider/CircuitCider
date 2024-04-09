use bevy::prelude::*;

use crate::resources::BuildToolMode;
use crate::ui::select_build_tool;

use super::resources::*;
use super::systems::*;
use super::ui::*;

pub struct CachePrefabsPlugin;

impl Plugin for CachePrefabsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BuildToolMode::PlacerMode)
            //.init_resource::<DisplayModelImage>()
            .insert_resource(ModelFolder::default())
            .add_systems(Startup, cache_initial_folders)
            .add_systems(Update, select_build_tool)
            
            ;
    }
}

/// stuff required to run individual tools of robot editor
pub struct PlacingToolingPlugin;

impl Plugin for PlacingToolingPlugin {
    fn build(&self, app: &mut App) {
        // placers
        app.add_systems(Update, move_placer_to_cursor)
            .add_systems(Update, attach_placer)
            .add_systems(Update, delete_placers)
            .add_systems(Update, placer_editor_ui)
            .add_systems(Update, placer_spawner_ui)


            ;
    }
}
