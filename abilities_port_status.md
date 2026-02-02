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
- [ ] Ability-specific systems from GKC: `dashSystem`
- [ ] Ability-specific systems from GKC: `magicSpellActionSystem`
- [ ] Ability-specific systems from GKC: `oxygenSystem`
- [ ] Ability-specific systems from GKC: `staminaSystem`
- [ ] Ability-specific systems from GKC: `throwObjectTrayectorySystem`
- [ ] Ability-specific systems from GKC: `wallRunningZoneSystem`
- [ ] Particle collision/trigger helpers (`particleCollisionDetection`, `particleTriggerDetection`)
- [ ] Custom ability base/template behavior (`Custom Abilities/templateAbilitySystem`)
- [ ] Abilities + weapons/powers integration hooks (GKC: `playerWeaponSystem`, `powersAndAbilitiesSystem`)
- [ ] Ability pickup/enable hooks from inventory/pickups
- [ ] Editor tooling equivalents (Unity inspectors) not applicable / not ported

## Next Suggested Steps
- [ ] Wire ability events and add `add_event` registrations in the plugin
- [ ] Connect ability activation to input actions (press/hold/release)
- [ ] Implement hold behavior (continuous use + energy drain)
- [ ] Integrate grounded / first-person constraints with character/camera systems
- [ ] Port stamina/energy to shared Stats system (or a dedicated Energy resource)
- [ ] Build minimal UI indicators (current ability + cooldown/limit)
