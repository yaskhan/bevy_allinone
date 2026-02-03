use bevy::prelude::*;

/// Event sent when a particle collision occurs.
#[derive(Debug, Clone)]
pub struct ParticleCollisionEvent {
    pub detector: Entity,
    pub other: Entity,
}

/// Event sent when a particle trigger occurs.
#[derive(Debug, Clone)]
pub struct ParticleTriggerEvent {
    pub detector: Entity,
    pub other: Entity,
    pub entered: bool,
}

#[derive(Resource, Default)]
pub struct ParticleCollisionEventQueue(pub Vec<ParticleCollisionEvent>);

#[derive(Resource, Default)]
pub struct ParticleTriggerEventQueue(pub Vec<ParticleTriggerEvent>);

/// Helper component for particle collision detection.
///
///
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct ParticleCollisionDetector {
    pub last_hit: Option<Entity>,
}

/// Helper component for particle trigger detection.
///
///
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct ParticleTriggerDetector {
    pub last_trigger: Option<Entity>,
    pub is_inside: bool,
}

/// Update collision detector state from events.
pub fn handle_particle_collision_events(
    mut events: ResMut<ParticleCollisionEventQueue>,
    mut detectors: Query<&mut ParticleCollisionDetector>,
) {
    for event in events.0.drain(..) {
        if let Ok(mut detector) = detectors.get_mut(event.detector) {
            detector.last_hit = Some(event.other);
        }
    }
}

/// Update trigger detector state from events.
pub fn handle_particle_trigger_events(
    mut events: ResMut<ParticleTriggerEventQueue>,
    mut detectors: Query<&mut ParticleTriggerDetector>,
) {
    for event in events.0.drain(..) {
        if let Ok(mut detector) = detectors.get_mut(event.detector) {
            detector.last_trigger = Some(event.other);
            detector.is_inside = event.entered;
        }
    }
}
