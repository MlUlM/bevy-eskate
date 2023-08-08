#![allow(clippy::type_complexity)]

use bevy::app::{App, PluginGroup, Startup, Update};
use bevy::DefaultPlugins;
use bevy::input::Input;
use bevy::prelude::{Camera2dBundle, Commands, KeyCode, Res};
use bevy::utils::default;
use bevy::window::{Window, WindowPlugin, WindowResolution};
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_tweening::TweeningPlugin;
use bevy_undo::prelude::*;
use bevy_undo::prelude::UndoPlugin;

use page::page_count::PageCount;

use crate::assets::font::FontAssets;
use crate::button::SpriteButtonPlugin;
use crate::gama_state::GameState;
use crate::gimmick_assets::GimmickAssets;
use crate::loader::{StageLoadable, StageLoader};
use crate::stage::StagePlugin;
use crate::stage_edit::StageEditPlugin;
use crate::title::TitlePlugin;

mod gama_state;
mod title;
mod stage_edit;
mod loader;
mod button;
mod error;
mod page;
mod stage;
mod gimmick_assets;
mod assets;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(500., 300.),
                title: "Eskate".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Title),
        )
        .add_collection_to_loading_state::<_, GimmickAssets>(GameState::AssetLoading)
        .add_collection_to_loading_state::<_, FontAssets>(GameState::AssetLoading)
        .add_plugins((
            WorldInspectorPlugin::new(),
            TweeningPlugin,
            UndoPlugin,
            SpriteButtonPlugin
        ))
        .add_plugins((
            TitlePlugin,
            StageEditPlugin,
            StagePlugin
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, undo_if_input_keycode)
        .add_state::<GameState>()
        .run();
}


fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
    let stages = StageLoader::new().load().unwrap();
    let stage = stages.first().unwrap();

    commands.insert_resource(stage.clone());
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


