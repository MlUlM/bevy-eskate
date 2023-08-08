use bevy::app::{App, Plugin};
use bevy::core::Name;
use bevy::hierarchy::BuildChildren;
use bevy::prelude::{Bundle, ButtonBundle, Color, Commands, JustifyContent, NodeBundle, OnEnter, Res, TextBundle, Val};
use bevy::text::{Text, TextStyle};
use bevy::ui::Style;
use bevy::utils::default;

use crate::assets::font::FontAssets;
use crate::gama_state::GameState;

#[derive(Default, PartialEq, Debug)]
pub struct TitlePlugin;


impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Title), setup);
    }
}


fn setup(
    font: Res<FontAssets>,
    mut commands: Commands,
) {
    commands
        .spawn(ScreenBundle::screen())
        .with_children(|parent| {
            parent.spawn(ButtonBundle {
                button: Default::default(),
                ..default()
            })
                .insert(Name::new("StageEditButton"))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "StageEdit",
                            TextStyle {
                                font: font.button_text.clone(),
                                font_size: 30.,
                                color: Color::BLUE,
                            },
                        ),
                        style: Style {
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        });
}


#[derive(Bundle, Clone)]
struct ScreenBundle {
    node: NodeBundle,
    name: Name,
}


impl ScreenBundle {
    pub fn screen() -> Self {
        Self {
            node: NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                background_color: Default::default(),
                ..default()
            },
            name: Name::new("Screen"),
        }
    }
}