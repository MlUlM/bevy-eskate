use bevy::prelude::States;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    AssetLoading,

    Title,

    BeforeStageEdit,

    StageEdit,

    StageSelect,

    StageSetup,
    StagePlayingIdle,
    StagePlayingMove,
}



