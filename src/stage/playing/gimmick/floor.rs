use bevy::math::Vec3;
use bevy::prelude::{Bundle, Commands};

use crate::assets::gimmick::GimmickAssets;
use crate::page::page_index::PageIndex;
use crate::stage::playing::gimmick::core::GimmickCoreBundle;

#[derive(Bundle, Clone)]
pub struct FloorBundle {
    core: GimmickCoreBundle,
}


impl FloorBundle {
    #[inline]
    pub fn new(
        assets: &GimmickAssets,
        pos: Vec3,
        page_index: PageIndex,
    ) -> Self {
        Self {
            core: GimmickCoreBundle::new("Floor", assets.floor.clone(), pos, page_index)
        }
    }
}


#[inline]
pub fn spawn(
    commands: &mut Commands,
    assets: &GimmickAssets,
    pos: Vec3,
    page_index: PageIndex,
) {
    commands.spawn(FloorBundle::new(assets, pos, page_index));
}

