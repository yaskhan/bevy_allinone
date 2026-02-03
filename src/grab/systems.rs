use bevy::prelude::*;

use avian3d::prelude::*;
use crate::input::{InputState, InputAction};
use crate::combat::{DamageEventQueue, DamageEvent, DamageType, DamageZone, Blocking, AreaEffect};
use crate::stats::stats_system::StatsSystem;
use crate::stats::types::DerivedStat;
use crate::abilities::types::{SetAbilityEnabledEventQueue, SetAbilityEnabledEvent};
use super::types::*;
use bevy::audio::{AudioSource, PlaybackSettings};

/// System to handle grab/drop input.
pub fn handle_grab_input(
    input: Res<InputState>,
    mut grabber_query: Query<(Entity, &mut Grabber)>,
    mut powerer_query: Query<(Entity, &mut GrabPowerer)>,
    mut event_queue: ResMut<GrabEventQueue>,
) {
    for (entity, mut grabber) in grabber_query.iter_mut() {
        if let Some(held) = grabber.held_object {
            if input.interact_pressed {
                 event_queue.0.push(GrabEvent::Drop(entity, held));
            }
        }
    }

    for (entity, mut powerer) in powerer_query.iter_mut() {
        if !powerer.held_objects.is_empty() {
            if input.interact_pressed {
                for held in powerer.held_objects.clone() {
                    event_queue.0.push(GrabEvent::Drop(entity, held));
                }
                powerer.held_objects.clear();
            }
        }
    }
}

/// System to process Grab events.
pub fn process_grab_events(
    mut commands: Commands,
    mut event_queue: ResMut<GrabEventQueue>,
    mut grabber_query: Query<&mut Grabber>,
    mut powerer_query: Query<&mut GrabPowerer>,
    mut grabbable_query: Query<(&Grabbable, &mut LinearVelocity, Option<&mut Mass>, Option<&mut Collider>, Option<&mut GravityScale>, (Option<&mut LinearDamping>, Option<&mut AngularDamping>))>,
    weapon_query: Query<&GrabMeleeWeapon>,
    parent_redirect_query: Query<&GrabObjectParent>,
    event_system_query: Query<&GrabObjectEventSystem>,
    physical_settings_query: Query<&GrabPhysicalObjectSettings>,
    mut ability_queue: ResMut<SetAbilityEnabledEventQueue>,
) {
    let events: Vec<GrabEvent> = event_queue.0.drain(..).collect();

    for event in events {
        match event {
            GrabEvent::Grab(grabber_entity, mut target_entity) => {
                // Check for redirection
                if let Ok(redirect) = parent_redirect_query.get(target_entity) {
                    target_entity = redirect.object_to_grab;
                }

                // Handle single grabber
                if let Ok(mut grabber) = grabber_query.get_mut(grabber_entity) {
                    if grabber.held_object.is_none() {
                        grabber.held_object = Some(target_entity);
                        info!("Object grabbed: {:?}", target_entity);

                        // Apply physical settings
                        if let Ok(settings) = physical_settings_query.get(target_entity) {
                            if let Ok((_g, _v, mut mass, mut _collider, mut gravity, (mut lin_damping, mut ang_damping))) = grabbable_query.get_mut(target_entity) {
                                if settings.set_mass {
                                    if let Some(ref mut m) = mass {
                                        m.0 = settings.mass_value;
                                    }
                                }
                                // Collider toggle usually needs to be deferred or handled via commands
                                if settings.disable_gravity_on_grab {
                                    if let Some(ref mut g) = gravity {
                                        g.0 = 0.0;
                                    }
                                }
                                if let Some(drag) = settings.drag_override {
                                    if let Some(ref mut damping) = lin_damping {
                                        damping.0 = drag;
                                    }
                                }
                                if let Some(angular_drag) = settings.angular_drag_override {
                                    if let Some(ref mut damping) = ang_damping {
                                        damping.0 = angular_drag;
                                    }
                                }
                            }
                            if let Some(sound) = &settings.grab_sound {
                                commands.spawn((
                                    AudioPlayer::<AudioSource>(sound.clone()),
                                    PlaybackSettings::ONCE,
                                ));
                            }
                        }

                        commands.entity(target_entity).insert(ImprovisedWeapon);
                        commands.entity(target_entity).insert(GrabMeleeAttackState::default());
                        if weapon_query.get(target_entity).is_err() {
                            commands.entity(target_entity).insert(GrabMeleeWeapon {
                                attacks: vec![GrabAttackInfo {
                                    name: "Grab Swing".to_string(),
                                    damage: 8.0,
                                    damage_multiplier: 1.0,
                                    attack_type: "Bash".to_string(),
                                    stamina_cost: 8.0,
                                    duration: 0.45,
                                    force_on_hit: 0.0,
                                    animation_id: "GrabSwing".to_string(),
                                }],
                                ..default()
                            });
                        }
                    }
                }
                
                // Handle power powerer
                if let Ok(mut powerer) = powerer_query.get_mut(grabber_entity) {
                    if !powerer.held_objects.contains(&target_entity) {
                        powerer.held_objects.push(target_entity);
                        info!("Object grabbed by power: {:?}", target_entity);
                    }
                }

                // Trigger event
                if let Ok(_event_sys) = event_system_query.get(target_entity) {
                    info!("Grab event triggered for object: {:?}", target_entity);
                }

                if let Ok(weapon) = weapon_query.get(target_entity) {
                    if weapon.can_block {
                        commands.entity(grabber_entity).insert(GrabBlockShield);
                        commands.entity(grabber_entity).insert(Blocking {
                            is_blocking: false,
                            block_reduction: weapon.block_protection,
                            parry_window: 0.2,
                            current_block_time: 0.0,
                        });
                    }

                    for ability in &weapon.unlock_abilities {
                        ability_queue.0.push(SetAbilityEnabledEvent {
                            player_entity: grabber_entity,
                            ability_name: ability.clone(),
                            enabled: true,
                        });
                    }
                }
            }
            GrabEvent::Drop(grabber_entity, target_entity) => {
                if let Ok(mut grabber) = grabber_query.get_mut(grabber_entity) {
                    if grabber.held_object == Some(target_entity) {
                        grabber.held_object = None;
                        grabber.is_rotating = false;
                        info!("Object dropped: {:?}", target_entity);
                    }
                }

                if let Ok(mut powerer) = powerer_query.get_mut(grabber_entity) {
                    powerer.held_objects.retain(|&e| e != target_entity);
                    info!("Object dropped by power: {:?}", target_entity);
                }

                // Trigger event
                if let Ok(_event_sys) = event_system_query.get(target_entity) {
                    info!("Drop event triggered for object: {:?}", target_entity);
                }

                commands.entity(target_entity).remove::<ImprovisedWeapon>();
                commands.entity(target_entity).remove::<GrabMeleeAttackState>();
                commands.entity(grabber_entity).remove::<GrabBlockShield>();
                commands.entity(grabber_entity).remove::<Blocking>();

                if let Ok(settings) = physical_settings_query.get(target_entity) {
                    if let Some(sound) = &settings.drop_sound {
                        commands.spawn((
                            AudioPlayer::<AudioSource>(sound.clone()),
                            PlaybackSettings::ONCE,
                        ));
                    }
                    if settings.disable_gravity_on_grab {
                        if let Ok((_g, _v, _mass, _collider, mut gravity, _damping)) = grabbable_query.get_mut(target_entity) {
                            if let Some(ref mut g) = gravity {
                                g.0 = 1.0;
                            }
                        }
                    }
                }

                if let Ok(weapon) = weapon_query.get(target_entity) {
                    for ability in &weapon.unlock_abilities {
                        ability_queue.0.push(SetAbilityEnabledEvent {
                            player_entity: grabber_entity,
                            ability_name: ability.clone(),
                            enabled: false,
                        });
                    }
                }
            }
            GrabEvent::Throw(grabber_entity, target_entity, direction, force) => {
                let mut thrown = false;
                if let Ok(mut grabber) = grabber_query.get_mut(grabber_entity) {
                    if grabber.held_object == Some(target_entity) {
                        grabber.held_object = None;
                        grabber.is_rotating = false;
                        thrown = true;
                    }
                }

                if let Ok(mut powerer) = powerer_query.get_mut(grabber_entity) {
                    if powerer.held_objects.contains(&target_entity) {
                        powerer.held_objects.retain(|&e| e != target_entity);
                        thrown = true;
                    }
                }
                
                if thrown {
                    if let Ok((_grabbable, mut velocity, _, _, _, _)) = grabbable_query.get_mut(target_entity) {
                        velocity.0 += direction * (force * 0.1); 
                        info!("Object thrown with force: {}", force);

                        // Trigger event
                        if let Ok(_event_sys) = event_system_query.get(target_entity) {
                            info!("Throw event triggered for object: {:?}", target_entity);
                        }
                    }
                }

                commands.entity(target_entity).remove::<ImprovisedWeapon>();
                commands.entity(target_entity).remove::<GrabMeleeAttackState>();
                commands.entity(grabber_entity).remove::<GrabBlockShield>();
                commands.entity(grabber_entity).remove::<Blocking>();

                if let Ok(weapon) = weapon_query.get(target_entity) {
                    for ability in &weapon.unlock_abilities {
                        ability_queue.0.push(SetAbilityEnabledEvent {
                            player_entity: grabber_entity,
                            ability_name: ability.clone(),
                            enabled: false,
                        });
                    }
                }
            }
        }
    }
}

/// System to update the position of the held object.
pub fn update_held_object(
    grabber_query: Query<(&Grabber, &GlobalTransform)>,
    mut held_query: Query<(&GlobalTransform, &mut LinearVelocity), With<Grabbable>>,
) {
    for (grabber, transform) in grabber_query.iter() {
        let Some(held_entity) = grabber.held_object else { continue };
        
        if let Ok((held_transform, mut velocity)) = held_query.get_mut(held_entity) {
            let target_pos = transform.translation() + transform.forward() * grabber.hold_distance;
            let current_pos = held_transform.translation();
            
            let dir = target_pos - current_pos;
            let distance = dir.length();
            
            if distance > grabber.max_hold_distance {
                // Should force-drop here. For now just dampen.
            }

            // Power-based follow
            velocity.0 = dir * grabber.hold_speed;
        }
    }
}

/// System to handle object rotation while held.
pub fn handle_rotation(
    input: Res<InputState>,
    mut grabber_query: Query<(&mut Grabber, &GlobalTransform)>,
    mut held_query: Query<&mut Transform, With<Grabbable>>,
) {
    for (mut grabber, grabber_xf) in grabber_query.iter_mut() {
        let Some(held_entity) = grabber.held_object else { continue };
        
        if input.aim_pressed {
            grabber.is_rotating = true;
            if let Ok(mut transform) = held_query.get_mut(held_entity) {
                let axis = input.look;
                
                let sensitivity = 0.1;
                let rot_y = Quat::from_rotation_y(-axis.x * grabber.rotation_speed * sensitivity);
                let rot_x = Quat::from_rotation_x(-axis.y * grabber.rotation_speed * sensitivity);
                
                let rotation = rot_y * rot_x;
                let pivot = transform.translation;
                
                // transform.rotate_around(pivot, rotation) manual implementation
                transform.rotation = rotation * transform.rotation;
            }
        } else {
            grabber.is_rotating = false;
        }
    }
}

/// System to handle throwing logic.
pub fn handle_throwing(
    input: Res<InputState>,
    mut grabber_query: Query<(Entity, &mut Grabber, &GlobalTransform)>,
    mut event_queue: ResMut<GrabEventQueue>,
) {
    for (entity, mut grabber, transform) in grabber_query.iter_mut() {
        let Some(held) = grabber.held_object else { continue };

        if input.attack_pressed {
            grabber.is_charging_throw = true;
            grabber.throw_force = (grabber.throw_force + 10.0).min(grabber.max_throw_force);
        } else if grabber.is_charging_throw {
            let dir = transform.forward();
            event_queue.0.push(GrabEvent::Throw(entity, held, *dir, grabber.throw_force));
            grabber.is_charging_throw = false;
            grabber.throw_force = 500.0;
        }
    }
}
/// System to handle object placement in slots.
pub fn update_put_object_systems(
    mut _commands: Commands,
    _time: Res<Time>,
    mut put_query: Query<(Entity, &mut PutObjectSystem, &GlobalTransform)>,
    mut to_place_query: Query<(Entity, &mut ObjectToPlace, &mut Transform, &mut LinearVelocity)>,
    _event_queue: ResMut<GrabEventQueue>,
    grabber_query: Query<(Entity, &Grabber)>,
) {
    for (_put_entity, mut put_system, put_xf) in put_query.iter_mut() {
        if put_system.is_object_placed {
            // Check if object is removed
            if let Some(placed_entity) = put_system.current_object_placed {
                if let Ok((_entity, mut to_place, transform, _vel)) = to_place_query.get_mut(placed_entity) {
                    let target_pos = put_xf.translation();

                    let dist = transform.translation.distance(target_pos);
                    if dist > put_system.max_distance_to_place * 1.5 {
                        // Object was moved away (e.g. grabbed by player)
                        info!("Object removed from slot: {:?}", placed_entity);
                        put_system.is_object_placed = false;
                        put_system.current_object_placed = None;
                        to_place.is_placed = false;
                    }
                } else {
                    // Object was deleted or similar
                    put_system.is_object_placed = false;
                    put_system.current_object_placed = None;
                }
                continue;
            }
        }

        // Logic to detect and place object
        for (to_place_entity, mut to_place, mut transform, mut velocity) in to_place_query.iter_mut() {
            if to_place.is_placed || to_place.object_name != put_system.object_name_to_place {
                continue;
            }

            if put_system.use_certain_object {
                if Some(to_place_entity) != put_system.certain_object_to_place {
                    continue;
                }
            }

            let target_pos = put_xf.translation();
            let dist = transform.translation.distance(target_pos);

            if dist < put_system.max_distance_to_place {
                // Check if currently held
                let mut is_held = false;
                for (_grabber_entity, grabber) in grabber_query.iter() {
                    if grabber.held_object == Some(to_place_entity) {
                        is_held = true;
                        break;
                    }
                }

                if !is_held {
                    // Place it!
                    info!("Object placed in slot: {:?}", to_place_entity);
                    put_system.is_object_placed = true;
                    put_system.current_object_placed = Some(to_place_entity);
                    to_place.is_placed = true;

                    // Magnetize to center
                    transform.translation = target_pos;
                    velocity.0 = Vec3::ZERO;
                }
            }
        }
    }
}
/// System to handle radius-based power grabbing.
pub fn handle_power_grabbing(
    input: Res<InputState>,
    mut powerer_query: Query<(Entity, &mut GrabPowerer, &GlobalTransform)>,
    grabbable_query: Query<(Entity, &GlobalTransform), With<Grabbable>>,
    mut event_queue: ResMut<GrabEventQueue>,
) {
    for (entity, mut powerer, transform) in powerer_query.iter_mut() {
        if !powerer.is_enabled { continue; }

        if input.interact_pressed && powerer.held_objects.is_empty() {
             let center = transform.translation();
             for (target_entity, target_xf) in grabbable_query.iter() {
                 if target_xf.translation().distance(center) < powerer.grab_radius {
                     event_queue.0.push(GrabEvent::Grab(entity, target_entity));
                 }
             }
        }
    }
}

/// System to update positions of multiple held objects (Power mode).
pub fn update_power_held_objects(
    mut powerer_query: Query<(&mut GrabPowerer, &GlobalTransform)>,
    mut held_query: Query<(&GlobalTransform, &mut LinearVelocity), With<Grabbable>>,
) {
    for (mut powerer, transform) in powerer_query.iter_mut() {
        let mut to_remove = Vec::new();
        let forward = transform.forward();
        let right = transform.right();
        
        let held_count = powerer.held_objects.len();
        for (i, &held_entity) in powerer.held_objects.iter().enumerate() {
            if let Ok((held_xf, mut velocity)) = held_query.get_mut(held_entity) {
                // Arrange objects in a fan or arc in front of the player
                let angle = (i as f32 - (held_count as f32 - 1.0) / 2.0) * 0.5;
                let offset = (*forward * 3.0) + (*right * angle * 2.0);
                let target_pos = transform.translation() + offset;
                
                let dir = target_pos - held_xf.translation();
                velocity.0 = dir * 5.0; // Slow follow for "powers"
            } else {
                to_remove.push(held_entity);
            }
        }
        
        powerer.held_objects.retain(|e| !to_remove.contains(e));
    }
}

/// System to update object outlines.
pub fn update_outlines(
    mut _objects: Query<(&mut OutlineSettings, &Children)>,
    mut _materials: Query<&mut MeshMaterial3d<StandardMaterial>>,
) {
    // Placeholder for visual outline logic
}

/// System to handle melee attack input for grabbed objects.
pub fn handle_grab_melee(
    input: Res<InputState>,
    mut grabber_query: Query<(Entity, &Grabber, Option<&mut StatsSystem>)>,
    mut weapon_query: Query<(&GrabMeleeWeapon, &mut GrabMeleeAttackState, Option<&ImprovisedWeapon>, &GlobalTransform, Option<&ImprovisedWeaponStats>)>,
    spatial_query: SpatialQuery,
) {
    for (grabber_entity, grabber, stats_opt) in grabber_query.iter_mut() {
        let Some(held) = grabber.held_object else { continue };
        let Ok((weapon, mut state, _improv, weapon_transform, improvised_stats)) = weapon_query.get_mut(held) else { continue };
        if !input.attack_pressed || state.cooldown_timer > 0.0 || state.recoil_timer > 0.0 {
            continue;
        }

        let Some(attack) = weapon.attacks.first() else { continue };
        let (attack_damage, stamina_cost, attack_range) = if let Some(stats) = improvised_stats {
            (stats.damage * stats.damage_multiplier, stats.stamina_cost, stats.range)
        } else {
            (attack.damage * attack.damage_multiplier, attack.stamina_cost, 1.2)
        };

        // Check stamina
        if let Some(mut stats) = stats_opt {
            if let Some(current) = stats.get_derived_stat(DerivedStat::CurrentStamina).copied() {
                if current < stamina_cost {
                    continue;
                }
                stats.decrease_derived_stat(DerivedStat::CurrentStamina, stamina_cost);
            }
        }

        // Recoil if wall hit
        let ray_origin = weapon_transform.translation();
        let ray_dir = weapon_transform.forward();
        let filter = SpatialQueryFilter::from_excluded_entities([grabber_entity, held]);
        if let Some(_hit) = spatial_query.cast_ray(
            ray_origin,
            Dir3::new(*ray_dir).unwrap_or(Dir3::NEG_Z),
            0.6,
            true,
            &filter,
        ) {
            state.recoil_timer = 0.25;
            state.hitbox_active = false;
            continue;
        }

        state.attack_timer = attack.duration;
        state.cooldown_timer = attack.duration * 0.75;
        state.hitbox_active = true;
        state.attack_range = attack_range;
        state.damage = attack_damage;
        info!("Melee attack with grabbed object: {:?}", held);
    }
}

pub fn handle_placement_slots(
    input: Res<InputState>,
    mut slot_query: Query<(Entity, &mut PlacementSlot, &GlobalTransform)>,
    mut grabber_query: Query<(Entity, &mut Grabber)>,
    mut object_query: Query<(Entity, &ObjectToPlace, &mut Transform, Option<&mut LinearVelocity>, Option<&mut RigidBody>, Option<&mut GravityScale>)>,
    mut placement_events: ResMut<PlacementEventQueue>,
) {
    if !input.interact_pressed {
        return;
    }

    for (grabber_entity, mut grabber) in grabber_query.iter_mut() {
        let Some(held_entity) = grabber.held_object else { continue };
        let Ok((object_entity, object_to_place, mut object_transform, mut velocity_opt, mut rb_opt, mut gravity_opt)) =
            object_query.get_mut(held_entity) else { continue };

        for (slot_entity, mut slot, slot_xf) in slot_query.iter_mut() {
            if slot.is_occupied {
                continue;
            }

            let slot_pos = slot_xf.translation();
            let dist = object_transform.translation.distance(slot_pos);
            if dist > slot.max_distance {
                continue;
            }

            if !slot.accepted_names.is_empty()
                && !slot.accepted_names.iter().any(|name| name == &object_to_place.object_name)
            {
                continue;
            }

            let world_pos = slot_xf.transform_point(slot.snap_offset.translation);
            let world_rot = slot_xf.rotation() * slot.snap_offset.rotation;
            object_transform.translation = world_pos;
            object_transform.rotation = world_rot;

            if let Some(mut velocity) = velocity_opt.as_mut() {
                velocity.0 = Vec3::ZERO;
            }
            if let Some(mut gravity) = gravity_opt.as_mut() {
                if slot.disable_physics_on_place {
                    gravity.0 = 0.0;
                }
            }
            if let Some(mut rb) = rb_opt.as_mut() {
                if slot.disable_physics_on_place {
                    *rb = RigidBody::Static;
                }
            }

            grabber.held_object = None;
            slot.is_occupied = true;
            slot.current_object = Some(object_entity);

            if slot.use_events {
                placement_events.0.push(PlacementEvent {
                    slot: slot_entity,
                    placed_object: object_entity,
                });
            }

            break;
        }
    }
}

pub fn handle_power_throwing(
    time: Res<Time>,
    input: Res<InputState>,
    mut commands: Commands,
    mut grabber_query: Query<(Entity, &mut Grabber, &GlobalTransform, &mut GrabPowerThrow)>,
    mut event_queue: ResMut<GrabEventQueue>,
) {
    for (entity, mut grabber, transform, mut power_throw) in grabber_query.iter_mut() {
        let Some(held) = grabber.held_object else { continue };
        if !power_throw.enabled || !input.ability_use_pressed {
            continue;
        }

        let now = time.elapsed_secs();
        if now - power_throw.last_throw_time < power_throw.cooldown {
            continue;
        }

        let dir = transform.forward();
        let force = grabber.throw_force * power_throw.force_multiplier;
        event_queue.0.push(GrabEvent::Throw(entity, held, *dir, force));
        power_throw.last_throw_time = now;

        commands.entity(held).insert(PowerThrown {
            damage: power_throw.explosion_damage,
            radius: power_throw.explosion_radius,
            spawn_fx: power_throw.spawn_fx,
            fx_color: power_throw.fx_color,
            fx_radius: power_throw.fx_radius,
            fx_lifetime: power_throw.fx_lifetime,
        });
    }
}

pub fn update_grab_melee_attacks(
    time: Res<Time>,
    mut query: Query<(&GrabMeleeWeapon, &mut GrabMeleeAttackState)>,
) {
    let dt = time.delta_secs();
    for (weapon, mut state) in query.iter_mut() {
        if state.attack_timer > 0.0 {
            state.attack_timer = (state.attack_timer - dt).max(0.0);
            if state.attack_timer <= 0.0 {
                state.hitbox_active = false;
            }
        }
        if state.cooldown_timer > 0.0 {
            state.cooldown_timer = (state.cooldown_timer - dt).max(0.0);
        }
        if state.recoil_timer > 0.0 {
            state.recoil_timer = (state.recoil_timer - dt).max(0.0);
        }
        if weapon.attacks.is_empty() {
            state.hitbox_active = false;
        }
    }
}

pub fn update_grab_blocking(
    input: Res<InputState>,
    time: Res<Time>,
    mut query: Query<&mut Blocking, With<GrabBlockShield>>,
) {
    for mut blocking in query.iter_mut() {
        if input.block_pressed {
            if !blocking.is_blocking {
                blocking.is_blocking = true;
                blocking.current_block_time = 0.0;
            } else {
                blocking.current_block_time += time.delta_secs();
            }
        } else {
            blocking.is_blocking = false;
            blocking.current_block_time = 0.0;
        }
    }
}

pub fn perform_grab_melee_damage(
    mut damage_queue: ResMut<DamageEventQueue>,
    spatial_query: SpatialQuery,
    grabber_query: Query<(Entity, &Grabber)>,
    weapon_query: Query<(Entity, &GlobalTransform, &GrabMeleeWeapon, &GrabMeleeAttackState)>,
    targets: Query<Entity, Or<(With<crate::combat::Health>, With<crate::combat::DamageReceiver>)>>,
) {
    for (weapon_entity, transform, weapon, state) in weapon_query.iter() {
        if !state.hitbox_active {
            continue;
        }

        let Some(attack) = weapon.attacks.first() else { continue };
        let damage_type = map_attack_type(&attack.attack_type, weapon.damage_type_id);

        // Find owner to exclude
        let mut owner_entity = None;
        for (grabber_entity, grabber) in grabber_query.iter() {
            if grabber.held_object == Some(weapon_entity) {
                owner_entity = Some(grabber_entity);
                break;
            }
        }

        let origin = transform.translation();
        let forward = transform.forward();
        let shape = Collider::sphere(0.6);

        if let Some(hit) = spatial_query.cast_shape(
            &shape,
            origin,
            transform.rotation(),
            forward.into(),
            &ShapeCastConfig::default().with_max_distance(state.attack_range),
            &SpatialQueryFilter::default().with_excluded_entities([weapon_entity]),
        ) {
            if targets.get(hit.entity).is_ok() {
                damage_queue.0.push(DamageEvent {
                    amount: state.damage,
                    damage_type,
                    source: owner_entity,
                    target: hit.entity,
                    position: Some(origin + *forward * hit.distance),
                    direction: Some(*forward),
                    ignore_shield: false,
                });
            }
        }
    }
}

pub fn apply_throw_damage_on_collision(
    mut damage_queue: ResMut<DamageEventQueue>,
    collision_events: EventReader<CollisionStarted>,
    settings_query: Query<&GrabPhysicalObjectSettings>,
    mut commands: Commands,
    power_query: Query<&PowerThrown>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    transform_query: Query<&GlobalTransform>,
) {
    for event in collision_events.read() {
        let (a, b) = (event.entity1, event.entity2);
        if let Ok(settings) = settings_query.get(a) {
            if settings.throw_damage > 0.0 {
                damage_queue.0.push(DamageEvent {
                    amount: settings.throw_damage,
                    damage_type: DamageType::Melee,
                    source: Some(a),
                    target: b,
                    position: None,
                    direction: None,
                    ignore_shield: false,
                });
            }
        }
        if let Ok(power) = power_query.get(a) {
            let position = transform_query
                .get(a)
                .map(|t| t.translation())
                .unwrap_or(Vec3::ZERO);
            commands.spawn((
                AreaEffect {
                    damage_type: DamageType::Explosion,
                    amount: power.damage,
                    radius: power.radius,
                    interval: 0.05,
                    timer: 0.0,
                    duration: Some(0.05),
                    ignore_shield: false,
                    source: Some(a),
                },
                Transform::from_translation(position),
                GlobalTransform::default(),
                Name::new("PowerThrowExplosion"),
            ));

            if power.spawn_fx {
                let mesh = meshes.add(Mesh::from(Sphere::new(power.fx_radius)));
                let material = materials.add(StandardMaterial {
                    base_color: power.fx_color,
                    unlit: true,
                    ..default()
                });
                commands.spawn((
                    Mesh3d(mesh),
                    MeshMaterial3d(material),
                    Transform::from_translation(position),
                    GlobalTransform::default(),
                    Visibility::default(),
                    GrabPowerFx {
                        lifetime: power.fx_lifetime,
                    },
                    Name::new("PowerThrowFx"),
                ));
            }

            commands.entity(a).remove::<PowerThrown>();
        }
        if let Ok(settings) = settings_query.get(b) {
            if settings.throw_damage > 0.0 {
                damage_queue.0.push(DamageEvent {
                    amount: settings.throw_damage,
                    damage_type: DamageType::Melee,
                    source: Some(b),
                    target: a,
                    position: None,
                    direction: None,
                    ignore_shield: false,
                });
            }
        }
        if let Ok(power) = power_query.get(b) {
            let position = transform_query
                .get(b)
                .map(|t| t.translation())
                .unwrap_or(Vec3::ZERO);
            commands.spawn((
                AreaEffect {
                    damage_type: DamageType::Explosion,
                    amount: power.damage,
                    radius: power.radius,
                    interval: 0.05,
                    timer: 0.0,
                    duration: Some(0.05),
                    ignore_shield: false,
                    source: Some(b),
                },
                Transform::from_translation(position),
                GlobalTransform::default(),
                Name::new("PowerThrowExplosion"),
            ));

            if power.spawn_fx {
                let mesh = meshes.add(Mesh::from(Sphere::new(power.fx_radius)));
                let material = materials.add(StandardMaterial {
                    base_color: power.fx_color,
                    unlit: true,
                    ..default()
                });
                commands.spawn((
                    Mesh3d(mesh),
                    MeshMaterial3d(material),
                    Transform::from_translation(position),
                    GlobalTransform::default(),
                    Visibility::default(),
                    GrabPowerFx {
                        lifetime: power.fx_lifetime,
                    },
                    Name::new("PowerThrowFx"),
                ));
            }

            commands.entity(b).remove::<PowerThrown>();
        }
    }
}

pub fn update_power_throw_fx(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut GrabPowerFx)>,
) {
    let dt = time.delta_secs();
    for (entity, mut fx) in query.iter_mut() {
        fx.lifetime -= dt;
        if fx.lifetime <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn map_attack_type(attack_type: &str, damage_type_id: i32) -> DamageType {
    let lower = attack_type.to_lowercase();
    if lower.contains("fire") {
        DamageType::Fire
    } else if lower.contains("poison") {
        DamageType::Poison
    } else if lower.contains("electric") || lower.contains("shock") {
        DamageType::Electric
    } else if lower.contains("explosion") || damage_type_id == 3 {
        DamageType::Explosion
    } else {
        DamageType::Melee
    }
}

pub fn update_grab_melee_hitboxes(
    mut zones: Query<&mut DamageZone>,
    weapon_query: Query<(Entity, &GrabMeleeAttackState)>,
) {
    for (weapon_entity, state) in weapon_query.iter() {
        for mut zone in zones.iter_mut() {
            if zone.owner == weapon_entity {
                zone.active = state.hitbox_active;
            }
        }
    }
}
