use bevy::prelude::*;
use strum_macros::Display;

use crate::{Spacing, Targeter};

/// entity used to place other similar entities.
#[derive(Component, Default, Display, Reflect)]
#[reflect(Component)]
pub enum Placer {
    #[default]
    Hull,
    Wheel,
}

impl Targeter for Placer {
    fn targets(&self) -> Option<Entity> {
        None
    }
}

impl Spacing for Placer {
    fn spacing() -> crate::SpacingKind {
        crate::SpacingKind::Uplift(0.01)
    }
}

impl Placer {
    /// infer placer type from path
    pub fn from_path(path: &str) -> Self {
        let lower_case = path.to_lowercase();
        let split_path = lower_case.split(&['/', '.']).collect::<Vec<_>>();

        // println!("split path of placer is {:#?}", split_path);
        if split_path.contains(&"wheel") {
            Self::Wheel
        } else if split_path.contains(&"hull") {
            Self::Hull
        }
        // default to hull if no valid placer type if found
        else {
            info!("cannot infer placer type from path. Defaulting to hull");
            Self::Hull
        }
    }
}
