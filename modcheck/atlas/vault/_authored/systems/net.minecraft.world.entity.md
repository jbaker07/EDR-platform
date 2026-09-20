---
type: "system_note"
id: "net.minecraft.world.entity"
side: "shared_by_design"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Entities, living entities, mobs and AI

Package `net.minecraft.world.entity` -- generated view: [[20-Systems/net.minecraft.world.entity|inventory and hooked types]]

**Responsibility.** Entity, LivingEntity, Mob, Player and their subclasses, plus the ai subpackages (goals, brain behaviours, navigation, sensing). Server-side AI ticks here; client-side copies of entities exist for rendering and prediction.

**Side.** shared_by_design

**Threads.** Ticks on the owning side's thread; AI runs on the server only.

**Persistence.** Entity NBT save/load; attachments from [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] attach to entities and can be persisted and synced.

## Extension points

- [[40-Interfaces/net.minecraft.world.entity.Mob|Mob]] declares `protected final GoalSelector goalSelector` (extracted); adding a goal from outside the class needs an accessor or access widener.
- [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] declares getBrain(); brain-driven mobs (villagers) are not modified through goalSelector ([[80-Unresolved/q.villager_ai_architecture|q.villager_ai_architecture]]).
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.ServerLivingEntityEvents.ALLOW_DEATH|ALLOW_DEATH]] is a Redirect of LivingEntity.isDeadOrDying inside hurtServer, so a listener returning false keeps the entity alive.
- [[50-Interactions/events/net.fabricmc.fabric.api.event.player.AttackEntityCallback.EVENT|EVENT]] is injected at HEAD of Player.attack on both sides.
- Entity types are registered into [[30-Mechanisms/Registries|Registries]] ENTITY_TYPE; default attributes through [[50-Interactions/events/net.fabricmc.fabric.api.object.builder.v1.entity.FabricDefaultAttributeRegistry.MODIFY|MODIFY]].

## Interactions to expect

- A goal added on ENTITY_LOAD is added on every load of that entity, including chunk reloads; the listener must check for its own goal before adding.

## Evidence

- `extracted/minecraft_surface.json.gz`
- `extracted/edges.json#wraps`
- `extracted/edges.json#injects_into`

## Open questions

- [[80-Unresolved/q.villager_ai_architecture|q.villager_ai_architecture]]
- [[80-Unresolved/q.unhooked_vanilla_members|q.unhooked_vanilla_members]]

