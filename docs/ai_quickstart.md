# AI System - Quick Start Guide

## Basic Setup

```rust
use bevy::prelude::*;
use bevy_allinone::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameControllerPlugin)  // Includes AiPlugin
        .add_systems(Startup, setup_game)
        .run();
}

fn setup_game(mut commands: Commands) {
    // Setup factions first
    setup_factions(&mut commands);
    
    // Spawn a basic AI
    spawn_basic_ai(&mut commands, Vec3::new(2.0, 0.0, 2.0));
    
    // Spawn player
    spawn_player(&mut commands, Vec3::new(0.0, 0.0, 0.0));
}
```

## Key Components

### AiController
Main AI control component:

```rust
AiController {
    state: AiBehaviorState::Idle,      // Current state
    detection_range: 15.0,            // How far AI can detect
    attack_range: 2.5,                // How close to attack
    patrol_speed_mult: 0.5,           // Patrol speed multiplier
    chase_speed_mult: 1.0,            // Chase speed multiplier
    ..default()
}
```

### AiPerception
Sensory capabilities:

```rust
AiPerception {
    fov: 90.0,           // Field of view in degrees
    vision_range: 20.0,  // Maximum vision distance
    ..default()
}
```

### CharacterFaction
Faction membership:

```rust
CharacterFaction {
    name: "Enemy".to_string(),  // Faction name
}
```

## Common AI Types

### Basic Enemy AI

```rust
fn spawn_basic_ai(commands: &mut Commands, position: Vec3) -> Entity {
    commands.spawn((
        Name::new("Basic Enemy"),
        AiController {
            state: AiBehaviorState::Idle,
            detection_range: 15.0,
            attack_range: 2.5,
            ..default()
        },
        AiMovement::default(),
        AiPerception {
            fov: 90.0,
            vision_range: 20.0,
            ..default()
        },
        AiVisionVisualizer::default(),
        CharacterFaction {
            name: "Enemy".to_string(),
        },
        Transform::from_translation(position),
        GlobalTransform::default(),
    ))
    .insert((
        // Character controller components
        CharacterController::default(),
        CharacterMovementState::default(),
        InputState::default(),
        
        // Physics components
        RigidBody::Dynamic,
        Collider::capsule(0.4, 1.0),
        LockedAxes::ROTATION_LOCKED,
        GravityScale(1.0),
        Friction::new(0.0),
        Restitution::new(0.0),
        LinearVelocity::default(),
        AngularVelocity::default(),
    ))
    .id()
}
```

### Patrol AI

```rust
fn spawn_patrol_ai(commands: &mut Commands) -> Entity {
    let patrol_path = vec![
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(10.0, 0.0, 0.0),
        Vec3::new(10.0, 0.0, 10.0),
        Vec3::new(0.0, 0.0, 10.0),
    ];
    
    commands.spawn((
        Name::new("Patrol Guard"),
        AiController {
            state: AiBehaviorState::Patrol,
            patrol_path: patrol_path,
            wait_time_between_waypoints: 3.0,
            patrol_speed_mult: 0.7,
            ..default()
        },
        AiPerception {
            fov: 120.0,  // Wider FOV for guards
            vision_range: 25.0,
            ..default()
        },
        // ... other components
    ))
    .id()
}
```

### Turret AI

```rust
fn spawn_turret(commands: &mut Commands, position: Vec3) -> Entity {
    commands.spawn((
        Name::new("Security Turret"),
        AiController {
            state: AiBehaviorState::Turret,
            detection_range: 30.0,  // Longer range for turrets
            attack_range: 20.0,     // Longer attack range
            ..default()
        },
        AiPerception {
            fov: 180.0,  // Wide FOV for turrets
            vision_range: 35.0,
            ..default()
        },
        // Add turret-specific components
        Turret::default(),
        
        // Position but no movement components
        Transform::from_translation(position),
        GlobalTransform::default(),
    ))
    .id()
}
```

## Faction Setup

```rust
fn setup_factions(commands: &mut Commands) {
    let mut faction_system = FactionSystem::default();
    
    // Define factions
    faction_system.factions.push(FactionInfo {
        name: "Player".to_string(),
        turn_to_enemy_if_attacked: true,
        turn_faction_to_enemy: false,
        friendly_fire_turn_into_enemies: false,
    });
    
    faction_system.factions.push(FactionInfo {
        name: "Enemy".to_string(),
        turn_to_enemy_if_attacked: true,
        turn_faction_to_enemy: false,
        friendly_fire_turn_into_enemies: true,
    });
    
    faction_system.factions.push(FactionInfo {
        name: "Neutral".to_string(),
        turn_to_enemy_if_attacked: true,
        turn_faction_to_enemy: false,
        friendly_fire_turn_into_enemies: false,
    });
    
    // Set faction relationships
    faction_system.relations.push(FactionRelationInfo {
        faction_a: "Player".to_string(),
        faction_b: "Enemy".to_string(),
        relation: FactionRelation::Enemy,
    });
    
    faction_system.relations.push(FactionRelationInfo {
        faction_a: "Player".to_string(),
        faction_b: "Neutral".to_string(),
        relation: FactionRelation::Neutral,
    });
    
    commands.insert_resource(faction_system);
}
```

## AI Behavior Control

### Changing AI State

```rust
fn change_ai_state(
    mut ai_query: Query<&mut AiController>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::T) {
        for mut ai in ai_query.iter_mut() {
            ai.state = AiBehaviorState::Chase;  // Make all AI chase
        }
    }
}
```

### Setting AI Target

```rust
fn set_ai_target(
    mut ai_query: Query<&mut AiController>,
    player_query: Query<Entity, With<Player>>,
) {
    if let Ok(player_entity) = player_query.get_single() {
        for mut ai in ai_query.iter_mut() {
            ai.target = Some(player_entity);  // Make AI target player
            ai.state = AiBehaviorState::Chase;
        }
    }
}
```

## Noise and Hearing

### Creating Noise Events

```rust
fn create_noise_on_action(
    mut noise_queue: ResMut<NoiseEventQueue>,
    player_query: Query<&Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::N) {
        if let Ok(player_transform) = player_query.get_single() {
            noise_queue.0.push(NoiseEvent {
                position: player_transform.translation,
                volume: 1.0,  // Full volume
                source: player_query.single(),
            });
        }
    }
}
```

### Adjusting Hearing Settings

```rust
AiPerceptionSettings {
    fov: 90.0,
    range: 20.0,
    hearing_range: 15.0,  // How far AI can hear
    layer_mask: 0xFFFFFFFF,  // What layers to detect
}
```

## Common Customizations

### Adjust Detection Range
```rust
ai_controller.detection_range = 25.0;  // Longer detection
ai_controller.attack_range = 5.0;     // Longer attack range
```

### Change FOV
```rust
ai_perception.fov = 120.0;  // Wider field of view
ai_perception.vision_range = 30.0;  // Longer vision range
```

### Modify Patrol Behavior
```rust
ai_controller.patrol_speed_mult = 0.8;  // Faster patrol
ai_controller.wait_time_between_waypoints = 5.0;  // Longer wait at waypoints
```

### Adjust Suspicion Settings
```rust
ai_controller.max_suspicion_time = 10.0;  // Longer suspicion timer
ai_controller.suspicion_timer = 10.0;    // Reset suspicion timer
```

## Debugging and Visualization

### Enable Vision Cones
```rust
AiVisionVisualizer {
    active: true,  // Enable visualization
    normal_color: Color::WHITE,
    alert_color: Color::srgb(1.0, 0.0, 0.0),  // Red when alert
}
```

### Disable Visualization
```rust
AiVisionVisualizer {
    active: false,  // Disable for production
    ..default()
}
```

## Troubleshooting Tips

**AI doesn't detect player:**
- Check faction relations are set to Enemy
- Verify perception settings (FOV, range)
- Ensure line of sight is clear
- Check if AI is in appropriate state (not Dead/Flee)

**AI doesn't move:**
- Verify character controller components are present
- Check if `can_move` is true in CharacterController
- Ensure physics components are properly set up
- Verify AI state allows movement

**AI gets stuck:**
- Check patrol path waypoints are valid
- Ensure no obstacles blocking path
- Verify collision settings
- Check movement speed settings

**Performance issues:**
- Limit number of active AI entities
- Reduce perception ranges for distant AI
- Disable visualization in production
- Use simpler behavior for background AI

## Best Practices

1. **Start Simple**: Begin with basic AI and add complexity
2. **Test Frequently**: Test AI in various scenarios
3. **Use Visualization**: Enable debug visualization during development
4. **Balance Settings**: Adjust detection ranges and speeds for gameplay
5. **Optimize**: Reduce AI complexity for background characters
6. **Document**: Document custom AI behaviors and settings