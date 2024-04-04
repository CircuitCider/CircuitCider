use bevy::prelude::*;
use crate::model_display::systems::*;


pub const DISPLAY_MODEL_TRANSLATION: Vec3 = Vec3 {
    x: 0.0, 
    y: -0.2, 
    z: -1.3
};


/// displays models in a pretty way. 
pub struct ModelDisplayerPlugin;

impl Plugin for ModelDisplayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup_display_area)
        .add_systems(Update, rotate_display_model)
        .add_systems(Update, manage_display_platform_visibility)
        ;
    }
}

