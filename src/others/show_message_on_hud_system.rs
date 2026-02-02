use bevy::prelude::*;

/// Shows temporary messages on HUD.
///
/// GKC reference: `showMessageOnHUDSystem.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ShowMessageOnHudSystem {
    pub messages: Vec<String>,
}

impl Default for ShowMessageOnHudSystem {
    fn default() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
}

#[derive(Event, Debug)]
pub struct ShowHudMessageEvent {
    pub message: String,
}

pub fn update_show_message_on_hud_system(
    mut events: EventReader<ShowHudMessageEvent>,
    mut query: Query<&mut ShowMessageOnHudSystem>,
) {
    for event in events.read() {
        for mut hud in query.iter_mut() {
            hud.messages.push(event.message.clone());
        }
    }
}
