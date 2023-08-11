#![allow(clippy::type_complexity)]

extern crate core;

use bevy::app::{App, PluginGroup, Update};
use bevy::asset::Assets;
use bevy::DefaultPlugins;
use bevy::input::Input;
use bevy::prelude::{AssetServer, Camera, Camera2dBundle, Commands, Component, Entity, in_state, IntoSystemConfigs, KeyCode, not, OnExit, Query, Res, ResMut, UiImage, With, Without};
use bevy::ui::{Style, Val};
use bevy::utils::default;
use bevy::window::{Cursor, Window, WindowPlugin, WindowResolution};
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_tweening::TweeningPlugin;
use bevy_undo::prelude::*;
use bevy_undo::prelude::UndoPlugin;

use crate::assets::font::FontAssets;
use crate::assets::gimmick::GimmickAssets;
use crate::assets::stage::{BuiltInStages, StageAssets};
use crate::before_stage_edit::BeforeStageEditPlugin;
use crate::button::SpriteButtonPlugin;
use crate::cursor::{GameCursor, GameCursorBundle};
use crate::gama_state::GameState;
use crate::loader::json::StageJson;
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
mod before_stage_edit;
mod cursor;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                cursor: Cursor {
                    visible: false,
                    ..default()
                },
                resolution: WindowResolution::new(800., 800.),
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
            BeforeStageEditPlugin,
            StageEditPlugin,
            StageSelectPlugin,
            StagePlugin
        ))
        .add_systems(OnExit(GameState::AssetLoading), setup)
        .add_systems(Update, (
            undo_if_input_keycode,
            move_cursor
        ).run_if(not(in_state(GameState::AssetLoading))))
        .add_state::<GameState>()
        .run();
}


#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash, Component)]
pub struct MainCamera;


fn setup(
    mut commands: Commands,
    stages: Res<StageAssets>,
    stage: ResMut<Assets<StageJson>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(MainCamera);

    commands.spawn(GameCursorBundle::new(&asset_server));

    let stages = stages
        .stages
        .iter()
        .filter_map(|stage_handle| stage.get(stage_handle).cloned())
        .collect::<Vec<StageJson>>();

    commands.insert_resource(BuiltInStages(stages));
}


fn undo_if_input_keycode(
    keycode: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    if keycode.just_pressed(KeyCode::R) {
        commands.undo();
    }
}


fn move_cursor(window: Query<&Window>, mut cursor: Query<&mut Style, With<GameCursor>>) {
    let window: &Window = window.single();
    if let Some(position) = window.cursor_position() {
        let mut img_style = cursor.single_mut();
        img_style.left = Val::Px(position.x - 15.);
        img_style.top = Val::Px(position.y - 15.);
    }
}


#[inline]
pub(crate) fn reset_game_cursor(
    asset_server: Res<AssetServer>,
    mut cursor: Query<&mut UiImage, With<GameCursor>>,
) {
    cursor.single_mut().texture = asset_server.load("game_cursor.png");
}


pub(crate) fn destroy_all(mut commands: Commands, entities: Query<Entity, (Without<Camera>, Without<Window>, Without<GameCursor>)>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}


