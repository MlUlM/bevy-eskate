use bevy::prelude::*;

use crate::assets::gimmick::GimmickAssets;
use crate::page::page_index::PageIndex;
use crate::stage::playing::gimmick::core::GimmickCoreBundle;
use crate::stage::playing::gimmick::GimmickItemSpawned;
use crate::stage::playing::gimmick::tag::GimmickTag;

#[derive(Default, Clone, Copy, Component)]
pub struct Player;


#[derive(Default, Clone, Copy, Component)]
pub struct Movable;


#[derive(Default, Component, Copy, Clone, Debug)]
pub struct Moving;


#[derive(Bundle, Clone)]
pub struct PlayerBundle {
    core: GimmickCoreBundle,
    movable: Movable,
    spawned: GimmickItemSpawned,
    player: Player,
}


impl PlayerBundle {
    #[inline(always)]
    pub fn new(
        assets: &GimmickAssets,
        pos: Vec3,
        page_index: PageIndex,
    ) -> Self {
        Self {
            core: GimmickCoreBundle::new("Player", assets.player.clone(), pos, page_index, GimmickTag::Player),
            movable: Movable,
            spawned: GimmickItemSpawned(GimmickTag::Player),
            player: Player,
        }
    }
}










