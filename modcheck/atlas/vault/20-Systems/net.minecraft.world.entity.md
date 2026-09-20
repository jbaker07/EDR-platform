---
type: "system"
package: "net.minecraft.world.entity"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity

Analyst note: [[_authored/systems/net.minecraft.world.entity|Entities, living entities, mobs and AI]]

1157 classes in the jar. Hooked types: 17

- [[40-Interfaces/net.minecraft.world.entity.Entity|Entity]] -- calls:24, injects_into:6 -- by fabric-api-lookup-api-v1, fabric-content-registries-v0, fabric-data-attachment-api-v1, fabric-debug-api-v1, fabric-entity-events-v1, fabric-events-interaction-v0, fabric-lifecycle-events-v1, fabric-object-builder-api-v1, fabric-permission-api-v1
- [[40-Interfaces/net.minecraft.world.entity.EntityFluidInteraction|EntityFluidInteraction]] -- calls:3 -- by fabric-content-registries-v0
- [[40-Interfaces/net.minecraft.world.entity.EntitySpawnRequest|EntitySpawnRequest]] -- calls:1 -- by fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.world.entity.EntityType|EntityType]] -- calls:2, injects_into:3 -- by fabric-api-lookup-api-v1, fabric-client-gametest-api-v1, fabric-lifecycle-events-v1, fabric-object-builder-api-v1
- [[40-Interfaces/net.minecraft.world.entity.EntityType_Builder|EntityType$Builder]] -- injects_into:1 -- by fabric-object-builder-api-v1
- [[40-Interfaces/net.minecraft.world.entity.EntityTypes|EntityTypes]] -- reads:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] -- calls:20, injects_into:23, wraps:4 -- by fabric-block-api-v1, fabric-content-registries-v0, fabric-entity-events-v1, fabric-item-api-v1, fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.world.entity.Mob|Mob]] -- injects_into:2 -- by fabric-debug-api-v1, fabric-entity-events-v1
- [[40-Interfaces/net.minecraft.world.entity.ai.attributes.AttributeSupplier|AttributeSupplier]] -- calls:1 -- by fabric-object-builder-api-v1
- [[40-Interfaces/net.minecraft.world.entity.ai.attributes.DefaultAttributes|DefaultAttributes]] -- injects_into:1 -- by fabric-object-builder-api-v1
- [[40-Interfaces/net.minecraft.world.entity.ai.behavior.GiveGiftToHero|GiveGiftToHero]] -- injects_into:1 -- by fabric-content-registries-v0
- [[40-Interfaces/net.minecraft.world.entity.monster.Monster|Monster]] -- calls:2 -- by fabric-particles-v1
- [[40-Interfaces/net.minecraft.world.entity.player.Abilities|Abilities]] -- reads:1 -- by fabric-events-interaction-v0
- [[40-Interfaces/net.minecraft.world.entity.player.Inventory|Inventory]] -- calls:3 -- by fabric-events-interaction-v0, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.entity.player.Player|Player]] -- calls:17, injects_into:3 -- by fabric-content-registries-v0, fabric-entity-events-v1, fabric-events-interaction-v0, fabric-menu-api-v1, fabric-permission-api-v1, fabric-rendering-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.entity.player.Player_BedSleepingProblem|Player$BedSleepingProblem]] -- reads:1 -- by fabric-entity-events-v1
- [[40-Interfaces/net.minecraft.world.entity.vehicle.minecart.AbstractMinecart|AbstractMinecart]] -- calls:1 -- by fabric-object-builder-api-v1
