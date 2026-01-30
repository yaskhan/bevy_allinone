# Stealth System - Quick Start Guide

This guide will help you quickly set up and use the Stealth System in your Bevy game.

## Quick Setup

### 1. Add the Plugin

Add the `StealthPlugin` to your Bevy app:

```rust
use bevy::prelude::*;
use bevy_allinone::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameControllerPlugin)  // Includes StealthPlugin
        .run();
}
```

### 2. Spawn a Stealth Character

Create a character with stealth capabilities:

```rust
fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Name::new("Player"),
        
        // Stealth components
        StealthController::default(),
        StealthState::default(),
        CoverDetection::default(),
        VisibilityMeter::default(),
        
        // Character components
        CharacterController::default(),
        CharacterMovementState::default(),
        InputState::default(),
        
        // Transform
        Transform::from_xyz(0.0, 1.0, 0.0),
        GlobalTransform::default(),
        
        // Physics
        RigidBody::Dynamic,
        Collider::capsule(0.4, 1.0),
        LockedAxes::ROTATION_LOCKED,
    ));
}
```

### 3. Create Cover Objects

Add cover objects to your scene:

```rust
fn spawn_cover(mut commands: Commands) {
    commands.spawn((
        Name::new("Cover Crate"),
        Transform::from_xyz(5.0, 0.5, 0.0),
        GlobalTransform::default(),
        
        // Physics - must be on cover layer
        RigidBody::Static,
        Collider::cuboid(1.0, 0.5, 1.0),
        CollisionLayers::new([Layer::Cover], [Layer::All]),
    ));
}
```

## Basic Controls

Default input actions:

- **C** - Toggle hiding
- **Q** - Toggle peeking (while hidden)
- **E** - Toggle corner leaning (while hidden)
- **R** - Reset camera (while hidden)
- **Mouse** - Look around (while hidden)
- **Scroll Wheel** - Zoom in/out (if enabled)

## Common Configurations

### Silent Stealth Character

Hide without requiring crouch:

```rust
StealthController {
    character_need_to_crouch: false,
    character_cant_move: true,
    max_move_amount: 0.0,
    ..default()
}
```

### Dynamic Stealth Character

Hide while moving:

```rust
StealthController {
    character_need_to_crouch: true,
    character_cant_move: false,
    max_move_amount: 2.0,
    ..default()
}
```

### Detection-Aware Hiding

Enable AI detection checks:

```rust
StealthController {
    check_if_detected_while_hidden: true,
    time_delay_to_hide_again_if_discovered: 3.0,
    ..default()
}
```

### Enhanced Camera Control

More camera freedom while hiding:

```rust
StealthController {
    camera_can_rotate: true,
    rotation_speed: 15.0,
    range_angle_x: Vec2::new(-90.0, 90.0),
    range_angle_y: Vec2::new(-120.0, 120.0),
    
    camera_can_move: true,
    move_camera_speed: 10.0,
    move_camera_limits_x: Vec2::new(-2.0, 2.0),
    move_camera_limits_y: Vec2::new(-2.0, 2.0),
    
    ..default()
}
```

### Spring Camera

Auto-returning camera:

```rust
StealthController {
    use_spring_rotation: true,
    spring_rotation_delay: 2.0,
    
    use_spring_movement: true,
    spring_movement_delay: 2.0,
    
    ..default()
}
```

### Zoom Enabled

Allow zooming while hidden:

```rust
StealthController {
    zoom_enabled: true,
    zoom_speed: 20.0,
    max_zoom: 15.0,
    min_zoom: 75.0,
    
    ..default()
}
```

## Reading Stealth State

Access stealth information in your systems:

```rust
fn check_stealth_status(
    query: Query<(&StealthState, &VisibilityMeter)>
) {
    for (state, visibility) in query.iter() {
        if state.is_hidden {
            println!("Character is hidden");
        }
        
        if state.is_detected {
            println!("Character was detected!");
        }
        
        println!("Visibility: {:.2}", visibility.current_visibility);
        println!("Detection: {:.2}", visibility.detection_level);
        println!("Sound level: {:.2}", visibility.sound_level);
    }
}
```

## Hide States

The system supports multiple hide states:

- **Visible** - Normal gameplay, not hiding
- **CrouchHide** - Hiding while crouched
- **ProneHide** - Hiding while prone
- **Peek** - Peeking from cover
- **CornerLean** - Leaning around corner
- **FixedPlaceHide** - Hiding at fixed location

## Cover Types

Cover is automatically classified:

- **Low** - Waist-high (< 0.5m)
- **Medium** - Chest-high (0.5m - 1.5m)
- **High** - Full cover (>= 1.5m)
- **Corner** - Corner cover for leaning
- **Full** - Complete concealment

## Integration with AI

The stealth system works with the AI system:

```rust
fn spawn_enemy_ai(mut commands: Commands) {
    commands.spawn((
        Name::new("Enemy Guard"),
        
        // AI components
        AiController {
            state: AiBehaviorState::Patrol,
            detection_range: 15.0,
            ..default()
        },
        AiPerception {
            fov: 90.0,
            vision_range: 20.0,
            ..default()
        },
        
        // Character components
        CharacterController::default(),
        CharacterMovementState::default(),
        
        // Faction
        CharacterFaction {
            name: "Enemy".to_string(),
        },
        
        Transform::from_xyz(10.0, 1.0, 0.0),
        GlobalTransform::default(),
    ));
}
```

When the AI detects a hidden player:
- Player's `is_detected` flag is set
- Player is forced out of hiding
- AI enters chase/attack state

## Visibility Meter

The visibility meter tracks detection risk:

```rust
fn display_visibility_ui(
    query: Query<&VisibilityMeter>,
    mut egui_contexts: EguiContexts,
) {
    for visibility in query.iter() {
        egui::Window::new("Stealth Status").show(egui_contexts.ctx_mut(), |ui| {
            ui.label(format!("Visibility: {:.0}%", visibility.current_visibility * 100.0));
            ui.label(format!("Detection: {:.0}%", visibility.detection_level * 100.0));
            ui.label(format!("Sound: {:.0}%", visibility.sound_level * 100.0));
            
            let color = if visibility.is_detected_by_ai {
                egui::Color32::RED
            } else if visibility.detection_level > 0.5 {
                egui::Color32::YELLOW
            } else {
                egui::Color32::GREEN
            };
            
            ui.colored_label(color, if visibility.is_visible_to_ai {
                "VISIBLE"
            } else {
                "HIDDEN"
            });
        });
    }
}
```

## Physics Layers

Set up collision layers for cover detection:

```rust
// Define layers
#[derive(PhysicsLayer)]
pub enum Layer {
    World,      // Ground, walls
    Player,     // Player character
    Enemy,      // AI enemies
    Cover,      // Cover objects (layer 8)
}

// Configure layer in physics plugin
// Cover objects must be on Layer::Cover (bit 8)
```

## Tips & Best Practices

1. **Cover Placement**: Place cover objects on the Cover physics layer (layer 8)

2. **Detection Balance**: Start with generous detection times and tune based on testing

3. **Camera Limits**: Set comfortable rotation/movement limits for your game

4. **Input Mapping**: Customize input actions to match your control scheme

5. **Visual Feedback**: Provide clear UI feedback for visibility and detection states

6. **Level Design**: Design levels with stealth in mind - provide cover paths and hiding spots

7. **Sound Design**: Add audio feedback for state changes (entering hide, detection warnings)

8. **AI Tuning**: Balance AI detection ranges and speeds with stealth mechanics

## Next Steps

For more detailed information, see the [Stealth System Comprehensive Documentation](stealth_system.md).

Key topics to explore:
- Advanced camera configurations
- Custom hide states
- Environmental detection modifiers
- Multiplayer stealth considerations
- Animation integration
- Performance optimization
