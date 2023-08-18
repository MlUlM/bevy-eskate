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
    pub pages: Vec<PageJson>,
}


impl StageJson {
    #[inline]
    pub fn empty_stage(
        page_count: PageCount,
        page_width: u8,
        page_height: u8,
        top_left: Vec2,
    ) -> Self {
        Self {
            name: "".to_string(),
            pages: (0..*page_count).map(|page_index| empty_page(page_width, page_height, top_left, page_index, *page_count)).collect(),
        }
    }
}


fn empty_page(
    page_width: u8,
    page_height: u8,
    top_left: Vec2,
    page_index: usize,
    page_count: usize,
) -> PageJson {
    let mut cells = Vec::with_capacity(page_width as usize * page_height as usize);
    let w = page_width - 1;
    let h = page_height - 1;

    for x in 0..page_width {
        for y in 0..page_height {
            let tag = if x == 0 || y == 0 || x == w || y == h {
                if (x == 0 || x == w) &&  y < h { GimmickTag::WallSide } else { GimmickTag::Wall }
            } else {
                GimmickTag::Floor
            };

            let x = f32::from(x) * GIMMICK_WIDTH;
            let y = -f32::from(y) * GIMMICK_HEIGHT;
            cells.push(StageCell::new(Vec2::new(x, y), vec![tag]));
        }
    }
    let item_area = ItemAreaJson {
        width: GIMMICK_WIDTH * 1.3,
        height: f32::from(page_height - 1) * GIMMICK_HEIGHT,
        tags: vec![],
    };
    let d = diff(page_width, page_height, top_left, page_index, page_count);
    PageJson {
        x: d.x - GIMMICK_WIDTH * 1.3,
        y: d.y,
        cells,
        item_area,
    }
}


fn diff(
    page_width: u8,
    page_height: u8,
    top_left: Vec2,
    page_index: usize,
    page_count: usize,
) -> Vec2 {
    let i = f32::from(page_index as u8);
    let x = top_left.x;
    let y = top_left.y;
    match page_count {
        1 => Vec2::new(-f32::from(page_width) * GIMMICK_WIDTH / 2., f32::from(page_height) * GIMMICK_HEIGHT / 2.),
        2 => Vec2::new(GIMMICK_WIDTH - f32::from(page_width) * GIMMICK_WIDTH / 2., y - ((i + 1.) * GIMMICK_HEIGHT) - (i * GIMMICK_HEIGHT * f32::from(page_height))),
        3 | 4 => Vec2::new(x + ((i % 2. + 1.) * GIMMICK_WIDTH * 1.3) + GIMMICK_WIDTH + ((i % 2.) * GIMMICK_WIDTH * f32::from(page_width)) + ((i % 2.) * GIMMICK_WIDTH), y - if 1. < i { GIMMICK_HEIGHT * f32::from(page_height) + 2. * GIMMICK_HEIGHT } else { GIMMICK_HEIGHT }),
        _ => { panic!("Not supported page count"); }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PageJson {
    pub x: f32,
    pub y: f32,
    pub cells: Vec<StageCell>,
    pub item_area: ItemAreaJson,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ItemAreaJson {
    pub width: f32,
    pub height: f32,
    pub tags: Vec<ItemCell>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ItemCell {
    pub x: f32,
    pub y: f32,
    pub tag: GimmickTag,
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


