use bevy::asset::Handle;
use bevy::math::Vec3;
use bevy::prelude::{Bundle, Commands, Image};

use crate::assets::gimmick::GimmickAssets;
use crate::page::page_index::PageIndex;
use crate::stage::playing::collide::GimmickCollide;
use crate::stage::playing::gimmick::core::{GimmickCollideBundle, GimmickCoreBundle};
use crate::stage::playing::move_position::MoveToFront;

#[derive(Bundle, Clone)]
pub struct WallBundle {
    core: GimmickCoreBundle,
    collide: GimmickCollideBundle<MoveToFront>,
}


impl WallBundle {
    #[inline]
    pub fn new(
        texture: Handle<Image>,
        pos: Vec3,
        page_index: PageIndex,
    ) -> Self {
        Self {
            core: GimmickCoreBundle::new("Wall", texture, pos, page_index),
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
    commands.spawn(WallBundle::new(assets.wall.clone(), pos, page_index));
}


#[inline]
pub fn spawn_side(
    commands: &mut Commands,
    assets: &GimmickAssets,
    pos: Vec3,
    page_index: PageIndex,
) {
    commands.spawn(WallBundle::new(assets.wall_side.clone(), pos, page_index));
}


#[cfg(test)]
mod tests {
    #[test]
    fn move_to_front_of_rock() {}
}