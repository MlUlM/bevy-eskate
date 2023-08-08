use bevy::ecs::system::EntityCommands;
use bevy::math::Vec2;
use bevy::prelude::{Bundle, Commands, Component, Image, Transform};
use bevy::sprite::SpriteBundle;

use crate::gimmick::{move_linear, new_gimmick_sprite_bundle, PlayerControllable};
use crate::gimmick::asset::GimmickAssets;
use crate::playing::PageIndex;
use crate::playing::start_move::MoveDirection;

#[derive(Default, Debug, Copy, Clone, Component)]
pub struct NextPageProcessing;


#[derive(Default, Debug, Copy, Clone, Component)]
pub struct NextPageCollide;


impl PlayerControllable for NextPageCollide {
    fn move_player(
        &self,
        collide_cmd: &mut EntityCommands,
        collide_transform: &mut Transform,
        player_transform: &mut Transform,
        _direction: &MoveDirection,
    ) {
        move_linear(
            collide_cmd,
            player_transform,
            collide_transform.translation,
            |commands| {
                commands.insert(NextPageProcessing);
            },
        )
    }
}


#[derive(Bundle, Clone)]
pub struct NextPageBundle {
    sprite: SpriteBundle,
    collide: NextPageCollide,
    page_index: PageIndex,
}


impl NextPageBundle {
    #[inline]
    pub fn new(
        texture: Handle<Image>,
        pos: Vec2,
        page_index: PageIndex,
    ) -> Self {
        Self {
            sprite: new_gimmick_sprite_bundle(texture, pos),
            collide: NextPageCollide,
            page_index,
        }
    }
}


#[inline]
pub fn spawn(
    commands: &mut Commands,
    assets: &GimmickAssets,
    pos: Vec2,
    page_index: PageIndex,
) {
    commands.spawn(NextPageBundle::new(assets.next_page.clone(), pos, page_index));
}