# Character Controller - Quick Start Guide

## Basic Setup

```rust
use bevy::prelude::*;
use bevy_allinone::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameControllerPlugin)
        .add_systems(Startup, setup_game)
        .run();
}

fn setup_game(mut commands: Commands) {
    // Spawn ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::srgb(0.3, 0.5, 0.3)))),
    ));
    
    // Spawn player using helper function
    spawn_character(&mut commands, Vec3::new(0.0, 2.0, 0.0));
}
```

## Key Components

### CharacterController
Main configuration component with all movement parameters:

```rust
CharacterController {
    walk_speed: 4.0,
    run_speed: 7.0,
    sprint_speed: 10.0,
    crouch_speed: 2.5,
    jump_power: 6.0,
    // ... and many more settings
}
```

### CharacterMovementState
Tracks current movement state:

```rust
CharacterMovementState {
    is_running: true/false,
    is_sprinting: true/false,
    is_crouching: true/false,
    current_speed: f32,
    // ... movement data
}
```

## Common Customizations

### Adjust Movement Speeds
```rust
commands.entity(player_entity).insert(CharacterController {
    walk_speed: 5.0,
    run_speed: 8.0,
    sprint_speed: 12.0,
    ..default()
});
```

### Enable/Disable Features
```rust
CharacterController {
    crouch_sliding_enabled: false,  // Disable crouch sliding
    fall_damage_enabled: false,      // Disable fall damage
    use_tank_controls: true,         // Use tank controls
    ..default()
}
```

### Configure Jumping
```rust
CharacterController {
    jump_power: 7.5,                 // Higher jumps
    jump_hold_bonus: 2.5,           // More variable jump height
    max_jump_hold_time: 0.3,        // Longer jump hold
    ..default()
}
```

## Input System Integration

The controller works with the input system. Ensure you have:

```rust
InputState {
    movement: Vec2,      // Movement input (-1 to 1)
    jump_pressed: bool,  // Jump button
    sprint_pressed: bool, // Sprint button
    crouch_pressed: bool, // Crouch button
}
```

## Animation Integration

The system automatically updates `CharacterAnimationState`:

```rust
CharacterAnimationState {
    mode: CharacterAnimationMode::Run,  // Current animation mode
    forward: 0.75,                      // Forward blend (0-1)
    turn: 0.0,                          // Turn blend (-1 to 1)
}
```

## Physics Requirements

Ensure your character has these physics components:

```rust
(
    RigidBody::Dynamic,
    Collider::capsule(0.4, 1.0),      // Adjust size as needed
    LockedAxes::ROTATION_LOCKED,     // Prevent physics rotation
    GravityScale(1.0),
    Friction::new(0.0),              // Managed by system
    Restitution::new(0.0),
    LinearVelocity::default(),
    AngularVelocity::default(),
)
```

## Troubleshooting Tips

**Character doesn't move:**
- Check `CharacterController.can_move` is true
- Verify input system is sending movement data
- Ensure physics components are properly set up

**Character falls through ground:**
- Check ground detection settings
- Verify collision layers
- Ensure ground has proper collider

**Animation issues:**
- Check `CharacterAnimationState` values
- Verify animation controller setup
- Ensure blend values are reasonable

## Advanced Features Quick Reference

### Wall Running
Automatically enabled when:
- Character is airborne
- Moving parallel to a wall
- Wall is within detection range

### Crouch Sliding
Triggered when:
- Sprinting (`is_sprinting = true`)
- Crouch button pressed
- `crouch_sliding_enabled = true`

### Obstacle Detection
- Uses raycasting to detect obstacles
- Prevents movement through walls
- Configurable detection distance

### Step Up/Down
- Automatically handles small steps
- Prevents getting stuck on ledges
- Configurable step height
