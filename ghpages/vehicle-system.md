# Vehicle System

Physics-based vehicle mechanics and interaction.

## Features

- **Vehicle Movement**: Driving and steering logic.
- **Entry/Exit**: Transitions between player and vehicle control.
- **Damage/Fuel**: Resource management for vehicles.

## Components

- `Vehicle`: Marker and core data for vehicles.
- `VehicleSeat`: Definies entry points.

## Systems

- `vehicle_physics_system`: Handles wheel friction and engine forces.
- `vehicle_interaction_system`: Manages player entering/exiting.
