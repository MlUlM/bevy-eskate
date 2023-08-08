use bevy::asset::Handle;
use bevy::prelude::{AssetServer, Resource};
use bevy::text::Font;
use bevy_asset_loader::prelude::AssetCollection;

#[derive(AssetCollection, Resource, Default)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub button_text: Handle<Font>,
}
