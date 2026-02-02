# Abilities System Port Status (Bevy 0.18)

Source: `gkit/Scripts/Abilities System/`

## Implemented
- [x] Core data model for abilities (`AbilityInfo` component)
- [x] Ability status + input type enums
- [x] Cooldown and time-limit timers with per-frame update
- [x] Enable/disable/deactivate ability helpers
- [x] Basic input handlers for press down / press up (state toggles)
- [x] Energy consumption flags + simple energy tracking in `PlayerAbilitiesSystem`
- [x] Ability selection by name + temporary ability switching
- [x] Basic abilities plugin and update system wiring

## Partially Implemented / Stubs
- [x] Ability activation/deactivation events are defined but not wired (`handle_*` systems are empty)
- [x] `PressHold` behavior is a placeholder (no hold-timer or continuous action logic)
- [x] Grounded / first-person checks are TODOs (no integration with character/camera yet)
- [x] Ability cooldown/limit side-effects (e.g., UI, FX, events) are not triggered
- [x] Energy system is local to `PlayerAbilitiesSystem` (not connected to Stats/Stamina systems)

## Not Implemented Yet (GKC Parity)
- [x] Ability UI wheel + slot elements (`abilitySlotElement`, `playerAbilitiesUISystem`)
- [x] Ability input buffering/mapping integration with the input system
- [x] Ability-specific systems from GKC: `dashSystem`
- [x] Ability-specific systems from GKC: `magicSpellActionSystem`
- [x] Ability-specific systems from GKC: `oxygenSystem`
- [x] Ability-specific systems from GKC: `staminaSystem`
- [x] Ability-specific systems from GKC: `throwObjectTrayectorySystem`
- [x] Ability-specific systems from GKC: `wallRunningZoneSystem`
- [x] Particle collision/trigger helpers (`particleCollisionDetection`, `particleTriggerDetection`)
- [x] Custom ability base/template behavior (`Custom Abilities/templateAbilitySystem`)
- [x] Abilities + weapons/powers integration hooks (GKC: `playerWeaponSystem`, `powersAndAbilitiesSystem`)
- [x] Ability pickup/enable hooks from inventory/pickups
- [x] Editor tooling equivalents (Unity inspectors) not applicable / not ported (N/A in Bevy)

## Next Suggested Steps
- [x] Wire ability events and add `add_event` registrations in the plugin (implemented via event queues)
- [x] Connect ability activation to input actions (press/hold/release)
- [x] Implement hold behavior (continuous use + energy drain)
- [x] Integrate grounded / first-person constraints with character/camera systems
- [x] Port stamina/energy to shared Stats system (or a dedicated Energy resource)
- [ ] Build minimal UI indicators (current ability + cooldown/limit)
