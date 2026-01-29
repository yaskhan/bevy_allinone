use bevy::prelude::*;

pub struct CameraCapturesPlugin;

impl Plugin for CameraCapturesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScreenshotEventQueue>()
           .add_systems(Update, handle_screenshot_requests);
    }
}

pub struct TakeScreenshotEvent {
    pub path: Option<String>,
}

#[derive(Resource, Default)]
pub struct ScreenshotEventQueue(pub Vec<TakeScreenshotEvent>);

pub fn handle_screenshot_requests(
    mut events: ResMut<ScreenshotEventQueue>,
) {
    for _event in events.0.drain(..) {
        info!("Screenshot requested (logic pending Bevy environment verification)");
    }
}
