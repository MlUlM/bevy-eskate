use bevy::app::{App, Update};
use bevy::input::Input;
use bevy::math::Vec3Swizzles;
use bevy::prelude::{Bundle, Camera, Camera2d, Component, GlobalTransform, MouseButton, Plugin, Query, Rect, Res, Resource, Sprite, SpriteBundle, Window, With};
use bevy::window::PrimaryWindow;
use itertools::Itertools;

use crate::gimmick::Stage;

#[derive(Debug, Copy, Clone, Default, Hash, Eq, PartialEq)]
pub struct SpriteButtonPlugin;


impl Plugin for SpriteButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update);
    }
}


#[derive(Clone, Default, Bundle)]
pub struct SpriteButtonBundle {
    sprite: SpriteBundle,
    interaction: SpriteInteraction,
    button: SpriteButton,
}


impl SpriteButtonBundle {
    #[inline]
    pub fn new(sprite: SpriteBundle) -> Self {
        Self {
            sprite,
            interaction: SpriteInteraction::None,
            button: SpriteButton,
        }
    }
}


fn update(
    mouse: Res<Input<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), (With<Camera2d>, With<Stage>)>,
    buttons: Query<(&Sprite, &GlobalTransform, &mut SpriteInteraction), (
        With<Sprite>,
        With<SpriteButton>,
        With<SpriteInteraction>
    )>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        clicked(window, camera, buttons);
    } else if mouse.just_released(MouseButton::Left) {
        releases(buttons);
    }
}


fn clicked(
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), (With<Camera2d>, With<Stage>)>,
    mut buttons: Query<(&Sprite, &GlobalTransform, &mut SpriteInteraction), (
        With<Sprite>,
        With<SpriteButton>,
        With<SpriteInteraction>
    )>,
) {
    let (camera, camera_transform) = camera.single();

    if let Some(position) = window
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        if let Some((_, _, mut interaction)) = buttons
            .iter_mut()
            .filter(|(sprite, t, _)| {
                sprite.custom_size.is_some_and(|size| Rect::from_center_size(t.translation().xy(), size).contains(position))
            })
            .sorted_by(|(_, t1, _), (_, t2, _)| t1.translation().z.partial_cmp(&t2.translation().z).unwrap())
            .next() {
            *interaction = SpriteInteraction::Clicked;
        }
    }
}


fn releases(
    mut buttons: Query<(&Sprite, &GlobalTransform, &mut SpriteInteraction), (
        With<Sprite>,
        With<SpriteButton>,
        With<SpriteInteraction>
    )>,
) {
    for (_, _, mut interaction) in buttons
        .iter_mut()
        .filter(|(_, _, interaction)| interaction.is_clicked()) {
        *interaction = SpriteInteraction::None;
    }
}


#[derive(
Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Resource, Component,
)]
pub enum SpriteInteraction {
    #[default]
    None,

    Clicked,
}


impl SpriteInteraction {
    #[inline]
    pub const fn is_clicked(&self) -> bool {
        matches!(self, Self::Clicked)
    }
}


#[derive(
Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Resource, Component,
)]
pub struct SpriteButton;