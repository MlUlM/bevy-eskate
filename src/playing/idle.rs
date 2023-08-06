use bevy::input::Input;
use bevy::prelude::{Commands, Component, Entity, KeyCode, Query, Res, With};

use crate::playing::start_moving::StartMoving;

#[derive(Default, Component, Copy, Clone, Debug)]
pub struct PlayingIdle;


pub fn update_move_input_handle(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    status: Query<Entity, With<PlayingIdle>>,
) {
    let mut emit = |start_moving: StartMoving| {
        let mut status = commands.entity(status.single());
        status.remove::<PlayingIdle>();
        status.insert(start_moving);
    };

    if keys.pressed(KeyCode::Left) {
        emit(StartMoving::left());
    } else if keys.pressed(KeyCode::Up) {
        emit(StartMoving::up());
    } else if keys.pressed(KeyCode::Right) {
        emit(StartMoving::right());
    } else if keys.pressed(KeyCode::Down) {
        emit(StartMoving::down());
    }
}