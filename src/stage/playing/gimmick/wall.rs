use bevy::asset::Handle;
use bevy::math::Vec3;
use bevy::prelude::{Bundle, Image};

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


#[cfg(test)]
mod tests {
    #[test]
    fn move_to_front_of_rock() {}
}