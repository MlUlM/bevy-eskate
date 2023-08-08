use bevy::prelude::Resource;

use crate::playing::move_direction::MoveDirection;

pub mod idle;
pub mod start_move;
pub mod next_page;

#[derive(PartialEq, Default, Copy, Clone, Resource, Debug)]
pub enum PlayingPhase {
    #[default]
    Idle,

    StartMove(MoveDirection),

    Moving,

    NextPage,
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
