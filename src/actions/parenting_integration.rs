use bevy::prelude::*;
use super::types::*;

/// Process parenting events triggered by actions
pub fn process_parenting_events(
    mut parenting_queue: ResMut<ParentingEventQueue>,
    mut player_query: Query<(Entity, &mut PlayerActionSystem, &Children)>,
    name_query: Query<&Name>,
    children_query: Query<&Children>,
    mut commands: Commands,
) {
    for event in parenting_queue.0.drain(..) {
        if let Ok((player_entity, mut player_action, children)) = player_query.get_mut(event.player_entity) {
            match event.event_type {
                BoneParentingEventType::ParentToBone { object_entity, bone_name, config } => {
                    let target_to_parent = if let Some(ent) = object_entity {
                        ent
                    } else {
                        warn!("ParentToBone: No object entity provided for action");
                        continue;
                    };

                    let mut found_bone = None;
                    for child in children.iter() {
                        if let Some(bone) = recursive_find_bone(child, &bone_name, &name_query, &children_query) {
                            found_bone = Some(bone);
                            break;
                        }
                    }

                    if let Some(bone_entity) = found_bone {
                        let parented_obj = ParentedObject {
                            entity: target_to_parent,
                            bone_entity,
                            original_parent: None, 
                            config: config.clone(),
                            is_transitioning: config.use_smooth_transition,
                            current_lerp: 0.0,
                        };
                        
                        player_action.parented_objects.push(parented_obj);
                        
                        commands.entity(target_to_parent).set_parent_in_place(bone_entity);
                        
                        if !config.use_smooth_transition {
                            commands.entity(target_to_parent).insert(Transform::from_translation(config.local_offset).with_rotation(config.local_rotation));
                        }
                        
                        info!("Action parented {:?} to bone '{}' ({:?})", target_to_parent, bone_name, bone_entity);
                    } else {
                        warn!("Action could not find bone '{}' on player {:?}", bone_name, player_entity);
                    }
                }
                BoneParentingEventType::Unparent { object_entity, restore_original_parent } => {
                    if let Some(index) = player_action.parented_objects.iter().position(|obj| {
                        object_entity.map_or(true, |target| obj.entity == target)
                    }) {
                        let obj = player_action.parented_objects.remove(index);
                        
                        if restore_original_parent {
                            if let Some(orig) = obj.original_parent {
                                commands.entity(obj.entity).set_parent_in_place(orig);
                            } else {
                                commands.entity(obj.entity).remove_parent_in_place();
                            }
                        } else {
                            commands.entity(obj.entity).remove_parent_in_place();
                        }
                        
                        info!("Action unparented {:?}", obj.entity);
                    }
                }
                BoneParentingEventType::None => {}
            }
        }
    }
}

pub fn update_bone_parenting_system(
    mut player_query: Query<&mut PlayerActionSystem>,
    mut target_query: Query<(&mut Transform, &GlobalTransform)>,
    bone_query: Query<&GlobalTransform>,
    time: Res<Time>,
) {
    for mut player_action in player_query.iter_mut() {
        for obj in player_action.parented_objects.iter_mut() {
            if let Ok(_bone_gt) = bone_query.get(obj.bone_entity) {
                if obj.is_transitioning {
                    obj.current_lerp = (obj.current_lerp + time.delta_secs() * obj.config.transition_speed).min(1.0);
                    
                    if let Ok((mut transform, _gt)) = target_query.get_mut(obj.entity) {
                        let target_local_pos = obj.config.local_offset;
                        let target_local_rot = obj.config.local_rotation;
                        
                        transform.translation = transform.translation.lerp(target_local_pos, obj.current_lerp);
                        transform.rotation = transform.rotation.slerp(target_local_rot, obj.current_lerp);
                        
                        if obj.current_lerp >= 1.0 {
                            obj.is_transitioning = false;
                        }
                    }
                }
            }
        }
    }
}

pub fn recursive_find_bone(
    entity: Entity,
    name_to_find: &str,
    name_query: &Query<&Name>,
    children_query: &Query<&Children>,
) -> Option<Entity> {
    if let Ok(name) = name_query.get(entity) {
        if name.as_str() == name_to_find {
            return Some(entity);
        }
    }
    
    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            if let Some(found) = recursive_find_bone(child, name_to_find, name_query, children_query) {
                return Some(found);
            }
        }
    }
    
    None
}
