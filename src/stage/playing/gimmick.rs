use bevy::asset::Handle;
use bevy::ecs::system::EntityCommands;
use bevy::math::{Vec2, Vec3, Vec3Swizzles};
use bevy::prelude::{Component, default, EventWriter, Image, Sprite, SpriteBundle, Transform};
use bevy_tweening::{Animator, EaseMethod, Tween, TweenCompleted};
use bevy_tweening::lens::TransformPositionLens;

use crate::stage::playing::gimmick::tag::GimmickTag;
use crate::stage::playing::move_direction::MoveDirection;

pub mod floor;
pub mod player;
pub mod rock;
pub mod next_page;
pub mod tag;
pub mod goal;
pub mod wall;
pub mod stop;
pub mod ice_box;
pub mod core;
pub mod turn;
pub mod key;
pub mod lock;


pub const GIMMICK_WIDTH: f32 = 32.;
pub const GIMMICK_HEIGHT: f32 = 32.;
// pub const GIMMICK_WIDTH_PX: Val = Val::Px(GIMMICK_WIDTH);
// pub const GIMMICK_HEIGHT_PX: Val = Val::Px(GIMMICK_HEIGHT);
pub const GIMMICK_SIZE_VEC3: Vec3 = Vec3::new(GIMMICK_WIDTH, GIMMICK_HEIGHT, 0.);
pub const GIMMICK_SIZE: Vec2 = Vec2::new(GIMMICK_WIDTH, GIMMICK_HEIGHT);

#[derive(Default, Debug, Hash, Copy, Clone, Component, Eq, PartialEq)]
pub struct Floor;


#[derive(Copy, Clone, Component, Eq, PartialEq, Debug)]
pub struct GimmickItem(pub GimmickTag);


#[derive(Copy, Clone, Component, Eq, PartialEq, Debug)]
pub struct GimmickItemDisabled(pub GimmickTag);


#[derive(Copy, Clone, Component)]
pub struct GimmickItemSpawned(pub GimmickTag);


#[derive(Copy, Clone, Component)]
pub struct Gimmick;


pub(crate) fn move_linear(
    commands: &mut EntityCommands,
    tween_writer: &mut EventWriter<TweenCompleted>,
    player_transform: &mut Transform,
    end: Vec3,
    move_direction: MoveDirection,
) {
    player_transform.rotation = move_direction.quat();
    if player_transform.translation.xy().abs_diff_eq(end.xy(), 0.1) {
        tween_writer.send(TweenCompleted {
            entity: commands.id(),
            user_data: 1,
        });
    } else {
        let start = player_transform.translation;
        let distance = end.distance(start) / 0.3;
        let tween = Tween::new(
            EaseMethod::Linear,
            std::time::Duration::from_millis(distance as u64),
            TransformPositionLens {
                start,
                end,
            },
        )
            .with_completed_event(1);

        commands.insert(Animator::new(tween));
    }
}


#[inline]
pub(crate) fn new_gimmick_sprite_bundle(
    texture: Handle<Image>,
    pos: Vec3,
) -> SpriteBundle {
    create_gimmick_sprite_bundle(texture, pos)
}


pub(crate) fn create_gimmick_sprite_bundle(
    texture: Handle<Image>,
    pos: Vec3,
) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(GIMMICK_WIDTH, GIMMICK_HEIGHT)),
            ..default()
        },
        texture,
        transform: Transform::from_xyz(pos.x, pos.y, pos.z),
        ..default()
    }
}