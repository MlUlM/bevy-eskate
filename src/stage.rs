use bevy::app::{App, Plugin, PreUpdate, Update};
use bevy::input::Input;
use bevy::prelude::{Commands, Condition, Event, EventReader, in_state, IntoSystemConfigs, KeyCode, NextState, OnEnter, OnExit, Query, Res, ResMut, resource_exists_and_changed, With};
use bevy::text::Text;
use bevy_trait_query::imports::Component;
use bevy_undo2::prelude::{AppUndoEx, UndoRequester};

use crate::assets::font::FontAssets;
use crate::assets::gimmick::GimmickAssets;
use crate::destroy_all;
use crate::gama_state::GameState;
use crate::loader::json::StageJson;
use crate::page::page_count::PageCount;
use crate::page::page_index::PageIndex;
use crate::stage::playing::phase::idle::UndoPlayerIdleEvent;
use crate::stage::playing::phase::moving::key::KeyCounter;
use crate::stage::playing::phase::moving::MoveEvent;
use crate::stage::playing::phase::moving::stop_move::StopMoveEvent;
use crate::stage::playing::PlayingPlugin;
use crate::stage::state::StageState;
use crate::stage_edit::page::spawn_page;

mod state;
pub mod playing;
mod ui;


#[derive(Default, Clone)]
pub struct StagePlugin;


impl Plugin for StagePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PlayingPlugin)
            .add_state::<StageState>()
            .add_event::<MoveEvent>()
            .add_event::<StopMoveEvent>()
            .add_undo_event::<UndoPlayerIdleEvent>()
            .init_resource::<PageIndex>()
            .init_resource::<PageCount>()
            .add_systems(OnEnter(GameState::Stage), setup)
            .add_systems(OnExit(GameState::Stage), (
                destroy_all,
                reset_stage_state,
            ))
            .add_systems(Update, (
                undo_if_input_keycode
            ).run_if(in_state(GameState::Stage).and_then(in_state(StageState::Idle))), )
            .add_systems(PreUpdate, (
                undo_player_idle_event_system
            ).run_if(in_state(GameState::Stage)))
            .add_systems(Update, (
                change_keys_count_system
            ).run_if(in_state(GameState::Stage).and_then(resource_exists_and_changed::<KeyCounter>())));
    }
}


#[derive(Component)]
struct KeysCountText;


fn setup(
    mut commands: Commands,
    assets: Res<GimmickAssets>,
    stage: Res<StageJson>,
    fonts: Res<FontAssets>,
) {
    commands.insert_resource(PageIndex::new(0));
    commands.insert_resource(PageCount::new(stage.pages.len()));

    for (page_index, page) in stage.pages.iter().enumerate() {
        let page_index = PageIndex(page_index);
        spawn_page(&mut commands, page, page_index, &assets);
    }
    
    ui::spawn_ui(&mut commands, &fonts);
}


fn change_keys_count_system(
    mut key_count_text: Query<&mut Text, With<KeysCountText>>,
    key_count: Res<KeyCounter>,
) {
    let mut text = key_count_text.single_mut();
    text.sections[0].value = format!("Key: {}", **key_count);
}


fn reset_stage_state(
    mut state: ResMut<NextState<StageState>>
) {
    state.set(StageState::Idle);
}


fn undo_if_input_keycode(
    mut requester: UndoRequester,
    keycode: Res<Input<KeyCode>>,
) {
    if keycode.just_pressed(KeyCode::R) {
        requester.undo();
    }
}


fn undo_player_idle_event_system(
    mut state: ResMut<NextState<StageState>>,
    mut er: EventReader<UndoPlayerIdleEvent>,
) {
    if er.iter().next().is_some() {
        state.set(StageState::Idle);
    }
}


#[cfg(test)]
mod tests {
    use bevy::app::{App, Startup};
    use bevy_undo2::UndoPlugin;

    use crate::assets::gimmick::GimmickAssets;
    use crate::loader::{StageLoadable, StageLoader};
    use crate::page::page_count::PageCount;
    use crate::page::page_index::PageIndex;
    use crate::stage::playing::phase::start_move::StartMoveEvent;
    use crate::stage::setup;
    use crate::stage::state::StageState;

    pub(crate) fn new_playing_app() -> App {
        let mut app = App::new();
        app.add_state::<StageState>();
        app.add_plugins(UndoPlugin);
        app.add_event::<StartMoveEvent>();
        app.insert_resource(GimmickAssets::default());
        let stages = StageLoader::new().load().unwrap();
        let stage = stages
            .iter()
            .find(|stage| {
                stage.name == "test"
            })
            .unwrap();

        app.insert_resource(stage.clone());

        app
    }


    #[test]
    fn setup_resources() {
        let mut app = new_playing_app();
        app.add_systems(Startup, setup);

        app.update();

        assert_eq!(app.world.resource::<PageIndex>().0, 0);
        assert_eq!(app.world.resource::<PageCount>().0, 0);
    }
}