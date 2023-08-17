use bevy::ecs::system::SystemParam;
use bevy::math::Vec2;
use bevy::prelude::Query;
use bevy::window::Window;

#[derive(SystemParam)]
pub struct WindowParams<'w, 's> {
    window: Query<'w, 's, &'static Window>,
}


impl<'w, 's> WindowParams<'w, 's> {
    #[inline]
    pub fn top_left(&self) -> Vec2 {
        let w = self.window.single();
        Vec2::new(-w.resolution.width() / 2., w.resolution.height() / 2.)
    }
}