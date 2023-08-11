use bevy::math::Vec3;
use bevy::prelude::Bundle;

use crate::assets::gimmick::GimmickAssets;
use crate::page::page_index::PageIndex;
use crate::stage::playing::collide::GimmickCollide;
use crate::stage::playing::gimmick::core::{GimmickCollideBundle, GimmickCoreBundle};
use crate::stage::playing::move_position::MoveToFront;

#[derive(Bundle)]
pub struct IceBoxBundle {
    core: GimmickCoreBundle,
    collide: GimmickCollideBundle<MoveToFront>,
}


impl IceBoxBundle {
    #[inline]
    pub fn new(
        assets: &GimmickAssets,
        pos: Vec3,
        page_index: PageIndex,
    ) -> Self {
        Self {
            core: GimmickCoreBundle::new("IceBox", assets.ice_box.clone(), pos, page_index),
            collide: GimmickCollideBundle::new(GimmickCollide::IceBox),
        }
    }
}
