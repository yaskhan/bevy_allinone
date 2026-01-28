# Bevy All in one Controller

A powerful 3D/2.5D game controller plugin for Bevy Engine.

> [!CAUTION]
> **WARNING: This plugin is currently NOT TESTED.**
> It is provided "as is" and may contain bugs or breaking changes. Use with caution in production projects.

## 🚀 Features

The plugin provides a comprehensive set of systems for building complex 3D and 2.5D games:

- ✅ **Character Controller**: Full-body awareness 3rd/1st person controller with walk, run, jump, crouch, and sprint.
- ✅ **Camera System**: Advanced camera management with multiple modes (follow, orbiting, first-person), collision detection, and smooth transitions.
- ✅ **Movement Systems**: Ladder climbing, wall running, ledge climbing, and character movement physics.
- ✅ **Combat & Stats**: Health system, core attributes (Strength, Agility, etc.), derived stats, and stat modifiers (buffs/debuffs).
- ✅ **Skills & Abilities**: Configurable skill trees and ability management system.
- ✅ **Inventory & Vendor**: Item management, equipment system, and shop/trading mechanics.
- ✅ **AI & Stealth**: NPC behavior, detection systems, line-of-sight, and stealth/hiding mechanics.
- ✅ **Interaction System**: Flexible object interaction framework (switches, doors, pressure plates, etc.).
- ✅ **Narrative Systems**: Dialog system with branching paths and a robust Quest system.
- ✅ **Save System**: Game state persistence for all major systems.
- ✅ **Puzzle & Devices**: Logic gates, puzzle elements, and electronic device simulations.
- ✅ **Tutorial System**: Dynamic tutorial logging and display system.
- ✅ **Vehicles**: Basic vehicle physics and controller support components.

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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::srgb(0.3, 0.5, 0.3)))),
    ));

    // Spawn the player
    commands.spawn((
        Player,
        CharacterController::default(),
        // Add other components as needed
    ));
}
```

## Architecture

The plugin is organized into modular components:

- `character` - Core character controller logic
- `camera` - Camera management and follow logic
- `input` - Platform-agnostic input mapping
- `physics` - Custom gravity and ground detection
- `combat` - Health and damage processing
- `stats` - Attributes and derived stats
- `skills` / `abilities` - Progression systems
- `inventory` / `vendor` - Item and trade systems
- `stealth` / `ai` - Perception and behavior
- `interaction` - Interactive object framework
- `dialog` / `quest` - Story and progression
- `tutorial` - Instructional feedback system

## Examples

The repository includes numerous examples demonstrating each system. You can run them using `cargo run --example <example_name>`:
- `tutorial_demo`
- `stats_demo`
- `stealth_demo`
- `abilities_demo`
- `skills_demo`
- `quest_demo`
- `vendor_demo`
- `ladder_demo`
- `climb_demo`
- ...and more in the `examples/` directory.

## License

MIT OR Apache-2.0
