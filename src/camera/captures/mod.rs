use bevy::prelude::*;
use crate::camera::types::CameraController;

pub struct CameraCapturesPlugin;

impl Plugin for CameraCapturesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScreenshotEventQueue>()
           .register_type::<CameraPerspective>()
           .add_systems(Update, (
               handle_screenshot_requests,
               update_camera_perspective,
           ));
    }
}

pub struct TakeScreenshotEvent {
    pub path: Option<String>,
    pub metadata: Option<CaptureSlot>,
}

#[derive(Resource, Default)]
pub struct ScreenshotEventQueue(pub Vec<TakeScreenshotEvent>);

/// Stored information about a taken capture/screenshot
#[derive(Debug, Clone, Reflect)]
pub struct CaptureSlot {
    pub name: String,
    pub date: String,
    pub camera_pos: Vec3,
    pub camera_rot: Quat,
    pub fov: f32,
}

/// Component for objects that need to be captured/photographed
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct CameraPerspective {
    pub name: String,
    pub max_distance: f32,
    pub max_angle: f32,
    pub active: bool,
    pub capture_taken: bool,
    pub target_on_screen: bool,
}

impl Default for CameraPerspective {
    fn default() -> Self {
        Self {
            name: "Objective".to_string(),
            max_distance: 10.0,
            max_angle: 20.0,
            active: true,
            capture_taken: false,
            target_on_screen: false,
        }
    }
}

pub fn handle_screenshot_requests(
    mut events: ResMut<ScreenshotEventQueue>,
) {
    for _event in events.0.drain(..) {
        info!("Screenshot requested (using Bevy's built-in screenshot API logic)");
    }
}

pub fn update_camera_perspective(
    camera_query: Query<(&GlobalTransform, &Camera), With<CameraController>>,
    mut perspective_query: Query<(&mut CameraPerspective, &GlobalTransform)>,
) {
    let (camera_gt, camera) = match camera_query.iter().next() {
        Some(c) => c,
        None => return,
    };

    let camera_pos = camera_gt.translation();
    let camera_fwd = camera_gt.forward();

    for (mut perspective, target_gt) in perspective_query.iter_mut() {
        if !perspective.active || perspective.capture_taken { continue; }

        let target_pos = target_gt.translation();
        let dist = camera_pos.distance(target_pos);

        if dist <= perspective.max_distance {
            let dir_to_target = (target_pos - camera_pos).normalize();
            let dot = camera_fwd.dot(dir_to_target);
            let angle = dot.acos().to_degrees();

            if angle <= perspective.max_angle {
                // Check if on screen
                if let Ok(_viewport_pos) = camera.world_to_viewport(camera_gt, target_pos) {
                    // Check if within viewport bounds [0, 1]
                    // (Assuming viewport covers full screen or we check against specific camera size)
                    // For now, world_to_viewport returns screen coordinates.
                    // We need to normalize or check against viewport size.
                    perspective.target_on_screen = true; // Placeholder for exact bounds check
                } else {
                    perspective.target_on_screen = false;
                }
            } else {
                perspective.target_on_screen = false;
            }
        } else {
            perspective.target_on_screen = false;
        }
    }
}

fn dir_norm(v: Vec3) -> Dir3 {
    Dir3::new(v).unwrap_or(Dir3::NEG_Z)
}
