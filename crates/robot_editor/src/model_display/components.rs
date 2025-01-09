use bevy::prelude::*;

/// camera that renders models loaded to images for display.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct DisplayModelCamera;

// /// model only rendered for display
// #[derive(Component, Reflect)]
// #[reflect(Component)]
// pub struct DisplayModel;


/// Root of model being displayed
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct DisplayRoot;

/// thing display model is sat on top of.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct DisplayModelStaging;
