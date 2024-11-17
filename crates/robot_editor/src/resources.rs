use bevy::{asset::{AssetServer, Assets, Handle, LoadState}, ecs::system::Resource, prelude::{Image, KeyCode, NextState, Res, ResMut, States}, reflect::Reflect, render::render_resource::{TextureViewDescriptor, TextureViewDimension}, utils::default};
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
    None
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
    //pub unfreeze_key: KeyCode,
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
            //unfreeze_key: KeyCode::KeyO,
        }
    }
}
