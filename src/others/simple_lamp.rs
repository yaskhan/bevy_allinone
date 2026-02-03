use bevy::prelude::*;

/// Simple lamp component.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SimpleLamp {
    pub color: Color,
    pub intensity: f32,
}

impl Default for SimpleLamp {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            intensity: 1.0,
        }
    }
}
