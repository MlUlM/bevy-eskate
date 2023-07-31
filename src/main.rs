#![allow(clippy::type_complexity)]

use bevy::app::{App, PluginGroup};
use bevy::DefaultPlugins;
use bevy::utils::default;
use bevy::window::{Window, WindowPlugin, WindowResolution};
use bevy_tweening::TweeningPlugin;

use crate::playing::PlayingPlugin;

mod playing;
pub mod gimmick;

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
        .add_plugins(PlayingPlugin)
        .add_plugins(TweeningPlugin)
        .run();
}


