use std::f32::consts::PI;

use bevy::math::{Quat, Vec3};
use bevy::prelude::{Resource, Transform};
use bevy_trait_query::imports::Component;

use crate::stage::playing::gimmick::GIMMICK_SIZE_VEC3;
use crate::stage::playing::move_direction::MoveDirection::{Down, Left, Right, Up};

#[derive(Component, Resource, Debug, Copy, Clone, Eq, PartialEq, Hash)]
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


    #[inline]
    pub fn turn(&self, rhs: MoveDirection) -> MoveDirection {
        match self {
            Up | Down => {
                match rhs {
                    Right => Up,
                    Down => Left,
                    Up => Right,
                    Left => Down
                }
            }
            Right | Left => {
                match rhs {
                    Up => Left,
                    Right => Down,
                    Left => Up,
                    Down => Right
                }
            }
        }
    }


    #[inline]
    pub fn quat(&self) -> Quat {
        match self {
            MoveDirection::Up => Quat::from_rotation_z(0.),
            MoveDirection::Left => Quat::from_rotation_z(0.5 * PI),
            MoveDirection::Down => Quat::from_rotation_z(PI),
            MoveDirection::Right => Quat::from_rotation_z(1.5 * PI),
        }
    }


    #[inline]
    pub fn from_angle(angle: f32) -> Self {
        match angle / PI {
            a if (0.0..0.5).contains(&a) => MoveDirection::Up,
            a if (0.5..1.).contains(&a) => MoveDirection::Left,
            a if (1.0..1.5).contains(&a) => MoveDirection::Down,
            _ => MoveDirection::Right,
        }
    }


    #[inline]
    pub fn from_transform(transform: &Transform) -> Self {
        let (_, angle) = transform.rotation.to_axis_angle();
        println!("{:?}", angle);
        Self::from_angle(angle)
    }
}