use bevy::app::{App, Plugin};
use bevy_trait_query::RegisterExt;

use phase::idle::PlayingIdlePlugin;

use crate::stage::playing::move_position::{MovePosition, MoveToFront, MoveUp};
use crate::stage::playing::phase::moving::PlayingMovingPlugin;
use crate::stage::playing::phase::next_page::PlayingNextPagePlugin;
use crate::stage::playing::phase::picked_item::PlayingPickedItemPlugin;
use crate::stage::playing::phase::start_move::PlayingStartMovePlugin;

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
                PlayingMovingPlugin,
                PlayingNextPagePlugin,
                PlayingPickedItemPlugin
            ))
            .register_component_as::<dyn MovePosition, MoveToFront>()
            .register_component_as::<dyn MovePosition, MoveUp>();
    }
}






