# Interaction System

The interaction system allows the player to detect and interact with objects in the world.

## Key Features

- **Raycast/Volume Detection**: High-performance detection of interactable entities.
- **Context-Sensitive Prompts**: Dynamic UI prompts (e.g., "Press E to Open").
- **Interaction Types**: Support for single press, hold, and sequence interactions.

## Components

- `Interactable`: Marker for objects that can be interacted with.
- `InteractionSource`: Component for the player/camera to initiate interactions.

## Systems

- `interaction_detection_system`: Runs on the camera to find candidates.
- `interaction_ui_system`: Manages the display of interaction prompts.
