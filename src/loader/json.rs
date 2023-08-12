use bevy::math::Vec2;
use bevy::prelude::{Component, Resource};
use bevy::reflect::{TypePath, TypeUuid};
use bevy::reflect::erased_serde::__private::serde::{Deserialize, Serialize};

use crate::stage::playing::gimmick::tag::GimmickTag;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Resource, TypePath, TypeUuid, Component)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c43"]
pub struct StageJson {
    pub name: String,
    pub pages: Vec<Page>,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Page {
    pub cells: Vec<StageCell>,
    pub items: Vec<GimmickTag>,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StageCell {
    pub x: f32,
    pub y: f32,
    pub tags: Vec<GimmickTag>,
}


impl StageCell {
    #[inline]
    pub const fn new(pos: Vec2, tags: Vec<GimmickTag>) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            tags,
        }
    }
}


