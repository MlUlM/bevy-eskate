use bevy::asset::Handle;
use bevy::prelude::{Image, Resource, AssetServer};
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Clone, Resource, Default)]
pub struct StageEditAssets{
    #[asset(path = "stage_edit/item_plus.png")]
    pub item_plus: Handle<Image>,
}