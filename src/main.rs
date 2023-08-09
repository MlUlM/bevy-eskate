#![allow(clippy::type_complexity)]

use bevy::app::{App, PluginGroup, Update};
use bevy::asset::Assets;
use bevy::DefaultPlugins;
use bevy::input::Input;
use bevy::prelude::{Camera, Camera2dBundle, Commands, Component, Entity, KeyCode, OnExit, Query, Res, ResMut, Without};
use bevy::utils::default;
use bevy::window::{Window, WindowPlugin, WindowResolution};
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_tweening::TweeningPlugin;
use bevy_undo::prelude::*;
use bevy_undo::prelude::UndoPlugin;

use crate::assets::font::FontAssets;
use crate::assets::gimmick::GimmickAssets;
use crate::assets::stage::{BuiltInStages, StageAssets};
use crate::button::SpriteButtonPlugin;
use crate::gama_state::GameState;
use crate::loader::json::StageJson;
use crate::page::page_count::PageCount;
use crate::stage::StagePlugin;
use crate::stage_edit::StageEditPlugin;
use crate::stage_select::StageSelectPlugin;
use crate::title::TitlePlugin;

mod gama_state;
mod title;
mod stage_edit;
mod loader;
mod button;
mod error;
mod page;
mod stage;
mod assets;
mod extension;
mod stage_select;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1431., 971.),
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
        .add_collection_to_loading_state::<_, StageAssets>(GameState::AssetLoading)
        .add_plugins((
            JsonAssetPlugin::<StageJson>::new(&["stage.json"]),
            WorldInspectorPlugin::new(),
            TweeningPlugin,
            UndoPlugin,
            SpriteButtonPlugin
        ))
        .add_plugins((
            TitlePlugin,
            StageEditPlugin,
            StageSelectPlugin,
            StagePlugin
        ))
        .add_systems(OnExit(GameState::AssetLoading), setup)
        .add_systems(Update, undo_if_input_keycode)
        .add_state::<GameState>()
        .run();
}


#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash, Component)]
pub struct MainCamera;


fn setup(
    mut commands: Commands,
    stages: Res<StageAssets>,
    stage: ResMut<Assets<StageJson>>,
) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(MainCamera);

    let stages = stages
        .stages
        .iter()
        .filter_map(|stage_handle| stage.get(stage_handle).cloned())
        .collect::<Vec<StageJson>>();

    commands.insert_resource(BuiltInStages(stages));
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


pub(crate) fn destroy_all(mut commands: Commands, entities: Query<Entity, (Without<Camera>, Without<Window>)>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}


