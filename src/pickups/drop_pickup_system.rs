use bevy::prelude::*;
use super::PickUpElementInfo;

/// Drops a pickup prefab.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct DropPickUpSystem {
    pub drop_pickups_enabled: bool,
    pub drop_pickup_list: Vec<DropPickUpElementInfo>,
    pub manager_pickup_list: Vec<PickUpElementInfo>,
    pub drop_delay: f32,
    pub destroy_after_dropping: bool,
    pub pickup_scale: f32,
    pub set_pickup_scale: bool,
    pub random_content: bool,
    pub max_radius_to_instantiate: f32,
    pub pickup_offset: Vec3,
    pub extra_force_to_pickup: f32,
    pub extra_force_to_pickup_radius: f32,
    pub force_mode_impulse: bool,
    pub main_pickup_manager_name: String,
    pub dropped: bool,
}

impl Default for DropPickUpSystem {
    fn default() -> Self {
        Self {
            drop_pickups_enabled: true,
            drop_pickup_list: Vec::new(),
            manager_pickup_list: Vec::new(),
            drop_delay: 0.0,
            destroy_after_dropping: false,
            pickup_scale: 1.0,
            set_pickup_scale: false,
            random_content: false,
            max_radius_to_instantiate: 1.0,
            pickup_offset: Vec3::ZERO,
            extra_force_to_pickup: 5.0,
            extra_force_to_pickup_radius: 5.0,
            force_mode_impulse: true,
            main_pickup_manager_name: "Pickup Manager".to_string(),
            dropped: false,
        }
    }
}

pub fn update_drop_pickup_system(
    mut commands: Commands,
    mut query: Query<&mut DropPickUpSystem>,
) {
    for mut drop in query.iter_mut() {
        if drop.dropped {
            continue;
        }
        if !drop.drop_pickups_enabled {
            continue;
        }
        commands.spawn((
            SpatialBundle::default(),
            Name::new("Dropped Pickup"),
        ));
        drop.dropped = true;
    }
}

#[derive(Debug, Clone, Reflect)]
pub struct DropPickUpElementInfo {
    pub pick_up_type: String,
    pub type_index: i32,
    pub drop_pick_up_type_list: Vec<DropPickUpTypeElementInfo>,
}

impl Default for DropPickUpElementInfo {
    fn default() -> Self {
        Self {
            pick_up_type: String::new(),
            type_index: 0,
            drop_pick_up_type_list: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Reflect)]
pub struct DropPickUpTypeElementInfo {
    pub name: String,
    pub amount: i32,
    pub quantity: i32,
    pub amount_limits: Vec2,
    pub quantity_limits: Vec2,
    pub name_index: i32,
}

impl Default for DropPickUpTypeElementInfo {
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
