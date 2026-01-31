//! Weapon manager system
//!
//! Manages weapon inventory, switching, pockets, and dual weapons

use bevy::prelude::*;
use crate::input::InputState;
use crate::character::Player;
use super::types::{Weapon, WeaponPocket, WeaponListOnPocket, PocketType};
use super::attachments::{WeaponAttachmentSystem, AttachmentStatModifiers};
use std::collections::HashMap;

/// Weapon manager component
/// Manages weapon inventory, switching, pockets, and dual weapons
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct WeaponManager {
    // Core weapon management
    pub weapons_list: Vec<Entity>,
    pub current_index: usize,
    pub weapons_slots_amount: u32,
    pub weapons_mode_active: bool,
    pub any_weapon_available: bool,
    pub weapon_list_count: usize,

    // Dual weapon support
    pub using_dual_weapon: bool,
    pub current_right_weapon_index: usize,
    pub current_left_weapon_index: usize,
    pub dual_weapons_enabled: bool,
    pub current_right_weapon_name: String,
    pub current_left_weapon_name: String,

    // Weapon pockets (organized storage)
    pub weapon_pockets: Vec<WeaponPocket>,
    pub weapon_pockets_map: HashMap<String, usize>, // pocket_id -> index in weapon_pockets

    // Weapon state flags
    pub carrying_weapon_in_third_person: bool,
    pub carrying_weapon_in_first_person: bool,
    pub aiming_in_third_person: bool,
    pub aiming_in_first_person: bool,
    pub shooting_single_weapon: bool,
    pub shooting_right_weapon: bool,
    pub shooting_left_weapon: bool,
    pub reloading_with_animation: bool,
    pub refueling_active: bool,

    // Transition states
    pub changing_weapon: bool,
    pub keeping_weapon: bool,
    pub changing_dual_weapon: bool,
    pub changing_single_weapon: bool,

    // Input settings
    pub change_weapons_with_keys: bool,
    pub change_weapons_with_mouse_wheel: bool,
    pub change_weapons_with_number_keys: bool,

    // Timers and Last Used
    pub last_time_draw_weapon: f32,
    pub last_time_reload: f32,
    pub last_time_used: f32,
    pub last_time_fired: f32,
    pub last_time_moved: f32,

    // Selection
    pub choosed_weapon: usize,
    pub choose_dual_weapon_index: i32,

    // Aim Assist
    pub use_aim_assist_in_third_person: bool,
    pub use_aim_assist_in_first_person: bool,
    pub aim_assist_look_at_target_speed: f32,
    pub check_aim_assist_state: bool,

    // Character State integration
    pub player_on_ground: bool,
    pub can_move: bool,
    pub player_is_dead: bool,
    pub player_currently_busy: bool,
    pub run_when_aiming_weapon_in_third_person: bool,
    pub aim_mode_input_pressed: bool,
    pub reloading_with_animation_active: bool,

    // Inventory & Ammo
    pub use_ammo_from_inventory: bool,
    pub change_to_next_weapon_if_ammo_empty: bool,

    // Drop settings
    pub can_drop_weapons: bool,
    pub drop_current_weapon_when_die: bool,
    pub drop_all_weapons_when_die: bool,
    pub drop_weapons_only_if_using: bool,

    // Draw settings
    pub draw_weapon_when_picked: bool,
    pub draw_picked_weapon_only_if_not_previous: bool,
    pub change_to_next_weapon_when_equipped: bool,
    pub change_to_next_weapon_when_unequipped: bool,
    pub change_to_next_weapon_when_drop: bool,

    // Quick draw
    pub use_quick_draw_weapon: bool,
    pub keep_weapon_after_delay_third_person: bool,
    pub keep_weapon_after_delay_first_person: bool,
    pub keep_weapon_delay: f32,

    // Stats
    pub damage_multiplier_stat: f32,
    pub extra_damage_stat: f32,
    pub spread_multiplier_stat: f32,
    pub fire_rate_multiplier_stat: f32,
    pub extra_reload_speed_stat: f32,
    pub magazine_extra_size_stat: i32,

    // Debug
    pub show_debug_log: bool,

    // Selection Menu Support
    pub selecting_weapon: bool,
    pub selecting_right_weapon: bool,
}

impl Default for WeaponManager {
    fn default() -> Self {
        Self {
            weapons_list: Vec::new(),
            current_index: 0,
            weapons_slots_amount: 10,
            weapons_mode_active: true,
            any_weapon_available: false,
            weapon_list_count: 0,
            using_dual_weapon: false,
            current_right_weapon_index: 0,
            current_left_weapon_index: 0,
            dual_weapons_enabled: true,
            current_right_weapon_name: String::new(),
            current_left_weapon_name: String::new(),
            weapon_pockets: Vec::new(),
            weapon_pockets_map: HashMap::new(),
            carrying_weapon_in_third_person: false,
            carrying_weapon_in_first_person: false,
            aiming_in_third_person: false,
            aiming_in_first_person: false,
            shooting_single_weapon: false,
            shooting_right_weapon: false,
            shooting_left_weapon: false,
            reloading_with_animation: false,
            refueling_active: false,
            changing_weapon: false,
            keeping_weapon: false,
            changing_dual_weapon: false,
            changing_single_weapon: false,
            change_weapons_with_keys: true,
            change_weapons_with_mouse_wheel: true,
            change_weapons_with_number_keys: true,
            last_time_draw_weapon: 0.0,
            last_time_reload: 0.0,
            last_time_used: 0.0,
            last_time_fired: 0.0,
            last_time_moved: 0.0,
            choosed_weapon: 0,
            choose_dual_weapon_index: -1,
            use_aim_assist_in_third_person: false,
            use_aim_assist_in_first_person: false,
            aim_assist_look_at_target_speed: 4.0,
            check_aim_assist_state: false,
            player_on_ground: true,
            can_move: true,
            player_is_dead: false,
            player_currently_busy: false,
            run_when_aiming_weapon_in_third_person: false,
            aim_mode_input_pressed: false,
            reloading_with_animation_active: false,
            use_ammo_from_inventory: false,
            change_to_next_weapon_if_ammo_empty: false,
            can_drop_weapons: true,
            drop_current_weapon_when_die: true,
            drop_all_weapons_when_die: false,
            drop_weapons_only_if_using: true,
            draw_weapon_when_picked: true,
            draw_picked_weapon_only_if_not_previous: false,
            change_to_next_weapon_when_equipped: true,
            change_to_next_weapon_when_unequipped: true,
            change_to_next_weapon_when_drop: true,
            use_quick_draw_weapon: false,
            keep_weapon_after_delay_third_person: false,
            keep_weapon_after_delay_first_person: false,
            keep_weapon_delay: 2.0,
            damage_multiplier_stat: 1.0,
            extra_damage_stat: 0.0,
            spread_multiplier_stat: 1.0,
            fire_rate_multiplier_stat: 1.0,
            extra_reload_speed_stat: 1.0,
            magazine_extra_size_stat: 0,
            show_debug_log: false,

            selecting_weapon: false,
            selecting_right_weapon: true, // Default to right hand
        }
    }
}

/// Handle weapon switching via input
pub fn handle_weapon_switching(
    time: Res<Time>,
    mut query: Query<(&InputState, &mut WeaponManager)>,
) {
    for (input, mut manager) in query.iter_mut() {
        // Skip if weapon is moving or reloading
        if manager.changing_weapon || manager.reloading_with_animation {
            continue;
        }

        let mut should_switch = false;
        let mut switch_direction = 0; // 1 = next, -1 = previous

        // Check for next weapon input
        if input.next_weapon_pressed {
            should_switch = true;
            switch_direction = 1;
        }
        // Check for previous weapon input
        else if input.prev_weapon_pressed {
            should_switch = true;
            switch_direction = -1;
        }
        // Check for direct weapon selection
        else if let Some(target_index) = input.select_weapon {
            if target_index < manager.weapons_list.len() {
                if manager.selecting_weapon {
                    // Update right/left weapon indices if in selection mode
                    if manager.selecting_right_weapon {
                        if target_index != manager.current_right_weapon_index {
                            manager.current_right_weapon_index = target_index;
                        }
                    } else {
                        if target_index != manager.current_left_weapon_index {
                            manager.current_left_weapon_index = target_index;
                        }
                    }
                    
                    if manager.show_debug_log {
                        info!("Selection Menu: Set {} hand weapon to index {}", 
                            if manager.selecting_right_weapon { "right" } else { "left" }, 
                            target_index);
                    }
                } else if target_index != manager.current_index {
                    // Normal weapon switching
                    manager.current_index = target_index;
                    manager.choosed_weapon = manager.current_index;
                    
                    if manager.carrying_weapon_in_third_person || manager.carrying_weapon_in_first_person {
                        manager.changing_weapon = true;
                        manager.keeping_weapon = false;
                    }
                    
                    if manager.show_debug_log {
                        info!("Selecting weapon index: {}", manager.current_index);
                    }
                }
                continue; // Skip the directional logic below
            }
        }

        if should_switch && !manager.weapons_list.is_empty() {
            // Update current index based on direction
            if switch_direction == 1 {
                manager.current_index = (manager.current_index + 1) % manager.weapons_list.len();
            } else {
                if manager.current_index == 0 {
                    manager.current_index = manager.weapons_list.len().saturating_sub(1);
                } else {
                    manager.current_index -= 1;
                }
            }

            // Update choosed_weapon for compatibility
            manager.choosed_weapon = manager.current_index;

            // Set changing weapon state
            if manager.carrying_weapon_in_third_person || manager.carrying_weapon_in_first_person {
                manager.changing_weapon = true;
                manager.keeping_weapon = false;
            }

            // Log for debug
            if manager.show_debug_log {
                info!("Switching to weapon index: {}", manager.current_index);
            }
        }
    }
}

/// Handle weapon manager input (draw/keep/aim)
pub fn handle_weapon_manager_input(
    time: Res<Time>,
    mut query: Query<(&InputState, &mut WeaponManager), With<Player>>,
) {
    for (input, mut manager) in query.iter_mut() {
        // Skip if weapons mode is not active
        if !manager.weapons_mode_active {
            continue;
        }

        // Draw/Keep weapon input (toggle)
        if input.fire_pressed && !manager.carrying_weapon_in_third_person && !manager.carrying_weapon_in_first_person {
            // Draw weapon
            if manager.use_quick_draw_weapon && !manager.aiming_in_third_person {
                // Quick draw
                manager.carrying_weapon_in_third_person = true;
                manager.changing_weapon = false;
                manager.keeping_weapon = false;
            } else {
                // Normal draw
                manager.carrying_weapon_in_third_person = true;
                manager.changing_weapon = true;
                manager.keeping_weapon = false;
            }
            manager.last_time_draw_weapon = time.elapsed_secs();
        }

        // Aim weapon input
        if input.aim_pressed {
            if manager.carrying_weapon_in_third_person || manager.carrying_weapon_in_first_person {
                //  aiming usually toggles or holds based on settings. 
                // Here we'll stick to the existing toggle logic but ensure it's responsive.
                manager.aiming_in_third_person = !manager.aiming_in_third_person;
                manager.aiming_in_first_person = !manager.aiming_in_first_person; // Mirror for 1P
                manager.aim_mode_input_pressed = manager.aiming_in_third_person || manager.aiming_in_first_person;
            }
        }

        // Reload weapon input
        if input.reload_pressed {
            manager.reloading_with_animation_active = true;
            manager.last_time_reload = time.elapsed_secs();
        }

        // Update last time used
        if input.fire_pressed || input.aim_pressed || input.reload_pressed {
            manager.last_time_used = time.elapsed_secs();
        }
    }
}

/// Update weapon manager state
pub fn update_weapon_manager(
    time: Res<Time>,
    mut manager_query: Query<(Entity, &mut WeaponManager)>,
    mut weapon_query: Query<(&mut Weapon, &mut Visibility)>,
) {
    for (player_entity, mut manager) in manager_query.iter_mut() {
        // Update player on ground state (simplified - would normally come from character controller)
        manager.player_on_ground = true;

        // Update can move state (simplified - would normally check if player is dead/busy)
        manager.can_move = !manager.player_is_dead && !manager.player_currently_busy;

        // Handle changing weapon state (entity activation/deactivation)
        if manager.changing_weapon {
            if !manager.keeping_weapon {
                // Keep weapon (hide current)
                if manager.carrying_weapon_in_third_person || manager.carrying_weapon_in_first_person {
                    manager.carrying_weapon_in_third_person = false;
                    manager.carrying_weapon_in_first_person = false;
                } else {
                    // Draw weapon
                    manager.carrying_weapon_in_third_person = true;
                }
                manager.keeping_weapon = true;
            }

            // Apply visibility changes to weapon entities
            for (i, &weapon_entity) in manager.weapons_list.iter().enumerate() {
                if let Ok((mut weapon, mut visibility)) = weapon_query.get_mut(weapon_entity) {
                    if i == manager.current_index && manager.carrying_weapon_in_third_person {
                        *visibility = Visibility::Inherited;
                    } else {
                        *visibility = Visibility::Hidden;
                    }
                }
            }

            // Check if weapon change is complete
            if !manager.changing_dual_weapon && !manager.changing_single_weapon {
                // Single weapon change complete
                manager.changing_weapon = false;
                manager.keeping_weapon = false;

                // Update current weapon info
                if manager.show_debug_log {
                    info!("Weapon change complete at index {}", manager.current_index);
                }
            }
        }

        // Handle dual weapon changing
        if manager.changing_dual_weapon {
            if !manager.keeping_weapon {
                // Keep dual weapons first
                manager.carrying_weapon_in_third_person = false;
                manager.keeping_weapon = true;
            }

            // Check if both weapons are ready
            if manager.keeping_weapon {
                // Draw dual weapons
                manager.carrying_weapon_in_third_person = true;
                manager.changing_dual_weapon = false;
                manager.keeping_weapon = false;
                manager.using_dual_weapon = true;
            }
        }

        // Handle single weapon changing from dual
        if manager.changing_single_weapon {
            if !manager.keeping_weapon {
                // Keep dual weapons
                manager.carrying_weapon_in_third_person = false;
                manager.keeping_weapon = true;
            }

            // Check if ready to switch to single
            if manager.keeping_weapon {
                // Switch to single weapon
                manager.using_dual_weapon = false;
                manager.changing_single_weapon = false;
                manager.keeping_weapon = false;
                manager.carrying_weapon_in_third_person = true;
                manager.changing_weapon = true;
            }
        }

        // Update reload timer for current weapon
        if manager.reloading_with_animation_active {
            if let Some(&weapon_entity) = manager.weapons_list.get(manager.current_index) {
                if let Ok((mut weapon, _)) = weapon_query.get_mut(weapon_entity) {
                    if time.elapsed_secs() - manager.last_time_reload > weapon.reload_time {
                        manager.reloading_with_animation_active = false;
                        weapon.current_ammo = weapon.ammo_capacity;
                        weapon.is_reloading = false;
                        if manager.show_debug_log {
                            info!("Reload complete for {}", weapon.weapon_name);
                        }
                    } else {
                        weapon.is_reloading = true;
                    }
                }
            }
        }

        // Check if weapon is out of ammo and should switch
        if manager.change_to_next_weapon_if_ammo_empty && !manager.reloading_with_animation_active {
             if let Some(&weapon_entity) = manager.weapons_list.get(manager.current_index) {
                if let Ok((weapon, _)) = weapon_query.get(weapon_entity) {
                    if weapon.current_ammo <= 0 {
                        if manager.weapons_list.len() > 1 {
                            manager.changing_weapon = true;
                            manager.current_index = (manager.current_index + 1) % manager.weapons_list.len();
                            if manager.show_debug_log {
                                info!("Out of ammo, switching to next weapon");
                            }
                        }
                    }
                }
            }
        }

        // Update last time fired
        if manager.shooting_single_weapon || manager.shooting_right_weapon || manager.shooting_left_weapon {
            manager.last_time_fired = time.elapsed_secs();
        }

        // Update aim assist state
        if manager.use_aim_assist_in_third_person && manager.aiming_in_third_person {
            manager.check_aim_assist_state = true;
        } else {
            manager.check_aim_assist_state = false;
        }

        // Update any weapon available
        manager.any_weapon_available = !manager.weapons_list.is_empty();

        // Update weapon list count
        manager.weapon_list_count = manager.weapons_list.len();
    }
}

// ============================================================================
// Weapon Pocket Management Methods
// ============================================================================

impl WeaponManager {
    /// Add a weapon pocket to the manager
    pub fn add_pocket(&mut self, pocket: WeaponPocket) -> Result<(), String> {
        if self.weapon_pockets_map.contains_key(&pocket.id) {
            return Err(format!("Pocket with id '{}' already exists", pocket.id));
        }

        let index = self.weapon_pockets.len();
        self.weapon_pockets.push(pocket.clone());
        self.weapon_pockets_map.insert(pocket.id.clone(), index);
        Ok(())
    }

    /// Get a pocket by ID
    pub fn get_pocket(&self, pocket_id: &str) -> Option<&WeaponPocket> {
        self.weapon_pockets_map.get(pocket_id)
            .and_then(|&index| self.weapon_pockets.get(index))
    }

    /// Get a mutable pocket by ID
    pub fn get_pocket_mut(&mut self, pocket_id: &str) -> Option<&mut WeaponPocket> {
        if let Some(&index) = self.weapon_pockets_map.get(pocket_id) {
            self.weapon_pockets.get_mut(index)
        } else {
            None
        }
    }

    /// Add a weapon to a pocket
    pub fn add_weapon_to_pocket(&mut self, weapon_id: &str, pocket_id: &str) -> Result<(), String> {
        if let Some(pocket) = self.get_pocket_mut(pocket_id) {
            if !pocket.add_weapon(weapon_id) {
                return Err(format!(
                    "Cannot add weapon '{}' to pocket '{}': {}",
                    weapon_id,
                    pocket_id,
                    if !pocket.has_room() {
                        "pocket is full"
                    } else {
                        "weapon already in pocket"
                    }
                ));
            }
            Ok(())
        } else {
            Err(format!("Pocket '{}' not found", pocket_id))
        }
    }

    /// Remove a weapon from a pocket
    pub fn remove_weapon_from_pocket(&mut self, weapon_id: &str, pocket_id: &str) -> Result<(), String> {
        if let Some(pocket) = self.get_pocket_mut(pocket_id) {
            if !pocket.remove_weapon(weapon_id) {
                return Err(format!("Weapon '{}' not found in pocket '{}'", weapon_id, pocket_id));
            }
            Ok(())
        } else {
            Err(format!("Pocket '{}' not found", pocket_id))
        }
    }

    /// Get weapon pocket for a weapon
    pub fn get_weapon_pocket(&self, weapon_id: &str) -> Option<&WeaponPocket> {
        for pocket in &self.weapon_pockets {
            if pocket.contains_weapon(weapon_id) {
                return Some(pocket);
            }
        }
        None
    }

    /// Get all weapons in a pocket
    pub fn get_weapons_in_pocket(&self, pocket_id: &str) -> Vec<String> {
        if let Some(pocket) = self.get_pocket(pocket_id) {
            pocket.weapon_ids.clone()
        } else {
            Vec::new()
        }
    }

    /// Get all pockets
    pub fn get_all_pockets(&self) -> Vec<&WeaponPocket> {
        self.weapon_pockets.iter().collect()
    }

    /// Get pocket by type
    pub fn get_pocket_by_type(&self, pocket_type: &PocketType) -> Option<&WeaponPocket> {
        self.weapon_pockets.iter().find(|p| &p.pocket_type == pocket_type)
    }

    /// Get number of pockets
    pub fn pocket_count(&self) -> usize {
        self.weapon_pockets.len()
    }

    /// Clear all pockets
    pub fn clear_pockets(&mut self) {
        self.weapon_pockets.clear();
        self.weapon_pockets_map.clear();
    }

    /// Set weapon selection menu state
    pub fn set_selecting_weapon_state(&mut self, state: bool) {
        self.selecting_weapon = state;
        if self.show_debug_log {
            info!("Weapon selection menu: {}", if state { "Opened" } else { "Closed" });
        }
    }

    /// Set which hand is being configured in selection menu
    pub fn set_right_or_left_weapon(&mut self, is_right: bool) {
        if !self.dual_weapons_enabled {
            return;
        }
        self.selecting_right_weapon = is_right;
        if self.show_debug_log {
            info!("Selecting weapon for hand: {}", if is_right { "Right" } else { "Left" });
        }
    }

    /// Create default pockets
    pub fn create_default_pockets(&mut self) -> Result<(), String> {
        let default_pockets = vec![
            WeaponPocket::new("primary", "Primary Weapons", 3, PocketType::Primary),
            WeaponPocket::new("secondary", "Secondary Weapons", 3, PocketType::Secondary),
            WeaponPocket::new("melee", "Melee Weapons", 2, PocketType::Melee),
            WeaponPocket::new("special", "Special Weapons", 2, PocketType::Special),
            WeaponPocket::new("grenade", "Grenades", 3, PocketType::Grenade),
        ];

        for pocket in default_pockets {
            self.add_pocket(pocket)?;
        }
        Ok(())
    }

    // ============================================================================
    // Weapon Attachment Management Methods
    // ============================================================================

    /// Open or close the weapon attachment editor
    pub fn open_attachment_editor(
        &mut self,
        weapon_entity: Entity,
        commands: &mut Commands,
        attachment_query: &mut Query<&mut WeaponAttachmentSystem>,
    ) -> Result<(), String> {
        if let Ok(mut attachment_system) = attachment_query.get_mut(weapon_entity) {
            attachment_system.editing_attachments = !attachment_system.editing_attachments;

            if attachment_system.editing_attachments {
                info!("Opening attachment editor");
            } else {
                info!("Closing attachment editor");
            }

            Ok(())
        } else {
            Err("Weapon attachment system not found".to_string())
        }
    }

    /// Select an attachment for a weapon
    pub fn select_attachment(
        &mut self,
        weapon_entity: Entity,
        place_id: &str,
        attachment_id: &str,
        commands: &mut Commands,
        weapon_query: &mut Query<&mut Weapon>,
        attachment_query: &mut Query<&mut WeaponAttachmentSystem>,
    ) -> Result<(), String> {
        // Get the attachment system
        let mut attachment_system = match attachment_query.get_mut(weapon_entity) {
            Ok(system) => system,
            Err(_) => return Err("Weapon attachment system not found".to_string()),
        };

        // Find the attachment place
        let place_index = match attachment_system
            .attachment_places
            .iter()
            .position(|p| p.id == place_id)
        {
            Some(index) => index,
            None => return Err(format!("Attachment place '{}' not found", place_id)),
        };

        // Find the attachment
        let attachment_index = match attachment_system.attachment_places[place_index]
            .available_attachments
            .iter()
            .position(|a| a.id == attachment_id)
        {
            Some(index) => index,
            None => return Err(format!("Attachment '{}' not found", attachment_id)),
        };

        // Check if attachment is enabled
        if !attachment_system.attachment_places[place_index].available_attachments[attachment_index]
            .enabled
        {
            return Err(format!("Attachment '{}' is not enabled", attachment_id));
        }

        // Deactivate previous attachment
        let previous_selection = attachment_system.attachment_places[place_index].current_selection;
        if previous_selection >= 0 {
            attachment_system.attachment_places[place_index]
                .available_attachments
                .get_mut(previous_selection as usize)
                .unwrap()
                .active = false;
        }

        // Activate new attachment
        attachment_system.attachment_places[place_index]
            .available_attachments
            .get_mut(attachment_index)
            .unwrap()
            .active = true;
        attachment_system.attachment_places[place_index].current_selection = attachment_index as i32;

        // Update selected attachments map
        attachment_system.selected_attachments.insert(
            place_id.to_string(),
            attachment_id.to_string(),
        );

        // Apply attachment modifiers to weapon
        if let Ok(mut weapon) = weapon_query.get_mut(weapon_entity) {
            let attachment = &attachment_system.attachment_places[place_index]
                .available_attachments[attachment_index];

            // First remove previous attachment modifiers
            if previous_selection >= 0 {
                let prev_attachment = &attachment_system.attachment_places[place_index]
                    .available_attachments[previous_selection as usize];
                prev_attachment.stat_modifiers.remove_from_weapon(&mut weapon);
            }

            // Then apply new attachment modifiers
            attachment.stat_modifiers.apply_to_weapon(&mut weapon);

            info!(
                "Applied attachment '{}' to weapon. New stats: damage={}, spread={}",
                attachment.name, weapon.damage, weapon.spread
            );
        }

        info!(
            "Selected attachment '{}' for place '{}'",
            attachment_id, place_id
        );

        Ok(())
    }

    /// Remove an attachment from a weapon
    pub fn remove_attachment(
        &mut self,
        weapon_entity: Entity,
        place_id: &str,
        commands: &mut Commands,
        weapon_query: &mut Query<&mut Weapon>,
        attachment_query: &mut Query<&mut WeaponAttachmentSystem>,
    ) -> Result<(), String> {
        // Get the attachment system
        let mut attachment_system = match attachment_query.get_mut(weapon_entity) {
            Ok(system) => system,
            Err(_) => return Err("Weapon attachment system not found".to_string()),
        };

        // Find the attachment place
        let place_index = match attachment_system
            .attachment_places
            .iter()
            .position(|p| p.id == place_id)
        {
            Some(index) => index,
            None => return Err(format!("Attachment place '{}' not found", place_id)),
        };

        // Get current selection
        let current_selection = attachment_system.attachment_places[place_index].current_selection;

        if current_selection < 0 {
            return Err(format!("No attachment selected for place '{}'", place_id));
        }

        // Deactivate current attachment
        let attachment = &mut attachment_system.attachment_places[place_index]
            .available_attachments[current_selection as usize];

        // Remove modifiers from weapon
        if let Ok(mut weapon) = weapon_query.get_mut(weapon_entity) {
            attachment.stat_modifiers.remove_from_weapon(&mut weapon);
        }

        attachment.active = false;

        // Clear selection
        attachment_system.attachment_places[place_index].current_selection = -1;
        attachment_system.selected_attachments.remove(place_id);

        info!("Removed attachment from place '{}'", place_id);

        Ok(())
    }

    /// Get current attachment for a place
    pub fn get_current_attachment(
        &self,
        weapon_entity: Entity,
        place_id: &str,
        attachment_query: &Query<&WeaponAttachmentSystem>,
    ) -> Option<String> {
        if let Ok(attachment_system) = attachment_query.get(weapon_entity) {
            attachment_system.selected_attachments.get(place_id).cloned()
        } else {
            None
        }
    }

    /// Check if a weapon has attachments
    pub fn has_attachments(
        &self,
        weapon_entity: Entity,
        attachment_query: &Query<&WeaponAttachmentSystem>,
    ) -> bool {
        if let Ok(attachment_system) = attachment_query.get(weapon_entity) {
            attachment_system.attachments_active && !attachment_system.attachment_places.is_empty()
        } else {
            false
        }
    }

    /// Toggle attachments active state
    pub fn toggle_attachments(
        &mut self,
        weapon_entity: Entity,
        active: bool,
        attachment_query: &mut Query<&mut WeaponAttachmentSystem>,
    ) -> Result<(), String> {
        if let Ok(mut attachment_system) = attachment_query.get_mut(weapon_entity) {
            attachment_system.attachments_active = active;
            info!("Attachments {} for weapon", if active { "enabled" } else { "disabled" });
            Ok(())
        } else {
            Err("Weapon attachment system not found".to_string())
        }
    }

    // ============================================================================
    // Weapon Selection and Quick Access Methods
    // ============================================================================

    /// Select a weapon by its index in the weapons list
    pub fn select_weapon_by_index(
        &mut self,
        index: usize,
        commands: &mut Commands,
        weapon_query: &mut Query<(&mut Weapon, &mut Visibility)>,
    ) -> Result<(), String> {
        if index >= self.weapons_list.len() {
            return Err(format!("Weapon index {} out of range", index));
        }

        let weapon_entity = self.weapons_list[index];

        // Update current index
        self.current_index = index;
        self.choosed_weapon = index;

        // Update visibility for all weapons
        for (i, &weapon_ent) in self.weapons_list.iter().enumerate() {
            if let Ok((_, mut visibility)) = weapon_query.get_mut(weapon_ent) {
                if i == index && self.carrying_weapon_in_third_person {
                    *visibility = Visibility::Inherited;
                } else {
                    *visibility = Visibility::Hidden;
                }
            }
        }

        // Set changing weapon state
        if self.carrying_weapon_in_third_person || self.carrying_weapon_in_first_person {
            self.changing_weapon = true;
            self.keeping_weapon = false;
        }

        if self.show_debug_log {
            info!("Selected weapon at index {}: {}", index, self.get_weapon_name(weapon_entity));
        }

        Ok(())
    }

    /// Select a weapon by its key number (1-10)
    pub fn select_weapon_by_key(
        &mut self,
        key_number: u8,
        commands: &mut Commands,
        weapon_query: &mut Query<(&mut Weapon, &mut Visibility)>,
    ) -> Result<(), String> {
        if key_number < 1 || key_number > 10 {
            return Err(format!("Key number {} out of range (1-10)", key_number));
        }

        // Find weapon with matching key number
        for (index, &weapon_entity) in self.weapons_list.iter().enumerate() {
            if let Ok((weapon, _)) = weapon_query.get(weapon_entity) {
                if weapon.key_number == key_number {
                    return self.select_weapon_by_index(index, commands, weapon_query);
                }
            }
        }

        Err(format!("No weapon assigned to key {}", key_number))
    }

    /// Assign a weapon to a quick access slot
    pub fn assign_weapon_to_slot(
        &mut self,
        weapon_entity: Entity,
        slot_number: u8,
        weapon_query: &mut Query<&mut Weapon>,
    ) -> Result<(), String> {
        if slot_number < 1 || slot_number > 10 {
            return Err(format!("Slot number {} out of range (1-10)", slot_number));
        }

        // Check if weapon exists in list
        let weapon_index = match self.weapons_list.iter().position(|&e| e == weapon_entity) {
            Some(index) => index,
            None => return Err("Weapon not found in manager".to_string()),
        };

        // Update weapon's key number
        if let Ok(mut weapon) = weapon_query.get_mut(weapon_entity) {
            weapon.key_number = slot_number;
        }

        if self.show_debug_log {
            info!(
                "Assigned weapon to slot {}: {}",
                slot_number,
                self.get_weapon_name(weapon_entity)
            );
        }

        Ok(())
    }

    /// Get weapon assigned to a specific slot
    pub fn get_weapon_from_slot(
        &self,
        slot_number: u8,
        weapon_query: &Query<&Weapon>,
    ) -> Option<Entity> {
        if slot_number < 1 || slot_number > 10 {
            return None;
        }

        for &weapon_entity in &self.weapons_list {
            if let Ok(weapon) = weapon_query.get(weapon_entity) {
                if weapon.key_number == slot_number {
                    return Some(weapon_entity);
                }
            }
        }

        None
    }

    /// Get all weapons with their slot assignments
    pub fn get_all_weapon_slots(
        &self,
        weapon_query: &Query<&mut Weapon>,
    ) -> Vec<(u8, Entity, String)> {
        let mut slots = Vec::new();

        for &weapon_entity in &self.weapons_list {
            if let Ok(weapon) = weapon_query.get(weapon_entity) {
                if weapon.key_number > 0 {
                    slots.push((
                        weapon.key_number,
                        weapon_entity,
                        weapon.weapon_name.clone(),
                    ));
                }
            }
        }

        // Sort by slot number
        slots.sort_by_key(|(slot, _, _)| *slot);

        slots
    }

    /// Clear all slot assignments
    pub fn clear_all_slots(&mut self, weapon_query: &mut Query<&mut Weapon>) -> Result<(), String> {
        for &weapon_entity in &self.weapons_list {
            if let Ok(mut weapon) = weapon_query.get_mut(weapon_entity) {
                weapon.key_number = 0;
            }
        }

        if self.show_debug_log {
            info!("Cleared all weapon slot assignments");
        }

        Ok(())
    }

    /// Select next weapon in list
    pub fn select_next_weapon(
        &mut self,
        commands: &mut Commands,
        weapon_query: &mut Query<(&mut Weapon, &mut Visibility)>,
    ) -> Result<(), String> {
        if self.weapons_list.is_empty() {
            return Err("No weapons available".to_string());
        }

        let next_index = (self.current_index + 1) % self.weapons_list.len();
        self.select_weapon_by_index(next_index, commands, weapon_query)
    }

    /// Select previous weapon in list
    pub fn select_previous_weapon(
        &mut self,
        commands: &mut Commands,
        weapon_query: &mut Query<(&mut Weapon, &mut Visibility)>,
    ) -> Result<(), String> {
        if self.weapons_list.is_empty() {
            return Err("No weapons available".to_string());
        }

        let prev_index = if self.current_index == 0 {
            self.weapons_list.len() - 1
        } else {
            self.current_index - 1
        };

        self.select_weapon_by_index(prev_index, commands, weapon_query)
    }

    /// Select weapon by name
    pub fn select_weapon_by_name(
        &mut self,
        name: &str,
        commands: &mut Commands,
        weapon_query: &mut Query<(&mut Weapon, &mut Visibility)>,
    ) -> Result<(), String> {
        for (index, &weapon_entity) in self.weapons_list.iter().enumerate() {
            if let Ok(weapon_data) = weapon_query.get(weapon_entity) {
                if weapon_data.0.weapon_name == name {
                    return self.select_weapon_by_index(index, commands, weapon_query);
                }
            }
        }

        Err(format!("Weapon '{}' not found", name))
    }

    /// Get the name of a weapon entity
    fn get_weapon_name(&self, weapon_entity: Entity) -> String {
        // This is a helper - in real usage, you'd pass the query
        format!("Weapon {:?}", weapon_entity)
    }

    /// Check if a weapon is currently selected
    pub fn is_weapon_selected(&self, weapon_entity: Entity) -> bool {
        if self.current_index >= self.weapons_list.len() {
            return false;
        }
        self.weapons_list[self.current_index] == weapon_entity
    }

    /// Get the currently selected weapon entity
    pub fn get_current_weapon(&self) -> Option<Entity> {
        if self.current_index < self.weapons_list.len() {
            Some(self.weapons_list[self.current_index])
        } else {
            None
        }
    }

    /// Get all available weapons (for UI display)
    pub fn get_available_weapons(&self, weapon_query: &Query<&Weapon>) -> Vec<(usize, Entity, String, bool)> {
        let mut weapons = Vec::new();

        for (index, &weapon_entity) in self.weapons_list.iter().enumerate() {
            if let Ok(weapon) = weapon_query.get(weapon_entity) {
                if weapon.enabled {
                    weapons.push((
                        index,
                        weapon_entity,
                        weapon.weapon_name.clone(),
                        index == self.current_index,
                    ));
                }
            }
        }

        weapons
    }

    /// Add a weapon to the quick access list
    pub fn add_weapon_to_quick_access(
        &mut self,
        weapon_entity: Entity,
        slot_number: u8,
        weapon_query: &mut Query<&mut Weapon>,
    ) -> Result<(), String> {
        // Check if weapon is already in the list
        if !self.weapons_list.contains(&weapon_entity) {
            self.weapons_list.push(weapon_entity);
        }

        // Assign to slot
        self.assign_weapon_to_slot(weapon_entity, slot_number, weapon_query)
    }

    /// Remove weapon from quick access
    pub fn remove_weapon_from_quick_access(
        &mut self,
        weapon_entity: Entity,
        weapon_query: &mut Query<&mut Weapon>,
    ) -> Result<(), String> {
        // Clear slot assignment
        if let Ok(mut weapon) = weapon_query.get_mut(weapon_entity) {
            weapon.key_number = 0;
        }

        // Remove from list
        if let Some(index) = self.weapons_list.iter().position(|&e| e == weapon_entity) {
            self.weapons_list.remove(index);

            // Adjust current index if needed
            if self.current_index >= self.weapons_list.len() {
                self.current_index = 0;
            }

            if self.show_debug_log {
                info!("Removed weapon from quick access");
            }

            Ok(())
        } else {
            Err("Weapon not found in manager".to_string())
        }
    }

    /// Get weapon info for UI display
    pub fn get_weapon_info_for_ui(
        &self,
        weapon_entity: Entity,
        weapon_query: &Query<&mut Weapon>,
    ) -> Option<WeaponUIInfo> {
        if let Ok(weapon) = weapon_query.get(weapon_entity) {
            Some(WeaponUIInfo {
                name: weapon.weapon_name.clone(),
                key_number: weapon.key_number,
                current_ammo: weapon.current_ammo,
                max_ammo: weapon.ammo_capacity,
                is_reloading: weapon.is_reloading,
                is_selected: self.is_weapon_selected(weapon_entity),
                enabled: weapon.enabled,
                equipped: weapon.equipped,
            })
        } else {
            None
        }
    }
}

/// Information about a weapon for UI display
#[derive(Debug, Clone)]
pub struct WeaponUIInfo {
    pub name: String,
    pub key_number: u8,
    pub current_ammo: i32,
    pub max_ammo: i32,
    pub is_reloading: bool,
    pub is_selected: bool,
    pub enabled: bool,
    pub equipped: bool,
}

/// System to handle weapon selection via number keys
pub fn handle_weapon_selection_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut manager_query: Query<(&mut WeaponManager, &mut WeaponAttachmentSystem)>,
    mut weapon_query: Query<(&mut Weapon, &mut Visibility)>,
    mut commands: Commands,
) {
    for (mut manager, mut attachment_system) in manager_query.iter_mut() {
        // Skip if attachment editor is open
        if attachment_system.editing_attachments {
            continue;
        }

        // Skip if weapon is changing or reloading
        if manager.changing_weapon || manager.reloading_with_animation_active {
            continue;
        }

        // Handle number keys 1-10
        let keys = [
            (KeyCode::Digit1, 1),
            (KeyCode::Digit2, 2),
            (KeyCode::Digit3, 3),
            (KeyCode::Digit4, 4),
            (KeyCode::Digit5, 5),
            (KeyCode::Digit6, 6),
            (KeyCode::Digit7, 7),
            (KeyCode::Digit8, 8),
            (KeyCode::Digit9, 9),
            (KeyCode::Digit0, 10),
        ];

        for (key, slot_number) in keys {
            if keyboard_input.just_pressed(key) {
                // Check if weapon is assigned to this slot
                let mut weapon_entity = None;
                for &ent in &manager.weapons_list {
                    if let Ok(weapon_data) = weapon_query.get(ent) {
                        if weapon_data.0.key_number == slot_number {
                            weapon_entity = Some(ent);
                            break;
                        }
                    }
                }
                
                if let Some(ent) = weapon_entity {
                    // Find index of this weapon
                    if let Some(index) = manager.weapons_list.iter().position(|&e| e == ent) {
                        let result = manager.select_weapon_by_index(index, &mut commands, &mut weapon_query);
                        if let Err(e) = result {
                            info!("Error selecting weapon: {}", e);
                        }
                    }
                } else {
                    if manager.show_debug_log {
                        info!("No weapon assigned to slot {}", slot_number);
                    }
                }
            }
        }

        // Handle next/previous weapon (scroll wheel or keys)
        if keyboard_input.just_pressed(KeyCode::BracketRight) || keyboard_input.just_pressed(KeyCode::Period) {
            let result = manager.select_next_weapon(&mut commands, &mut weapon_query);
            if let Err(e) = result {
                info!("Error selecting next weapon: {}", e);
            }
        }

        if keyboard_input.just_pressed(KeyCode::BracketLeft) || keyboard_input.just_pressed(KeyCode::Comma) {
            let result = manager.select_previous_weapon(&mut commands, &mut weapon_query);
            if let Err(e) = result {
                info!("Error selecting previous weapon: {}", e);
            }
        }
    }
}

/// System to display weapon selection UI
pub fn update_weapon_selection_ui(
    manager_query: Query<&WeaponManager>,
    weapon_query: Query<&Weapon>,
    mut text_query: Query<&mut Text, With<WeaponSelectionUI>>,
) {
    let Some(manager) = manager_query.iter().next() else {
        return;
    };

    let weapons = manager.get_available_weapons(&weapon_query);

    if let Some(mut text) = text_query.iter_mut().next() {
        let mut ui_text = String::from("Weapon Selection:\n");

        for (index, _, name, is_selected) in weapons {
            let marker = if is_selected { "▶" } else { " " };
            ui_text.push_str(&format!("{} [{}] {}\n", marker, index + 1, name));
        }

        text.0 = ui_text;
    }
}

/// Marker component for weapon selection UI
#[derive(Component)]
pub struct WeaponSelectionUI;

