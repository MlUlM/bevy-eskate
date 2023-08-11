use bevy::math::Vec3;
use bevy::prelude::{Bundle, Commands};
use bevy_trait_query::imports::Component;

use crate::assets::gimmick::GimmickAssets;
use crate::page::page_index::PageIndex;
use crate::stage::playing::collide::GimmickCollide;
use crate::stage::playing::gimmick::core::{GimmickCollideBundle, GimmickCoreBundle};
use crate::stage::playing::move_position::MoveUp;

#[derive(Component, Copy, Clone, PartialEq, Eq, Debug)]
pub struct Goaled;


#[derive(Bundle, Clone)]
pub struct GoalBundle {
    core: GimmickCoreBundle,
    collide: GimmickCollideBundle<MoveUp>,
}


impl GoalBundle {
    #[inline]
    pub fn new(
        assets: &GimmickAssets,
        pos: Vec3,
        page_index: PageIndex,
    ) -> Self {
        Self {
            core: GimmickCoreBundle::new("Goal", assets.goal.clone(), pos, page_index),
            collide: GimmickCollideBundle::new(GimmickCollide::Goal),
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
    commands.spawn(GoalBundle::new(assets, pos, page_index));
}




