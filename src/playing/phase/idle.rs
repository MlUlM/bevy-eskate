use bevy::app::{App, Plugin, Update};
use bevy::input::Input;
use bevy::prelude::*;

use crate::gama_state::GameState;
use crate::playing::move_direction::MoveDirection;
use crate::playing::phase::PlayingPhase;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct PlayingIdlePlugin;


impl Plugin for PlayingIdlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, input_move
                .run_if(in_state(GameState::Playing).and_then(run_if_idle)),
            );
    }
}


#[inline]
fn run_if_idle(
    phase: Res<PlayingPhase>,
) -> bool
{
    matches!(*phase, PlayingPhase::Idle)
}


fn input_move(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
) {
    let mut emit = |direction: MoveDirection| {
        commands.insert_resource(PlayingPhase::StartMove(direction));
    };

    if keys.just_pressed(KeyCode::Left) {
        emit(MoveDirection::Left);
    } else if keys.just_pressed(KeyCode::Up) {
        emit(MoveDirection::Up);
    } else if keys.just_pressed(KeyCode::Right) {
        emit(MoveDirection::Right);
    } else if keys.just_pressed(KeyCode::Down) {
        emit(MoveDirection::Down);
    }
}


#[cfg(test)]
mod tests {
    use bevy::app::{App, Update};
    use bevy::input::Input;
    use bevy::prelude::KeyCode;

    use crate::playing::move_direction::MoveDirection;
    use crate::playing::phase::idle::input_move;
    use crate::playing::phase::PlayingPhase;
    use crate::playing::tests::new_playing_app;

    #[test]
    fn input_left() {
        let mut app = new_playing_app();
        app.insert_resource(PlayingPhase::Idle);
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
        let phase = app.world.resource::<PlayingPhase>();
        assert_eq!(*phase, PlayingPhase::StartMove(expect));
    }
}