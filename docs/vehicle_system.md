# Vehicle System Documentation
 
## Overview
 
The Vehicle System is a comprehensive physics-based vehicle controller designed for 3D games built with Bevy Engine. It provides realistic vehicle mechanics including car-like physics, wheel dynamics, seating management, weapon systems, damage modeling, and advanced audio feedback. The system supports multiple vehicle types and can be extended to handle various vehicle classes from cars and trucks to boats and aircraft.
 
## Architecture
 
The Vehicle System is built on Bevy's Entity Component System (ECS) architecture and integrates with Avian3D physics engine for realistic collision detection and rigid body dynamics. The system is organized into modular components that handle different aspects of vehicle simulation:
 
### Core Components
 
- **Vehicle**: Main component containing all vehicle properties, physics settings, and state management
- **VehicleWheel**: Individual wheel simulation with suspension, steering, and powered/driven states
- **VehicleSeat**: Passenger seating system with enter/exit mechanics and animation support
- **VehicleDriver**: Marker component identifying the current driver of a vehicle
- **VehicleSeatingManager**: Centralized management for multi-passenger vehicles
 
### Advanced Systems
 
- **VehicleWeaponSystem**: Integrated weapon mounting and targeting for military vehicles
- **VehicleStats**: Health, fuel, and boost resource management
- **VehicleAudio**: Dynamic engine and skid sound effects
- **SkidManager**: Real-time tire mark generation for enhanced visual feedback
 
### Physics Integration
 
The system leverages Avian3D's SpatialQuery for ground detection, wheel-ground collision detection, and physics-based movement calculations. Each wheel performs independent physics calculations for realistic suspension behavior and traction control.
 
## Vehicle Component Structure
 
### Core Vehicle Settings
 
The `Vehicle` component contains over 60 configurable parameters organized into logical categories:
 
#### Movement Parameters
- **Speed Limits**: `max_forward_speed` (default: 25.0), `max_backward_speed` (default: 10.0)
- **Engine Power**: `engine_torque` (default: 2500.0), `rear_engine_torque` (default: 2500.0)
- **Braking**: `brake_power` (default: 4000.0)
- **Steering**: `steering_angle` (default: 35.0°), dynamic steering adjustment based on speed
 
#### Advanced Movement Features
- **Boost System**: Configurable boost with energy cost and regeneration rates
- **Jump Mechanics**: Optional vehicle jumping with configurable impulse force
- **Anti-roll Bars**: `anti_roll` parameter (default: 10000.0) for body roll control
- **Chassis Leaning**: Dynamic body lean based on turning and terrain
 
#### State Management
The system maintains real-time vehicle state including:
- **Gear System**: Multi-gear transmission with automatic or manual shifting
- **Engine RPM**: Current RPM tracking with redline protection
- **Speed Monitoring**: Forward and reverse speed calculations
- **Ground Contact**: Continuous wheel-ground contact detection
 
### Wheel System Architecture
 
Each vehicle wheel operates as an independent physics entity with the following capabilities:
 
#### Physical Properties
- **Dimensions**: `radius` (default: 0.3 meters)
- **Suspension**: Configurable `suspension_distance` (default: 0.2 meters)
- **Powered State**: Boolean flags for engine-driven wheels
- **Steering Capability**: Independent wheel steering configuration
 
#### Visual Integration
- **Mesh Association**: Optional wheel mesh, mudguard, and suspension visualization
- **Offset Configuration**: Precise positioning of visual components
- **Rotation Tracking**: Real-time wheel rotation state for animation sync
 
#### Physics Simulation
- **Slip Detection**: Forward and sideways slip amount calculations
- **RPM Calculation**: Wheel rotation speed based on vehicle movement
- **Suspension Compression**: Dynamic spring position tracking
 
## Input and Control Systems
 
### Vehicle Input Processing
 
The input system operates through a dedicated `vehicle_input_system` that:
 
1. **Driver Detection**: Continuously monitors for active drivers in vehicle seats
2. **Input Synchronization**: Transfers character input state to vehicle controls
3. **Input Smoothing**: Applies temporal smoothing to prevent jerky movements:
   - Steering smoothing: 10x delta factor
   - Motor smoothing: 5x forward, 3x reverse
4. **Special Actions**: Handles boost, jump, and brake activation
 
### Control State Management
 
The system maintains separate input states for:
- **Movement Vector**: 2D directional input (WASD/analog stick)
- **Jump Input**: Configurable for boost or vehicle jumping
- **Interact Input**: Vehicle entry/exit control
 
## Physics Engine Integration
 
### Ground Detection System
 
The physics system uses Avian3D's `SpatialQuery.cast_ray` for reliable ground detection:
 
```rust
// Pseudo-code representation of ground detection logic
for wheel in vehicle_wheels {
    let ray_origin = wheel_position + Vec3::Y * 2.0;
    let ray_direction = Vec3::DOWN;
    let ray_length = wheel.suspension_distance + 2.0;
    
    if spatial_query.cast_ray(ray_origin, ray_direction, ray_length) {
        wheel.is_on_ground = true;
        calculate_suspension_compression(ray_result);
    }
}
```
 
### Velocity and Force Calculation
 
Vehicle movement calculations include:
 
1. **Forward Force Application**: Engine torque converted to forward momentum
2. **Lateral Force Handling**: Tire grip and drift simulation
3. **Gravity Compensation**: Maintains vehicle stability on slopes
4. **Air Resistance**: Drag forces proportional to speed squared
 
### Suspension System
 
Each wheel independently calculates:
- **Spring Compression**: Based on ground distance and vehicle weight
- **Damping Effects**: Prevents oscillation and maintains stability
- **Load Transfer**: Body roll and pitch calculations during acceleration/turning
 
## Seating and Passenger Management
 
### Seat Configuration
 
Vehicle seats are configured with precise positioning:
 
```rust
struct VehicleSeat {
    seat_index: usize,
    is_driver_seat: bool,
    offset: Vec3,           // Local position relative to vehicle
    exit_position: Vec3,    // Local position for exit spawning
    enter_animation: String,
    exit_animation: String,
    bounce_on_enter: bool,
}
```
 
### Enter/Exit Mechanics
 
The interaction system handles:
 
1. **Proximity Detection**: Automatic seat selection based on character distance
2. **Occupancy Tracking**: Real-time seat availability monitoring
3. **Animation Triggers**: Seamless integration with character animation systems
4. **State Synchronization**: Updates character movement state when entering vehicles
 
### Multi-Passenger Support
 
The `VehicleSeatingManager` component enables:
- **Ejection Systems**: Configurable ejection on vehicle destruction
- **Weapon Hiding**: Automatic weapon concealment for passengers
- **Door Control**: Automatic door opening/closing mechanics
 
## Weapon Integration
 
### Vehicle Weapon System
 
Military and combat vehicles can be equipped with weapon systems:
 
```rust
struct VehicleWeapon {
    name: String,
    damage: f32,
    fire_rate: f32,
    ammo_in_clip: u32,
    clip_size: u32,
    total_ammo: u32,
    reload_time: f32,
    projectile_speed: f32,
    is_laser: bool,
    is_homing: bool,
}
```
 
### Targeting and Aiming
 
The system supports:
- **Dual-Axis Rotation**: Independent horizontal and vertical aiming
- **Rotation Speed**: Configurable aiming response rates
- **Entity Tracking**: Automatic tracking of weapon base entities
- **Ammo Management**: Real-time ammunition tracking and reload systems
 
### Fire Control
 
Weapon firing includes:
- **Rate Limiting**: Prevents weapon spam with configurable fire rates
- **Projectile Physics**: Integration with existing projectile systems
- **Recoil Simulation**: Weapon kickback and stabilization mechanics
 
## Damage and Health Systems
 
### Vehicle Statistics
 
The `VehicleStats` component manages:
 
#### Health Management
- **Health Tracking**: Current and maximum health values
- **Regeneration**: Configurable health regen with delay timers
- **Damage Multipliers**: Customizable damage resistance by vehicle type
 
#### Resource Systems
- **Fuel Management**: Optional fuel consumption and regeneration
- **Boost Energy**: Special energy system for boost capabilities
- **Regeneration Rates**: Individual regen speeds for each resource
 
### Damage Processing
 
Vehicle damage includes:
- **Collision Detection**: Physics-based collision damage calculation
- **Component Damage**: Differential damage to wheels, weapons, and chassis
- **State Changes**: Automatic vehicle state changes based on damage levels
- **Destruction Effects**: Cleanup and removal of destroyed vehicles
 
## Audio and Visual Effects
 
### Dynamic Audio System
 
The `VehicleAudio` component provides:
 
#### Engine Audio
- **RPM-Based Pitch**: Engine sound pitch varies with RPM
- **Volume Control**: Dynamic volume based on engine load and speed
- **Multiple States**: Different audio profiles for idle, acceleration, and cruising
 
#### Tire Audio
- **Skid Detection**: Automatic skid sound generation
- **Volume Scaling**: Skid volume based on slip intensity
- **Surface Variation**: Different tire sounds for various ground types
 
### Visual Effects
 
#### Skid Mark System
 
The skid mark system provides:
- **Real-Time Generation**: Dynamic tire mark creation during sliding
- **Intensity Variation**: Mark darkness based on slip amount
- **Trail Management**: Efficient pooling and cleanup of skid mark entities
- **Surface Adaptation**: Mark appearance varies by ground surface type
 
#### Suspension Visualization
 
Visual wheel systems include:
- **Rotation Animation**: Wheel mesh rotation synchronized with physics
- **Suspension Movement**: Visual suspension compression and extension
- **Body Roll**: Vehicle chassis lean visualization during turns
 
## Performance Optimization
 
### Efficient Query Management
 
The system uses strategic query separation:
- **Read-Only Queries**: Minimizes mutable access conflicts
- **Filtered Queries**: Efficient component filtering for different systems
- **Batch Processing**: Groups similar operations for cache efficiency
 
### Memory Management
 
- **Entity Pooling**: Reuses entity allocations for temporary objects
- **Component Compression**: Efficient storage of frequently accessed data
- **Resource Management**: Proper cleanup of audio, visual, and physics resources
 
### Physics Optimization
 
- **Selective Updates**: Physics calculations only for active vehicles
- **Distance-Based LOD**: Reduced physics precision for distant vehicles
- **Sleeping Detection**: Automatic suspension of inactive vehicles
 
## Integration Points
 
### Character System Integration
 
The vehicle system integrates with:
- **CharacterController**: Seamless character-to-vehicle transitions
- **InputState**: Shared input processing between character and vehicle modes
- **Animation System**: Coordinated character animations for vehicle interactions
 
### Combat System Integration
 
Vehicle weapons integrate with:
- **Projectile System**: Shared projectile creation and management
- **Damage System**: Unified damage processing across vehicle and character combat
- **Stats System**: Character and vehicle stat interactions
 
### Save System Compatibility
 
Vehicle states are saved through:
- **Component Serialization**: All vehicle components support serialization
- **State Persistence**: Complete vehicle state including position, health, and inventory
- **Load Restoration**: Automatic restoration of vehicle states on game load
 
## Configuration and Customization
 
### Vehicle Type Variations
 
The system supports multiple vehicle types through the `VehicleType` enum:
- **Car**: Standard road vehicle with balanced performance
- **Truck**: Heavy vehicle with high torque, lower top speed
- **Motorcycle**: Lightweight vehicle with high agility
- **Boat**: Water vehicle with different physics model
- **Plane**: Aircraft with flight dynamics
- **Hovercraft**: Hover vehicle with unique ground interaction
 
### Performance Tuning
 
Each vehicle can be fine-tuned through:
- **Physics Parameters**: Mass distribution, friction coefficients, drag values
- **Handling Characteristics**: Steering response, suspension stiffness, weight balance
- **Visual Settings**: Audio levels, effect intensities, animation speeds
 
## Troubleshooting and Debug
 
### Common Issues
 
1. **Wheel Physics Problems**
   - Check suspension distance settings
   - Verify ground detection ray length
   - Ensure proper collision layers
 
2. **Input Not Responding**
   - Verify VehicleDriver component assignment
   - Check input state synchronization
   - Confirm character is properly seated
 
3. **Performance Issues**
   - Reduce skid mark count limits
   - Lower physics update rates for distant vehicles
   - Disable unnecessary audio effects
 
### Debug Tools
 
The system provides debug visualization for:
- **Wheel Ground Contact**: Visual indicators for wheel-ground contact state
- **Vehicle State Display**: Real-time display of vehicle parameters
- **Input State Monitoring**: Visual feedback for input processing
- **Physics Force Visualization**: Force vector displays for debugging movement
 
## Future Enhancements
 
The vehicle system architecture supports future expansions including:
 
- **Advanced Suspension**: Customizable suspension types (MacPherson strut, double wishbone)
- **Weather Effects**: Rain, snow, and ice affecting vehicle handling
- **Traffic AI**: Autonomous vehicle traffic simulation
- **Vehicle Customization**: Part swapping and visual customization systems
- **Multiplayer Support**: Network synchronization of vehicle states
 
This comprehensive vehicle system provides a solid foundation for realistic vehicle simulation in Bevy-based games while maintaining the flexibility to adapt to various game design requirements.