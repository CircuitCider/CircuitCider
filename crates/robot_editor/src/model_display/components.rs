use bevy::prelude::*;

/// camera that renders models loaded to images for display.
#[derive(Component, Reflect, Default)]
pub struct DisplayModelCamera;

/// model only rendered for display
#[derive(Component)]
pub struct DisplayModel;

/// thing display model is sat on top of. 
#[derive(Component)]
pub struct DisplayModelStaging;