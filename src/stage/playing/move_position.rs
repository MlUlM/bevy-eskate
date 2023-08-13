use bevy::math::Vec3;
use bevy::prelude::Component;

use crate::stage::playing::move_direction::MoveDirection;

#[bevy_trait_query::queryable]
pub trait MovePosition {
    fn move_pos(
        &self,
        collide: Vec3,
        move_direction: MoveDirection,
    ) -> Vec3;
}


#[derive(Default, Debug, Copy, Clone, Component, Hash)]
pub struct MoveToFront;


impl MovePosition for MoveToFront {
    #[inline]
    fn move_pos(
        &self,
        collide: Vec3,
        move_direction: MoveDirection,
    ) -> Vec3 {
        collide + move_direction.reverse().vec3() + Vec3::new(0., 0., 1.)
    }
}


#[derive(Default, Debug, Copy, Clone, Component, Eq, PartialEq, Hash)]
pub struct MoveUp;

impl MovePosition for MoveUp {
    #[inline]
    fn move_pos(
        &self,
        collide: Vec3,
        _move_direction: MoveDirection,
    ) -> Vec3 {
        collide + Vec3::new(0., 0., 1.)
    }
}
