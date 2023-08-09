use bevy::ui::Interaction;

mod interaction;


pub trait InteractionCondition {
    fn pressed(&self) -> bool;


    fn hovered(&self) -> bool;
}


impl InteractionCondition for Interaction {
    #[inline]
    fn pressed(&self) -> bool {
        matches!(self, Interaction::Pressed)
    }


    #[inline]
    fn hovered(&self) -> bool {
        matches!(self, Interaction::Hovered)
    }
}