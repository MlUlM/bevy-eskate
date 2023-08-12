use bevy::math::Vec3;
use bevy::prelude::{Bundle, Component};

use crate::assets::gimmick::GimmickAssets;
use crate::page::page_index::PageIndex;
use crate::stage::playing::collide::GimmickCollide;
use crate::stage::playing::gimmick::core::{GimmickCollideBundle, GimmickCoreBundle};
use crate::stage::playing::move_position::MoveUp;

#[derive(Default, Debug, Copy, Clone, Component)]
pub struct NextPageProcessing;


#[derive(Bundle, Clone)]
pub struct NextPageBundle {
    core: GimmickCoreBundle,
    collide: GimmickCollideBundle<MoveUp>,
}


impl NextPageBundle {
    #[inline]
    pub fn new(
        assets: &GimmickAssets,
        pos: Vec3,
        page_index: PageIndex,
    ) -> Self {
        Self {
            core: GimmickCoreBundle::new("NextPage", assets.next_page.clone(), pos, page_index),
            collide: GimmickCollideBundle::new(GimmickCollide::NextPage),
        }
    }
}


