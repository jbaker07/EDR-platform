---
type: "mechanism"
module: "fabric-events-interaction-v0"
version: "5.3.6+3434d6d95d"
sha256: "f57dd8df1d78cbcaf6ee1073aebd64e5c959cc7224002098e653832f0ecd7de8"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-events-interaction-v0

**Version** `5.3.6+3434d6d95d` -- **artifact sha256** `f57dd8df1d78cbcaf6ee1073aebd64e5c959cc7224002098e653832f0ecd7de8`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "fabric-api-base": "*", "fabric-networking-api-v1": "*", "minecraft": ">=1.15-alpha.19.37.a"}`
- entrypoints: `{"main": ["net.fabricmc.fabric.impl.event.interaction.InteractionEventsRouter"]}`
- mixin configs: `["fabric-events-interaction-v0.mixins.json", {"config": "fabric-events-interaction-v0.client.mixins.json", "environment": "client"}]`

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.event.client.player.ClientHotbarScrollEvents.AFTER|ClientHotbarScrollEvents.AFTER]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.client.player.ClientHotbarScrollEvents.ALLOW|ClientHotbarScrollEvents.ALLOW]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.client.player.ClientHotbarScrollEvents.BEFORE|ClientHotbarScrollEvents.BEFORE]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.client.player.ClientPlayerBlockBreakEvents.AFTER|ClientPlayerBlockBreakEvents.AFTER]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.client.player.ClientPreAttackCallback.EVENT|ClientPreAttackCallback.EVENT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.player.AttackBlockCallback.EVENT|AttackBlockCallback.EVENT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.player.AttackEntityCallback.EVENT|AttackEntityCallback.EVENT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.player.BlockEvents.USE_ITEM_ON|BlockEvents.USE_ITEM_ON]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.player.BlockEvents.USE_WITHOUT_ITEM|BlockEvents.USE_WITHOUT_ITEM]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.player.ItemEvents.USE|ItemEvents.USE]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.player.ItemEvents.USE_ON|ItemEvents.USE_ON]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.player.PlayerBlockBreakEvents.AFTER|PlayerBlockBreakEvents.AFTER]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.player.PlayerBlockBreakEvents.BEFORE|PlayerBlockBreakEvents.BEFORE]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.player.PlayerBlockBreakEvents.CANCELED|PlayerBlockBreakEvents.CANCELED]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.player.PlayerPickItemEvents.BLOCK|PlayerPickItemEvents.BLOCK]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.player.PlayerPickItemEvents.ENTITY|PlayerPickItemEvents.ENTITY]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.player.UseBlockCallback.EVENT|UseBlockCallback.EVENT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.player.UseEntityCallback.EVENT|UseEntityCallback.EVENT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.player.UseItemCallback.EVENT|UseItemCallback.EVENT]]

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]] | `continueAttack` | injects_into `@Inject at HEAD` | client | `MinecraftMixin.injectHandleBlockBreakingForCancelling` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]] | `handleKeybinds` | injects_into `@Inject at INVOKE Lnet/minecraft/client/player/LocalPlayer;isUsingItem()Z` | client | `MinecraftMixin.injectHandleInputEventsForPreAttackCallback` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]] | `startAttack` | injects_into `@Inject at HEAD` | client | `MinecraftMixin.injectDoAttackForCancelling` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]] | `startUseItem` | injects_into `@Inject at INVOKE Lnet/minecraft/client/multiplayer/MultiPlayerGameMode;interact(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/EntityHitResult;Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/InteractionResult;` | client | `MinecraftMixin.injectUseEntityCallback` |
| [[40-Interfaces/net.minecraft.client.multiplayer.MultiPlayerGameMode|MultiPlayerGameMode]] | `attack` | injects_into `@Inject at INVOKE Lnet/minecraft/client/multiplayer/ClientPacketListener;send(Lnet/minecraft/network/protocol/Packet;)V` | client | `MultiPlayerGameModeMixin.attackEntity` |
| [[40-Interfaces/net.minecraft.client.multiplayer.MultiPlayerGameMode|MultiPlayerGameMode]] | `continueDestroyBlock` | injects_into `@Inject at INVOKE Lnet/minecraft/client/player/LocalPlayer;getAbilities()Lnet/minecraft/world/entity/player/Abilities;` | client | `MultiPlayerGameModeMixin.method_2902` |
| [[40-Interfaces/net.minecraft.client.multiplayer.MultiPlayerGameMode|MultiPlayerGameMode]] | `destroyBlock` | injects_into `@Inject at INVOKE Lnet/minecraft/world/level/block/Block;destroy(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V` | client | `MultiPlayerGameModeMixin.fabric$onBlockBroken` |
| [[40-Interfaces/net.minecraft.client.multiplayer.MultiPlayerGameMode|MultiPlayerGameMode]] | `startDestroyBlock` | injects_into `@Inject at INVOKE Lnet/minecraft/client/player/LocalPlayer;getAbilities()Lnet/minecraft/world/entity/player/Abilities;` | client | `MultiPlayerGameModeMixin.attackBlock` |
| [[40-Interfaces/net.minecraft.client.multiplayer.MultiPlayerGameMode|MultiPlayerGameMode]] | `useItem` | injects_into `@Inject at INVOKE Lnet/minecraft/client/multiplayer/MultiPlayerGameMode;ensureHasSentCarriedItem()V` | client | `MultiPlayerGameModeMixin.interactItem` |
| [[40-Interfaces/net.minecraft.client.multiplayer.MultiPlayerGameMode|MultiPlayerGameMode]] | `useItemOn` | injects_into `@Inject at INVOKE Lnet/minecraft/client/multiplayer/MultiPlayerGameMode;startPrediction(Lnet/minecraft/client/multiplayer/ClientLevel;Lnet/minecraft/client/multiplayer/prediction/PredictiveAction;)V` | client | `MultiPlayerGameModeMixin.interactBlock` |
| [[40-Interfaces/net.minecraft.server.PlayerAdvancements|PlayerAdvancements]] | `award` | injects_into `@Inject at HEAD` | both | `PlayerAdvancementsMixin.preventGrantCriterion` |
| [[40-Interfaces/net.minecraft.server.PlayerAdvancements|PlayerAdvancements]] | `setPlayer` | injects_into `@Inject at HEAD` | both | `PlayerAdvancementsMixin.preventOwnerOverride` |
| [[40-Interfaces/net.minecraft.server.level.ServerPlayer|ServerPlayer]] | `calculateGameModeForNewPlayer` | injects_into `@Inject at HEAD` | both | `ServerPlayerMixin.fakePlayerGameMode` |
| [[40-Interfaces/net.minecraft.server.level.ServerPlayerGameMode|ServerPlayerGameMode]] | `destroyBlock` | injects_into `@Inject at INVOKE Lnet/minecraft/world/level/block/Block;playerWillDestroy(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/entity/player/Player;)Lnet/minecraft/world/level/block/state/BlockState;` | both | `ServerPlayerGameModeMixin.breakBlock` |
| [[40-Interfaces/net.minecraft.server.level.ServerPlayerGameMode|ServerPlayerGameMode]] | `destroyBlock` | injects_into `@Inject at INVOKE Lnet/minecraft/world/level/block/Block;destroy(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V` | both | `ServerPlayerGameModeMixin.onBlockBroken` |
| [[40-Interfaces/net.minecraft.server.level.ServerPlayerGameMode|ServerPlayerGameMode]] | `handleBlockBreakAction` | injects_into `@Inject at HEAD` | both | `ServerPlayerGameModeMixin.startBlockBreak` |
| [[40-Interfaces/net.minecraft.server.level.ServerPlayerGameMode|ServerPlayerGameMode]] | `useItem` | injects_into `@Inject at HEAD` | both | `ServerPlayerGameModeMixin.interactItem` |
| [[40-Interfaces/net.minecraft.server.level.ServerPlayerGameMode|ServerPlayerGameMode]] | `useItemOn` | injects_into `@Inject at HEAD` | both | `ServerPlayerGameModeMixin.interactBlock` |
| [[40-Interfaces/net.minecraft.server.network.ServerGamePacketListenerImpl|ServerGamePacketListenerImpl]] | `handleInteract` | injects_into `@Inject at INVOKE Lnet/minecraft/server/level/ServerPlayer;getItemInHand(Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/item/ItemStack;` | both | `ServerGamePacketListenerImplMixin.handleInteract` |
| [[40-Interfaces/net.minecraft.world.entity.player.Player|Player]] | `attack` | injects_into `@Inject at HEAD` | both | `PlayerMixin.onPlayerInteractEntity` |
| [[40-Interfaces/net.minecraft.world.level.block.state.BlockBehaviour_BlockStateBase|BlockBehaviour$BlockStateBase]] | `useItemOn` | injects_into `@Inject at HEAD` | both | `BlockBehaviourBlockStateBaseMixin.callUseItemOnEvent` |
| [[40-Interfaces/net.minecraft.world.level.block.state.BlockBehaviour_BlockStateBase|BlockBehaviour$BlockStateBase]] | `useWithoutItem` | injects_into `@Inject at HEAD` | both | `BlockBehaviourBlockStateBaseMixin.callUseWithoutItemEvent` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.block.BlockAttackInteractionAware|BlockAttackInteractionAware]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.entity.FakePlayer|FakePlayer]] (class, 16 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.client.player.ClientHotbarScrollEvents|ClientHotbarScrollEvents]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.client.player.ClientPlayerBlockBreakEvents|ClientPlayerBlockBreakEvents]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.client.player.ClientPreAttackCallback|ClientPreAttackCallback]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.player.AttackBlockCallback|AttackBlockCallback]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.player.AttackEntityCallback|AttackEntityCallback]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.player.BlockEvents|BlockEvents]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.player.ItemEvents|ItemEvents]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.player.PlayerBlockBreakEvents|PlayerBlockBreakEvents]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.player.PlayerPickItemEvents|PlayerPickItemEvents]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.player.UseBlockCallback|UseBlockCallback]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.player.UseEntityCallback|UseEntityCallback]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.player.UseItemCallback|UseItemCallback]] (interface, 3 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
