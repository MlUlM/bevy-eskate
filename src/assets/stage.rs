use bevy::asset::Handle;
use bevy::prelude::{AssetServer, Image, Resource};
use bevy::reflect::TypeUuid;
use bevy_asset_loader::prelude::AssetCollection;
use bevy_inspector_egui::__macro_exports::bevy_reflect::TypePath;

use crate::loader::json::StageJson;

#[derive(AssetCollection, Resource, Default)]
pub struct StageAssets {
    #[asset(path = "stages", collection(typed))]
    pub stages: Vec<Handle<StageJson>>,
}


#[derive(Resource, TypePath, TypeUuid, Debug)]
#[uuid = "413be529-33eb-41b3-9db0-4b8b380a2c38"]
pub struct StageHandle(pub Handle<StageJson>);