use bevy::prelude::*;

/// UI mouse hover event marker.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct UIMouseHoverEvent {
    pub hover: bool,
}

impl Default for UIMouseHoverEvent {
    fn default() -> Self {
        Self { hover: false }
    }
}

#[derive(Event, Debug)]
pub struct UIMouseHoverChangedEvent {
    pub entity: Entity,
    pub hovered: bool,
}
