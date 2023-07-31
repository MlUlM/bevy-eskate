use bevy::input::Input;
use bevy::prelude::{Commands, Component, Entity, KeyCode, Query, Res, With};

use crate::gimmick::player::Movable;
use crate::playing::start_moving::StartMoving;

#[derive(Default, Component, Copy, Clone, Debug)]
pub struct Idle;


pub fn update_move_input_handle(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    players: Query<Entity, (With<Movable>, With<Idle>)>,
) {
    let mut emit = |start_moving: StartMoving| {
        for player in players.iter() {
            let mut entity = commands.entity(player);
            entity.remove::<Idle>();
            entity.insert(start_moving);
        }
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