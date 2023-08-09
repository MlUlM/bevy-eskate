use bevy::app::{App, Plugin, Update};
use bevy::core::Name;
use bevy::hierarchy::BuildChildren;
use bevy::prelude::{Bundle, ButtonBundle, Color, Commands, in_state, IntoSystemConfigs, JustifyContent, NextState, NodeBundle, OnEnter, OnExit, Query, Res, ResMut, TextBundle, Val, With};
use bevy::text::{Text, TextStyle};
use bevy::ui::{AlignItems, BackgroundColor, Interaction, Style};
use bevy::utils::default;
use bevy_trait_query::imports::Component;

use crate::assets::font::FontAssets;
use crate::destroy_all;
use crate::extension::InteractionCondition;
use crate::gama_state::GameState;

#[derive(Default, PartialEq, Debug)]
pub struct TitlePlugin;


impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Title), setup)
            .add_systems(OnExit(GameState::Title), destroy_all)
            .add_systems(Update, input.run_if(in_state(GameState::Title)));
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
                .insert(Name::new("Stage"))
                .insert(StageButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Stage",
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


            parent.spawn(ButtonBundle {
                button: Default::default(),
                ..default()
            })
                .insert(Name::new("StageEditButton"))
                .insert(StageEditButton)
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


fn input(
    mut state: ResMut<NextState<GameState>>,
    stage: Query<&Interaction, (With<Interaction>, With<StageButton>)>,
    stage_edit: Query<&Interaction, (With<Interaction>, With<StageEditButton>)>,
) {
    if stage_edit.single().pressed() {
        state.set(GameState::StageEdit);
    } else if stage.single().pressed() {
        state.set(GameState::StageSelect);
    }
}

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct StageEditButton;

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct StageButton;


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
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(10.),
                    ..default()
                },
                background_color: BackgroundColor::from(Color::NONE),
                ..default()
            },
            name: Name::new("Screen"),
        }
    }
}