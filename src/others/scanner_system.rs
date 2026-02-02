use bevy::prelude::*;

/// Scanner system component.
///
/// GKC reference: `scannerSystem.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ScannerSystem {
    pub range: f32,
    pub active: bool,
}

impl Default for ScannerSystem {
    fn default() -> Self {
        Self {
            range: 10.0,
            active: false,
        }
    }
}

#[derive(Event, Debug)]
pub struct ScannerPingEvent {
    pub entity: Entity,
}

pub fn update_scanner_system(
    mut events: EventReader<ScannerPingEvent>,
    query: Query<&ScannerSystem>,
) {
    for event in events.read() {
        if let Ok(scanner) = query.get(event.entity) {
            if scanner.active {
                debug!("Scanner ping from {:?} range {}", event.entity, scanner.range);
            }
        }
    }
}
