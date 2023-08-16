use bevy::asset::Handle;
use bevy::ecs::system::EntityCommands;
use bevy::math::Vec3;
use bevy::prelude::{Commands, Image};
use bevy::ui::UiImage;
use bevy_trait_query::imports::Component;
use serde::{Deserialize, Serialize};

use crate::assets::gimmick::GimmickAssets;
use crate::page::page_index::PageIndex;
use crate::stage::playing::gimmick::floor::FloorBundle;
use crate::stage::playing::gimmick::goal::GoalBundle;
use crate::stage::playing::gimmick::ice_box::IceBoxBundle;
use crate::stage::playing::gimmick::key::KeyBundle;
use crate::stage::playing::gimmick::next_page::NextPageBundle;
use crate::stage::playing::gimmick::player::PlayerBundle;
use crate::stage::playing::gimmick::rock::RockBundle;
use crate::stage::playing::gimmick::stop::StopBundle;
use crate::stage::playing::gimmick::turn::TurnBundle;
use crate::stage::playing::gimmick::wall::WallBundle;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Deserialize, Serialize, Component, PartialOrd, Ord)]
pub enum GimmickTag {
    Floor,
    NextPage,
    Goal,
    Stop,
    Rock,
    IceBox,
    Wall,
    WallSide,
    Player,
    Turn,
    Key
}


impl GimmickTag {
    pub fn spawn<'w, 's, 'a>(
        &self,
        commands: &'a mut Commands<'w, 's>,
        assets: &GimmickAssets,
        pos: Vec3,
        page_index: PageIndex,
    ) -> EntityCommands<'w, 's, 'a> {
        match self {
            GimmickTag::Floor => commands.spawn(FloorBundle::new(assets, pos, page_index)),
            GimmickTag::Wall => commands.spawn(WallBundle::new(assets.wall.clone(), pos, page_index)),
            GimmickTag::WallSide => commands.spawn(WallBundle::new(assets.wall_side.clone(), pos, page_index)),
            GimmickTag::Rock => commands.spawn(RockBundle::new(assets, pos, page_index)),
            GimmickTag::Player => commands.spawn(PlayerBundle::new(assets, pos, page_index)),
            GimmickTag::NextPage => commands.spawn(NextPageBundle::new(assets, pos, page_index)),
            GimmickTag::Goal => commands.spawn(GoalBundle::new(assets, pos, page_index)),
            GimmickTag::Stop => commands.spawn(StopBundle::new(assets, pos, page_index)),
            GimmickTag::IceBox => commands.spawn(IceBoxBundle::new(assets, pos, page_index)),
            GimmickTag::Turn => commands.spawn(TurnBundle::new(assets, pos, page_index)),
            GimmickTag::Key => commands.spawn(KeyBundle::new(assets, pos, page_index))
        }
    }


    #[inline]
    pub fn image(&self, assets: &GimmickAssets) -> Handle<Image> {
        match self {
            GimmickTag::Floor => assets.floor.clone(),
            GimmickTag::Wall => assets.wall.clone(),
            GimmickTag::WallSide => assets.wall_side.clone(),
            GimmickTag::Rock => assets.rock.clone(),
            GimmickTag::Player => assets.player.clone(),
            GimmickTag::NextPage => assets.next_page.clone(),
            GimmickTag::Goal => assets.goal.clone(),
            GimmickTag::Stop => assets.stop.clone(),
            GimmickTag::IceBox => assets.ice_box.clone(),
            GimmickTag::Turn => assets.turn.clone(),
            GimmickTag::Key => assets.key.clone()
        }
    }


    #[inline]
    pub fn ui_image(&self, asset: &GimmickAssets) -> UiImage {
        UiImage::new(self.image(asset))
    }


    // #[inline]
    // #[allow(unused)]
    // pub fn asset_path(&self) -> String {
    //     match self {
    //         GimmickTag::Floor => "gimmick/floor.png".to_string(),
    //         GimmickTag::Wall => "gimmick/wall.png".to_string(),
    //         GimmickTag::WallSide => "gimmick/wall_side.png".to_string(),
    //         GimmickTag::Rock => "gimmick/rock.png".to_string(),
    //         GimmickTag::Player => "gimmick/player.png".to_string(),
    //         GimmickTag::NextPage => "gimmick/next_page.png".to_string(),
    //         GimmickTag::Goal => "gimmick/goal.png".to_string()
    //     }
    // }
}