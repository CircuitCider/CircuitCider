mod resources;
pub mod plugins;
pub mod shaders;

use std::any::TypeId;

use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use bevy::{
    ecs::system::Resource,
    prelude::{Deref, DerefMut},
    window::PrimaryWindow,
};

use bevy_egui::EguiContext;
use derive_more::From;
use egui_tiles::{SimplificationOptions, Tile, TileId, Tree};
use naga::front::wgsl;
use resources::{ShadersFolder, WgslCache};

#[derive(Resource, From, Deref, DerefMut)]
pub struct WgslInUi(Tree<Pane>);

#[derive(Resource, From, Deref, DerefMut)]
pub struct NagaUi(Tree<Pane>);

#[derive(Resource, From, Deref, DerefMut)]
pub struct WgslOutUi(Tree<Pane>);

pub struct TreeBehavior {}

#[derive(Default, Debug, Clone)]
pub struct Pane {
    pub name: String,
    pub content: String,
    //pub nr: usize,
}

/// bind a tree ui item to a resource.
pub fn bind_tree<T: From<Tree<Pane>> + Resource>() -> T {
    let tiles = egui_tiles::Tiles::default();
    T::from(egui_tiles::Tree::new(
        "my_tree",
        TileId::from_u64(9999),
        tiles,
    ))
}

/// loads assets of type T in a given folder.
pub fn load_assets_for<T: Asset>(
    folders: &Res<Assets<LoadedFolder>>,
    folder_handle: &bevy::asset::Handle<LoadedFolder>,
) -> Option<Vec<bevy::asset::Handle<T>>> {
    let typeid = TypeId::of::<T>();

    if let Some(folder) = folders.get(folder_handle) {
        let handles: Vec<bevy::asset::Handle<T>> = folder
            .handles
            .clone()
            .into_iter()
            .filter(|handle| handle.type_id() == typeid)
            .map(|handle| handle.typed::<T>())
            .collect::<Vec<_>>();
        Some(handles)
    } else {
        None
    }
}

/// update cache of wgsl files that visualizers read from
pub fn update_wgsl_cache(
    folders: Res<Assets<LoadedFolder>>,
    shader_folder_handle: Res<ShadersFolder>,
    mut wgsl_cache: ResMut<WgslCache>,
    shaders: Res<Assets<Shader>>,
) {
    if wgsl_cache.iter().len() <= 0 {
        let Some(shader_handles) = load_assets_for::<Shader>(&folders, &shader_folder_handle)
        else {
            return;
        };

        let shaders = shader_handles
            .iter()
            .filter_map(|handle| shaders.get(handle))
            .collect::<Vec<_>>();

        for shader in shaders.iter() {
            let name = shader
                .path
                .split("/")
                .last()
                .unwrap_or_default()
                .to_string();

            let source_str = match &shader.source {
                bevy::render::render_resource::Source::Wgsl(str) => str,
                _ => todo!("implement sources other then wgsl later."),
            }
            .to_string();

            wgsl_cache.insert(name, source_str);
        }
    }
}


/// Adds a folder to app by path and binds it to a given newtype struct resource with its handle.
pub fn add_folder<T: From<bevy::asset::Handle<LoadedFolder>> + Resource>(
    local_path: String,
) -> impl Fn(&mut World) {
    move |world| {
        let asset_server = world.get_resource_ref::<AssetServer>().unwrap();
        let folder_handle = asset_server.load_folder(local_path.to_owned());
        world.insert_resource(T::from(folder_handle));
    }
}

impl egui_tiles::Behavior<Pane> for TreeBehavior {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        pane.name.clone().into()
    }

    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: egui_tiles::TileId,
        pane: &mut Pane,
    ) -> egui_tiles::UiResponse {
        egui::ScrollArea::both().show(ui, |ui| {
            let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx());

            let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                let mut layout_job =
                    egui_extras::syntax_highlighting::highlight(ui.ctx(), &theme, string, "rust");
                layout_job.wrap.max_width = wrap_width;
                ui.fonts(|f| f.layout_job(layout_job))
            };
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut pane.content)
                        .font(egui::TextStyle::Monospace) // for cursor height
                        .code_editor()
                        .desired_rows(10)
                        .lock_focus(true)
                        .desired_width(f32::INFINITY)
                        .layouter(&mut layouter),
                );
            });
        });
        egui_tiles::UiResponse::None
    }
    fn simplification_options(&self) -> egui_tiles::SimplificationOptions {
        // tree keeps getting deleted so all of this gets set to fault for the time being.
        SimplificationOptions {
            prune_empty_containers: false,
            prune_empty_tabs: false,
            prune_single_child_containers: false,
            prune_single_child_tabs: false,
            all_panes_must_have_tabs: false,
            join_nested_linear_containers: false,
        }
    }
    // fn min_size(&self) -> f32 {
    //     0.0
    // }
}

/// displays relevant wgsl file info
pub fn display_wgslin_info(
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut wgsl_in_tree: ResMut<WgslInUi>,
    wgsl_cache: Res<WgslCache>,
) {
    for mut context in primary_window.iter_mut() {
        let mut tiles = egui_tiles::Tiles::default();
        let mut tabs = Vec::new();

        egui::Window::new("Wgsl In").show(context.get_mut(), |ui| {
            let mut behavior = TreeBehavior {};

            if ui.button("refresh shader list").clicked() || wgsl_in_tree.tiles.len() <= 0 {
                for (name, src) in wgsl_cache.iter() {
                    let panel = Pane {
                        name: name.clone(),
                        content: src.to_string(),
                    };

                    tabs.push(tiles.insert_pane(panel));
                }
                let root = tiles.insert_tab_tile(tabs.clone());

                //TODO: tree needs to be overwriten every time something is added because I havent found a way to get items to add properly
                // this needs to be fixed at some point though.
                **wgsl_in_tree = egui_tiles::Tree::new("wgslin_tree", root, tiles);
            }

            wgsl_in_tree.ui(&mut behavior, ui);
            //ui.allocate_space(ui.available_size());
        });
    }
}

pub fn display_naga_info(
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut naga_tree: ResMut<NagaUi>,
    wgsl_cache: Res<WgslCache>,
) {
    for mut context in primary_window.iter_mut() {
        let mut tiles = egui_tiles::Tiles::default();
        let mut tabs = Vec::new();

        egui::Window::new("naga info").show(context.get_mut(), |ui| {
            let mut behavior = TreeBehavior {};

            if ui.button("refresh").clicked() || naga_tree.is_empty() {
                for (name, src) in wgsl_cache.iter() {
                    let module_check = wgsl::parse_str(&src);
                    let debug_str = match module_check {
                        Ok(module) => {
                            format!("{:#?}", module)
                        }
                        Err(err) => err.to_string(),
                    };

                    let pane = Pane {
                        name: name.clone(),
                        content: debug_str,
                    };
                    tabs.push(tiles.insert_pane(pane));
                }
                let root = tiles.insert_tab_tile(tabs.clone());
                **naga_tree = egui_tiles::Tree::new("naga_tree", root, tiles);
            }

            naga_tree.ui(&mut behavior, ui);
            //ui.allocate_space(ui.available_size());
        });
    }
}

// pub fn display_wgslout_info(
//     mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
//     mut tree: ResMut<WgslInUi>,
// ) {

// }
