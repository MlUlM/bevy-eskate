use bevy::math::Vec3;
use bevy::prelude::Bundle;

use crate::assets::gimmick::GimmickAssets;
use crate::button::{SpriteButton, SpriteInteraction};
use crate::page::page_index::PageIndex;
use crate::stage::playing::gimmick::core::GimmickCoreBundle;
use crate::stage::playing::gimmick::Floor;

#[derive(Bundle, Clone)]
pub struct FloorBundle {
    core: GimmickCoreBundle,
    floor: Floor,
    sprite_button: SpriteButton,
    sprite_interaction: SpriteInteraction
}


impl FloorBundle {
    #[inline]
    pub fn new(
        assets: &GimmickAssets,
        pos: Vec3,
        page_index: PageIndex,
    ) -> Self {
        Self {
            core: GimmickCoreBundle::new("Floor", assets.floor.clone(), pos, page_index),
            floor: Floor,
            sprite_button: SpriteButton,
            sprite_interaction: SpriteInteraction::None
        }
    }
}



