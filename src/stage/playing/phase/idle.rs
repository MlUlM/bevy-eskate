use bevy::app::{App, Plugin, Update};
use bevy::input::Input;
use bevy::prelude::*;

use crate::gama_state::GameState;
use crate::stage::playing::move_direction::MoveDirection;
use crate::stage::playing::phase::PlayingPhase;
use crate::stage::status::StageStatus;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct PlayingIdlePlugin;


impl Plugin for PlayingIdlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, input_move
                .run_if(in_state(GameState::Stage).and_then(run_if_idle)),
            );
    }
}


#[inline]
fn run_if_idle(
    phase: Res<StageStatus>,
) -> bool
{
    matches!(*phase, StageStatus::Playing(PlayingPhase::Idle))
}


fn input_move(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
) {
    let mut emit = |direction: MoveDirection| {
        commands.insert_resource(StageStatus::playing_start_move(direction));
    };

    if keys.any_just_pressed([KeyCode::Left, KeyCode::A])  {
        emit(MoveDirection::Left);
    } else if keys.any_just_pressed([KeyCode::Up, KeyCode::W]) {
        emit(MoveDirection::Up);
    } else if keys.any_just_pressed([KeyCode::Right, KeyCode::D]) {
        emit(MoveDirection::Right);
    } else if keys.any_just_pressed([KeyCode::Down, KeyCode::S]) {
        emit(MoveDirection::Down);
    }
}


#[cfg(test)]
mod tests {
    use bevy::app::{App, Update};
    use bevy::input::Input;
    use bevy::prelude::KeyCode;

    use crate::stage::playing::move_direction::MoveDirection;
    use crate::stage::playing::phase::idle::input_move;
    use crate::stage::playing::phase::PlayingPhase;
    use crate::stage::status::StageStatus;
    use crate::stage::tests::new_playing_app;

    #[test]
    fn input_left() {
        let mut app = new_playing_app();
        app.add_systems(Update, input_move);

        input(&mut app, KeyCode::Left, MoveDirection::Left);
        input(&mut app, KeyCode::Up, MoveDirection::Up);
        input(&mut app, KeyCode::Right, MoveDirection::Right);
        input(&mut app, KeyCode::Down, MoveDirection::Down);
    }


    fn input(app: &mut App, key: KeyCode, expect: MoveDirection) {
        let mut input = Input::<KeyCode>::default();
        input.press(key);
        app.insert_resource(input);

        app.update();
        let phase = app.world.resource::<StageStatus>();
        assert_eq!(*phase, StageStatus::Playing(PlayingPhase::StartMove(expect)));
    }
}