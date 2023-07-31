use bevy::math::Vec3;
use bevy::prelude::{Commands, Component, Entity, EventReader, Query, Transform, With, Without};
use bevy_trait_query::One;
use bevy_tweening::TweenCompleted;

use crate::gimmick::{GIMMICK_SIZE, PlayerControllable};
use crate::gimmick::player::{Movable, Moving};
use crate::playing::idle::Idle;

#[derive(Debug, Copy, Clone)]
pub enum MoveDirection {
    Left,
    Right,
    Up,
    Down,
}


impl MoveDirection {
    #[inline]
    pub fn reverse(&self) -> Self {
        match self {
            MoveDirection::Left => MoveDirection::Right,
            MoveDirection::Right => MoveDirection::Left,
            MoveDirection::Up => MoveDirection::Down,
            MoveDirection::Down => MoveDirection::Up
        }
    }


    #[inline]
    pub fn vec3_unit(&self) -> Vec3 {
        match self {
            MoveDirection::Left => Vec3::new(-1., 0., 0.),
            MoveDirection::Right => Vec3::new(1., 0., 0.),
            MoveDirection::Up => Vec3::new(0., 1., 0.),
            MoveDirection::Down => Vec3::new(0., -1., 0.),
        }
    }


    #[inline]
    pub fn vec3(&self) -> Vec3 {
        self.vec3_unit() * GIMMICK_SIZE
    }
}


#[derive(Component, Copy, Clone, Debug)]
pub struct StartMoving(pub MoveDirection);


impl StartMoving {
    #[inline]
    pub const fn left() -> Self {
        Self(MoveDirection::Left)
    }

    #[inline]
    pub const fn up() -> Self {
        Self(MoveDirection::Up)
    }


    #[inline]
    pub const fn right() -> Self {
        Self(MoveDirection::Right)
    }


    #[inline]
    pub const fn down() -> Self {
        Self(MoveDirection::Down)
    }
}


pub fn update_start_moving(
    mut commands: Commands,
    mut players: Query<(Entity, &mut Transform, &StartMoving), (With<Movable>, With<StartMoving>)>,
    mut controllers: Query<(One<&dyn PlayerControllable>, &mut Transform), Without<Movable>>,
) {
    for (player, mut target, StartMoving(move_direction)) in players.iter_mut() {
        for (controller, mut transform) in controllers.iter_mut() {
            controller.move_player(
                &mut commands.get_entity(player).unwrap(),
                &mut transform,
                &mut target,
                move_direction,
            );
        }
    }
}


pub fn on_move_completed(
    mut commands: Commands,
    reader: EventReader<TweenCompleted>,
    players: Query<Entity, With<Moving>>,
) {
    if !reader.is_empty() {
        for player in players.iter() {
            let mut entity = commands.entity(player);
            entity.remove::<Moving>();
            entity.insert(Idle);
        }
    }
}