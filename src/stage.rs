use bevy::app::{App, Plugin};
use bevy::math::Vec3;
use bevy::prelude::{Commands, OnEnter, OnExit, Res};

use crate::assets::gimmick::GimmickAssets;
use crate::destroy_all;
use crate::gama_state::GameState;
use crate::loader::json::{StageCell, StageJson};
use crate::page::page_count::PageCount;
use crate::page::page_index::PageIndex;
use crate::stage::playing::{gimmick, PlayingPlugin};
use crate::stage::playing::gimmick::{floor, player, rock, stop, wall};
use crate::stage::playing::gimmick::ice_box::IceBoxBundle;
use crate::stage::playing::gimmick::tag::GimmickTag;
use crate::stage::status::StageStatus;
use crate::stage::ui::spawn_item_area;

mod status;
pub mod playing;
mod ui;


#[derive(Default, Clone)]
pub struct StagePlugin;


impl Plugin for StagePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PlayingPlugin)
            .add_systems(OnEnter(GameState::Stage), setup)
            .add_systems(OnExit(GameState::Stage), destroy_all);
    }
}


fn setup(
      mut commands: Commands,
    assets: Res<GimmickAssets>,
    stage: Res<StageJson>,
) {
    commands.insert_resource(PageIndex::new(0));
    commands.insert_resource(StageStatus::default());
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
    for (z, tag) in stage_cell.tags.iter().enumerate() {
        let pos = Vec3::new(stage_cell.x, stage_cell.y, f32::from(z as u8));
        match tag {
            GimmickTag::Floor => {
                floor::spawn(commands, assets, pos, page_index);
            }
            GimmickTag::Wall => {
                wall::spawn(commands, assets, pos, page_index);
            }
            GimmickTag::WallSide => {
                wall::spawn_side(commands, assets, pos, page_index);
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
            GimmickTag::Stop => {
                commands.spawn(stop::StopCollide::new(assets, pos, page_index));
            }
            GimmickTag::IceBox => {
                commands.spawn(IceBoxBundle::new(assets, pos, page_index));
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use bevy::app::{App, Startup};

    use crate::assets::gimmick::GimmickAssets;
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
        assert_eq!(app.world.resource::<PageCount>().0, 0);
        assert_eq!(*app.world.resource::<StageStatus>(), StageStatus::Playing(PlayingPhase::Idle));
    }
}