use bevy::prelude::*;
use crate::vehicles::types::*;

pub fn update_skidmarks(
    mut commands: Commands,
    skid_manager_query: Query<&SkidManager>,
    mut wheel_query: Query<(Entity, &GlobalTransform, &mut VehicleWheel, &ChildOf)>,
    mut trail_query: Query<(Entity, &mut SkidMarkTrail)>,
    _time: Res<Time>,
) {
    for (wheel_entity, wheel_gt, mut wheel, parent) in wheel_query.iter_mut() {
        if let Ok(manager) = skid_manager_query.get(parent.0) {
            if !manager.enabled { continue; }

            // Intensity of the skid based on sideways and forward slip
            let slip_intensity = (wheel.slip_amount_sideways.abs() + wheel.slip_amount_forward.abs() * 0.5 - 0.2).max(0.0) * 2.0;
            
            if slip_intensity > 0.1 {
                let pos = wheel_gt.translation() - wheel_gt.up() * wheel.radius;
                
                // Find or create trail for this wheel
                let mut found_trail = false;
                for (_trail_entity, mut trail) in trail_query.iter_mut() {
                    if trail.wheel_entity == wheel_entity {
                        // Check distance to last point
                        if let Some(last_pos) = trail.positions.last() {
                            if pos.distance(*last_pos) > manager.min_distance {
                                trail.positions.push(pos);
                                trail.intensities.push(slip_intensity.min(1.0));
                                
                                // Limit segments
                                if trail.positions.len() > manager.max_marks {
                                    trail.positions.remove(0);
                                    trail.intensities.remove(0);
                                }
                            }
                        }
                        found_trail = true;
                        break;
                    }
                }

                if !found_trail {
                    commands.spawn(SkidMarkTrail {
                        wheel_entity,
                        last_index: -1,
                        positions: vec![pos],
                        intensities: vec![slip_intensity.min(1.0)],
                    });
                }
            }
        }
    }
}
