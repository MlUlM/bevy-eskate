#![allow(clippy::type_complexity)]

use bevy::app::{App, PluginGroup, Startup, Update};
use bevy::DefaultPlugins;
use bevy::input::Input;
use bevy::prelude::{Commands, KeyCode, Res};
use bevy::utils::default;
use bevy::window::{Window, WindowPlugin, WindowResolution};
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_tweening::TweeningPlugin;
use bevy_undo::prelude::*;
use bevy_undo::prelude::UndoPlugin;

use crate::button::SpriteButtonPlugin;
use crate::gama_state::GameState;
use crate::gimmick::asset::GimmickAssets;
use crate::playing::PlayingPlugin;
use crate::stage_edit::page_count::PageCount;
use crate::stage_edit::StageEditPlugin;

mod playing;
pub mod gimmick;
mod gama_state;
mod title;
mod stage_edit;
mod loader;
mod button;
mod error;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1920., 1080.),
                title: "Eskate".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Playing),
        )
        .add_collection_to_loading_state::<_, GimmickAssets>(GameState::AssetLoading)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(StageEditPlugin)
        .add_plugins(PlayingPlugin)
        .add_plugins(TweeningPlugin)
        .add_plugins(UndoPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, undo_if_input_keycode)
        .add_plugins(SpriteButtonPlugin)
        .add_state::<GameState>()
        .run();
}


fn setup(
    mut commands: Commands
) {
    commands.insert_resource(PageCount::new(2));
}


fn undo_if_input_keycode(
    keycode: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    if keycode.just_pressed(KeyCode::R) {
        commands.undo();
    }
}


