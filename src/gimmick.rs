use bevy::ecs::system::EntityCommands;
use bevy::math::Vec3;
use bevy::prelude::{Component, Transform};
use bevy_tweening::{Animator, EaseMethod, Tween};
use bevy_tweening::lens::TransformPositionLens;

use crate::gimmick::player::Moving;
use crate::playing::start_moving::{MoveDirection, StartMoving};

pub mod floor;
pub mod player;
pub mod rock;


pub const GIMMICK_WIDTH: f32 = 50.;
pub const GIMMICK_HEIGHT: f32 = 50.;
pub const GIMMICK_SIZE: Vec3 = Vec3::new(GIMMICK_WIDTH, GIMMICK_HEIGHT, 0.);


#[derive(Default, Copy, Clone, Component)]
pub struct Gimmick;


#[bevy_trait_query::queryable]
pub trait PlayerControllable {
    fn move_player(
        &self,
        entity: &mut EntityCommands,
        transform: &mut Transform,
        target: &mut Transform,
        direction: &MoveDirection,
    );
}


#[derive(Default, Debug, Copy, Clone, Component)]
pub struct GimmickCollide;


impl PlayerControllable for GimmickCollide {
    fn move_player(
        &self,
        entity: &mut EntityCommands,
        transform: &mut Transform,
        target: &mut Transform,
        direction: &MoveDirection,
    ) {
        let tween = Tween::new(
            EaseMethod::Linear,
            std::time::Duration::from_secs(1),
            TransformPositionLens {
                start: transform.translation,
                end: target.translation + direction.vec3(),
            },
        )
            .with_completed_event(0);

        entity.insert(Animator::new(tween));
        entity.insert(Moving);
        entity.remove::<StartMoving>();
    }
}