use bevy::prelude::*;

/// Currency balance stored on an entity.
///
/// GKC reference: `currencySystem.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct CurrencyBalance {
    pub amount: i32,
}

impl Default for CurrencyBalance {
    fn default() -> Self {
        Self { amount: 0 }
    }
}

/// Transaction event for changing currency.
#[derive(Event, Debug)]
pub struct CurrencyTransactionEvent {
    pub entity: Entity,
    pub delta: i32,
}

pub fn update_currency_system(
    mut events: EventReader<CurrencyTransactionEvent>,
    mut balances: Query<&mut CurrencyBalance>,
) {
    for event in events.read() {
        if let Ok(mut balance) = balances.get_mut(event.entity) {
            balance.amount = balance.amount.saturating_add(event.delta);
        }
    }
}
