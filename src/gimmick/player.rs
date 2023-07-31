use bevy::prelude::*;

use crate::gimmick::Gimmick;
use crate::playing::idle::Idle;

#[derive(Default, Clone, Copy, Component)]
pub struct Movable;


#[derive(Default, Component, Copy, Clone, Debug)]
pub struct Moving;


pub fn spawn(commands: &mut Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(50., 50.)),
                color: Color::WHITE,
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        })
        .insert((Movable, Gimmick, Idle));
}








