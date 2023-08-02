use bevy::prelude::States;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Title,

    Maker,

    StageSelect,

    Playing,
}



