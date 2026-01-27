# Bevy All in one Controller

A powerful 3D/2.5D game controller plugin for Bevy Engine.

## ⚠️ Work in Progress

This plugin is currently in early development. The skeleton structure is in place, but most functionality is marked with `TODO` and needs implementation.

## Features (Planned)

- ✅ **Character Controller**: Full-body awareness 3rd/1st person controller
- ✅ **Camera System**: Advanced camera management with multiple modes
- ✅ **Input System**: Flexible input handling for multiple platforms
- ⏳ **Combat System**: Melee and ranged combat mechanics
- ⏳ **Inventory System**: Item management and equipment
- ⏳ **AI System**: NPC behavior and pathfinding
- ⏳ **Vehicles System**: Drivable vehicles with physics
- ⏳ **Save System**: Game state persistence

Legend: ✅ Skeleton implemented | ⏳ TODO

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
bevy_allinone = { path = "../bevy_allinone" }  # Adjust path as needed
```

Use in your game:

```rust
use bevy::prelude::*;
use bevy_allinone::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameControllerPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn a character
    spawn_character(&mut commands, Vec3::ZERO);
    
    // TODO: Add more setup
}
```

## Architecture

The plugin is organized into modules:

- `character` - Character controller (walk, run, jump, crouch, etc.)
- `camera` - Camera follow, rotation, zoom, collision
- `input` - Input mapping and handling
- `physics` - Gravity, ground detection, slopes
- `combat` - Health, damage, melee combat
- `weapons` - Weapon management and firing
- `inventory` - Item storage and management
- `interaction` - Object interaction system
- `ai` - NPC behavior and pathfinding
- `vehicles` - Vehicle physics and controls
- `save` - Save/load game state

## Implementation Roadmap

See [AGENTS.md](AGENTS.md) for the detailed implementation plan.

**Recommended implementation order:**
1. Physics module (gravity, ground detection)
2. Character controller (movement, rotation, animation)
3. Input system (keyboard, mouse, gamepad)
4. Camera system (follow, rotation, collision)
5. Combat and weapons
6. Other systems


## Development

Build the project:

```bash
cargo build
```

Run tests:

```bash
cargo test
```

## License

MIT OR Apache-2.0

## Contributing

This is currently a skeleton project. Contributions are welcome! 
