use bevy::prelude::*;

/// Pickup type enumeration.
///
/// GKC reference: `pickupType.cs`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum PickupType {
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

impl Default for PickupType {
    fn default() -> Self {
        PickupType::General
    }
}
