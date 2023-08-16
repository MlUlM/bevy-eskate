use bevy::math::Vec3;
use bevy::prelude::{Bundle, Component};

use crate::assets::gimmick::GimmickAssets;
use crate::page::page_index::PageIndex;
use crate::stage::playing::collide::GimmickCollide;
use crate::stage::playing::gimmick::core::{GimmickCollideBundle, GimmickCoreBundle};
use crate::stage::playing::move_position::MoveUp;


#[derive(Default, Debug, Component, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct RequireKeys(pub usize);



#[derive(Bundle, Clone)]
pub struct LockBundle {
    core: GimmickCoreBundle,
    collide: GimmickCollideBundle<MoveUp>,
    require_keys: RequireKeys
}


impl LockBundle {
    #[inline]
    pub fn new(
        assets: &GimmickAssets,
        pos: Vec3,
        page_index: PageIndex,
    ) -> Self {
        Self {
            core: GimmickCoreBundle::new("Lock", assets.lock1.clone(), pos, page_index),
            collide: GimmickCollideBundle::new(GimmickCollide::Lock),
            require_keys: RequireKeys(1)
        }
    }
}
