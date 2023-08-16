use bevy::math::Vec3;
use bevy::prelude::Bundle;

use crate::assets::gimmick::GimmickAssets;
use crate::page::page_index::PageIndex;
use crate::stage::playing::collide::GimmickCollide;
use crate::stage::playing::gimmick::core::{GimmickCollideBundle, GimmickCoreBundle};
use crate::stage::playing::move_position::MoveUp;

#[derive(Bundle, Clone)]
pub struct KeyBundle {
    core: GimmickCoreBundle,
    collide: GimmickCollideBundle<MoveUp>,
}


impl KeyBundle {
    #[inline]
    pub fn new(
        assets: &GimmickAssets,
        pos: Vec3,
        page_index: PageIndex,
    ) -> Self {
        Self {
            core: GimmickCoreBundle::new("Key", assets.key.clone(), pos, page_index),
            collide: GimmickCollideBundle::new(GimmickCollide::Key),
        }
    }
}
