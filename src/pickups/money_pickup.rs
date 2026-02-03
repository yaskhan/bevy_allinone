use bevy::prelude::*;

/// Money pickup data.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MoneyPickup {
    pub amount: f32,
    pub currency_type: String,
}

impl Default for MoneyPickup {
    fn default() -> Self {
        Self {
            amount: 0.0,
            currency_type: String::new(),
        }
    }
}
