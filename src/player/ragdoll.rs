//! Ragdoll System
//!
//! Manages physics-based player deaths and falls (ragdolling).

use bevy::prelude::*;

pub struct RagdollPlugin;

impl Plugin for RagdollPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Ragdoll>()
            .register_type::<RagdollState>()
            .init_resource::<ActivateRagdollQueue>()
            .init_resource::<DeactivateRagdollQueue>()
            .add_systems(Update, (
                handle_ragdoll_activation,
                update_ragdoll_state,
            ).chain());
    }
}

/// Helper enum for ragdoll states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Default)]
pub enum RagdollState {
    #[default]
    Animated,
    Ragdolled,
    BlendToAnim,
}

/// Component to manage ragdoll physics on a player
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Ragdoll {
    pub current_state: RagdollState,
    pub active: bool,
    pub time_to_get_up: f32,
    pub max_ragdoll_velocity: f32,
    pub timer: f32,
    pub on_ground: bool,
    // In a real physics integration (like Rapier or Avian), these would reference rigidbodies/colliders
    pub root_bone: Option<Entity>, 
    pub body_parts: Vec<Entity>,
}

impl Default for Ragdoll {
    fn default() -> Self {
        Self {
            current_state: RagdollState::Animated,
            active: true,
            time_to_get_up: 3.0,
            max_ragdoll_velocity: 50.0,
            timer: 0.0,
            on_ground: false,
            root_bone: None,
            body_parts: Vec::new(),
        }
    }
}

/// Event to activate ragdoll physics
#[derive(Debug, Clone, Copy)]
pub struct ActivateRagdollEvent {
    pub entity: Entity,
    pub force_direction: Option<Vec3>,
    pub force_magnitude: f32,
}

#[derive(Resource, Default)]
pub struct ActivateRagdollQueue(pub Vec<ActivateRagdollEvent>);

/// Event to deactivate ragdoll and return to animation
#[derive(Debug, Clone, Copy)]
pub struct DeactivateRagdollEvent {
    pub entity: Entity,
}

#[derive(Resource, Default)]
pub struct DeactivateRagdollQueue(pub Vec<DeactivateRagdollEvent>);

/// System to handle activation events
pub fn handle_ragdoll_activation(
    mut activate_queue: ResMut<ActivateRagdollQueue>,
    mut deactivate_queue: ResMut<DeactivateRagdollQueue>,
    mut query: Query<&mut Ragdoll>,
) {
    for event in activate_queue.0.drain(..) {
        if let Ok(mut ragdoll) = query.get_mut(event.entity) {
             if ragdoll.current_state != RagdollState::Ragdolled {
                ragdoll.current_state = RagdollState::Ragdolled;
                ragdoll.timer = 0.0;
                
                info!("Ragdoll: Activated for {:?}. Apply force: {:?} * {}", event.entity, event.force_direction, event.force_magnitude);
             }
        }
    }

    for event in deactivate_queue.0.drain(..) {
        if let Ok(mut ragdoll) = query.get_mut(event.entity) {
            if ragdoll.current_state == RagdollState::Ragdolled {
                // Transition to blending state
                ragdoll.current_state = RagdollState::BlendToAnim;
                ragdoll.timer = 0.0;
                info!("Ragdoll: Deactivating for {:?}, blending to anim", event.entity);
            }
        }
    }
}

/// System to update state timers and checks
pub fn update_ragdoll_state(
    mut query: Query<&mut Ragdoll>,
    time: Res<Time>,
) {
    for mut ragdoll in query.iter_mut() {
        match ragdoll.current_state {
            RagdollState::Ragdolled => {
                // Placeholder logic:
                ragdoll.timer += time.delta_secs();
                if ragdoll.timer > 10.0 { // Failsafe auto get up?
                     // Verify conditions
                }
            }
            RagdollState::BlendToAnim => {
                ragdoll.timer += time.delta_secs();
                if ragdoll.timer >= 1.0 { 
                    ragdoll.current_state = RagdollState::Animated;
                    info!("Ragdoll: Fully recovered to Animated state");
                }
            }
            RagdollState::Animated => {
                // Normal state
            }
        }
    }
}
