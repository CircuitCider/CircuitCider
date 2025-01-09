use crate::model_display::systems::*;
use bevy::prelude::*;

use super::{components::{DisplayModelCamera, DisplayModelStaging, DisplayRoot}, DisplayModel};

pub const DISPLAY_MODEL_TRANSLATION: Vec3 = Vec3 {
    x: 0.0,
    y: -0.2,
    z: -1.3,
};

/// displays models in a pretty way.
pub struct ModelDisplayerPlugin;

impl Plugin for ModelDisplayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<DisplayModel>()
        .register_type::<DisplayModel>()
        .register_type::<DisplayRoot>()
            .register_type::<DisplayModelStaging>()
            .register_type::<DisplayModelCamera>()
            .add_systems(Startup, setup_display_area)
            .add_systems(Update, setup_display_model.run_if(resource_changed::<DisplayModel>))
            .add_systems(Update, rotate_display_model)
            .add_systems(Update, manage_display_platform_visibility);
    }
}
