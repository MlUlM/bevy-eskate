use bevy::app::{App, Plugin, Update};
use bevy::core::Name;
use bevy::hierarchy::BuildChildren;
use bevy::prelude::{AlignItems, ButtonBundle, ChildBuilder, Color, Commands, Event, EventReader, in_state, Input, IntoSystemConfigs, JustifyContent, KeyCode, NextState, NodeBundle, OnEnter, OnExit, Query, RepeatedGridTrack, Res, ResMut, Text, TextBundle, TextStyle, Val, With};
use bevy::ui::{BackgroundColor, Display, Interaction, Style};
use bevy::utils::default;
use bevy_input_sequence::AddInputSequenceEvent;
use bevy_input_sequence::prelude::{InputSequence, Timeout};
use bevy_trait_query::imports::Component;

use crate::assets::font::FontAssets;
use crate::assets::stage::BuiltInStages;
use crate::destroy_all;
use crate::extension::InteractionCondition;
use crate::gama_state::GameState;
use crate::loader::json::StageJson;
use crate::page::page_count::PageCount;

#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub struct StageSelectPlugin;


impl Plugin for StageSelectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_input_sequence_event::<SecretCommandEvent>()
            .add_systems(OnEnter(GameState::StageSelect), setup)
            .add_systems(OnExit(GameState::StageSelect), destroy_all)
            .add_systems(Update, (
                select_stage,
                back_scene_system,
                before_stage_edit_system
            ).run_if(in_state(GameState::StageSelect)));
    }
}


#[derive(Event, Clone)]
struct SecretCommandEvent;


fn setup(
    font: Res<FontAssets>,
    stages: Res<BuiltInStages>,
    mut commands: Commands,
) {
    commands.spawn(InputSequence::from_keycodes(
        SecretCommandEvent,
        Timeout::None,
        &[
            KeyCode::Up,
            KeyCode::Up,
            KeyCode::Down,
            KeyCode::Down,
            KeyCode::Left,
            KeyCode::Right,
            KeyCode::Left,
            KeyCode::Right,
            KeyCode::B,
            KeyCode::A
        ],
    ));

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
        .with_children(|parent| { spawn_stage_panel(parent, &font, &stages); });
}


fn before_stage_edit_system(
    mut state: ResMut<NextState<GameState>>,
    mut er: EventReader<SecretCommandEvent>,
) {
    for _ in er.iter() {
        state.set(GameState::BeforeStageEdit);
    }
}


fn spawn_stage_panel(parent: &mut ChildBuilder, font: &FontAssets, stages: &BuiltInStages) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(80.),
            height: Val::Percent(80.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
            column_gap: Val::Px(12.0),
            display: Display::Grid,
            ..default()
        },
        ..default()
    })
        .insert(Name::new("StagePanel"))
        .with_children(|parent| { spawn_stages(parent, font, stages); });
}


#[derive(Component, Default, Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct StagePlayButton;


fn spawn_stages(parent: &mut ChildBuilder, font: &FontAssets, stages: &BuiltInStages) {
    for stage in stages.0.iter() {
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(200.),
                height: Val::Px(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor::from(Color::from([80. / 255., 150. / 255., 250. / 255., 0.8])),
            ..default()
        })
            .insert((
                Name::new(stage.name.clone()),
                stage.clone(),
                StagePlayButton,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        stage.name.clone(),
                        TextStyle {
                            font: font.button_text.clone(),
                            font_size: 30.,
                            color: Color::BLACK,
                        },
                    ),
                    style: Style {
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                });
            });
    }
}


fn select_stage(
    mut state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    stage_buttons: Query<(&Interaction, &StageJson), (
        With<Interaction>,
        With<StageJson>,
        With<StagePlayButton>,
    )>,
) {
    for (interaction, stage_json) in stage_buttons.iter() {
        if interaction.pressed() {
            commands.insert_resource(stage_json.clone());
            commands.insert_resource(PageCount::new(stage_json.pages.len()));
            state.set(GameState::Stage);
            return;
        }
    }
}


fn back_scene_system(
    mut state: ResMut<NextState<GameState>>,
    key: Res<Input<KeyCode>>,
) {
    if key.just_pressed(KeyCode::Escape) {
        state.set(GameState::Title);
    }
}