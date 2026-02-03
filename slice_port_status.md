# Slice System Port Status

## GKit Files (gkit/Scripts/Combat System/Melee Combat System/Slice System)
- [ ] sliceSystem.cs
- [ ] sliceSystemUtils.cs
- [ ] simpleSliceSystem.cs
- [ ] cuttingModeSystem.cs
- [ ] surfaceToSlice.cs
- [ ] sliceObject.cs

## Related GKit Files
- [ ] plasmaCutterProjectile.cs
- [ ] grabbedObjectMeleeAttackSystem.cs (slice hooks)
- [ ] addSliceSystemToCharacterCreatorEditor.cs (editor tooling)

## Bevy Implementation
- [x] Slice events and result queues
- [x] Sliceable component (cooldown + radius)
- [x] LaserVision -> SliceEvent adapter
- [ ] Mesh slicing (CSG / slicer integration)
- [ ] Cut surface caps + UVs
- [ ] Physics bodies on sliced parts
- [ ] DamageEvent -> SliceEvent (Slashing)
