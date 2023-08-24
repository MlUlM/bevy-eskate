use bevy::asset::{Assets, Handle};
use bevy::prelude::{AssetServer, Resource};
use bevy::reflect::TypeUuid;
use bevy_asset_loader::prelude::AssetCollection;
use bevy_inspector_egui::__macro_exports::bevy_reflect::TypePath;

use crate::loader::json::StageJson;

macro_rules! stage_assets {
    ($($stage: ident, $path: expr),*) => {
        #[derive(AssetCollection, Resource, Default, Debug)]
        pub struct StageAssets {
            $(
                #[asset(path = $path)]
                pub $stage: Handle<StageJson>,
            )*
        }


        impl StageAssets{
            pub fn stages(&self, asset: &mut Assets<StageJson>) -> Vec<StageJson>{
                vec![
                    $(self.$stage.clone()),*
                ]
                    .iter()
                    .map(|s|asset.get(s).unwrap().clone())
                    .collect()
            }
        }
    };
}


stage_assets!(
    stage1, "stages/stage1.stage.json",
    stage2, "stages/stage2.stage.json",
    stage3, "stages/stage3.stage.json",
    stage4, "stages/stage4.stage.json",
    stage5, "stages/stage5.stage.json",
    stage6, "stages/stage6.stage.json",
    stage7, "stages/stage7.stage.json",
    stage8, "stages/stage8.stage.json",
    stage9, "stages/stage9.stage.json"
);





#[derive(Resource, TypePath, TypeUuid, Debug)]
#[uuid = "413be529-33eb-41b3-9db0-4b8b380a2c38"]
pub struct StageHandle(pub Handle<StageJson>);


#[derive(Resource, Debug, Clone, PartialEq)]
pub struct BuiltInStages(pub Vec<StageJson>);
