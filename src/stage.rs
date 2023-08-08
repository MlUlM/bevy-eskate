use bevy::app::{App, Plugin};
use bevy::math::Vec2;
use bevy::prelude::{Camera2dBundle, Commands, OnEnter, Res};

use crate::gama_state::GameState;
use crate::gimmick_assets::GimmickAssets;
use crate::loader::json::{StageCell, StageJson};
use crate::page::page_count::PageCount;
use crate::page::page_index::PageIndex;
use crate::stage::playing::{gimmick, PlayingPlugin};
use crate::stage::playing::gimmick::{floor, player, rock};
use crate::stage::playing::gimmick::tag::GimmickTag;
use crate::stage::status::StageStatus;

mod status;
pub mod playing;


#[derive(Default, Clone)]
pub struct StagePlugin;


impl Plugin for StagePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PlayingPlugin)
            .add_systems(OnEnter(GameState::Stage), setup);
    }
}


fn setup(
    assets: Res<GimmickAssets>,
    stage: Res<StageJson>,
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(PageIndex::new(0));
    commands.insert_resource(StageStatus::default());
    commands.insert_resource(PageCount::new(stage.pages.len()));

    for (page_index, page) in stage.pages.iter().enumerate() {
        let page_index = PageIndex(page_index);
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
    let pos = Vec2::new(stage_cell.x, stage_cell.y);
    for tag in stage_cell.tags.iter() {
        match tag {
            GimmickTag::Floor => {
                floor::spawn(commands, assets, pos, page_index);
            }
            GimmickTag::Rock => {
                rock::spawn(commands, assets, pos, page_index);
            }
            GimmickTag::Player => {
                player::spawn(commands, assets, pos, page_index);
            }
            GimmickTag::NextPage => {
                gimmick::next_page::spawn(commands, assets, pos, page_index);
            }
            GimmickTag::Goal => {
                gimmick::goal::spawn(commands, assets, pos, page_index)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use bevy::app::{App, Startup};

    use crate::gimmick_assets::GimmickAssets;
    use crate::loader::{StageLoadable, StageLoader};
    use crate::page::page_count::PageCount;
    use crate::page::page_index::PageIndex;
    use crate::stage::playing::phase::PlayingPhase;
    use crate::stage::setup;
    use crate::stage::status::StageStatus;

    pub(crate) fn new_playing_app() -> App {
        let mut app = App::new();
        app.insert_resource(GimmickAssets::default());
        let stages = StageLoader::new().load().unwrap();
        let stage = stages.first().unwrap();
        app.insert_resource(stage.clone());

        app
    }


    #[test]
    fn setup_resources() {
        let mut app = new_playing_app();
        app.add_systems(Startup, setup);

        app.update();

        assert_eq!(app.world.resource::<PageIndex>().0, 0);
        assert_eq!(app.world.resource::<PageCount>().0, 2);
        assert_eq!(*app.world.resource::<StageStatus>(), StageStatus::Playing(PlayingPhase::Idle));
    }
}