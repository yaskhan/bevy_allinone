use bevy::prelude::*;

mod types;
mod systems;

pub use types::*;
pub use systems::*;

pub struct PointAndClickPlugin;

impl Plugin for PointAndClickPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PointAndClickElement>()
           .register_type::<PointAndClickController>()
           .register_type::<PointAndClickElementType>()
           .add_systems(Update, (
               handle_mouse_click,
               move_agent,
           ));
    }
}
