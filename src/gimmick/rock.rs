use bevy::asset::{AssetServer, Handle};
use bevy::math::Vec2;
use bevy::prelude::{Bundle, Commands, Image};
use bevy::sprite::SpriteBundle;

use crate::gimmick::{create_front_gimmick_sprite_bundle, GimmickCollide};
use crate::playing::PageIndex;

#[derive(Bundle, Clone)]
pub struct RockBundle {
    sprite: SpriteBundle,
    collide: GimmickCollide,
    page_index: PageIndex,
}


impl RockBundle {
    #[inline]
    pub fn new(
        texture: Handle<Image>,
        pos: Vec2,
        page_index: PageIndex,
    ) -> Self {
        Self {
            sprite: create_front_gimmick_sprite_bundle(texture, pos),
            collide: GimmickCollide,
            page_index
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
    let texture = asset_sever.load("gimmick/rock.png");
    commands.spawn(RockBundle::new(texture, pos, page_index));
}