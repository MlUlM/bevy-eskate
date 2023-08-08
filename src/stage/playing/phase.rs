use bevy::prelude::Resource;

use crate::stage::playing::move_direction::MoveDirection;

pub mod idle;
pub mod start_move;
pub mod next_page;
mod goaled;

#[derive(PartialEq, Default, Copy, Clone, Resource, Debug)]
pub enum PlayingPhase {
    #[default]
    Idle,

    StartMove(MoveDirection),

    Moving,

    NextPage,

    Goaled,
}


impl PlayingPhase {
    pub fn require_start_move(self) -> MoveDirection {
        if let Self::StartMove(dir) = self {
            dir
        } else {
            panic!("Not start move");
        }
    }
}
