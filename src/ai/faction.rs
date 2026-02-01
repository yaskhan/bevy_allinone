use bevy::prelude::*;
use crate::ai::types::*;

/// System to handle faction-based targeting and perception filtering.
pub fn update_faction_relations(
    mut faction_system: ResMut<FactionSystem>,
    // Add logic for dynamic relation changes (e.g., turn to enemy if attacked)
) {
    
}

/// System to alert other members of the same faction on spotted target.
pub fn alert_faction_members(
    _faction_system: Res<FactionSystem>,
    mut query: Query<(&CharacterFaction, &mut AiController, &GlobalTransform)>,
) {
    // Collect all targets spotted by members of each faction
    // For now, if one member of faction A has a target, others in 20m radius are alerted
    let mut alerts = Vec::new();
    for (faction, ai, transform) in query.iter() {
        if let Some(target) = ai.target {
            alerts.push((faction.name.clone(), target, transform.translation()));
        }
    }

    for (faction, mut ai, transform) in query.iter_mut() {
        if ai.target.is_some() { continue; }
        for (alert_faction, alert_target, alert_pos) in &alerts {
            if faction.name == *alert_faction {
                if transform.translation().distance(*alert_pos) < 20.0 {
                    ai.target = Some(*alert_target);
                    ai.state = AiBehaviorState::Chase;
                }
            }
        }
    }
}

impl FactionSystem {
    pub fn is_friendly(&self, own_faction: &str, other_faction: &str) -> bool {
        self.get_relation(own_faction, other_faction) == FactionRelation::Friend
    }

    pub fn is_enemy(&self, own_faction: &str, other_faction: &str) -> bool {
        self.get_relation(own_faction, other_faction) == FactionRelation::Enemy
    }

    pub fn set_relation(&mut self, f1: &str, f2: &str, relation: FactionRelation) {
        if let Some(rel) = self.relations.iter_mut().find(|r| 
            (r.faction_a == f1 && r.faction_b == f2) || (r.faction_a == f2 && r.faction_b == f1)
        ) {
            rel.relation = relation;
        } else {
            self.relations.push(FactionRelationInfo {
                faction_a: f1.to_string(),
                faction_b: f2.to_string(),
                relation,
            });
        }
    }
}
