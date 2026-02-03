use bevy::prelude::*;

/// Detects enable state changes.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct OnEnableCheckSystem {
    pub was_enabled: bool,
}

impl Default for OnEnableCheckSystem {
    fn default() -> Self {
        Self { was_enabled: true }
    }
}

pub fn update_on_enable_check_system(
    mut query: Query<(&mut OnEnableCheckSystem, Option<&Visibility>)>,
) {
    for (mut system, visibility) in query.iter_mut() {
        let enabled = visibility.map(|v| *v != Visibility::Hidden).unwrap_or(true);
        system.was_enabled = enabled;
    }
}
