use bevy::app::{App, Plugin, Update};
use bevy::input::Input;
use bevy::prelude::{Button, Commands, Component, Condition, Entity, Event, EventReader, EventWriter, in_state, Interaction, IntoSystemConfigs, KeyCode, NextState, Query, Res, ResMut, resource_exists_and_equals, UiImage, With};

use crate::assets::gimmick::GimmickAssets;
use crate::cursor::GameCursor;
use crate::extension::InteractionCondition;
use crate::gama_state::GameState;
use crate::mouse_just_pressed_left;
use crate::page::page_param::PageParams;
use crate::stage::playing::gimmick::GimmickItem;
use crate::stage::playing::gimmick::tag::GimmickTag;
use crate::stage_edit::eraser::OnPickedEraser;
use crate::stage_edit::pick::PickedItemsParam;
use crate::stage_edit::StageEditStatus;
use crate::stage_edit::ui::GimmickEraser;

#[derive(Debug, Copy, Clone, Component, Eq, PartialEq)]
pub struct OnPick(pub GimmickTag);


#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
pub struct StageEditIdlePlugin;


#[derive(Debug, Copy, Clone, Eq, PartialEq, Event)]
pub(crate) enum UserInputEvent {
    PickedItem(Entity, GimmickTag),
    PickedEraser,
    SaveStage,
    NextPage,
    PreviousPage,
    Settings,
}


impl Plugin for StageEditIdlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<UserInputEvent>()
            .add_systems(
                Update,
                (
                    input_key_system,
                    picked_gimmick_system.run_if(mouse_just_pressed_left),
                    user_input_event_system
                )
                    .run_if(in_state(GameState::StageEdit).and_then(resource_exists_and_equals(StageEditStatus::Idle))),
            );
    }
}


fn input_key_system(
    mut writer: EventWriter<UserInputEvent>,
    key: Res<Input<KeyCode>>,
) {
    if key.just_pressed(KeyCode::Escape) {
        return writer.send(UserInputEvent::Settings);
    }

    if key.just_pressed(KeyCode::Return) {
        return writer.send(UserInputEvent::SaveStage);
    }

    if key.just_pressed(KeyCode::Left) {
        return writer.send(UserInputEvent::NextPage);
    }

    if key.just_pressed(KeyCode::Right) {
        writer.send(UserInputEvent::PreviousPage);
    }
}


fn picked_gimmick_system(
    mut writer: EventWriter<UserInputEvent>,
    items: Query<(Entity, &Interaction, &GimmickItem), (With<Button>, With<GimmickItem>)>,
    eraser: Query<&Interaction, With<GimmickEraser>>,
) {
    for (entity, interaction, GimmickItem(tag)) in items.iter() {
        if interaction == &Interaction::Pressed {
            return writer.send(UserInputEvent::PickedItem(entity, *tag));
        }
    }

    let Some(eraser) = eraser.iter().next() else { return; };
    if eraser.pressed() {
        writer.send(UserInputEvent::PickedEraser);
    }
}


fn user_input_event_system(
    mut reader: EventReader<UserInputEvent>,
    mut state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    mut page_params: PageParams,
    mut cursor: Query<&mut UiImage, With<GameCursor>>,
    assets: Res<GimmickAssets>,
    picked_item_params: PickedItemsParam,
) {
    let Some(event) = reader.iter().next()  else { return; };

    match event {
        UserInputEvent::PickedItem(entity, gimmick_tag) => {
            picked_item_params.remove_picked(&mut commands);
            cursor.single_mut().texture = gimmick_tag.image(&assets);
            commands
                .entity(*entity)
                .insert(OnPick(*gimmick_tag));
        }
        UserInputEvent::PickedEraser => {
            picked_item_params.remove_picked(&mut commands);
            commands.insert_resource(OnPickedEraser);
            cursor.single_mut().texture = picked_item_params.edit_assets.eraser.clone();
        }
        UserInputEvent::SaveStage => {
            commands.insert_resource(StageEditStatus::SaveStage);
        }
        UserInputEvent::NextPage => {
            page_params.next_page();
        }
        UserInputEvent::PreviousPage => {
            page_params.previous_page();
        }
        UserInputEvent::Settings => {
            // TODO: Show settings panel
            state.set(GameState::BeforeStageEdit);
        }
    }
}


#[cfg(test)]
mod tests {
    use bevy::app::{Startup, Update};

    use crate::page::page_index::PageIndex;
    use crate::stage_edit::{PageCount, setup};
    use crate::stage_edit::idle::{user_input_event_system, UserInputEvent};
    use crate::stage_edit::tests::new_stage_edit_app;

    #[test]
    fn unchanged_next_page_if_last_page() {
        let mut app = new_stage_edit_app(PageCount::new(2));
        app.add_systems(Update, user_input_event_system);
        app.add_systems(Startup, setup);
        app.update();

        app.world.send_event(UserInputEvent::NextPage);

        *app
            .world
            .resource_mut::<PageIndex>()
            = PageIndex::new(2);

        app.update();

        let page_index = app
            .world
            .resource::<PageIndex>();

        assert_eq!(*page_index, PageIndex(2));
    }


    #[test]
    fn increment_page_index() {
        let mut app = new_stage_edit_app(PageCount::new(2));
        app.add_systems(Startup, setup);
        app.update();

        app.add_systems(Update, user_input_event_system);
        app.world.send_event(UserInputEvent::NextPage);
        app.update();

        let page_index = app
            .world
            .resource::<PageIndex>();
        assert_eq!(*page_index, PageIndex(1));
    }
}