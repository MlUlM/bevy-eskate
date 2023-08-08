use bevy::math::Vec3;

use crate::stage::playing::gimmick::GIMMICK_SIZE_VEC3;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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