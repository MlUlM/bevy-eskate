use bevy::asset::{AssetServer, Handle};
use bevy::math::Vec2;
use bevy::prelude::{Bundle, Commands, Image};
use bevy::sprite::SpriteBundle;

use crate::gimmick::create_floor_sprite_bundle;
use crate::playing::PageIndex;

#[derive(Bundle, Clone)]
pub struct FloorBundle {
    sprite: SpriteBundle,
    page_index: PageIndex,
}


impl FloorBundle {
    #[inline]
    pub fn new(
        texture: Handle<Image>,
        pos: Vec2,
        page_index: PageIndex,
    ) -> Self {
        Self {
            sprite: create_floor_sprite_bundle(texture, pos),
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
    let texture = asset_sever.load("gimmick/floor.png");
    commands.spawn(FloorBundle::new(texture, pos, page_index));
}