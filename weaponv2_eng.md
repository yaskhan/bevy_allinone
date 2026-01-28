Below is a Technical Specification for refactoring and expanding the current weapon system (based on Bevy) in order to integrate advanced ballistics.

---

# Terms of reference: Integration of the deterministic ballistics system in Bevy

## 1. General information

The aim of the project is to replace the simplified projectile system in `weapons.rs `for a high-precision physical simulation. The system must take into account external factors (wind, air density), the physical properties of the bullet (mass, resistance) and complex interaction with surfaces (ricochets, penetration).

## 2. Architectural requirements

To provide performance comparable to *Unity Job System + Burst*, the implementation in Bevy must:

* Use the **Parallel System** to process projectiles.
* Maximize the use of **SIMD** through the `glam` library (built into Bevy).
* Implement **Object Pooling** for visual effects of hits and tracers.

---

##3. Technical Specifications

### 3.1. Global Environment (Environment Settings)

Create a `BallisticsEnvironment' resource that stores:

* `gravity`: Vec3 (acceleration of free fall).
* `air_density': f32 (air density affecting resistance).
* `wind`: Vec3 (wind force vector).

### 3.2. Projectile Component (Bullet Settings)

Extend the `Projectile` structure by adding physical parameters:

* `mass': f32 (bullet mass in kg).
* `drag_coefficient': f32 (drag coefficient).
* `reference_area`: f32 (cross-sectional area).
* `penetration_power': f32 (initial penetrating power).
* `velocity': Vec3 (current velocity vector).

### 3.3. Mathematical model (Velocity Calculation)

Replace linear displacement with the **Runge-Kutta method of the 4th order (RK4)** to calculate the trajectory.

* **The equation:** Take into account gravity and air resistance .
* **Collision detection:** execute `spatial_query.cast_ray' strictly between the position in the frame and the calculated position in the frame to exclude "spans" through objects.

---

## 4. The system of interaction with surfaces (Surface Interactions)

It is necessary to implement a system of Tags or metadata for game objects.:

| Parameter | Description |
| --- | --- |
| **Ricochet Ability** | The angle at which the bullet is reflected rather than absorbed. |
| **Penetration Loss** | Coefficient of loss of speed/energy during penetration of the material. |
| **Hit Effects** | Type of effect (spark, dust, blood) when hit. |

### The penetration algorithm:

1. When hit, the "penetration_power" of the bullet is checked against the thickness and density of the obstacle.
2. If the penetration is successful: the bullet continues to fly from the exit point at a reduced speed.
3. If unsuccessful: the bullet is deflected, creating a hit effect.

---

## 5. Additional functionality

### 5.1. Targeting (Weapon Zeroing)

Add a method to the `Weapon`:

* Automatic calculation of the barrel angle (`Pitch`) to compensate for the bullet falling at a given distance (for example, 100m, 300m, 500m).

### 5.2. Separation of visualization and simulation

* **Backend:** Mathematical calculation of bullet coordinates.
* **Frontend:** A separate component `BulletTracer' (visual model/particles), which interpolates its position following the simulation data. This will allow you to customize the visual without breaking the physics.

---

## 6. Implementation plan in `weapons.rs `

1. **Data Update:** Add `mass`, `drag`, `area' to the structure of `Weapon` and `Projectile'.
2. **Resources:** Create a `BallisticsConfig` as a `Resource` in Bevy.
3. **Core System:** Rewrite `update_projectiles' by implementing RK4 integration.
4. **Collision Logic:** Add a penetration processing cycle inside `update_projectiles`: if the beam finds a collision, run the calculation of the residual energy.
5. **Pooling:** Implement an Entity reuse system for sparks and decals.

Sure. To implement a Spread system inspired by Bullet Dynamics, we will move away from a simple random angle and implement a dynamic model that depends on the player's condition, type of movement, and accumulated "returns."

The addendum to the terms of reference, focused on ballistic dispersion, is presented below.

---

## Addition to TK: Dynamic spread and accuracy system

###1. Precision cone model

The spread should not be calculated as a static number, but as an expanding cone based on several multipliers.

**Parameters of the `Weapon` component:**

* **Base Spread:** Minimum angle of spread (in radians) under ideal conditions.
* **Max Spread:** The ceiling of the cone expansion during continuous firing.
* **Bloom per Shot:** The value by which the spread increases after each shot.
* **Recovery Rate:** The rate at which the spread returns to the base value per second.

### 2. State Multipliers (Modifiers)

The accuracy should change dynamically depending on the actions of the `WeaponManager' or the player.:

* **Movement Multiplier:** Increase the spread when moving (read from the player's `Velocity').
* **Airborne Penalty:** Significant drop in accuracy if the player does not touch the ground.
* **Stance Multiplier:** Coefficients for the "standing", "sitting" and "lying" positions.
* **Aiming (ADS) Multiplier:** Reducing the base spread when aiming (already included in your code as `aim_spread_mult`, but requires integration into the general formula).

---

### 3. Calculation algorithm (Extension `fire_weapon')

Instead of the current simplified method, the calculation of the shot vector will look like this:

1. **Calculating the current Bloom:** 
2. **Generating a point inside a circle:**
Using a Gaussian distribution (instead of a uniform `random` one), so that bullets fly closer to the center more often, rather than evenly distributed along the edges of the circle.
3. **Application of transformation:**
Rotate the base vector `Forward' by the obtained deflection angles.

### 4. Integration with Recoil (Recoil)

A spread is a random deviation inside a cone. Recoil is the physical displacement of the barrel (camera) itself.

* **Visual Kick:** Shifting the `Transform` of the weapon up and back.
* **Camera Pitch/Yaw:** Adding a pulse to the camera control system.

---

### Example of a data structure for Rust:

```rust
#[derive(Component, Reflect)]
pub struct Accuracy {
    pub current_bloom: f32,
    pub base_spread: f32,
    pub max_spread: f32,
    pub bloom_per_shot: f32,
    pub recovery_rate: f32,
// Multipliers
    pub movement_penalty: f32,
    pub ads_modifier: f32, // e.g. 0.2
}

```

### Implementation plan:

1. **Update System:** Create a `calculate_dynamic_spread` system that reduces the `current_bloom` every iteration of the `Update' and checks the player's speed.
2. **Firing System:** In the `fire_weapon' function, add `bloom_per_shot' to the current state.
3. **Visuals:** (Optional) Add an interface (UI) in the form of a crosshair that expands and narrows depending on the `current_bloom'.