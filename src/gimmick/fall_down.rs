use bevy::asset::{AssetServer, Handle};
use bevy::ecs::system::EntityCommands;
use bevy::math::Vec2;
use bevy::prelude::{Bundle, Commands, Component, Image, Transform};
use bevy::sprite::SpriteBundle;

use crate::gimmick::{create_front_gimmick_sprite_bundle, FALL_DOWN_CODE, move_linear, PlayerControllable};
use crate::playing::PageIndex;
use crate::playing::start_moving::MoveDirection;

#[derive(Default, Debug, Copy, Clone, Component)]
pub struct FallDownCollide;


impl PlayerControllable for FallDownCollide {
    fn move_player(&self, controller_entity: &mut EntityCommands, controller_transform: &mut Transform, player_transform: &mut Transform, direction: &MoveDirection) {
        move_linear(
            controller_entity,
            player_transform,
            controller_transform.translation,
            FALL_DOWN_CODE,
        )
    }
}


#[derive(Bundle, Clone)]
pub struct FallDownBundle {
    sprite: SpriteBundle,
    collide: FallDownCollide,
    page_index: PageIndex,
}


impl FallDownBundle {
    #[inline]
    pub fn new(
        texture: Handle<Image>,
        pos: Vec2,
        page_index: PageIndex,
    ) -> Self {
        Self {
            sprite: create_front_gimmick_sprite_bundle(texture, pos),
            collide: FallDownCollide,
            page_index,
        }
    }
}


#[inline]
pub fn spawn(
    commands: &mut Commands,
    asset_sever: &AssetServer,
    pos: Vec2,
    page_index: PageIndex,
) {
    let texture = asset_sever.load("gimmick/fall_down.png");
    commands.spawn(FallDownBundle::new(texture, pos, page_index));
}