use bevy::{asset::load_internal_asset, prelude::*};

use crate::shaders::neon_glow::NeonGlowMaterial;

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

        //

        .register_type::<LastMouseInteraction>()
        .register_type::<Tug>()

        // gizmo management
        .add_systems(Update, gizmo_mark_on_click)
        .add_systems(Update, spawn_gizmo_when_needed)
        .add_systems(Update, despawn_gizmo_when_no_targets)
        .add_systems(Update, (
            average_gizmo_position,
            drag_tugs_with_mouse.after(average_gizmo_position),
            drag_rings_with_mouse.after(average_gizmo_position)
        )
        )
            
        
        
        // gizmo camera management
        .add_systems(Update, spawn_gizmo_rendering_camera)
        .add_systems(Update, align_gizmo_camera_to_marker)
        .add_systems(Update, despawn_gizmo_rendering_camera)
        // .add_systems(Update, (manage_tugs, /*manage_rings*/ widget_spawn_for_selected, transform_widget_behaviour)
        //     .before(widget_despawn_for_deselected)) 
            // COMPOSED SYSTEMS MUST RUN BEFORE DESPAWn BEHAVIOUR RUNS,
            // OTHERWISE, A CRASH FROM FAILING TO .insert(<thing>) INTO ENTITY WILL OCCUR
        ;
    }
}
