---
type: "workflow"
id: "wf.presentation.particles_sounds"
area: "presentation"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Particles and sounds

**Intent.** Feedback the player perceives: a particle effect at an event, a sound on an action, with the right attenuation and category.

## Must be preserved

- Vanilla sound categories and volume settings.

## Mechanisms that can serve it

- [[30-Mechanisms/Registries|Registries]] -- PARTICLE_TYPE and SOUND_EVENT.
- [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] -- particle factory registration on the client.
- [[30-Mechanisms/fabric-sound-api-v1|fabric-sound-api-v1]].
- Resource pack sounds.json and particle JSON (asset entry types in `extracted/corpus.json`).

## Tools and artifacts used today

- Registration in code plus JSON and audio files.

## Decisions the creator must make

- Server-spawned particles (visible to all) versus client-only.

## Information those decisions need

- The 26.3 particle registration signature (particle package hooked: 6 types in `extracted/minecraft_members.json`).

## Existing automation

- None.

## Remaining manual or unsupported work

- All.

## ModCheck's contribution

- Not yet planned.

## Evidence

- `extracted/minecraft_registries.json`
- `extracted/fabric_api.json#fabric-particles-v1`

## Status

- inventoried: True
- mechanically_inspected: True
- contract_mapped: False
- interaction_analysed: False
- implemented_in_modcheck: False
- validated_scope: none
