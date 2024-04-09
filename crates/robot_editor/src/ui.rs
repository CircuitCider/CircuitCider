use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
    io::ErrorKind,
    thread::spawn,
};

use crate::{
    raycast_utils::{resources::MouseOverWindow, systems::*},
    resources::BuildToolMode,
};
use bevy::{
    asset::{AssetContainer, LoadedFolder},
    ecs::query::{QueryData, QueryFilter, ReadOnlyQueryData, WorldQuery},
    input::mouse::MouseButtonInput,
    log::tracing_subscriber::field::display,
    prelude::*,
    reflect::erased_serde::Error,
    render::{render_asset::RenderAssets, render_resource::TextureFormat, view::RenderLayers},
    window::PrimaryWindow,
};
use bevy_egui::EguiContext;
use bevy_mod_raycast::{
    immediate::{Raycast, RaycastSettings, RaycastVisibility},
    primitives::IntersectionData,
    CursorRay,
};
use bevy_rapier3d::{
    geometry::{Collider, Sensor},
    plugin::RapierContext,
    rapier::geometry::CollisionEventFlags,
};
use bevy_serialization_extras::prelude::{colliders::ColliderFlag, link::StructureFlag};
use egui::Align2;
use std::hash::Hash;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use std::fmt::Debug;

/// creates a egui window that follows mouse when its given condition is satisfied.
/// 
/// if mouse is not in window, this will not create a iwndow
pub fn window_follow_mouse(window: &Window, condition: bool, ui_name: &str) -> Option<egui::Window<'static>> {
    let offset = 10.0;

    let cursor_pos = window.cursor_position()?;

    let window = egui::Window::new(ui_name);
    if condition {
        return Some(window.fixed_pos((cursor_pos.x + offset, cursor_pos.y + offset)))
    } else {
        return Some(window)
    }
}

pub fn select_build_tool(
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut tool_mode: ResMut<BuildToolMode>,
) {
    for mut context in primary_window.iter_mut() {
        egui::Window::new("BuildToolMode debug").show(context.get_mut(), |ui| {
            ui.heading("select mode");
            ui.label(format!("Current mode: {:#?}", *tool_mode));
            for tool in BuildToolMode::iter() {
                if ui.button(tool.to_string()).clicked() {
                    *tool_mode = tool
                }
            }
        });
    }
}

/// Sets mouse over window resource to true/false depending on mouse state.
pub fn check_if_mouse_over_ui(
    mut windows: Query<&mut EguiContext>,
    mut mouse_over_window: ResMut<MouseOverWindow>,
) {
    for mut window in windows.iter_mut() {
        if window.get_mut().is_pointer_over_area() {
            //println!("mouse is over window");
            **mouse_over_window = true
        } else {
            **mouse_over_window = false
        }
    }
    //**mouse_over_window = false
}

#[derive(Component, Default)]
pub struct Edited;

// /// editor mode for editing attached
// pub fn editor_mode_ui

pub fn save_load_model_ui(
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    //mut commands: Commands,
) {
    for mut context in primary_window.iter_mut() {
        let ui_name = "Save Load Model";
        egui::Window::new(ui_name)
            .anchor(Align2::RIGHT_TOP, [0.0, 0.0])
            .collapsible(false)
            .resizable(false)
            .show(context.get_mut(), |ui| {
                ui.label("save conditions");

                ui.horizontal(|ui| {
                    ui.button("save");

                });
            });
    }
}

// #[derive(Resource, Deref, Default)]
// pub struct DisplayModelImage(pub Handle<Image>);
