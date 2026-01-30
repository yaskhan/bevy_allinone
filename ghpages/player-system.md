# Player and Character Systems

Core systems for player control, movement, and camera management.

## Features

- **Advanced Movement**: Walking, running, jumping, and climbing (ladders/walls).
- **Dynamic Camera**: Smooth transitions between first-person and third-person views.
- **Character Controller**: Physics-based movement handling.

## Components

- `Player`: Marker for the player entity.
- `CharacterState`: Tracks locomotion states (Idle, Running, Jumping).
- `Climbable`: Marker for geometry that allows climbing.

## Systems

- `player_input_system`: Translates raw input to actions.
- `player_movement_system`: Applies forces and updates transforms.
- `camera_follow_system`: Manages camera position and rotation.
