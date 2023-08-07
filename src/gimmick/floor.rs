use bevy::asset::{AssetServer, Handle};
use bevy::core::Name;
use bevy::math::Vec2;
use bevy::prelude::{Bundle, Commands, Image};
use bevy::sprite::SpriteBundle;
use crate::gimmick::asset::GimmickAssets;

use crate::gimmick::new_floor_sprite_bundle;
use crate::playing::PageIndex;

#[derive(Bundle, Clone)]
pub struct FloorBundle {
    sprite: SpriteBundle,
    page_index: PageIndex,
    name: Name
}


impl FloorBundle {
    #[inline]
    pub fn new(
        texture: Handle<Image>,
        pos: Vec2,
        page_index: PageIndex,
    ) -> Self {
        Self {
            sprite: new_floor_sprite_bundle(texture, pos),
            page_index,
            name: Name::new("Floor")
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
    commands.spawn(FloorBundle::new(assets.floor.clone(), pos, page_index));
}


#[inline]
pub fn texture(asset: &AssetServer) -> Handle<Image> {
    asset.load("gimmick/floor.png")
}