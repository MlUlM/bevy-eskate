use bevy::math::Vec3;
use bevy::prelude::Bundle;

use crate::assets::gimmick::GimmickAssets;
use crate::page::page_index::PageIndex;
use crate::stage::playing::gimmick::core::GimmickCoreBundle;
use crate::stage::playing::gimmick::Floor;
use crate::stage::playing::gimmick::tag::GimmickTag;

#[derive(Bundle, Clone)]
pub struct FloorBundle {
    core: GimmickCoreBundle,
    floor: Floor,
}


impl FloorBundle {
    #[inline]
    pub fn new(
        assets: &GimmickAssets,
        pos: Vec3,
        page_index: PageIndex,
    ) -> Self {
        Self {
            core: GimmickCoreBundle::new("Floor", assets.floor.clone(), pos, page_index, GimmickTag::Floor),
            floor: Floor,

        }
    }
}



