use bevy::prelude::*;

use crate::abilities::LaserVisionSliceEventQueue;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Sliceable {
    pub enabled: bool,
    pub slice_radius: f32,
    pub min_delay_between_slices: f32,
    pub last_slice_time: f32,
}

impl Default for Sliceable {
    fn default() -> Self {
        Self {
            enabled: true,
            slice_radius: 1.0,
            min_delay_between_slices: 0.5,
            last_slice_time: -999.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SliceEvent {
    pub source: Option<Entity>,
    pub position: Vec3,
    pub normal: Vec3,
    pub direction: Vec3,
    pub radius: f32,
}

#[derive(Resource, Default)]
pub struct SliceEventQueue(pub Vec<SliceEvent>);

#[derive(Debug, Clone)]
pub struct SliceResultEvent {
    pub target: Entity,
    pub position: Vec3,
    pub normal: Vec3,
    pub source: Option<Entity>,
    pub sliced: bool,
}

#[derive(Resource, Default)]
pub struct SliceResultQueue(pub Vec<SliceResultEvent>);

pub fn queue_slice_events_from_laser(
    mut slice_queue: ResMut<SliceEventQueue>,
    mut laser_queue: Option<ResMut<LaserVisionSliceEventQueue>>,
) {
    let Some(mut laser_queue) = laser_queue else { return };

    for event in laser_queue.0.drain(..) {
        slice_queue.0.push(SliceEvent {
            source: Some(event.entity),
            position: event.position,
            normal: event.direction,
            direction: event.direction,
            radius: 0.8,
        });
    }
}

pub fn apply_slice_events(
    time: Res<Time>,
    mut slice_queue: ResMut<SliceEventQueue>,
    mut result_queue: ResMut<SliceResultQueue>,
    mut sliceables: Query<(Entity, &GlobalTransform, &mut Sliceable)>,
) {
    let now = time.elapsed_secs();

    for event in slice_queue.0.drain(..) {
        for (entity, transform, mut sliceable) in sliceables.iter_mut() {
            if !sliceable.enabled {
                continue;
            }

            if now - sliceable.last_slice_time < sliceable.min_delay_between_slices {
                continue;
            }

            let distance = transform.translation().distance(event.position);
            if distance > sliceable.slice_radius.max(event.radius) {
                continue;
            }

            sliceable.last_slice_time = now;

            result_queue.0.push(SliceResultEvent {
                target: entity,
                position: event.position,
                normal: event.normal,
                source: event.source,
                sliced: true,
            });

            // TODO: Integrate actual mesh slicing once a Bevy-compatible slicer is chosen.
        }
    }
}
