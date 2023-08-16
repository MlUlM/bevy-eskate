use bevy::asset::Handle;
use bevy::prelude::{Image, Resource, AssetServer};
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource, Default)]
pub struct GimmickAssets {
    #[asset(path = "gimmick/floor.png")]
    pub floor: Handle<Image>,

    #[asset(path = "gimmick/wall.png")]
    pub wall: Handle<Image>,

    #[asset(path = "gimmick/wall_side.png")]
    pub wall_side: Handle<Image>,

    #[asset(path = "gimmick/rock.png")]
    pub rock: Handle<Image>,

    #[asset(path = "gimmick/next_page.png")]
    pub next_page: Handle<Image>,

    #[asset(path = "gimmick/goal.png")]
    pub goal: Handle<Image>,

    #[asset(path = "gimmick/player.png")]
    pub player: Handle<Image>,

    #[asset(path = "gimmick/ice_box.png")]
    pub ice_box: Handle<Image>,

    #[asset(path = "gimmick/stop.png")]
    pub stop: Handle<Image>,

    #[asset(path = "gimmick/turn.png")]
    pub turn: Handle<Image>,

    #[asset(path = "gimmick/key.png")]
    pub key: Handle<Image>,

    #[asset(path = "gimmick/lock1.png")]
    pub lock1: Handle<Image>,
}


