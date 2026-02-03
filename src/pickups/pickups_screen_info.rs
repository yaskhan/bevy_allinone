use bevy::prelude::*;

/// Pickup screen UI settings.
///
/// GKC reference: `pickUpsScreenInfo.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PickUpsScreenInfo {
    pub show_icons: bool,
}

impl Default for PickUpsScreenInfo {
    fn default() -> Self {
        Self { show_icons: true }
    }
}
