use bevy::math::Vec3;
use bevy::prelude::{Commands, Component, Entity, EventReader, Query, Transform, With, Without};
use bevy_trait_query::One;
use bevy_tweening::TweenCompleted;
use itertools::Itertools;

use crate::gimmick::{GIMMICK_SIZE_VEC3, PlayerControllable};
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
            MoveDirection::Down => MoveDirection::Up,
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
        self.vec3_unit() * GIMMICK_SIZE_VEC3
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
    for (player, mut player_transform, StartMoving(move_direction)) in players.iter_mut() {
        if let Some((controller, mut controller_transform)) = controllers
            .iter_mut()
            .filter(|(_, transform)| {
                filter_move_direction(&player_transform, &transform, &move_direction)
            })
            .sorted_by(|(_, prev), (_, next)| {
                distance(&player_transform, &prev, &move_direction).partial_cmp(&distance(&player_transform, &next, &move_direction)).unwrap()
            })
            .next()
        {
            controller.move_player(
                &mut commands.get_entity(player).unwrap(),
                &mut controller_transform,
                &mut player_transform,
                move_direction,
            );
        }
    }
}

fn filter_move_direction(
    player_transform: &Transform,
    controller_transform: &Transform,
    direction: &MoveDirection,
) -> bool {
    match direction {
        MoveDirection::Left => controller_transform.translation.x < player_transform.translation.x && controller_transform.translation.y == player_transform.translation.y,
        MoveDirection::Right => player_transform.translation.x < controller_transform.translation.x && controller_transform.translation.y == player_transform.translation.y,
        MoveDirection::Up => player_transform.translation.y < controller_transform.translation.y && controller_transform.translation.x == player_transform.translation.x,
        MoveDirection::Down => controller_transform.translation.y < player_transform.translation.y && controller_transform.translation.x == player_transform.translation.x,
    }
}


fn distance(
    player_transform: &Transform,
    controller_transform: &Transform,
    direction: &MoveDirection,
) -> f32 {
    match direction {
        MoveDirection::Left | MoveDirection::Right => {
            (controller_transform.translation.x - player_transform.translation.x).abs()
        }
        MoveDirection::Up | MoveDirection::Down => {
            (player_transform.translation.y - controller_transform.translation.y).abs()
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
