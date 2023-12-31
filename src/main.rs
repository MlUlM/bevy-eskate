#![allow(clippy::type_complexity)]

extern crate core;

use bevy::app::{App, PluginGroup, Update};
use bevy::asset::Handle;
use bevy::DefaultPlugins;
use bevy::ecs::system::SystemParam;
use bevy::input::Input;
use bevy::prelude::{Assets, AssetServer, Camera, Camera2dBundle, Commands, Component, Entity, Image, in_state, IntoSystemConfigs, MouseButton, not, OnExit, Query, Res, ResMut, UiImage, With, Without};
use bevy::ui::{Style, Val};
use bevy::utils::default;
use bevy::window::{Cursor, Window, WindowPlugin, WindowResolution};
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_tweening::TweeningPlugin;
use bevy_undo2::UndoPlugin;

use crate::assets::cursor::CursorAssets;
use crate::assets::font::FontAssets;
use crate::assets::gimmick::GimmickAssets;
use crate::assets::stage::{BuiltInStages, StageAssets};
use crate::assets::stage_edit_assets::StageEditAssets;
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
mod undo;
mod window;


fn main() {
    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resizable: false,
            resolution: WindowResolution::new(1200., 800.),
            title: "eskate".to_string(),
            ..default()
        }),
        ..default()
    });

    // #[cfg(debug_assertions)]
    //     let default_plugins = default_plugins.set(LogPlugin {
    //     level: bevy::log::Level::DEBUG,
    //     filter: "debug,wgpu_core=warn,wgpu_hal=warn,mygame=debug".into(),
    // });

    App::new()
        .add_plugins(default_plugins)
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::StageSelect),
        )
        .add_collection_to_loading_state::<_, GimmickAssets>(GameState::AssetLoading)
        .add_collection_to_loading_state::<_, FontAssets>(GameState::AssetLoading)
        .add_collection_to_loading_state::<_, StageAssets>(GameState::AssetLoading)
        .add_collection_to_loading_state::<_, StageEditAssets>(GameState::AssetLoading)
        .add_collection_to_loading_state::<_, CursorAssets>(GameState::AssetLoading)
        .add_plugins((
            JsonAssetPlugin::<StageJson>::new(&["stage.json"]),
            // bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
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
        .add_systems(Update, move_cursor.run_if(not(in_state(GameState::AssetLoading))))
        .add_state::<GameState>()
        .run();
}


#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash, Component)]
pub struct MainCamera;


fn setup(
    mut commands: Commands,
    mut stage: ResMut<Assets<StageJson>>,
    stages: Res<StageAssets>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(MainCamera);

    commands.spawn(GameCursorBundle::new(&asset_server));
    commands.insert_resource(BuiltInStages(stages.stages(&mut stage)));
}


fn move_cursor(window: Query<&Window>, mut cursor: Query<&mut Style, With<GameCursor>>) {
    let window = window.single();
    if let Some(position) = window.cursor_position() {
        let mut img_style = cursor.single_mut();
        img_style.left = Val::Px(position.x - 15.);
        img_style.top = Val::Px(position.y - 15.);
    }
}


#[derive(SystemParam)]
pub(crate) struct GameCursorParams<'w, 's> {
    cursor: Query<'w, 's, &'static mut UiImage, With<GameCursor>>,
}


impl<'w, 's> GameCursorParams<'w, 's> {
    #[inline]
    pub fn reset(&mut self) {
        let Some(mut cursor) = self.cursor.iter_mut().next() else { return; };
        cursor.texture = Handle::default();
    }


    #[inline]
    pub fn set_cursor(&mut self, texture: Handle<Image>) {
        let Some(mut cursor) = self.cursor.iter_mut().next() else { return; };
        cursor.texture = texture;
    }
}


#[inline]
pub(crate) fn reset_game_cursor(
    asset_server: Res<AssetServer>,
    mut cursor: Query<&mut UiImage, With<GameCursor>>,
) {
    cursor.single_mut().texture = asset_server.load("game_cursor.png");
}


pub(crate) fn destroy_all(mut commands: Commands, entities: Query<Entity, (
    Without<Camera>,
    Without<Window>,
    Without<GameCursor>
)>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}


#[inline]
pub(crate) fn mouse_just_pressed_left(mouse: Res<Input<MouseButton>>) -> bool {
    mouse.just_pressed(MouseButton::Left)
}