//! # Currency System
//!
//! Money/gold management for the game.
//!
//! ## Features
//!
//! - **Currency Component**: Track player's money amount
//! - **Multiple Currency Types**: Support for different currencies (gold, silver, etc.)
//! - **Transaction Events**: Events for money changes
//! - **Integration**: Works with Vendor System and Inventory System
//!
//! ## Usage
//!
//! ```rust,no_run
//! use bevy::prelude::*;
//! use bevy_allinone::prelude::*;
//!
//! fn setup_currency(mut commands: Commands) {
//!     // Give player starting money
//!     commands.spawn((
//!         Player,
//!         Currency {
//!             amount: 100.0,
//!             currency_type: CurrencyType::Gold,
//!         },
//!     ));
//! }
//! ```

use bevy::prelude::*;

/// Component representing currency/money
#[derive(Component, Debug, Clone)]
pub struct Currency {
    /// Current amount of currency
    pub amount: f32,
    /// Type of currency
    pub currency_type: CurrencyType,
}

impl Default for Currency {
    fn default() -> Self {
        Self {
            amount: 0.0,
            currency_type: CurrencyType::Gold,
        }
    }
}

/// Types of currency
#[derive(Debug, Clone, PartialEq)]
pub enum CurrencyType {
    /// Gold coins (main currency)
    Gold,
    /// Silver coins
    Silver,
    /// Copper coins
    Copper,
    /// Special currency (premium, event, etc.)
    Special,
}

/// Event for adding currency to an entity
#[derive(Debug, Clone)]
pub struct AddCurrencyEvent {
    /// Entity to add currency to
    pub entity: Entity,
    /// Amount to add
    pub amount: f32,
    /// Currency type
    pub currency_type: CurrencyType,
}

/// Event for removing currency from an entity
#[derive(Debug, Clone)]
pub struct RemoveCurrencyEvent {
    /// Entity to remove currency from
    pub entity: Entity,
    /// Amount to remove
    pub amount: f32,
    /// Currency type
    pub currency_type: CurrencyType,
}

/// Event for when currency removal fails
#[derive(Debug, Clone)]
pub struct CurrencyRemovalFailedEvent {
    /// Entity that failed
    pub entity: Entity,
    /// Amount that was requested
    pub requested_amount: f32,
    /// Current amount available
    pub current_amount: f32,
}

/// System to handle adding currency
pub fn handle_add_currency(
    mut add_events: EventReader<AddCurrencyEvent>,
    mut currency_query: Query<&mut Currency>,
) {
    for event in add_events.iter() {
        if let Ok(mut currency) = currency_query.get_mut(event.entity) {
            if currency.currency_type == event.currency_type {
                currency.amount += event.amount;
                info!(
                    "Added {} {:?} to entity {:?}. New total: {}",
                    event.amount, event.currency_type, event.entity, currency.amount
                );
            }
        }
    }
}

/// System to handle removing currency
pub fn handle_remove_currency(
    mut remove_events: EventReader<RemoveCurrencyEvent>,
    mut currency_query: Query<&mut Currency>,
    mut failed_events: EventWriter<CurrencyRemovalFailedEvent>,
) {
    for event in remove_events.iter() {
        if let Ok(mut currency) = currency_query.get_mut(event.entity) {
            if currency.currency_type == event.currency_type {
                if currency.amount >= event.amount {
                    currency.amount -= event.amount;
                    info!(
                        "Removed {} {:?} from entity {:?}. Remaining: {}",
                        event.amount, event.currency_type, event.entity, currency.amount
                    );
                } else {
                    failed_events.send(CurrencyRemovalFailedEvent {
                        entity: event.entity,
                        requested_amount: event.amount,
                        current_amount: currency.amount,
                    });
                }
            }
        }
    }
}

/// System to check currency balance
pub fn check_currency_balance(
    currency_query: Query<(Entity, &Currency)>,
) {
    for (entity, currency) in currency_query.iter() {
        if currency.amount < 0.0 {
            warn!(
                "Entity {:?} has negative currency balance: {} {:?}",
                entity, currency.amount, currency.currency_type
            );
        }
    }
}

/// Plugin for the Currency System
pub struct CurrencyPlugin;

impl Plugin for CurrencyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add events
            .add_event::<AddCurrencyEvent>()
            .add_event::<RemoveCurrencyEvent>()
            .add_event::<CurrencyRemovalFailedEvent>()
            // Add systems
            .add_systems(Update, (
                handle_add_currency,
                handle_remove_currency,
                check_currency_balance,
            ));
    }
}
