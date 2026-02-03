use bevy::prelude::*;

/// Melee shield pickup data.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MeleeShieldPickup {
    pub shield_id: String,
    pub shield_name: String,
    pub store_picked_shields_on_inventory: bool,
}

impl Default for MeleeShieldPickup {
    fn default() -> Self {
        Self {
            shield_id: String::new(),
            shield_name: String::new(),
            store_picked_shields_on_inventory: false,
        }
    }
}
