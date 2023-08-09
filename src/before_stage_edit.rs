use bevy::app::{App, Plugin, Update};
use bevy::core::Name;
use bevy::hierarchy::{BuildChildren, ChildBuilder};
use bevy::input::Input;
use bevy::prelude::{ButtonBundle, Color, Commands, Component, default, in_state, IntoSystemConfigs, MouseButton, NextState, NodeBundle, OnEnter, OnExit, Query, Res, ResMut, Style, Text, TextBundle, With};
use bevy::text::TextStyle;
use bevy::ui::{AlignItems, BackgroundColor, Interaction, JustifyContent, Val};

use crate::assets::font::FontAssets;
use crate::destroy_all;
use crate::extension::InteractionCondition;
use crate::gama_state::GameState;
use crate::page::page_count::PageCount;

#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub struct BeforeStageEditPlugin;


impl Plugin for BeforeStageEditPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::BeforeStageEdit), setup)
            .add_systems(OnExit(GameState::BeforeStageEdit), destroy_all)
            .add_systems(Update, interaction
                .run_if(in_state(GameState::BeforeStageEdit)),
            )
        ;
    }
}


fn setup(
    font: Res<FontAssets>,
    mut commands: Commands,
) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    })
        .insert(Name::new("Screen"))
        .with_children(|parent| {
            page_count_ui(parent, &font);
            start_button(parent, &font);
        });
}


fn interaction(
    mut state: ResMut<NextState<GameState>>,
    mouse: Res<Input<MouseButton>>,
    mut commands: Commands,
    mut page_count: Query<&mut Text, With<PageCountText>>,
    down: Query<&Interaction, (With<Interaction>, With<PageDownButton>)>,
    up: Query<&Interaction, (With<Interaction>, With<PageUpButton>)>,
    start_button: Query<&Interaction, (With<Interaction>, With<StartButton>)>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let count = page_count.single().sections[0].value.parse::<usize>().unwrap();
        if down.single().pressed() && 0 < count  {
            page_count.single_mut().sections[0].value = (count - 1).to_string();
        } else if up.single().pressed() {
            page_count.single_mut().sections[0].value = (count + 1).to_string();
        } else if start_button.single().pressed() {
            commands.insert_resource(PageCount::new(count));
            state.set(GameState::StageEdit);
        }
    }
}


#[derive(Default, Debug, PartialEq, Copy, Clone, Component)]
struct PageCountText;


#[derive(Default, Debug, PartialEq, Copy, Clone, Component)]
struct PageDownButton;


#[derive(Default, Debug, PartialEq, Copy, Clone, Component)]
struct PageUpButton;

fn page_count_ui(parent: &mut ChildBuilder, font: &FontAssets) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Px(300.),
            height: Val::Px(100.),
            column_gap: Val::Px(8.),
            ..default()
        },
        ..default()
    })
        .insert(Name::new("PageCountArea"))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "2",
                    TextStyle {
                        font: font.button_text.clone(),
                        font_size: 30.0,
                        ..default()
                    },
                ),
                ..default()
            })
                .insert(PageCountText);


            parent.spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(30.),
                    height: Val::Px(30.),
                    ..default()
                },
                background_color: BackgroundColor::from(Color::BEIGE),
                ..default()
            })
                .insert((Name::new("PageDown"), PageDownButton));


            parent.spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(30.),
                    height: Val::Px(30.),
                    ..default()
                },
                background_color: BackgroundColor::from(Color::BEIGE),
                ..default()
            })
                .insert((Name::new("PageUp"), PageUpButton));
        });
}


#[derive(Default, Debug, PartialEq, Copy, Clone, Component)]
struct StartButton;


fn start_button(parent: &mut ChildBuilder, font: &FontAssets) {
    parent.spawn(ButtonBundle {
        style: Style {
            width: Val::Px(30.),
            height: Val::Px(30.),
            ..default()
        },
        background_color: BackgroundColor::from(Color::ORANGE_RED),
        ..default()
    })
        .insert((Name::new("StartButton"), StartButton))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section("Start", TextStyle {
                font: font.button_text.clone(),
                font_size: 23.,
                color: Color::BEIGE,
            }));
        })
    ;
}