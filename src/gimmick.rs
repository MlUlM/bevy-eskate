use bevy::asset::Handle;
use bevy::ecs::system::EntityCommands;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Component, default, Image, Sprite, SpriteBundle, Transform};
use bevy::ui::Val;
use bevy_tweening::{Animator, EaseMethod, Tween};
use bevy_tweening::lens::TransformPositionLens;
use bevy_undo::prelude::TweenOnUndoExt;

use crate::gimmick::player::Moving;
use crate::gimmick::tag::GimmickTag;
use crate::playing::idle::PlayingIdle;
use crate::playing::start_moving::MoveDirection;

pub mod floor;
pub mod player;
pub mod rock;
pub mod next_page;
pub mod tag;
pub mod goal;
pub mod asset;


pub const GIMMICK_WIDTH: f32 = 50.;
pub const GIMMICK_HEIGHT: f32 = 50.;
pub const GIMMICK_WIDTH_PX: Val = Val::Px(GIMMICK_WIDTH);
pub const GIMMICK_HEIGHT_PX: Val = Val::Px(GIMMICK_HEIGHT);
pub const GIMMICK_SIZE_VEC3: Vec3 = Vec3::new(GIMMICK_WIDTH, GIMMICK_HEIGHT, 0.);
pub const GIMMICK_SIZE: Vec2 = Vec2::new(GIMMICK_WIDTH, GIMMICK_HEIGHT);


pub const FALL_DOWN_CODE: u64 = 1;


#[derive(Default, Debug, Hash, Copy, Clone, Component)]
pub struct Stage;

#[derive(Default, Debug, Hash, Copy, Clone, Component)]
pub struct Floor;

#[derive(Copy, Clone, Component)]
pub struct GimmickItem(pub GimmickTag);


#[derive(Copy, Clone, Component)]
pub struct GimmickItemSpawned(pub GimmickTag);


#[derive(Copy, Clone, Component)]
pub struct Gimmick(pub GimmickTag);


#[bevy_trait_query::queryable]
pub trait PlayerControllable {
    fn move_player(
        &self,
        collide_cmd: &mut EntityCommands,
        collide_transform: &mut Transform,
        player_transform: &mut Transform,
        direction: &MoveDirection,
    );
}


#[derive(Default, Debug, Copy, Clone, Component)]
pub struct MoveToFront;


impl PlayerControllable for MoveToFront {
    #[inline]
    fn move_player(
        &self,
        commands: &mut EntityCommands,
        collide_transform: &mut Transform,
        player_transform: &mut Transform,
        direction: &MoveDirection,
    ) {
        move_linear(
            commands,
            player_transform,
            collide_transform.translation + direction.reverse().vec3(),
            |enty_cmd| {
                enty_cmd.remove::<Moving>();
                enty_cmd
                    .commands()
                    .spawn(PlayingIdle);
            },
        )
    }
}


pub(crate) fn move_linear(
    commands: &mut EntityCommands,
    player_transform: &mut Transform,
    end: Vec3,
    on_completed: impl Fn(&mut EntityCommands) + Send + Sync + 'static + Clone,
) {
    let tween = Tween::new(
        EaseMethod::Linear,
        std::time::Duration::from_secs(1),
        TransformPositionLens {
            start: player_transform.translation,
            end,
        },
    )
        .spawn_processing_and_remove_completed(commands.commands())
        .with_completed_entity_commands(commands.commands(), on_completed);

    commands.insert(Animator::new(tween));
}


#[inline]
pub(crate) fn new_gimmick_sprite_bundle(
    texture: Handle<Image>,
    pos: Vec2,
) -> SpriteBundle {
    create_gimmick_sprite_bundle(texture, Vec3::new(pos.x, pos.y, 1.))
}


#[inline]
pub(crate) fn new_floor_sprite_bundle(
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