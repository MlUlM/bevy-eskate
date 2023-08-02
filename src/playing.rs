use bevy::app::{App, Plugin, Update};
use bevy::math::Vec2;
use bevy::prelude::{any_with_component, AssetServer, Camera2dBundle, Commands, Component, in_state, IntoSystemConfigs, OnEnter, Query, Res, Resource, resource_changed, Visibility, With};
use bevy_trait_query::RegisterExt;

use crate::gama_state::GameState;
use crate::gimmick::{floor, GimmickCollide, GimmickItem, player, PlayerControllable, rock};
use crate::gimmick::fall_down::FallDownCollide;
use crate::gimmick::player::Moving;
use crate::playing::idle::{Idle, update_move_input_handle};
use crate::playing::start_moving::{on_move_completed, StartMoving, update_start_moving};

pub mod idle;
pub mod start_moving;
mod fall_down;

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Resource, Component)]
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
        app
            .register_component_as::<dyn PlayerControllable, GimmickCollide>()
            .register_component_as::<dyn PlayerControllable, FallDownCollide>()
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(Update, page
                .run_if(resource_changed::<PageIndex>())
                .run_if(in_state(GameState::Playing)),
            )
            .add_systems(Update, update_move_input_handle
                .run_if(any_with_component::<Idle>())
                .run_if(in_state(GameState::Playing)),
            )
            .add_systems(Update, update_start_moving
                .run_if(any_with_component::<StartMoving>())
                .run_if(in_state(GameState::Playing)),
            )
            .add_systems(Update, on_move_completed
                .run_if(any_with_component::<Moving>())
                .run_if(in_state(GameState::Playing)),
            );
    }
}


fn setup(
    mut commands: Commands,
    asset_sever: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Idle);
    commands.insert_resource(PageIndex::new(0));

    for x in 0..=24u8 {
        for y in 0..=14u8 {
            if x == 0 || y == 0 || x == 24 || y == 14 {
                let x = f32::from(x) * 50. - 12. * 50.;
                let y = f32::from(y) * 50. - 7. * 50.;

                rock::spawn(&mut commands, &asset_sever, Vec2::new(x, y), PageIndex::new(0));
            }
            let x = f32::from(x) * 50. - 12. * 50.;
            let y = f32::from(y) * 50. - 7. * 50.;

            floor::spawn(&mut commands, &asset_sever, Vec2::new(x, y), PageIndex::new(0));
        }
    }

    player::spawn(&mut commands);
}


fn page(
    page_idx: Res<PageIndex>,
    mut gimmicks: Query<(&mut Visibility, &mut PageIndex, Option<&mut GimmickItem>), With<PageIndex>>,
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


