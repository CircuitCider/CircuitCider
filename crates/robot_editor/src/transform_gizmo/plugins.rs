use bevy::{asset::load_internal_asset, prelude::*};

use super::systems::*;
//use bevy_component_extras::components::*;
//use crate::editor::systems::SelectedForEdit;
//use gizmo_material::GizmoMaterial;
/// plugin for managing transform widgets. Use this to spawn transform widgets to manipulate clicked models.
pub struct TransformWidgetPlugin;
use super::components::*;

//(todo) make a `Compose`, set which includes all composed systems, and have a `delete` set of systems, run only after compose
impl Plugin for TransformWidgetPlugin {
    fn build(&self, app: &mut App) {
        app


        .register_type::<LastMouseInteraction>()
        .register_type::<Tug>()

        .add_systems(Update, gizmo_mark_on_click)
        .add_systems(Update, spawn_gizmo_when_needed)
        .add_systems(Update, despawn_gizmo_when_no_targets)
        .add_systems(Update, average_gizmo_position)
        // .add_systems(Update, (manage_tugs, /*manage_rings*/ widget_spawn_for_selected, transform_widget_behaviour)
        //     .before(widget_despawn_for_deselected)) 
            // COMPOSED SYSTEMS MUST RUN BEFORE DESPAWn BEHAVIOUR RUNS,
            // OTHERWISE, A CRASH FROM FAILING TO .insert(<thing>) INTO ENTITY WILL OCCUR
        ;
    }
}