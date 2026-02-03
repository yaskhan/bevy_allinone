use bevy::prelude::*;

pub mod types;
pub mod systems;
pub mod damage_over_time;
pub mod destroyable;
pub mod area_effect;
pub mod damage_ui;
pub mod sync;
pub mod result_queue;

pub use types::*;
pub use systems::*;
pub use damage_over_time::*;
pub use destroyable::*;
pub use area_effect::*;
pub use damage_ui::*;
pub use sync::*;
pub use result_queue::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DamageEventQueue>()
            .init_resource::<DeathEventQueue>()
            .init_resource::<DamageResultQueue>()
            .init_resource::<DamageFeedbackSettings>()
            .init_resource::<AttackDatabase>()
            .register_type::<Health>()
            .register_type::<Shield>()
            .register_type::<DamageReceiver>()
            .register_type::<MeleeCombat>()
            .register_type::<AttackDefinition>()
            .register_type::<AttackChain>()
            .register_type::<MeleeAttackState>()
            .register_type::<DamageZone>()
            .register_type::<MeleeRangedWeaponSettings>()
            .register_type::<MeleeRangedAimState>()
            .register_type::<ReturnToOwner>()
            .register_type::<Blocking>()
            .register_type::<DamageOverTime>()
            .register_type::<DestroyableObject>()
            .register_type::<AreaEffect>()
            .register_type::<DamageScreenEffect>()
            .register_type::<DamageIndicator>()
            .add_systems(Startup, damage_ui::setup_damage_ui)
            .add_systems(Update, (
                systems::clear_damage_results, // Clear results at start of frame/update
                systems::update_timers,
                systems::update_melee_attack_state,
                systems::update_melee_hitboxes,
                systems::perform_melee_hitbox_damage,
                systems::update_melee_ranged_aim,
                systems::update_melee_ranged_camera,
                systems::perform_melee_ranged_attacks,
                systems::update_returning_projectiles,
                systems::regenerate_health,
                systems::regenerate_shields,
                systems::perform_melee_attacks,
                systems::perform_blocking,
                
                // Sync Stats <-> Combat
                sync::sync_stats_to_combat, // Push Max from Stats to Health
                sync::sync_combat_to_stats, // Push Current from Health to Stats
                
                // Damage Logic Chain
                damage_ui::trigger_damage_ui, // Read events before drain
                systems::process_damage_events, // Drains events
                
                systems::update_damage_numbers,
                damage_ui::update_damage_ui,
                damage_over_time::update_damage_over_time,
                destroyable::handle_destroyable_death,
                systems::handle_character_death, // Character Death -> Ragdoll
                area_effect::handle_area_effects,
            ).chain());
    }
}
