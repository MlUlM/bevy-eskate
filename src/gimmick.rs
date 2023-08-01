use bevy::asset::Handle;
use bevy::ecs::system::EntityCommands;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Component, default, Image, Sprite, SpriteBundle, Transform};
use bevy_tweening::{Animator, EaseMethod, Tween};
use bevy_tweening::lens::TransformPositionLens;

use crate::playing::start_moving::{MoveDirection, StartMoving};

pub mod floor;
pub mod player;
pub mod rock;
pub mod fall_down;


pub const GIMMICK_WIDTH: f32 = 50.;
pub const GIMMICK_HEIGHT: f32 = 50.;
pub const GIMMICK_SIZE_VEC3: Vec3 = Vec3::new(GIMMICK_WIDTH, GIMMICK_HEIGHT, 0.);
pub const GIMMICK_SIZE: Vec2 = Vec2::new(GIMMICK_WIDTH, GIMMICK_HEIGHT);


pub const FALL_DOWN_CODE: u64 = 1;



#[derive(Default, Copy, Clone, Component)]
pub struct GimmickItem;


#[derive(Default, Copy, Clone, Component)]
pub struct Gimmick;


#[bevy_trait_query::queryable]
pub trait PlayerControllable {
    fn move_player(
        &self,
        controller_entity: &mut EntityCommands,
        controller_transform: &mut Transform,
        player_transform: &mut Transform,
        direction: &MoveDirection,
    );
}


#[derive(Default, Debug, Copy, Clone, Component)]
pub struct GimmickCollide;


impl PlayerControllable for GimmickCollide {
    #[inline]
    fn move_player(
        &self,
        controller_entity: &mut EntityCommands,
        controller_transform: &mut Transform,
        player_transform: &mut Transform,
        direction: &MoveDirection,
    ) {
        move_linear(
            controller_entity,
            player_transform,
            controller_transform.translation + direction.reverse().vec3() + Vec3::new(0., 0., 1.),
            0,
        );
    }
}


pub(crate) fn move_linear(
    controller_entity: &mut EntityCommands,
    player_transform: &mut Transform,
    end: Vec3,
    complete_code: u64,
) {
    let tween = Tween::new(
        EaseMethod::Linear,
        std::time::Duration::from_secs(1),
        TransformPositionLens {
            start: player_transform.translation,
            end,
        },
    )
        .with_completed_event(complete_code);

    controller_entity.insert(Animator::new(tween));
    controller_entity.remove::<StartMoving>();
}


#[inline]
pub(crate) fn create_front_gimmick_sprite_bundle(
    texture: Handle<Image>,
    pos: Vec2,
) -> SpriteBundle {
    create_gimmick_sprite_bundle(texture, Vec3::new(pos.x, pos.y, 1.))
}


#[inline]
pub(crate) fn create_floor_sprite_bundle(
    texture: Handle<Image>,
    pos: Vec2,
) -> SpriteBundle {
    create_gimmick_sprite_bundle(texture, Vec3::new(pos.x, pos.y, 0.))
}


pub(crate) fn create_gimmick_sprite_bundle(
    texture: Handle<Image>,
    pos: Vec3,
) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(50., 50.)),
            ..default()
        },
        texture,
        transform: Transform::from_xyz(pos.x, pos.y, pos.z),
        ..default()
    }
}