use bevy::asset::Handle;
use bevy::core::Name;
use bevy::math::Vec2;
use bevy::prelude::{Bundle, Commands, Image};
use bevy::sprite::SpriteBundle;

use crate::assets::gimmick::GimmickAssets;
use crate::page::page_index::PageIndex;
use crate::stage::playing::gimmick::new_floor_sprite_bundle;

#[derive(Bundle, Clone)]
pub struct FloorBundle {
    sprite: SpriteBundle,
    page_index: PageIndex,
    name: Name,
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
            name: Name::new("Floor"),
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

