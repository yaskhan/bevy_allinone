use bevy::prelude::*;

/// Pickup object marker.
///
/// GKC reference: `pickUpObject.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PickUpObject {
    pub amount: i32,
    pub use_amount_per_unit: bool,
    pub amount_per_unit: i32,
    pub pick_up_sound: String,
    pub static_pickup: bool,
    pub move_to_player_on_trigger: bool,
    pub pick_up_mode: PickUpMode,
    pub can_be_examined: bool,
    pub usable_by_anything: bool,
    pub usable_by_player: bool,
    pub usable_by_vehicles: bool,
    pub usable_by_characters: bool,
    pub show_pickup_info_on_taken: bool,
    pub use_pickup_icon_on_taken: bool,
    pub pickup_icon_path: String,
    pub take_with_trigger: bool,
    pub use_pickup_icon_on_screen: bool,
    pub pickup_icon_general_name: String,
    pub pickup_icon_name: String,
    pub use_event_on_taken: bool,
    pub use_event_on_remaining_amount: bool,
    pub send_pickup_finder: bool,
    pub player: Option<Entity>,
    pub vehicle: Option<Entity>,
    pub npc: Option<Entity>,
    pub finder: Option<Entity>,
    pub finder_is_player: bool,
    pub finder_is_vehicle: bool,
    pub finder_is_character: bool,
    pub amount_taken: i32,
    pub ignore_examine_object_before_store_enabled: bool,
    pub pickup_kind: crate::pickups::PickupKind,
}

impl Default for PickUpObject {
    fn default() -> Self {
        Self {
            amount: 1,
            use_amount_per_unit: false,
            amount_per_unit: 0,
            pick_up_sound: String::new(),
            static_pickup: false,
            move_to_player_on_trigger: true,
            pick_up_mode: PickUpMode::Trigger,
            can_be_examined: false,
            usable_by_anything: false,
            usable_by_player: true,
            usable_by_vehicles: true,
            usable_by_characters: false,
            show_pickup_info_on_taken: true,
            use_pickup_icon_on_taken: false,
            pickup_icon_path: String::new(),
            take_with_trigger: true,
            use_pickup_icon_on_screen: true,
            pickup_icon_general_name: String::new(),
            pickup_icon_name: String::new(),
            use_event_on_taken: false,
            use_event_on_remaining_amount: false,
            send_pickup_finder: false,
            player: None,
            vehicle: None,
            npc: None,
            finder: None,
            finder_is_player: false,
            finder_is_vehicle: false,
            finder_is_character: false,
            amount_taken: 0,
            ignore_examine_object_before_store_enabled: false,
            pickup_kind: crate::pickups::PickupKind::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum PickUpMode {
    Trigger,
    Button,
}

impl Default for PickUpMode {
    fn default() -> Self {
        PickUpMode::Trigger
    }
}
