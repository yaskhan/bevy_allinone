use bevy::prelude::*;
use super::PickUpElementInfo;

/// Chest pickup container.
///
/// GKC reference: `chestSystem.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ChestSystem {
    pub chest_pickup_list: Vec<ChestPickUpElementInfo>,
    pub manager_pickup_list: Vec<PickUpElementInfo>,
    pub enable_pickups_trigger_at_start: bool,
    pub set_new_pickup_trigger_radius: bool,
    pub new_pickup_trigger_radius: f32,
    pub random_content: bool,
    pub refill_chest_after_delay: bool,
    pub time_opened_after_empty: f32,
    pub refilled_time: f32,
    pub open_animation_name: String,
    pub number_of_objects: i32,
    pub min_amount: i32,
    pub max_amount: i32,
    pub place_offset: Vec3,
    pub space: Vec3,
    pub amount_grid: Vec2,
    pub pickup_scale: f32,
    pub show_gizmo: bool,
    pub gizmo_radius: f32,
    pub settings: bool,
    pub is_locked: bool,
    pub open_when_unlocked: bool,
    pub use_event_on_open_chest: bool,
    pub use_event_on_close_chest: bool,
    pub opened: bool,
}

impl Default for ChestSystem {
    fn default() -> Self {
        Self {
            chest_pickup_list: Vec::new(),
            manager_pickup_list: Vec::new(),
            enable_pickups_trigger_at_start: true,
            set_new_pickup_trigger_radius: true,
            new_pickup_trigger_radius: 3.0,
            random_content: false,
            refill_chest_after_delay: false,
            time_opened_after_empty: 1.0,
            refilled_time: 0.0,
            open_animation_name: String::new(),
            number_of_objects: 0,
            min_amount: 0,
            max_amount: 0,
            place_offset: Vec3::ZERO,
            space: Vec3::ZERO,
            amount_grid: Vec2::ONE,
            pickup_scale: 1.0,
            show_gizmo: false,
            gizmo_radius: 0.25,
            settings: false,
            is_locked: false,
            open_when_unlocked: false,
            use_event_on_open_chest: false,
            use_event_on_close_chest: false,
            opened: false,
        }
    }
}

#[derive(Debug, Clone, Reflect)]
pub struct ChestPickUpElementInfo {
    pub pick_up_type: String,
    pub type_index: i32,
    pub chest_pick_up_type_list: Vec<ChestPickUpTypeElementInfo>,
}

impl Default for ChestPickUpElementInfo {
    fn default() -> Self {
        Self {
            pick_up_type: String::new(),
            type_index: 0,
            chest_pick_up_type_list: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Reflect)]
pub struct ChestPickUpTypeElementInfo {
    pub name: String,
    pub amount: i32,
    pub quantity: i32,
    pub amount_limits: Vec2,
    pub quantity_limits: Vec2,
    pub name_index: i32,
}

impl Default for ChestPickUpTypeElementInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            amount: 0,
            quantity: 0,
            amount_limits: Vec2::ZERO,
            quantity_limits: Vec2::ZERO,
            name_index: 0,
        }
    }
}

pub fn update_chest_system(
    mut query: Query<&mut ChestSystem>,
) {
    for mut chest in query.iter_mut() {
        if chest.opened {
            continue;
        }
        // Placeholder for opening logic.
    }
}
