//! Weapon animation handling logic
//!
//! Manages transitions between weapon animation states based on input and player movement.

use bevy::prelude::*;
use super::types::{Weapon, WeaponAnimationState, WeaponAnimationMode};
use crate::weapons::weapon_manager::WeaponManager;
use crate::input::InputState;
use crate::character::{CharacterAnimationState, CharacterAnimationMode, Player};

/// Automatically add WeaponAnimationState to entities with a Weapon component
pub fn initialize_weapon_animation(
    mut commands: Commands,
    query: Query<Entity, (With<Weapon>, Without<WeaponAnimationState>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(WeaponAnimationState::default());
    }
}

/// System to update weapon animation state based on player actions and movement
pub fn handle_weapon_animation(
    time: Res<Time>,
    mut manager_query: Query<(&WeaponManager, &InputState, &CharacterAnimationState), With<Player>>,
    mut weapon_query: Query<(&mut Weapon, &mut WeaponAnimationState)>,
) {
    let dt = time.delta_secs();

    for (manager, input, char_anim_state) in manager_query.iter_mut() {
        if let Some(&weapon_entity) = manager.weapons_list.get(manager.current_index) {
            if let Ok((weapon, mut anim_state)) = weapon_query.get_mut(weapon_entity) {
                let mut next_mode = anim_state.current_mode;
                let mut is_looping = true;

                // 1. High priority: Action-based animations (non-looping usually)
                if weapon.is_reloading {
                    next_mode = if weapon.current_ammo == 0 {
                        WeaponAnimationMode::ReloadWithoutAmmo
                    } else {
                        WeaponAnimationMode::ReloadWithAmmo
                    };
                    is_looping = false;
                } else if manager.shooting_single_weapon {
                    next_mode = if manager.aiming_in_third_person || manager.aiming_in_first_person {
                        WeaponAnimationMode::AimShoot
                    } else {
                        WeaponAnimationMode::Shoot
                    };
                    is_looping = false;
                } else if manager.changing_weapon {
                    // Logic for draw/holster would go here if tracked in manager
                    // For now, let's assume switching sets changing_weapon
                    next_mode = WeaponAnimationMode::Draw;
                    is_looping = false;
                } 
                // 2. Aim transitions
                else if input.aim_pressed && anim_state.current_mode != WeaponAnimationMode::AimIn && !manager.aim_mode_input_pressed {
                     next_mode = WeaponAnimationMode::AimIn;
                     is_looping = false;
                }
                // 3. Movement-based animations (looping)
                else {
                    match char_anim_state.mode {
                         CharacterAnimationMode::Idle => next_mode = WeaponAnimationMode::Idle,
                         CharacterAnimationMode::Walk => next_mode = WeaponAnimationMode::Walk,
                         CharacterAnimationMode::Run | CharacterAnimationMode::Sprint => next_mode = WeaponAnimationMode::Run,
                         _ => next_mode = WeaponAnimationMode::Idle,
                    }
                    is_looping = true;
                }

                // Handle state transition
                if next_mode != anim_state.current_mode {
                    anim_state.previous_mode = anim_state.current_mode;
                    anim_state.current_mode = next_mode;
                    anim_state.is_looping = is_looping;
                    
                    // Reset timer or trigger animation player here
                    anim_state.timer = 0.0;
                    
                    // TODO: Trigger actual Bevy AnimationPlayer if present
                }

                anim_state.timer += dt;
            }
        }
    }
}
