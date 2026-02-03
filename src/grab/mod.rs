use bevy::prelude::*;

pub mod types;
pub mod systems;

pub use types::*;

pub struct GrabPlugin;

impl Plugin for GrabPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GrabEventQueue>()
            .register_type::<Grabbable>()
            .register_type::<Grabber>()
            .register_type::<GrabObjectParent>()
            .register_type::<GrabObjectEventSystem>()
            .register_type::<ObjectToPlace>()
            .register_type::<PutObjectSystem>()
            .register_type::<GrabPowerer>()
            .register_type::<GrabPowerThrow>()
            .register_type::<OutlineSettings>()
            .register_type::<GrabMeleeWeapon>()
            .register_type::<ImprovisedWeapon>()
            .register_type::<ImprovisedWeaponStats>()
            .register_type::<GrabBlockShield>()
            .register_type::<GrabMeleeAttackState>()
            .register_type::<PowerThrown>()
            .register_type::<GrabPowerFx>()
            .register_type::<GrabPhysicalObjectSettings>()
            .register_type::<PlacementSlot>()
            .init_resource::<PlacementEventQueue>()
            .add_systems(Update, (
                systems::handle_grab_input,
                systems::process_grab_events,
                systems::update_held_object,
                systems::handle_rotation,
                systems::handle_throwing,
                systems::handle_power_throwing,
                systems::update_put_object_systems,
                systems::handle_placement_slots,
                systems::handle_power_grabbing,
                systems::update_power_held_objects,
                systems::update_outlines,
                systems::handle_grab_melee,
                systems::update_grab_melee_attacks,
                systems::update_grab_melee_hitboxes,
                systems::update_grab_blocking,
                systems::perform_grab_melee_damage,
                systems::apply_throw_damage_on_collision,
                systems::update_power_throw_fx,
            ));
    }
}
