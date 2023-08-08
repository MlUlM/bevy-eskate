use std::ops::{AddAssign, Deref, SubAssign};

use bevy::app::{App, Plugin, Update};
use bevy::math::Vec2;
use bevy::prelude::{Camera2dBundle, Commands, Component, Condition, in_state, IntoSystemConfigs, OnEnter, Query, Res, Resource, resource_changed, Visibility, With};
use bevy_trait_query::RegisterExt;

use crate::gama_state::GameState;
use crate::loader::{StageLoadable, StageLoader};
use crate::loader::json::StageCell;
use crate::playing::gimmick::next_page::NextPageCollide;
use crate::playing::gimmick::{MoveToFront, GimmickCollide, GimmickItem, floor, rock, player};
use crate::playing::gimmick::asset::GimmickAssets;
use crate::playing::gimmick::tag::GimmickTag;
use crate::playing::idle::PlayingIdlePlugin;
use crate::playing::phase::PlayingPhase;
use crate::playing::start_move::PlayingStartMovePlugin;
use crate::stage_edit::page_count::PageCount;

mod next_page;
pub mod idle;
pub mod start_move;
mod phase;
pub mod move_direction;
pub mod gimmick;

#[derive(
Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Resource, Component,
)]
pub struct PageIndex(pub usize);


impl AddAssign<usize> for PageIndex {
    fn add_assign(&mut self, rhs: usize) {
        *self = PageIndex::new(self.0 + rhs);
    }
}


impl SubAssign<usize> for PageIndex {
    fn sub_assign(&mut self, rhs: usize) {
        *self = PageIndex::new(self.0 - rhs);
    }
}


impl PageIndex {
    #[inline]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }
}


impl Deref for PageIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


#[derive(Default, Clone)]
pub struct PlayingPlugin;


impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PlayingIdlePlugin)
            .add_plugins(PlayingStartMovePlugin)
            .register_component_as::<dyn GimmickCollide, MoveToFront>()
            .register_component_as::<dyn GimmickCollide, NextPageCollide>()
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                Update,
                change_gimmicks_visible.run_if(in_state(GameState::Playing).and_then(resource_changed::<PageIndex>())),
            );
    }
}


fn change_gimmicks_visible(
    page_idx: Res<PageIndex>,
    mut gimmicks: Query<
        (&mut Visibility, &mut PageIndex, Option<&mut GimmickItem>),
        With<PageIndex>,
    >,
) {
    for (mut visible, mut index, item) in gimmicks.iter_mut() {
        if item.is_some() {
            *index = PageIndex::new(page_idx.0);
        }

        if page_idx.0 == index.0 {
            *visible = Visibility::Visible;
        } else {
            *visible = Visibility::Hidden;
        }
    }
}


fn setup(mut commands: Commands, assets: Res<GimmickAssets>) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(PageIndex::new(0));
    commands.insert_resource(PlayingPhase::Idle);

    let stages = StageLoader::new()
        .load()
        .unwrap();
    let stage = stages.first().unwrap();
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
    use crate::playing::gimmick::asset::GimmickAssets;


    use crate::playing::setup;

    pub(crate) fn new_playing_app() -> App {
        let mut app = App::new();
        app.insert_resource(GimmickAssets::default());

        app
    }


    #[test]
    fn new_app() {
        let mut app = new_playing_app();
        app.add_systems(Startup, setup);

        app.update();
    }
}