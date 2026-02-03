use bevy::prelude::*;

/// Pickup type enumeration.
///
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum PickupKind {
    Ammo,
    Energy,
    Experience,
    ExperienceMultiplier,
    General,
    GrabStrength,
    Health,
    Inventory,
    InventoryExtraSpace,
    InventoryWeightBag,
    JetpackFuel,
    Map,
    MeleeShield,
    MeleeWeapon,
    MeleeWeaponConsumable,
    Money,
    Oxygen,
    Power,
    Shield,
    SkillPoint,
    Stamina,
    VehicleFuel,
    Weapon,
    WeaponAttachment,
}

impl Default for PickupKind {
    fn default() -> Self {
        PickupKind::General
    }
}

/// Shared pickup settings.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PickupTypeSettings {
    pub store_pickup_on_inventory: bool,
    pub use_inventory_object_when_picked: bool,
    pub use_custom_pickup_message: bool,
    pub object_taken_as_pickup_message: String,
    pub object_taken_as_inventory_message: String,
    pub use_abilities_list_to_enable_on_take: bool,
    pub abilities_list_to_enable_on_take: Vec<String>,
    pub activate_ability_on_take: bool,
    pub ability_name_to_activate: String,
    pub ability_is_temporally_activated: bool,
    pub show_debug_print: bool,
}

impl Default for PickupTypeSettings {
    fn default() -> Self {
        Self {
            store_pickup_on_inventory: false,
            use_inventory_object_when_picked: false,
            use_custom_pickup_message: true,
            object_taken_as_pickup_message: String::new(),
            object_taken_as_inventory_message: String::new(),
            use_abilities_list_to_enable_on_take: false,
            abilities_list_to_enable_on_take: Vec::new(),
            activate_ability_on_take: false,
            ability_name_to_activate: String::new(),
            ability_is_temporally_activated: false,
            show_debug_print: false,
        }
    }
}
