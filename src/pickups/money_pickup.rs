use bevy::prelude::*;

/// Money pickup data.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MoneyPickup {
    pub amount: f32,
    pub currency_type: String,
    pub use_money_random_range: bool,
    pub money_random_range: Vec2,
}

impl Default for MoneyPickup {
    fn default() -> Self {
        Self {
            amount: 0.0,
            currency_type: String::new(),
            use_money_random_range: false,
            money_random_range: Vec2::ZERO,
        }
    }
}
