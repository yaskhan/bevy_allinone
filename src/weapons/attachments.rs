//! Weapon attachment system
//!
//! Manages weapon attachments like scopes, silencers, extended magazines, etc.
//!
//! ## Features
//!
//! - **Attachment Slots**: Multiple attachment slots per weapon
//! - **Attachment Types**: Scope, Silencer, Extended Mag, Laser Sight, etc.
//! - **Attachment Effects**: Modify weapon stats (damage, spread, reload speed, etc.)
//! - **Attachment UI**: Visual attachment selection interface
//! - **Attachment State**: Track active/inactive attachments
//! - **Attachment Events**: Trigger events on attachment changes

use bevy::prelude::*;
use std::collections::HashMap;

/// Main attachment system component
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct WeaponAttachmentSystem {
    /// Whether the attachment editor is currently open
    pub editing_attachments: bool,
    /// Whether to use universal attachments (shared across weapons)
    pub use_universal_attachments: bool,
    /// List of attachment places available on this weapon
    pub attachment_places: Vec<AttachmentPlace>,
    /// Currently selected attachment for each place
    pub selected_attachments: HashMap<String, String>, // place_id -> attachment_id
    /// Whether attachments are active in this weapon
    pub attachments_active: bool,
    /// Whether to allow changing attachments with number keys
    pub can_change_with_number_keys: bool,
    /// Whether to show attachment hover info
    pub show_hover_info: bool,
    /// Whether to use offset panels for UI
    pub use_offset_panels: bool,
    /// Whether to use smooth camera transitions
    pub use_smooth_transition: bool,
    /// Whether to disable HUD when editing attachments
    pub disable_hud_when_editing: bool,
}

/// Represents a place where an attachment can be mounted
#[derive(Debug, Clone, Reflect, Default)]
pub struct AttachmentPlace {
    /// Unique identifier for this attachment place
    pub id: String,
    /// Display name for this attachment place
    pub name: String,
    /// Whether this attachment place is enabled
    pub enabled: bool,
    /// List of available attachments for this place
    pub available_attachments: Vec<AttachmentInfo>,
    /// Currently selected attachment index (-1 = no attachment)
    pub current_selection: i32,
    /// Text to display when no attachment is selected
    pub no_attachment_text: String,
}

/// Information about a specific attachment
#[derive(Debug, Clone, Reflect, Default)]
pub struct AttachmentInfo {
    /// Unique identifier for this attachment
    pub id: String,
    /// Display name for this attachment
    pub name: String,
    /// Description of what this attachment does
    pub description: String,
    /// Whether this attachment is currently enabled/available
    pub enabled: bool,
    /// Whether this attachment is currently active
    pub active: bool,
    /// Whether this attachment is only available while carrying the weapon
    pub only_while_carrying: bool,
    /// Stat modifiers applied by this attachment
    pub stat_modifiers: AttachmentStatModifiers,
    /// Visual model for this attachment (optional)
    pub model: Option<String>,
}

/// Stat modifiers applied by an attachment
#[derive(Debug, Clone, Reflect, Default)]
pub struct AttachmentStatModifiers {
    /// Damage multiplier (1.0 = no change)
    pub damage_multiplier: f32,
    /// Extra damage added
    pub extra_damage: f32,
    /// Spread multiplier (1.0 = no change)
    pub spread_multiplier: f32,
    /// Fire rate multiplier (1.0 = no change)
    pub fire_rate_multiplier: f32,
    /// Reload speed multiplier (1.0 = no change)
    pub reload_speed_multiplier: f32,
    /// Magazine size modifier (0 = no change)
    pub magazine_size_modifier: i32,
    /// Range multiplier (1.0 = no change)
    pub range_multiplier: f32,
    /// Recoil multiplier (1.0 = no change)
    pub recoil_multiplier: f32,
    /// Aim down sights speed multiplier (1.0 = no change)
    pub ads_speed_multiplier: f32,
    /// Movement speed multiplier while aiming (1.0 = no change)
    pub aim_movement_speed_multiplier: f32,
    /// Noise level multiplier (1.0 = no change)
    pub noise_multiplier: f32,
    /// Visual recoil multiplier (1.0 = no change)
    pub visual_recoil_multiplier: f32,
}

impl AttachmentStatModifiers {
    /// Create a new empty modifier set
    pub fn new() -> Self {
        Self::default()
    }

    /// Create modifiers for a silencer
    pub fn silencer() -> Self {
        Self {
            damage_multiplier: 0.9, // 10% damage reduction
            noise_multiplier: 0.3,  // 70% noise reduction
            ..Default::default()
        }
    }

    /// Create modifiers for an extended magazine
    pub fn extended_magazine(magazine_size: i32) -> Self {
        Self {
            magazine_size_modifier: magazine_size,
            reload_speed_multiplier: 0.9, // 10% slower reload
            ..Default::default()
        }
    }

    /// Create modifiers for a scope
    pub fn scope(ads_speed_multiplier: f32) -> Self {
        Self {
            ads_speed_multiplier,
            spread_multiplier: 0.8, // 20% less spread when aiming
            ..Default::default()
        }
    }

    /// Create modifiers for a heavy barrel
    pub fn heavy_barrel() -> Self {
        Self {
            damage_multiplier: 1.15, // 15% more damage
            spread_multiplier: 1.2,  // 20% more spread
            recoil_multiplier: 1.3,  // 30% more recoil
            ..Default::default()
        }
    }

    /// Create modifiers for a laser sight
    pub fn laser_sight() -> Self {
        Self {
            spread_multiplier: 0.7, // 30% less spread
            aim_movement_speed_multiplier: 1.1, // 10% faster movement while aiming
            ..Default::default()
        }
    }

    /// Apply modifiers to weapon stats
    pub fn apply_to_weapon(&self, weapon: &mut super::types::Weapon) {
        weapon.damage *= self.damage_multiplier;
        weapon.damage += self.extra_damage;
        weapon.spread *= self.spread_multiplier;
        weapon.base_spread *= self.spread_multiplier;
        weapon.fire_rate *= self.fire_rate_multiplier;
        weapon.reload_time /= self.reload_speed_multiplier;
        weapon.ammo_capacity += self.magazine_size_modifier;
        weapon.range *= self.range_multiplier;
    }

    /// Remove modifiers from weapon stats (reverse operation)
    pub fn remove_from_weapon(&self, weapon: &mut super::types::Weapon) {
        if self.damage_multiplier != 0.0 {
            weapon.damage /= self.damage_multiplier;
        }
        weapon.damage -= self.extra_damage;
        if self.spread_multiplier != 0.0 {
            weapon.spread /= self.spread_multiplier;
            weapon.base_spread /= self.spread_multiplier;
        }
        if self.fire_rate_multiplier != 0.0 {
            weapon.fire_rate /= self.fire_rate_multiplier;
        }
        if self.reload_speed_multiplier != 0.0 {
            weapon.reload_time *= self.reload_speed_multiplier;
        }
        weapon.ammo_capacity -= self.magazine_size_modifier;
        if self.range_multiplier != 0.0 {
            weapon.range /= self.range_multiplier;
        }
    }
}

/// Event for opening/closing the attachment editor
#[derive(Event, Debug)]
pub struct ToggleAttachmentEditor {
    pub weapon_entity: Entity,
    pub open: bool,
}

/// Event for selecting an attachment
#[derive(Event, Debug)]
pub struct SelectAttachment {
    pub weapon_entity: Entity,
    pub place_id: String,
    pub attachment_id: String,
}

/// Event for removing an attachment
#[derive(Event, Debug)]
pub struct RemoveAttachment {
    pub weapon_entity: Entity,
    pub place_id: String,
}

/// System to handle attachment editor toggling
pub fn handle_attachment_editor_toggle(
    mut toggle_events: EventReader<ToggleAttachmentEditor>,
    mut query: Query<&mut WeaponAttachmentSystem>,
) {
    for event in toggle_events.read() {
        if let Ok(mut attachment_system) = query.get_mut(event.weapon_entity) {
            attachment_system.editing_attachments = event.open;
            
            if event.open {
                info!("Opening attachment editor for weapon");
            } else {
                info!("Closing attachment editor for weapon");
            }
        }
    }
}

/// System to handle attachment selection
pub fn handle_attachment_selection(
    mut select_events: EventReader<SelectAttachment>,
    mut query: Query<&mut WeaponAttachmentSystem>,
) {
    for event in select_events.read() {
        if let Ok(mut attachment_system) = query.get_mut(event.weapon_entity) {
            // Find the attachment place
            if let Some(place) = attachment_system
                .attachment_places
                .iter_mut()
                .find(|p| p.id == event.place_id)
            {
                // Find the attachment
                if let Some((index, attachment)) = place
                    .available_attachments
                    .iter_mut()
                    .enumerate()
                    .find(|(_, a)| a.id == event.attachment_id)
                {
                    if attachment.enabled {
                        // Deactivate previous attachment
                        if place.current_selection >= 0 {
                            if let Some(prev_attachment) = place
                                .available_attachments
                                .get_mut(place.current_selection as usize)
                            {
                                prev_attachment.active = false;
                            }
                        }

                        // Activate new attachment
                        attachment.active = true;
                        place.current_selection = index as i32;
                        attachment_system
                            .selected_attachments
                            .insert(place.id.clone(), attachment.id.clone());

                        info!(
                            "Selected attachment '{}' for place '{}'",
                            attachment.name, place.name
                        );
                    }
                }
            }
        }
    }
}

/// System to handle attachment removal
pub fn handle_attachment_removal(
    mut remove_events: EventReader<RemoveAttachment>,
    mut query: Query<&mut WeaponAttachmentSystem>,
) {
    for event in remove_events.read() {
        if let Ok(mut attachment_system) = query.get_mut(event.weapon_entity) {
            // Find the attachment place
            if let Some(place) = attachment_system
                .attachment_places
                .iter_mut()
                .find(|p| p.id == event.place_id)
            {
                // Deactivate current attachment
                if place.current_selection >= 0 {
                    if let Some(prev_attachment) = place
                        .available_attachments
                        .get_mut(place.current_selection as usize)
                    {
                        prev_attachment.active = false;
                    }
                }

                // Clear selection
                place.current_selection = -1;
                attachment_system.selected_attachments.remove(&place.id);

                info!("Removed attachment from place '{}'", place.name);
            }
        }
    }
}

/// System to update weapon stats based on active attachments
pub fn update_weapon_stats_from_attachments(
    mut weapon_query: Query<(&mut super::types::Weapon, &WeaponAttachmentSystem)>,
) {
    for (mut weapon, attachment_system) in weapon_query.iter_mut() {
        if !attachment_system.attachments_active {
            continue;
        }

        // Reset weapon to base stats first
        // Note: This assumes weapon stores base stats separately
        // For now, we'll just apply modifiers cumulatively

        for place in &attachment_system.attachment_places {
            if place.current_selection >= 0 {
                if let Some(attachment) = place
                    .available_attachments
                    .get(place.current_selection as usize)
                {
                    if attachment.active {
                        attachment.stat_modifiers.apply_to_weapon(&mut weapon);
                    }
                }
            }
        }
    }
}

/// Helper function to create default attachment places
pub fn create_default_attachment_places() -> Vec<AttachmentPlace> {
    vec![
        AttachmentPlace {
            id: "scope".to_string(),
            name: "Scope".to_string(),
            enabled: true,
            available_attachments: vec![
                AttachmentInfo {
                    id: "none".to_string(),
                    name: "No Scope".to_string(),
                    description: "Iron sights only".to_string(),
                    enabled: true,
                    active: true,
                    only_while_carrying: false,
                    stat_modifiers: AttachmentStatModifiers::new(),
                    model: None,
                },
                AttachmentInfo {
                    id: "red_dot".to_string(),
                    name: "Red Dot Sight".to_string(),
                    description: "Quick aiming, close range".to_string(),
                    enabled: true,
                    active: false,
                    only_while_carrying: false,
                    stat_modifiers: AttachmentStatModifiers::scope(1.2),
                    model: None,
                },
                AttachmentInfo {
                    id: "acog".to_string(),
                    name: "ACOG Scope".to_string(),
                    description: "Medium range magnification".to_string(),
                    enabled: true,
                    active: false,
                    only_while_carrying: false,
                    stat_modifiers: AttachmentStatModifiers::scope(0.9),
                    model: None,
                },
                AttachmentInfo {
                    id: "sniper".to_string(),
                    name: "Sniper Scope".to_string(),
                    description: "Long range magnification".to_string(),
                    enabled: true,
                    active: false,
                    only_while_carrying: false,
                    stat_modifiers: AttachmentStatModifiers::scope(0.7),
                    model: None,
                },
            ],
            current_selection: 0,
            no_attachment_text: "Iron Sights".to_string(),
        },
        AttachmentPlace {
            id: "muzzle".to_string(),
            name: "Muzzle".to_string(),
            enabled: true,
            available_attachments: vec![
                AttachmentInfo {
                    id: "none".to_string(),
                    name: "Standard Muzzle".to_string(),
                    description: "No modification".to_string(),
                    enabled: true,
                    active: true,
                    only_while_carrying: false,
                    stat_modifiers: AttachmentStatModifiers::new(),
                    model: None,
                },
                AttachmentInfo {
                    id: "silencer".to_string(),
                    name: "Silencer".to_string(),
                    description: "Reduces noise and damage".to_string(),
                    enabled: true,
                    active: false,
                    only_while_carrying: false,
                    stat_modifiers: AttachmentStatModifiers::silencer(),
                    model: None,
                },
                AttachmentInfo {
                    id: "heavy_barrel".to_string(),
                    name: "Heavy Barrel".to_string(),
                    description: "Increases damage but adds recoil".to_string(),
                    enabled: true,
                    active: false,
                    only_while_carrying: false,
                    stat_modifiers: AttachmentStatModifiers::heavy_barrel(),
                    model: None,
                },
            ],
            current_selection: 0,
            no_attachment_text: "Standard".to_string(),
        },
        AttachmentPlace {
            id: "magazine".to_string(),
            name: "Magazine".to_string(),
            enabled: true,
            available_attachments: vec![
                AttachmentInfo {
                    id: "none".to_string(),
                    name: "Standard Magazine".to_string(),
                    description: "Standard capacity".to_string(),
                    enabled: true,
                    active: true,
                    only_while_carrying: false,
                    stat_modifiers: AttachmentStatModifiers::new(),
                    model: None,
                },
                AttachmentInfo {
                    id: "extended".to_string(),
                    name: "Extended Magazine".to_string(),
                    description: "+50% magazine capacity".to_string(),
                    enabled: true,
                    active: false,
                    only_while_carrying: false,
                    stat_modifiers: AttachmentStatModifiers::extended_magazine(15),
                    model: None,
                },
            ],
            current_selection: 0,
            no_attachment_text: "Standard".to_string(),
        },
        AttachmentPlace {
            id: "underbarrel".to_string(),
            name: "Underbarrel".to_string(),
            enabled: true,
            available_attachments: vec![
                AttachmentInfo {
                    id: "none".to_string(),
                    name: "No Underbarrel".to_string(),
                    description: "No modification".to_string(),
                    enabled: true,
                    active: true,
                    only_while_carrying: false,
                    stat_modifiers: AttachmentStatModifiers::new(),
                    model: None,
                },
                AttachmentInfo {
                    id: "laser".to_string(),
                    name: "Laser Sight".to_string(),
                    description: "Improves accuracy when aiming".to_string(),
                    enabled: true,
                    active: false,
                    only_while_carrying: false,
                    stat_modifiers: AttachmentStatModifiers::laser_sight(),
                    model: None,
                },
            ],
            current_selection: 0,
            no_attachment_text: "None".to_string(),
        },
    ]
}

/// Helper function to create a weapon with default attachments
pub fn create_weapon_with_attachments() -> WeaponAttachmentSystem {
    WeaponAttachmentSystem {
        editing_attachments: false,
        use_universal_attachments: false,
        attachment_places: create_default_attachment_places(),
        selected_attachments: HashMap::new(),
        attachments_active: true,
        can_change_with_number_keys: true,
        show_hover_info: true,
        use_offset_panels: true,
        use_smooth_transition: true,
        disable_hud_when_editing: true,
    }
}
