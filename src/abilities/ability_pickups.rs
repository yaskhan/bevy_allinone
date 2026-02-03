use bevy::prelude::*;

/// Ability pickup marker.
///
/// (Ability activation on pickup)
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AbilityPickup {
    pub ability_name: String,
    pub activate_on_pickup: bool,
    pub temporary_activation: bool,
}

impl Default for AbilityPickup {
    fn default() -> Self {
        Self {
            ability_name: String::new(),
            activate_on_pickup: false,
            temporary_activation: false,
        }
    }
}
