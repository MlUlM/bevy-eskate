use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Condition, in_state, IntoSystemConfigs, Query, Res, resource_changed, Visibility, With};
use bevy_trait_query::RegisterExt;

use phase::idle::PlayingIdlePlugin;
use phase::start_move::PlayingStartMovePlugin;

use crate::gama_state::GameState;
use crate::page::page_index::PageIndex;
use crate::stage::playing::gimmick::GimmickItemSpawned;
use crate::stage::playing::move_position::{MovePosition, MoveToFront, MoveUp};
use crate::stage::playing::phase::goaled::PlayingGoaledPlugin;
use crate::stage::playing::phase::next_page::PlayingNextPagePlugin;

pub mod phase;
pub mod move_direction;
pub mod gimmick;
pub mod collide;
mod move_position;


#[derive(Default, Clone)]
pub struct PlayingPlugin;


impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                PlayingIdlePlugin,
                PlayingStartMovePlugin,
                PlayingNextPagePlugin,
                PlayingGoaledPlugin
            ))
            .register_component_as::<dyn MovePosition, MoveToFront>()
            .register_component_as::<dyn MovePosition, MoveUp>()
            .add_systems(
                Update,
                change_gimmicks_visible.run_if(in_state(GameState::Stage).and_then(resource_changed::<PageIndex>())),
            );
    }
}


fn change_gimmicks_visible(
    page_idx: Res<PageIndex>,
    mut gimmicks: Query<
        (&mut Visibility, &mut PageIndex, Option<&mut GimmickItemSpawned>),
        With<PageIndex>
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



