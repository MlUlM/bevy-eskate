use bevy::app::{App, Plugin, Update};
use bevy::input::Input;
use bevy::math::Vec3;
use bevy::prelude::{Commands, Condition, in_state, IntoSystemConfigs, KeyCode, OnEnter, Res};
use bevy_undo2::prelude::UndoRequester;
use itertools::Itertools;

use crate::assets::gimmick::GimmickAssets;
use crate::gama_state::GameState;
use crate::loader::json::{StageCell, StageJson};
use crate::page::page_count::PageCount;
use crate::page::page_index::PageIndex;
use crate::stage::playing::phase::moving::MoveEvent;
use crate::stage::playing::phase::moving::stop_move::StopMoveEvent;
use crate::stage::playing::PlayingPlugin;
use crate::stage::state::StageState;
use crate::stage::ui::spawn_item_area;

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
            .init_resource::<PageIndex>()
            .init_resource::<PageCount>()
            .add_systems(OnEnter(GameState::Stage), setup)
            .add_systems(Update, undo_if_input_keycode
                .run_if(in_state(GameState::Stage).and_then(in_state(StageState::Idle))),
            );
    }
}


fn setup(
    mut commands: Commands,
    assets: Res<GimmickAssets>,
    stage: Res<StageJson>,
) {
    commands.insert_resource(PageIndex::new(0));
    commands.insert_resource(PageCount::new(stage.pages.len()));

    for (page_index, page) in stage.pages.iter().enumerate() {
        let page_index = PageIndex(page_index);
        spawn_item_area(&mut commands, &assets, page.items.clone(), page_index);

        for stage_cell in page.cells.iter() {
            spawn_gimmick(&mut commands, &assets, stage_cell, page_index);
        }
    }
}


fn spawn_gimmick(
    commands: &mut Commands,
    assets: &GimmickAssets,
    stage_cell: &StageCell,
    page_index: PageIndex,
) {
    for (z, tag) in stage_cell.tags.iter().sorted().enumerate() {
        let pos = Vec3::new(stage_cell.x, stage_cell.y, f32::from(z as u8));
        tag.spawn(commands, assets, pos, page_index);
    }
}


fn undo_if_input_keycode(
    mut requester: UndoRequester,
    keycode: Res<Input<KeyCode>>,
) {
    if keycode.just_pressed(KeyCode::R) {
        requester.undo();
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