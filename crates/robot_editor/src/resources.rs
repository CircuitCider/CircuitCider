use bevy::{
    asset::Handle,
    ecs::system::Resource,
    prelude::{Deref, Image, KeyCode, States},
    reflect::Reflect,
};
use bevy_asset_loader::asset_collection::AssetCollection;
use strum_macros::{Display, EnumIter};

#[derive(Hash, States, Clone, Copy, Reflect, Debug, PartialEq, Eq, EnumIter, Display, Default)]
pub enum BuildToolMode {
    GizmoMode,
    PlacerMode,
    SelectorMode,
    //AttachMode,
    EditerMode,
    #[default]
    None,
}

#[derive(Hash, States, Clone, Copy, Reflect, Debug, PartialEq, Eq, EnumIter, Display, Default)]

pub enum BuildWidgetMode {
    Gizmo,
    #[default]
    Pointer
}

use bevy::{asset::LoadedFolder, prelude::*};

#[derive(Resource, Default, Deref)]
pub struct HullsFolder(pub Handle<LoadedFolder>);

#[derive(Resource, Default, Deref)]
pub struct WheelsFolder(pub Handle<LoadedFolder>);

#[derive(Resource, Default, Deref)]
pub struct WeaponsFolder(pub Handle<LoadedFolder>);

// #[derive(Resource)]
// pub struct BuildMenuTarget(pub BuildMenuTargets);

// #[derive(Resource, Default, Deref, DerefMut)]
// pub struct BuildMenuUi(pub Option<Ui>);

/// spawnable being focused on by build menu
#[derive(Resource, Clone, Default, PartialEq, Eq, PartialOrd, Ord, EnumIter, Display)]
pub enum BuildMenuTarget {
    #[default]
    Hulls,
    Weapons,
    Wheels,
}

#[derive(AssetCollection, Resource)]
pub struct ImageHandles {
    #[asset(path = "root://images/skybox.png")]
    pub skybox: Handle<Image>,
}

// /// weather skybox is preprocessed already(should be no by default)
// #[derive(Resource)]
// pub struct SkyBoxPreprocessed(pub bool);

#[derive(Resource, Reflect)]
pub struct RobotControls {
    pub target_speed: f32,

    pub leftward_key: KeyCode,
    pub rightward_key: KeyCode,
    pub forward_key: KeyCode,
    pub backward_key: KeyCode,

    pub freeze_key: KeyCode,
    pub unfreeze_key: KeyCode,
}

impl Default for RobotControls {
    fn default() -> Self {
        RobotControls {
            target_speed: 20.0,
            leftward_key: KeyCode::ArrowLeft,
            rightward_key: KeyCode::ArrowRight,
            forward_key: KeyCode::ArrowUp,
            backward_key: KeyCode::ArrowDown,
            freeze_key: KeyCode::KeyP,
            unfreeze_key: KeyCode::KeyO,
        }
    }
}
