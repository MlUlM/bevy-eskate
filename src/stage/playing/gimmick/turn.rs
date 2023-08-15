use bevy::math::Vec3;
use bevy::prelude::Bundle;

use crate::assets::gimmick::GimmickAssets;
use crate::page::page_index::PageIndex;
use crate::stage::playing::collide::GimmickCollide;
use crate::stage::playing::gimmick::core::{GimmickCollideBundle, GimmickCoreBundle};
use crate::stage::playing::move_position::{ MoveUp};

#[derive(Bundle, Clone)]
pub struct TurnBundle {
    core: GimmickCoreBundle,
    collide: GimmickCollideBundle<MoveUp>,
}


impl TurnBundle {
    #[inline]
    pub fn new(
        assets: &GimmickAssets,
        pos: Vec3,
        page_index: PageIndex,
    ) -> Self {
        Self {
            core: GimmickCoreBundle::new("Turn", assets.turn.clone(), pos, page_index),
            collide: GimmickCollideBundle::new(GimmickCollide::Turn),
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn move_to_front_of_rock() {}
}