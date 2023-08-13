use bevy::asset::Handle;
use bevy::prelude::{AssetServer, Image, Resource};
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Clone, Resource, Default, Debug)]
pub struct CursorAssets {
    #[asset(path = "game_cursor.png")]
    pub game_cursor: Handle<Image>,
}