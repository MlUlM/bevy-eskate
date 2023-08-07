use bevy::asset::Handle;
use bevy::prelude::Image;
use bevy::ui::UiImage;
use serde::{Deserialize, Serialize};

use crate::gimmick::asset::GimmickAssets;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Deserialize, Serialize)]
pub enum GimmickTag {
    Floor,
    Rock,
    Player,
    FallDown,
    Goal,
}


impl GimmickTag {
    #[inline]
    pub fn asset_path(&self) -> String {
        match self {
            GimmickTag::Floor => "gimmick/floor.png".to_string(),
            GimmickTag::Rock => "gimmick/rock.png".to_string(),
            GimmickTag::Player => "gimmick/player.png".to_string(),
            GimmickTag::FallDown => "gimmick/fall_down.png".to_string(),
            GimmickTag::Goal => "gimmick/goal.png".to_string()
        }
    }


    #[inline]
    pub fn image(&self, assets: &GimmickAssets) -> Handle<Image> {
        match self {
            GimmickTag::Floor => assets.floor.clone(),
            GimmickTag::Rock => assets.rock.clone(),
            GimmickTag::Player => assets.player.clone(),
            GimmickTag::FallDown => assets.fall_down.clone(),
            GimmickTag::Goal => assets.goal.clone()
        }
    }


    #[inline]
    pub fn ui_image(&self, asset: &GimmickAssets) -> UiImage {
        UiImage::new(self.image(asset))
    }
}