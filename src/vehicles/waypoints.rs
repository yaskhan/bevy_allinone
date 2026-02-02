use bevy::prelude::*;

/// Waypoint circuit for vehicle paths.
///
/// GKC reference: `WaypointCircuit.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct WaypointCircuit {
    pub points: Vec<Vec3>,
    pub loop_path: bool,
}

impl Default for WaypointCircuit {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            loop_path: true,
        }
    }
}

/// Tracks progress along a waypoint circuit.
///
/// GKC reference: `WaypointProgressTracker.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct WaypointProgressTracker {
    pub circuit: Entity,
    pub distance: f32,
    pub speed: f32,
    pub loop_path: bool,
}

impl Default for WaypointProgressTracker {
    fn default() -> Self {
        Self {
            circuit: Entity::PLACEHOLDER,
            distance: 0.0,
            speed: 5.0,
            loop_path: true,
        }
    }
}

pub fn update_waypoint_progress_tracker(
    time: Res<Time>,
    circuits: Query<&WaypointCircuit>,
    mut trackers: Query<(&mut WaypointProgressTracker, Option<&mut Transform>)>,
) {
    let delta = time.delta_seconds();
    if delta == 0.0 {
        return;
    }

    for (mut tracker, maybe_transform) in trackers.iter_mut() {
        let Ok(circuit) = circuits.get(tracker.circuit) else {
            continue;
        };
        if circuit.points.len() < 2 {
            continue;
        }

        let (lengths, total_length) = circuit_lengths(circuit);
        if total_length == 0.0 {
            continue;
        }

        tracker.distance += tracker.speed.max(0.0) * delta;
        if tracker.loop_path && circuit.loop_path {
            tracker.distance = tracker.distance % total_length;
        } else if tracker.distance > total_length {
            tracker.distance = total_length;
        }

        if let Some(mut transform) = maybe_transform {
            transform.translation = circuit_position_at_distance(circuit, &lengths, tracker.distance);
        }
    }
}

fn circuit_lengths(circuit: &WaypointCircuit) -> (Vec<f32>, f32) {
    let mut lengths = Vec::with_capacity(circuit.points.len());
    let mut total = 0.0;
    let mut previous = circuit.points[0];
    for &point in circuit.points.iter().skip(1) {
        let len = previous.distance(point);
        lengths.push(len);
        total += len;
        previous = point;
    }
    if circuit.loop_path {
        let len = previous.distance(circuit.points[0]);
        lengths.push(len);
        total += len;
    }
    (lengths, total)
}

fn circuit_position_at_distance(
    circuit: &WaypointCircuit,
    lengths: &[f32],
    mut distance: f32,
) -> Vec3 {
    let points_len = circuit.points.len();
    let last_index = points_len - 1;
    let mut current_index = 0usize;
    let mut next_index = 1usize;

    for (idx, &segment) in lengths.iter().enumerate() {
        if distance <= segment {
            current_index = idx;
            next_index = if idx + 1 < points_len { idx + 1 } else { 0 };
            break;
        }
        distance -= segment;
    }

    let start = circuit.points[current_index.min(last_index)];
    let end = circuit.points[next_index.min(last_index)];
    if (start - end).length_squared() <= f32::EPSILON {
        return start;
    }
    let t = (distance / start.distance(end)).clamp(0.0, 1.0);
    start.lerp(end, t)
}
