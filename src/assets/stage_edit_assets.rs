use bevy::asset::Handle;
use bevy::prelude::{AssetServer, Image, Resource};
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Clone, Resource, Default, Debug)]
pub struct StageEditAssets {
    #[asset(path = "stage_edit/item_plus.png")]
    pub item_plus: Handle<Image>,

    #[asset(path = "stage_edit/eraser.png")]
    pub eraser: Handle<Image>,
}