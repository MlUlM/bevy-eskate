use bevy::prelude::Resource;

use crate::stage::playing::move_direction::MoveDirection;
use crate::stage::playing::phase::PlayingPhase;

#[derive(Debug, PartialEq, Resource, Copy, Clone)]
pub enum StageStatus {
    Playing(PlayingPhase),

    Preview,
}


impl StageStatus {
    #[inline(always)]
    pub const fn playing_idle() -> Self {
        Self::Playing(PlayingPhase::Idle)
    }


    #[inline(always)]
    pub fn require_start_move(self) -> MoveDirection {
        if let Self::Playing(phase) = self {
            phase.require_start_move()
        } else {
            panic!("Not start move");
        }
    }


    #[inline(always)]
    pub const fn playing_start_move(move_direction: MoveDirection) -> Self {
        Self::Playing(PlayingPhase::StartMove(move_direction))
    }


    #[inline(always)]
    pub const fn playing_moving() -> Self {
        Self::Playing(PlayingPhase::Moving)
    }


    #[inline(always)]
    pub const fn playing_next_page() -> Self {
        Self::Playing(PlayingPhase::NextPage)
    }


    #[inline(always)]
    pub const fn playing_goaled() -> Self {
        Self::Playing(PlayingPhase::Goaled)
    }
}


impl Default for StageStatus {
    #[inline(always)]
    fn default() -> Self {
        Self::playing_idle()
    }
}

