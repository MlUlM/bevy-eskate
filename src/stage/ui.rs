use bevy::hierarchy::BuildChildren;
use bevy::prelude::{Commands, default, NodeBundle, Text, TextBundle, TextStyle, Transform, Val};
use bevy::ui::{Style, UiRect};
use crate::assets::font::FontAssets;
use crate::stage::KeysCountText;

pub fn spawn_ui(
    commands: &mut Commands,
    fonts: &FontAssets
) {
    commands
        .spawn(NodeBundle {
            style: Style{
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                padding: UiRect::all(Val::Px(10.)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section("Keys: 0", TextStyle {
                    font: fonts.button_text.clone(),
                    font_size: 32.,
                    ..default()
                }),
                transform: Transform::from_xyz(-540., 120., 0.),
                ..default()
            })
                .insert(KeysCountText);
        });
}