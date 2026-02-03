use bevy::prelude::*;

use super::initial_pop_up_window::InitialPopUpWindow;

/// Opens an initial pop-up window.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct OpenInitialPopUpWindow {
    pub target: Entity,
    pub open: bool,
}

impl Default for OpenInitialPopUpWindow {
    fn default() -> Self {
        Self {
            target: Entity::PLACEHOLDER,
            open: false,
        }
    }
}

pub fn update_open_initial_pop_up_window(
    mut query: Query<&OpenInitialPopUpWindow>,
    mut windows: Query<&mut InitialPopUpWindow>,
) {
    for opener in query.iter_mut() {
        let Ok(mut window) = windows.get_mut(opener.target) else { continue };
        window.is_open = opener.open;
    }
}
