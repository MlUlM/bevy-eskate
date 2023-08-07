use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource, Default)]
pub struct GimmickAssets {
    #[asset(path = "gimmick/floor.png")]
    pub floor: Handle<Image>,

    #[asset(path = "gimmick/rock.png")]
    pub rock: Handle<Image>,

    #[asset(path = "gimmick/fall_down.png")]
    pub fall_down: Handle<Image>,

    #[asset(path = "gimmick/goal.png")]
    pub goal: Handle<Image>,

    #[asset(path = "gimmick/player.png")]
    pub player: Handle<Image>,
}


