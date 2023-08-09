use bevy::core::Name;
use bevy::prelude::{Bundle, Component, ImageBundle, PositionType};
use bevy::ui::{Style, Val, ZIndex};
use bevy::utils::default;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash, Component)]
pub struct GameCursor;


#[derive(Bundle)]
pub struct GameCursorBundle {
    image: ImageBundle,
    cursor: GameCursor,
    name: Name,
}


impl GameCursorBundle {
    #[inline]
    pub fn new() -> Self {
        Self {
            image: ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Px(30.),
                    height: Val::Px(30.),
                    ..default()
                },
                z_index: ZIndex::Global(15),
                ..default()
            },
            cursor: GameCursor,
            name: Name::new("GameCursor"),
        }
    }
}