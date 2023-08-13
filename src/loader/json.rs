use bevy::math::Vec2;
use bevy::prelude::{Component, Resource};
use bevy::reflect::{TypePath, TypeUuid};
use bevy::reflect::erased_serde::__private::serde::{Deserialize, Serialize};

use crate::page::page_count::PageCount;
use crate::stage::playing::gimmick::{GIMMICK_HEIGHT, GIMMICK_WIDTH};
use crate::stage::playing::gimmick::tag::GimmickTag;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Resource, TypePath, TypeUuid, Component)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c43"]
pub struct StageJson {
    pub name: String,
    pub pages: Vec<Page>,
}


impl StageJson {
    #[inline]
    pub fn empty_stage(
        page_count: PageCount,
        page_width: u8,
        page_height: u8,
    ) -> Self {
        Self {
            name: "".to_string(),
            pages: (0..*page_count).map(|_|empty_page(page_width, page_height)).collect()
        }
    }
}


fn empty_page(
    page_width: u8,
    page_height: u8,
) -> Page {
    let mut cells = Vec::with_capacity(page_width  as usize * page_height as usize );

    for x in 0..page_width {
        for y in 0..page_height {
            let tag = if x == 0 || y == 0 || x == 24 || y == 14 {
                if (x == 0 || x == 24) && 0 < y { GimmickTag::WallSide } else { GimmickTag::Wall }
            } else {
                GimmickTag::Floor
            };

            let x = f32::from(x) * GIMMICK_WIDTH - 12. * GIMMICK_WIDTH;
            let y = f32::from(y) * GIMMICK_HEIGHT - 3.5 * GIMMICK_HEIGHT;
            cells.push(StageCell::new(Vec2::new(x, y), vec![tag]));
        }
    }

    Page{
        cells,
        items: vec![]
    }
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


