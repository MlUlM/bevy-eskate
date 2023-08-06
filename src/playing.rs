use bevy::app::{App, Plugin, Update};
use bevy::math::Vec2;
use bevy::prelude::{any_with_component, AssetServer, Camera2dBundle, Commands, Component, Condition, in_state, IntoSystemConfigs, OnEnter, Query, Res, Resource, resource_changed, Visibility, With};
use bevy_trait_query::RegisterExt;

use crate::gama_state::GameState;
use crate::gimmick;
use crate::gimmick::{floor, GimmickItem, MoveToFront, player, PlayerControllable, rock};
use crate::gimmick::fall_down::FallDownCollide;
use crate::gimmick::tag::GimmickTag;
use crate::loader::{StageLoadable, StageLoader};
use crate::loader::json::StageCell;
use crate::playing::idle::{PlayingIdle, update_move_input_handle};
use crate::playing::start_moving::{StartMoving, update_start_moving};

mod fall_down;
pub mod idle;
pub mod start_moving;

#[derive(
Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Resource, Component,
)]
pub struct PageIndex(pub usize);

impl PageIndex {
    #[inline]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }
}

#[derive(Default, Clone)]
pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.register_component_as::<dyn PlayerControllable, MoveToFront>()
            .register_component_as::<dyn PlayerControllable, FallDownCollide>()
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                Update,
                change_gimmicks_visible.run_if(in_state(GameState::Playing).and_then(resource_changed::<PageIndex>())),
            )
            .add_systems(
                Update,
                update_move_input_handle.run_if(in_state(GameState::Playing).and_then(any_with_component::<PlayingIdle>())),
            )
            .add_systems(
                Update,
                update_start_moving.run_if(in_state(GameState::Playing).and_then(any_with_component::<StartMoving>())),
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


fn setup(mut commands: Commands, asset_sever: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(PlayingIdle);
    commands.insert_resource(PageIndex::new(0));

    let stages = StageLoader::new().load().unwrap();
    let stage = stages.first().unwrap();
    for (page_index, page) in stage.pages.iter().enumerate() {
        let page_index = PageIndex(page_index);
        for stage_cell in page.cells.iter() {
            spawn_gimmick(&mut commands, &asset_sever, stage_cell, page_index);
        }
    }
}


fn spawn_gimmick(
    commands: &mut Commands,
    asset: &AssetServer,
    stage_cell: &StageCell,
    page_index: PageIndex,
) {
    let pos = Vec2::new(stage_cell.x, stage_cell.y);
    for tag in stage_cell.tags.iter() {
        match tag {
            GimmickTag::Floor => {
                floor::spawn(commands, asset, pos, page_index);
            }
            GimmickTag::Rock => {
                rock::spawn(commands, asset, pos, page_index);
            }
            GimmickTag::Player => {
                player::spawn(commands, asset, pos, page_index);
            }
            GimmickTag::FallDown => {
                gimmick::fall_down::spawn(commands, asset, pos, page_index);
            }
            GimmickTag::Goal => {
                gimmick::goal::spawn(commands, asset, pos, page_index)
            }
        }
    }
}