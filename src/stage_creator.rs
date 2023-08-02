use bevy::app::{App, Plugin};
use bevy::prelude::{AlignItems, Camera2dBundle, Color, Commands, JustifyContent, NodeBundle, OnEnter};
use bevy::ui::{BackgroundColor, Style, Val};

use crate::gama_state::GameState;

#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub struct StageCreatorPlugin;


impl Plugin for StageCreatorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Maker), setup);
    }
}


fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(NodeBundle {
        style: Style {
            height: Val::Percent(100.),
            width: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        background_color: BackgroundColor(Color::WHITE),
        ..Default::default()
    });
}


