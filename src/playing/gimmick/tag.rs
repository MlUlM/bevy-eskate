use bevy::asset::Handle;
use bevy::prelude::Image;
use bevy::ui::UiImage;
use serde::{Deserialize, Serialize};

use crate::playing::gimmick::asset::GimmickAssets;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Deserialize, Serialize)]
pub enum GimmickTag {
    Floor,
    Rock,
    Player,
    NextPage,
    Goal,
}


impl GimmickTag {
    #[inline]
    #[allow(unused)]
    pub fn asset_path(&self) -> String {
        match self {
            GimmickTag::Floor => "gimmick/floor.png".to_string(),
            GimmickTag::Rock => "gimmick/rock.png".to_string(),
            GimmickTag::Player => "gimmick/player.png".to_string(),
            GimmickTag::NextPage => "gimmick/next_page.png".to_string(),
            GimmickTag::Goal => "gimmick/goal.png".to_string()
        }
    }


    #[inline]
    pub fn image(&self, assets: &GimmickAssets) -> Handle<Image> {
        match self {
            GimmickTag::Floor => assets.floor.clone(),
            GimmickTag::Rock => assets.rock.clone(),
            GimmickTag::Player => assets.player.clone(),
            GimmickTag::NextPage => assets.next_page.clone(),
            GimmickTag::Goal => assets.goal.clone()
        }
    }


    #[inline]
    pub fn ui_image(&self, asset: &GimmickAssets) -> UiImage {
        UiImage::new(self.image(asset))
    }
}