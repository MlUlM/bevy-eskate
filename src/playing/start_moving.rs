use bevy::math::Vec3;
use bevy::prelude::{Commands, Component, Entity, Query, Transform, With, Without};
use bevy_trait_query::One;
use itertools::Itertools;

use crate::gimmick::{GIMMICK_SIZE_VEC3, PlayerControllable};
use crate::gimmick::player::{Movable, Moving};

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
    mut players: Query<(Entity, &mut Transform), With<Movable>>,
    mut controllers: Query<(One<&dyn PlayerControllable>, &mut Transform), Without<Movable>>,
    status: Query<(Entity, &StartMoving), With<StartMoving>>,
) {
    let (status_entity, StartMoving(move_direction)) = status.single();

    for (player, mut player_transform) in players.iter_mut() {
        if let Some((controller, mut controller_transform)) = controllers
            .iter_mut()
            .filter(|(_, transform)| {
                filter_move_direction(&player_transform, transform, move_direction)
            })
            .sorted_by(|(_, prev), (_, next)| {
                distance(&player_transform, prev, move_direction).partial_cmp(&distance(&player_transform, next, move_direction)).unwrap()
            })
            .next()
        {
            let mut status = commands.entity(status_entity);
            status.remove::<StartMoving>();
            status.insert(Moving);

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