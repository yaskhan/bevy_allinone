use bevy::prelude::*;

pub mod types;
pub mod systems;
pub mod damage_over_time;
pub mod destroyable;
pub mod area_effect;
pub mod damage_ui;

pub use types::*;
pub use systems::*;
pub use damage_over_time::*;
pub use destroyable::*;
pub use area_effect::*;
pub use damage_ui::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DamageEventQueue>()
            .init_resource::<DeathEventQueue>()
            .init_resource::<DamageFeedbackSettings>()
            .register_type::<Health>()
            .register_type::<Shield>()
            .register_type::<DamageReceiver>()
            .register_type::<MeleeCombat>()
            .register_type::<Blocking>()
            .register_type::<DamageOverTime>()
            .register_type::<DestroyableObject>()
            .register_type::<AreaEffect>()
            .register_type::<DamageScreenEffect>()
            .register_type::<DamageIndicator>()
            .add_systems(Startup, damage_ui::setup_damage_ui)
            .add_systems(Update, (
                systems::update_timers,
                systems::regenerate_health,
                systems::regenerate_shields,
                systems::perform_melee_attacks,
                systems::perform_blocking,
                
                // Damage Logic Chain
                damage_ui::trigger_damage_ui, // Read events before drain
                systems::process_damage_events, // Drains events
                
                systems::update_damage_numbers,
                damage_ui::update_damage_ui,
                damage_over_time::update_damage_over_time,
                destroyable::handle_destroyable_death,
                area_effect::handle_area_effects,
            ).chain());
    }
}
