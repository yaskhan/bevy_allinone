use bevy::prelude::*;

/// Displays console log lines on screen.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ConsoleLogOnScreenSystem {
    pub max_lines: usize,
    pub lines: Vec<String>,
}

impl Default for ConsoleLogOnScreenSystem {
    fn default() -> Self {
        Self {
            max_lines: 8,
            lines: Vec::new(),
        }
    }
}

#[derive(Event, Debug)]
pub struct ConsoleLogEvent {
    pub message: String,
}

pub fn update_console_log_on_screen_system(
    mut events: EventReader<ConsoleLogEvent>,
    mut query: Query<&mut ConsoleLogOnScreenSystem>,
) {
    for event in events.read() {
        for mut system in query.iter_mut() {
            system.lines.push(event.message.clone());
            if system.lines.len() > system.max_lines {
                let overflow = system.lines.len() - system.max_lines;
                system.lines.drain(0..overflow);
            }
        }
    }
}
