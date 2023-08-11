use bevy::app::{App, Plugin, Update};
use bevy::input::Input;
use bevy::prelude::{Button, Commands, Component, Condition, Entity, In, in_state, Interaction, IntoSystem, IntoSystemConfigs, KeyCode, NextState, Query, Res, ResMut, resource_exists_and_equals, UiImage, With};

use crate::assets::gimmick::GimmickAssets;
use crate::cursor::GameCursor;
use crate::gama_state::GameState;
use crate::page::page_param::PageParams;
use crate::stage::playing::gimmick::GimmickItem;
use crate::stage::playing::gimmick::tag::GimmickTag;
use crate::stage_edit::pick::PickedItemsParam;
use crate::stage_edit::StageEditStatus;

#[derive(Debug, Copy, Clone, Component, Eq, PartialEq)]
pub struct OnPick(pub GimmickTag);


#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
pub struct StageEditIdlePlugin;


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum InputStatus {
    None,
    PickedItem(Entity, GimmickTag),
    SaveStage,
    NextPage,
    PreviousPage,
    Settings,
}


impl Plugin for StageEditIdlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update,
                         user_input
                             .pipe(input_handle)
                             .run_if(in_state(GameState::StageEdit).and_then(resource_exists_and_equals(StageEditStatus::Idle))),
            );
    }
}


fn user_input(
    key: Res<Input<KeyCode>>,
    items: Query<(Entity, &Interaction, &GimmickItem), (With<Button>, With<GimmickItem>)>,
) -> InputStatus {
    if key.just_pressed(KeyCode::Escape) {
        return InputStatus::Settings;
    }

    if key.just_pressed(KeyCode::Return) {
        return InputStatus::SaveStage;
    }

    if key.just_pressed(KeyCode::Left) {
        return InputStatus::NextPage;
    }

    if key.just_pressed(KeyCode::Right) {
        return InputStatus::PreviousPage;
    }

    for (entity, interaction, GimmickItem(tag)) in items.iter() {
        if interaction == &Interaction::Pressed {
            return InputStatus::PickedItem(entity, *tag);
        }
    }

    InputStatus::None
}


fn input_handle(
    In(input_status): In<InputStatus>,
    mut state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    mut page_params: PageParams,
    mut cursor: Query<&mut UiImage, With<GameCursor>>,
    assets: Res<GimmickAssets>,
    picked_item_params: PickedItemsParam,
) {
    match input_status {
        InputStatus::PickedItem(entity, gimmick_tag) => {
            picked_item_params.remove_picked(&mut commands);
            cursor.single_mut().texture = gimmick_tag.image(&assets);
            commands
                .entity(entity)
                .insert(OnPick(gimmick_tag));
        }
        InputStatus::SaveStage => {
            commands.insert_resource(StageEditStatus::SaveStage);
        }
        InputStatus::NextPage => {
            page_params.next_page();
        }
        InputStatus::PreviousPage => {
            page_params.previous_page();
        }
        InputStatus::Settings => {
            // TODO: Show settings panel
            state.set(GameState::BeforeStageEdit);
        }
        _ => {}
    }
}


#[cfg(test)]
mod tests {
    use bevy::app::Update;
    use bevy::prelude::IntoSystem;

    use crate::page::page_index::PageIndex;
    use crate::stage_edit::idle::{input_handle, InputStatus};
    use crate::stage_edit::PageCount;
    use crate::stage_edit::tests::new_stage_edit_app;

    fn update_next_page() -> InputStatus {
        InputStatus::NextPage
    }

    #[test]
    fn unchanged_next_page_if_last_page() {
        let mut app = new_stage_edit_app(PageCount::new(2));
        app.add_systems(Update, update_next_page.pipe(input_handle));
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
        app.add_systems(Update, update_next_page.pipe(input_handle));

        app.update();

        let page_index = app
            .world
            .resource::<PageIndex>();
        assert_eq!(*page_index, PageIndex(1));
    }
}