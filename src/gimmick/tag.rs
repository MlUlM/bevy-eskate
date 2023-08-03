use bevy::asset::Handle;
use bevy::prelude::{AssetServer, Image};
use bevy::ui::UiImage;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum GimmickTag {
    Floor,
    Rock,
    Player,
}


impl GimmickTag {
    #[inline]
    pub fn asset_path(&self) -> String {
        match self {
            GimmickTag::Floor => "gimmick/floor.png".to_string(),
            GimmickTag::Rock => "gimmick/rock.png".to_string(),
            GimmickTag::Player => "gimmick/player.png".to_string()
        }
    }


    #[inline]
    pub fn load(&self, asset: &AssetServer) -> Handle<Image> {
        asset.load(self.asset_path())
    }


    #[inline]
    pub fn load_to_ui_image(&self, asset: &AssetServer) -> UiImage {
        UiImage::new(asset.load(self.asset_path()))
    }
}