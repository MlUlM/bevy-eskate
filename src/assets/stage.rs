use bevy::asset::{Handle, HandleUntyped};
use bevy::prelude::{AssetServer, Resource};
use bevy::reflect::TypeUuid;
use bevy_asset_loader::prelude::AssetCollection;
use bevy_inspector_egui::__macro_exports::bevy_reflect::TypePath;

use crate::loader::json::StageJson;

#[derive(AssetCollection, Resource, Default, Debug)]
pub struct StageAssets {
    #[asset(path = "gimmick", collection)]
    pub stages: Vec<HandleUntyped>,
}


#[derive(Resource, TypePath, TypeUuid, Debug)]
#[uuid = "413be529-33eb-41b3-9db0-4b8b380a2c38"]
pub struct StageHandle(pub Handle<StageJson>);


#[derive(Resource, Debug, Clone, PartialEq)]
pub struct BuiltInStages(pub Vec<StageJson>);
