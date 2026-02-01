# Architecture

This document describes the high-level architecture of the `bevy_allinone` project.

## Directory Structure

- `src/`: Main source code
  - `ai/`: AI logic and systems
  - `weapons/`: Weapon and attachment mechanics
  - `actions/`: Action execution framework
  - `camera/`: Advanced camera controllers
  - `combat/`: Combat and hit registration
  - `player/`: Player-specific logic
  - `map/`: Level and environment data
  - `experience/`: Skills and XP
  - `interaction.rs`: Logic for world interactions
  - `inventory.rs`: Item and container management
- `examples/`: Example usage of various systems
- `ghpages/`: This documentation

## Design Philosophy

The project follows Bevy's ECS (Entity Component System) pattern:
- **Components**: Plain data structures.
- **Systems**: Functions that process entities with specific components.
- **Resources**: Global data shared across systems.
