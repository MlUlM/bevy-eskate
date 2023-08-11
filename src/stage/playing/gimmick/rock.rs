use bevy::math::Vec3;
use bevy::prelude::{Bundle, Commands};

use crate::assets::gimmick::GimmickAssets;
use crate::page::page_index::PageIndex;
use crate::stage::playing::collide::GimmickCollide;
use crate::stage::playing::gimmick::core::{GimmickCollideBundle, GimmickCoreBundle};
use crate::stage::playing::move_position::MoveToFront;

#[derive(Bundle, Clone)]
pub struct RockBundle {
    core: GimmickCoreBundle,
    collide: GimmickCollideBundle<MoveToFront>,
}


impl RockBundle {
    #[inline]
    pub fn new(
        assets: &GimmickAssets,
        pos: Vec3,
        page_index: PageIndex,
    ) -> Self {
        Self {
            core: GimmickCoreBundle::new("Rock", assets.rock.clone(), pos, page_index),
            collide: GimmickCollideBundle::new(GimmickCollide::StopMove),
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
    commands.spawn(RockBundle::new(assets, pos, page_index));
}


#[cfg(test)]
mod tests {
    #[test]
    fn move_to_front_of_rock() {}
}