use bevy::reflect::erased_serde::__private::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StageJson {
    pub pages: Vec<Page>,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Page {
    pub cells: Vec<Cell>,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Cell {
    pub x: f32,
    pub y: f32,
    pub tag: GimmickTag,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum GimmickTag {
    Floor,
    Rock,
    Multiple(Vec<GimmickTag>),
}


#[cfg(test)]
mod tests {
    use crate::loader::json::{Cell, GimmickTag, StageJson};

    #[test]
    fn floor() {
        let json = r#"{"pages" : [{ "cells": [{"x": 1, "y": 2, "tag": "Floor"}]} ]}"#;
        let stage = serde_json::from_str::<StageJson>(json).unwrap();
        assert_eq!(stage.pages[0].cells[0], Cell { x: 1., y: 2., tag: GimmickTag::Floor });
    }


    #[test]
    fn multiple() {
        let json = r#"{"pages" : [{ "cells": [{"x": 1, "y": 2, "tag": {"Multiple": ["Floor", "Rock"]} }]} ]}"#;
        let stage = serde_json::from_str::<StageJson>(json).unwrap();
        assert_eq!(stage.pages[0].cells[0], Cell { x: 1., y: 2., tag: GimmickTag::Multiple(vec![GimmickTag::Floor, GimmickTag::Rock]) });
    }
}