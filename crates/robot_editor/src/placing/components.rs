use bevy::prelude::*;
use strum_macros::Display;

/// marker for objects that are not yet a part of a structure but could be
/// (placed build mode models)
#[derive(Component, Default)]
pub struct AttachCandidate;

/// entity used to place other similar entities.
#[derive(Component, Default, Display)]
pub enum Placer {
    #[default]
    Hull,
    Wheel,
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
