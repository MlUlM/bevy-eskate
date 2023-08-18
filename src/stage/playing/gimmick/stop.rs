use bevy::math::Vec3;
use bevy::prelude::Bundle;

use crate::assets::gimmick::GimmickAssets;
use crate::page::page_index::PageIndex;
use crate::stage::playing::collide::GimmickCollide;
use crate::stage::playing::gimmick::core::{GimmickCollideBundle, GimmickCoreBundle};
use crate::stage::playing::gimmick::tag::GimmickTag;
use crate::stage::playing::move_position::MoveUp;

#[derive(Bundle)]
pub struct StopBundle {
    core: GimmickCoreBundle,
    collide: GimmickCollideBundle<MoveUp>,
}


impl StopBundle {
    #[inline]
    pub fn new(
        assets: &GimmickAssets,
        pos: Vec3,
        page_index: PageIndex,
    ) -> Self {
        Self {
            core: GimmickCoreBundle::new("Stop", assets.stop.clone(), pos, page_index, GimmickTag::Stop),
            collide: GimmickCollideBundle::new(GimmickCollide::StopMove),
        }
    }
}