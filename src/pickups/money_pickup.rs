use bevy::prelude::*;

/// Money pickup data.
///
/// GKC reference: `moneyPickup.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MoneyPickup {
    pub amount: f32,
}

impl Default for MoneyPickup {
    fn default() -> Self {
        Self { amount: 0.0 }
    }
}
