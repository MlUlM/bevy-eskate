use bevy::prelude::States;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash, States)]
pub enum GameState {
    Title,
    #[default]
    Maker,

    StageSelect,

    Playing,
}



