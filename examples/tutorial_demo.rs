use bevy::prelude::*;
use bevy_allinone::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameControllerPlugin)
        .add_systems(Startup, setup_tutorial_demo)
        .add_systems(Update, trigger_tutorial)
        .run();
}

fn setup_tutorial_demo(
    mut commands: Commands,
    mut manager: ResMut<TutorialManager>,
) {
    // Spawn player with TutorialLog
    commands.spawn((
        NodeBundle::default(), // Dummy component for player-like entity
        TutorialLog::default(),
    ));

    // Define a sample tutorial
    let tutorial = Tutorial {
        id: 1,
        name: "Welcome Tutorial".to_string(),
        panels: vec![
            TutorialPanel {
                name: "Welcome".to_string(),
                title: "Welcome to Bevy!".to_string(),
                description: "This is a tutorial system.".to_string(),
                image_path: None,
            },
            TutorialPanel {
                name: "Movement".to_string(),
                title: "Basic Movement".to_string(),
                description: "Use WASD to move and Space to jump.".to_string(),
                image_path: None,
            },
        ],
        play_only_once: true,
        unlock_cursor: true,
        pause_input: true,
        set_custom_time_scale: true,
        custom_time_scale: 0.0, // Pause time
    };

    manager.tutorials.insert(tutorial.id, tutorial);

    println!("Press T to trigger the Welcome Tutorial");
}

fn trigger_tutorial(
    keys: Res<ButtonInput<KeyCode>>,
    mut events: EventWriter<TutorialEvent>,
) {
    if keys.just_pressed(KeyCode::KeyT) {
        events.send(TutorialEvent::Open(1));
    }
}
