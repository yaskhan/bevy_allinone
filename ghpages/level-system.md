# Level and Save Management

Utilities for handling scene transitions, world state, and persistence.

## Features

- **Scene Loading**: Seamless transitions between levels.
- **Save/Load System**: Serializing world state and player data.
- **Map Data**: Navigation meshes and spatial organization.

## Resources

- `LevelManager`: Metadata about current and pending levels.
- `SaveQueue`: Handles asynchronous save operations.

## Systems

- `level_loading_system`: Manages Bevy Scene spawning.
- `save_serialization_system`: Converts ECS data to persistent storage.
