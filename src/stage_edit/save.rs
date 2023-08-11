use std::collections::HashMap;

use bevy::app::{App, Plugin, Update};
use bevy::core::Name;
use bevy::ecs::system::SystemParam;
use bevy::hierarchy::{BuildChildren, DespawnRecursiveExt};
use bevy::input::Input;
use bevy::math::{I64Vec2, Vec2};
use bevy::prelude::{ButtonBundle, ChildBuilder, Color, Commands, Component, Condition, Entity, Event, EventReader, EventWriter, in_state, IntoSystemConfigs, JustifyContent, MouseButton, NextState, NodeBundle, Query, Res, ResMut, resource_changed, resource_exists_and_equals, TextBundle, Transform, With};
use bevy::text::{Text, TextStyle};
use bevy::ui::{AlignItems, BackgroundColor, Display, FlexDirection, Interaction, PositionType, Style, UiRect, Val};
use bevy::utils::default;
use bevy::window::ReceivedCharacter;

use crate::assets::font::FontAssets;
use crate::extension::InteractionCondition;
use crate::gama_state::GameState;
use crate::loader::{StageLoadable, StageLoader};
use crate::loader::json::{Page, StageCell, StageJson};
use crate::page::page_index::PageIndex;
use crate::page::page_param::PageParams;
use crate::stage::playing::gimmick::Gimmick;
use crate::stage::playing::gimmick::tag::GimmickTag;
use crate::stage_edit::StageEditStatus;

#[derive(Event)]
struct SaveUiDespawnEvent;


#[derive(Default, Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct StageEditSavePlugin;


impl Plugin for StageEditSavePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SaveUiDespawnEvent>()
            .add_systems(Update, setup
                .run_if(resource_exists_and_equals(StageEditStatus::SaveStage).and_then(resource_changed::<StageEditStatus>())),
            )
            .add_systems(Update, (
                input_key,
                click_handle,
                despawn_ui_event
            )
                .run_if(in_state(GameState::StageEdit).and_then(resource_exists_and_equals(StageEditStatus::SaveStage))),
            );
    }
}


#[derive(Component, Copy, Clone, Eq, PartialEq, Hash)]
struct SaveUiRootNode;

fn setup(
    mut commands: Commands,
    font: Res<FontAssets>,
) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            ..default()
        },
        background_color: BackgroundColor::from(Color::Rgba { red: 192. / 255., green: 192. / 255., blue: 192. / 255., alpha: 0.7 }),
        ..default()
    })
        .insert((Name::new("SaveUiScreen"), SaveUiRootNode))
        .with_children(|parent| {
            stage_name_text(parent, &font);
            parent.spawn(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    column_gap: Val::Px(16.),
                    margin: UiRect::top(Val::Px(32.)),
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                cancel_button(parent, &font);
                save_button(parent, &font);
            });
        });
}


#[derive(Component, Debug, Default, Eq, PartialEq, Copy, Clone)]
struct StageNameText;


fn stage_name_text(parent: &mut ChildBuilder, font: &FontAssets) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Px(300.),
            height: Val::Px(60.),
            padding: UiRect::new(Val::Px(8.), Val::Px(8.), Val::Px(8.), Val::Px(8.)),
            ..default()
        },
        background_color: BackgroundColor::from(Color::Rgba { red: 38. / 255., green: 38. / 255., blue: 38. / 255., alpha: 1. }),
        ..default()
    })
        .insert(Name::new("StageNameText"))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section("", TextStyle {
                    font: font.button_text.clone(),
                    font_size: 48.,
                    color: Color::WHITE,
                }),
                ..default()
            })
                .insert((StageNameText, Name::new("StageNameText")));
        });
}


#[derive(Component, Eq, PartialEq, Hash, Copy, Clone, Default, Debug)]
struct SaveButton;

#[derive(Component, Eq, PartialEq, Hash, Copy, Clone, Default, Debug)]
struct CancelButton;

fn save_button(parent: &mut ChildBuilder, font: &FontAssets) {
    parent.spawn(ButtonBundle {
        style: Style {
            width: Val::Px(130.),
            height: Val::Px(100.),
            padding: UiRect::all(Val::Px(8.)),
            ..default()
        },
        background_color: BackgroundColor::from(Color::BEIGE),
        ..default()
    })
        .insert((Name::new("SaveButton"), SaveButton))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section("Save", TextStyle {
                    font: font.button_text.clone(),
                    font_size: 32.,
                    color: Color::BLACK,
                }),
                ..default()
            });
        });
}


fn cancel_button(parent: &mut ChildBuilder, font: &FontAssets) {
    parent.spawn(ButtonBundle {
        style: Style {
            width: Val::Px(80.),
            height: Val::Px(50.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: BackgroundColor::from(Color::BEIGE),
        ..default()
    })
        .insert((Name::new("CancelButton"), CancelButton))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section("Cancel", TextStyle {
                    font: font.button_text.clone(),
                    font_size: 32.,
                    color: Color::WHITE,
                }),
                ..default()
            });
        });
}


#[derive(SystemParam)]
struct ButtonsParams<'w, 's> {
    mouse: Res<'w, Input<MouseButton>>,
    cancel_button: Query<'w, 's, &'static Interaction, With<CancelButton>>,
    save_button: Query<'w, 's, &'static Interaction, With<SaveButton>>,
}


#[derive(SystemParam)]
struct SaveParams<'w, 's> {
    state: ResMut<'w, NextState<GameState>>,
    despawn_writer: EventWriter<'w, SaveUiDespawnEvent>,
    page_params: PageParams<'w>,
    stage_name: Query<'w, 's, &'static mut Text, With<StageNameText>>,
    stage_cells: Query<'w, 's, (&'static Transform, &'static Gimmick, &'static PageIndex), (With<Transform>, With<Gimmick>, With<PageIndex>)>,
}


impl<'w, 's> SaveParams<'w, 's> {
    #[inline]
    fn save_stage(&mut self) {
        let stage_name = self.stage_name.single().sections[0].value.clone();
        save_stage(stage_name, &self.page_params, &self.stage_cells);
        self.state.set(GameState::Title);
        self.despawn_writer.send(SaveUiDespawnEvent);
    }
}


#[derive(Copy, Clone, PartialEq)]
enum ClickStatus {
    Canceled,
    Save,
    None,
}


impl<'w, 's> ButtonsParams<'w, 's> {
    fn click_status(&self) -> ClickStatus {
        if self.mouse.just_pressed(MouseButton::Left) {
            if self.save_button.single().pressed() {
                ClickStatus::Save
            } else if self.cancel_button.single().pressed() {
                ClickStatus::Canceled
            } else {
                ClickStatus::None
            }
        } else {
            ClickStatus::None
        }
    }
}


fn despawn_ui_event(
    mut reader: EventReader<SaveUiDespawnEvent>,
    mut commands: Commands,
    root: Query<Entity, With<SaveUiRootNode>>,
) {
    for _ in reader.iter() {
        if let Some(root) = root.iter().next() {
            commands.entity(root).despawn_recursive();
        }
    }
}


fn click_handle(
    mut commands: Commands,
    mut save_params: SaveParams,
    button_params: ButtonsParams,
) {
    match button_params.click_status() {
        ClickStatus::Save => {
            save_params.save_stage();
        }
        ClickStatus::Canceled => {
            save_params.despawn_writer.send(SaveUiDespawnEvent);
            commands.insert_resource(StageEditStatus::Idle);
        }
        ClickStatus::None => {}
    }
}


fn input_key(
    mut keys: EventReader<ReceivedCharacter>,
    mut save_params: SaveParams,
) {
    for key in keys.iter() {
        let mut text = save_params.stage_name.single_mut();
        let section = text.sections.first_mut().unwrap();

        if key.char == '\x08' {
            section.value.pop();
        } else if key.char == '\r' {
            save_params.save_stage();
        } else if !key.char.is_control() && section.value.len() < 10 {
            section.value.push(key.char);
        }
    }
}


fn save_stage(
    stage_name: String,
    page_params: &PageParams,
    stage_cells: &Query<(&Transform, &Gimmick, &PageIndex), (With<Transform>, With<Gimmick>, With<PageIndex>)>,
) {
    let pages = (0..page_params.page_count())
        .map(|page_index| create_page_asset(page_index, stage_cells))
        .collect::<Vec<Page>>();

    let json = StageJson {
        name: stage_name,
        pages,
    };
    StageLoader::new().save(&json).unwrap();
}


fn create_page_asset(
    page_index: usize,
    stage_cells: &Query<(&Transform, &Gimmick, &PageIndex), (With<Transform>, With<Gimmick>, With<PageIndex>)>,
) -> Page {
    let mut cells = Vec::new();

    for (pos, tags) in cells_in_page(page_index, stage_cells) {
        cells.push(StageCell::new(Vec2::new(pos.x as f32, pos.y as f32), tags));
    }

    Page {
        cells
    }
}


fn cells_in_page(
    page_index: usize,
    stage_cells: &Query<(&Transform, &Gimmick, &PageIndex), (With<Transform>, With<Gimmick>, With<PageIndex>)>,
) -> HashMap<I64Vec2, Vec<GimmickTag>> {
    let mut stage = HashMap::<I64Vec2, Vec<GimmickTag>>::new();

    stage_cells
        .iter()
        .filter(|(_, _, idx)| ***idx == page_index)
        .for_each(|(transform, gimmick, _)| {
            let key = transform.translation.truncate().as_i64vec2();
            if let std::collections::hash_map::Entry::Vacant(e) = stage.entry(key) {
                e.insert(vec![gimmick.0]);
            } else {
                stage
                    .get_mut(&key)
                    .unwrap()
                    .push(gimmick.0);
            }
        });

    stage
}