use bevy::prelude::Resource;

use crate::playing::move_direction::MoveDirection;

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