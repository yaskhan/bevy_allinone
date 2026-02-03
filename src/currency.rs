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
use crate::stats::{StatsSystem, StatValue};

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

/// Custom queues for currency events (Workaround for Bevy 0.18 EventReader issues)
#[derive(Resource, Default)]
pub struct AddCurrencyEventQueue(pub Vec<AddCurrencyEvent>);

#[derive(Resource, Default)]
pub struct RemoveCurrencyEventQueue(pub Vec<RemoveCurrencyEvent>);

#[derive(Resource, Default)]
pub struct CurrencyRemovalFailedEventQueue(pub Vec<CurrencyRemovalFailedEvent>);

#[derive(Debug, Clone)]
pub struct CurrencyChangeEvent {
    pub entity: Entity,
    pub delta: f32,
    pub new_total: f32,
    pub currency_type: CurrencyType,
}

#[derive(Resource, Default)]
pub struct CurrencyChangeEventQueue(pub Vec<CurrencyChangeEvent>);

#[derive(Component)]
pub struct CurrencyNotification {
    pub timer: Timer,
    pub velocity: Vec2,
}

#[derive(Resource, Debug, Clone)]
pub struct CurrencyNotificationSettings {
    pub duration: f32,
    pub start_pos: Vec2,
    pub velocity: Vec2,
    pub color_gain: Color,
    pub color_spend: Color,
}

impl Default for CurrencyNotificationSettings {
    fn default() -> Self {
        Self {
            duration: 1.5,
            start_pos: Vec2::new(20.0, 20.0),
            velocity: Vec2::new(0.0, 30.0),
            color_gain: Color::srgba(0.2, 1.0, 0.2, 1.0),
            color_spend: Color::srgba(1.0, 0.4, 0.4, 1.0),
        }
    }
}

/// System to handle adding currency
pub fn handle_add_currency(
    mut events: ResMut<AddCurrencyEventQueue>,
    mut currency_query: Query<&mut Currency>,
    mut change_events: ResMut<CurrencyChangeEventQueue>,
) {
    for event in events.0.drain(..) {
        if let Ok(mut currency) = currency_query.get_mut(event.entity) {
            if currency.currency_type == event.currency_type {
                currency.amount += event.amount;
                change_events.0.push(CurrencyChangeEvent {
                    entity: event.entity,
                    delta: event.amount,
                    new_total: currency.amount,
                    currency_type: event.currency_type.clone(),
                });
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
    mut events: ResMut<RemoveCurrencyEventQueue>,
    mut currency_query: Query<&mut Currency>,
    mut failed_events: ResMut<CurrencyRemovalFailedEventQueue>,
    mut change_events: ResMut<CurrencyChangeEventQueue>,
) {
    for event in events.0.drain(..) {
        if let Ok(mut currency) = currency_query.get_mut(event.entity) {
            if currency.currency_type == event.currency_type {
                if currency.amount >= event.amount {
                    currency.amount -= event.amount;
                    change_events.0.push(CurrencyChangeEvent {
                        entity: event.entity,
                        delta: -event.amount,
                        new_total: currency.amount,
                        currency_type: event.currency_type.clone(),
                    });
                    info!(
                        "Removed {} {:?} from entity {:?}. Remaining: {}",
                        event.amount, event.currency_type, event.entity, currency.amount
                    );
                } else {
                    failed_events.0.push(CurrencyRemovalFailedEvent {
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

pub fn sync_currency_to_stats(
    currency_query: Query<(Entity, &Currency)>,
    mut stats_query: Query<&mut StatsSystem>,
) {
    for (entity, currency) in currency_query.iter() {
        if let Ok(mut stats) = stats_query.get_mut(entity) {
            let key = format!("currency_{:?}", currency.currency_type).to_lowercase();
            stats.set_custom_stat(&key, StatValue::Amount(currency.amount));
        }
    }
}

pub fn spawn_currency_notifications(
    mut commands: Commands,
    mut events: ResMut<CurrencyChangeEventQueue>,
    settings: Res<CurrencyNotificationSettings>,
) {
    for event in events.0.drain(..) {
        let color = if event.delta >= 0.0 {
            settings.color_gain
        } else {
            settings.color_spend
        };

        let text = format!(
            "{:+.0} {:?}",
            event.delta,
            event.currency_type
        );

        commands.spawn((
            Text::from_section(text, TextStyle { color, ..default() }),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(settings.start_pos.x),
                top: Val::Px(settings.start_pos.y),
                ..default()
            },
            GlobalZIndex(110),
            CurrencyNotification {
                timer: Timer::from_seconds(settings.duration, TimerMode::Once),
                velocity: settings.velocity,
            },
        ));
    }
}

pub fn update_currency_notifications(
    mut commands: Commands,
    time: Res<Time>,
    settings: Res<CurrencyNotificationSettings>,
    mut query: Query<(Entity, &mut CurrencyNotification, &mut Node, &mut Text)>,
) {
    for (entity, mut notif, mut node, mut text) in query.iter_mut() {
        notif.timer.tick(time.delta());
        let t = notif.timer.elapsed_secs() / settings.duration.max(0.01);
        let alpha = (1.0 - t).clamp(0.0, 1.0);

        if let Val::Px(left) = node.left {
            node.left = Val::Px(left + notif.velocity.x * time.delta_secs());
        }
        if let Val::Px(top) = node.top {
            node.top = Val::Px(top - notif.velocity.y * time.delta_secs());
        }

        for section in text.sections.iter_mut() {
            let mut color = section.style.color;
            color.set_alpha(alpha);
            section.style.color = color;
        }

        if notif.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// Plugin for the Currency System
pub struct CurrencyPlugin;

impl Plugin for CurrencyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add custom event queues
            .init_resource::<AddCurrencyEventQueue>()
            .init_resource::<RemoveCurrencyEventQueue>()
            .init_resource::<CurrencyRemovalFailedEventQueue>()
            .init_resource::<CurrencyChangeEventQueue>()
            .init_resource::<CurrencyNotificationSettings>()
            // Add systems
            .add_systems(Update, (
                handle_add_currency,
                handle_remove_currency,
                check_currency_balance,
                sync_currency_to_stats,
                spawn_currency_notifications,
                update_currency_notifications,
            ));
    }
}
