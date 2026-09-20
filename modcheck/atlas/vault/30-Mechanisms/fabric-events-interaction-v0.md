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
- mixin classes: 11 found by annotation, 11 declared in configs; extraction failures: 0

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

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`continueAttack` | `(Z)V` | name_only | @Inject | HEAD | client | 1000 (default) | `MinecraftMixin.injectHandleBlockBreakingForCancelling` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`handleKeybinds` | `()V` | name_only | @Inject | INVOKE `Lnet/minecraft/client/player/LocalPlayer;isUsingItem()Z` (exact) | client | 1000 (default) | `MinecraftMixin.injectHandleInputEventsForPreAttackCallback` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`startAttack` | `()Z` | name_only | @Inject | HEAD | client | 1000 (default) | `MinecraftMixin.injectDoAttackForCancelling` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`startUseItem` | `()V` | name_only | @Inject | INVOKE `Lnet/minecraft/client/multiplayer/MultiPlayerGameMode;interact(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/EntityHitResult;Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/InteractionResult;` (exact) | client | 1000 (default) | `MinecraftMixin.injectUseEntityCallback` |
| [[40-Interfaces/net.minecraft.client.MouseHandler|MouseHandler]].`onScroll` | `(JDD)V` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/world/entity/player/Inventory;setSelectedSlot(I)V` (exact) | client | 1000 (default) | `MouseHandlerMixin.wrapSelectedSlot` |
| [[40-Interfaces/net.minecraft.client.multiplayer.MultiPlayerGameMode|MultiPlayerGameMode]].`attack` | `(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/entity/Entity;)V` | name_only | @Inject | INVOKE `Lnet/minecraft/client/multiplayer/ClientPacketListener;send(Lnet/minecraft/network/protocol/Packet;)V` (inherited_exact) | client | 1000 (default) | `MultiPlayerGameModeMixin.attackEntity` |
| [[40-Interfaces/net.minecraft.client.multiplayer.MultiPlayerGameMode|MultiPlayerGameMode]].`continueDestroyBlock` | `(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;)Z` | name_only | @Inject | INVOKE `Lnet/minecraft/client/player/LocalPlayer;getAbilities()Lnet/minecraft/world/entity/player/Abilities;` (inherited_exact) | client | 1000 (default) | `MultiPlayerGameModeMixin.method_2902` |
| [[40-Interfaces/net.minecraft.client.multiplayer.MultiPlayerGameMode|MultiPlayerGameMode]].`destroyBlock` | `(Lnet/minecraft/core/BlockPos;)Z` | name_only | @Inject | INVOKE `Lnet/minecraft/world/level/block/Block;destroy(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V` (exact) | client | 1000 (default) | `MultiPlayerGameModeMixin.fabric$onBlockBroken` |
| [[40-Interfaces/net.minecraft.client.multiplayer.MultiPlayerGameMode|MultiPlayerGameMode]].`startDestroyBlock` | `(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;)Z` | name_only | @Inject | INVOKE `Lnet/minecraft/client/player/LocalPlayer;getAbilities()Lnet/minecraft/world/entity/player/Abilities;` (inherited_exact) | client | 1000 (default) | `MultiPlayerGameModeMixin.attackBlock` |
| [[40-Interfaces/net.minecraft.client.multiplayer.MultiPlayerGameMode|MultiPlayerGameMode]].`useItem` | `(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/InteractionResult;` | name_only | @Inject | INVOKE `Lnet/minecraft/client/multiplayer/MultiPlayerGameMode;ensureHasSentCarriedItem()V` (exact) | client | 1000 (default) | `MultiPlayerGameModeMixin.interactItem` |
| [[40-Interfaces/net.minecraft.client.multiplayer.MultiPlayerGameMode|MultiPlayerGameMode]].`useItemOn` | `(Lnet/minecraft/client/player/LocalPlayer;Lnet/minecraft/world/InteractionHand;Lnet/minecraft/world/phys/BlockHitResult;)Lnet/minecraft/world/InteractionResult;` | name_only | @Inject | INVOKE `Lnet/minecraft/client/multiplayer/MultiPlayerGameMode;startPrediction(Lnet/minecraft/client/multiplayer/ClientLevel;Lnet/minecraft/client/multiplayer/prediction/PredictiveAction;)V` (exact) | client | 1000 (default) | `MultiPlayerGameModeMixin.interactBlock` |
| [[40-Interfaces/net.minecraft.server.PlayerAdvancements|PlayerAdvancements]].`award` | `(Lnet/minecraft/advancements/AdvancementHolder;Ljava/lang/String;)Z` | name_only | @Inject | HEAD | both | 1000 (default) | `PlayerAdvancementsMixin.preventGrantCriterion` |
| [[40-Interfaces/net.minecraft.server.PlayerAdvancements|PlayerAdvancements]].`setPlayer` | `(Lnet/minecraft/server/level/ServerPlayer;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `PlayerAdvancementsMixin.preventOwnerOverride` |
| [[40-Interfaces/net.minecraft.server.level.ServerPlayer|ServerPlayer]].`calculateGameModeForNewPlayer` | `(Lnet/minecraft/world/level/GameType;)Lnet/minecraft/world/level/GameType;` | name_only | @Inject | HEAD | both | 1000 (default) | `ServerPlayerMixin.fakePlayerGameMode` |
| [[40-Interfaces/net.minecraft.server.level.ServerPlayerGameMode|ServerPlayerGameMode]].`destroyBlock` | `(Lnet/minecraft/core/BlockPos;)Z` | name_only | @Inject | INVOKE `Lnet/minecraft/world/level/block/Block;playerWillDestroy(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/entity/player/Player;)Lnet/minecraft/world/level/block/state/BlockState;` (exact) | both | 1000 (default) | `ServerPlayerGameModeMixin.breakBlock` |
| [[40-Interfaces/net.minecraft.server.level.ServerPlayerGameMode|ServerPlayerGameMode]].`destroyBlock` | `(Lnet/minecraft/core/BlockPos;)Z` | name_only | @Inject | INVOKE `Lnet/minecraft/world/level/block/Block;destroy(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V` (exact) | both | 1000 (default) | `ServerPlayerGameModeMixin.onBlockBroken` |
| [[40-Interfaces/net.minecraft.server.level.ServerPlayerGameMode|ServerPlayerGameMode]].`handleBlockBreakAction` | `(Lnet/minecraft/core/BlockPos;Lnet/minecraft/network/protocol/game/ServerboundPlayerActionPacket$Action;Lnet/minecraft/core/Direction;II)V` | name_only | @Inject | HEAD | both | 1000 (default) | `ServerPlayerGameModeMixin.startBlockBreak` |
| [[40-Interfaces/net.minecraft.server.level.ServerPlayerGameMode|ServerPlayerGameMode]].`useItem` | `(Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/InteractionResult;` | name_only | @Inject | HEAD | both | 1000 (default) | `ServerPlayerGameModeMixin.interactItem` |
| [[40-Interfaces/net.minecraft.server.level.ServerPlayerGameMode|ServerPlayerGameMode]].`useItemOn` | `(Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/InteractionHand;Lnet/minecraft/world/phys/BlockHitResult;)Lnet/minecraft/world/InteractionResult;` | name_only | @Inject | HEAD | both | 1000 (default) | `ServerPlayerGameModeMixin.interactBlock` |
| [[40-Interfaces/net.minecraft.server.network.ServerGamePacketListenerImpl|ServerGamePacketListenerImpl]].`handleInteract` | `(Lnet/minecraft/network/protocol/game/ServerboundInteractPacket;)V` | name_only | @Inject | INVOKE `Lnet/minecraft/server/level/ServerPlayer;getItemInHand(Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/item/ItemStack;` (inherited_exact) | both | 1000 (default) | `ServerGamePacketListenerImplMixin.handleInteract` |
| [[40-Interfaces/net.minecraft.server.network.ServerGamePacketListenerImpl|ServerGamePacketListenerImpl]].`handlePickItemFromBlock` | `(Lnet/minecraft/network/protocol/game/ServerboundPickItemFromBlockPacket;)V` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/world/level/block/state/BlockState;getCloneItemStack(Lnet/minecraft/world/level/LevelReader;Lnet/minecraft/core/BlockPos;Z)Lnet/minecraft/world/item/ItemStack;` (inherited_exact) | both | 1000 (default) | `ServerGamePacketListenerImplMixin.onPickItemFromBlock` |
| [[40-Interfaces/net.minecraft.server.network.ServerGamePacketListenerImpl|ServerGamePacketListenerImpl]].`handlePickItemFromEntity` | `(Lnet/minecraft/network/protocol/game/ServerboundPickItemFromEntityPacket;)V` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/world/entity/Entity;getPickResult()Lnet/minecraft/world/item/ItemStack;` (exact) | both | 1000 (default) | `ServerGamePacketListenerImplMixin.onPickItemFromEntity` |
| [[40-Interfaces/net.minecraft.world.entity.player.Player|Player]].`attack` | `(Lnet/minecraft/world/entity/Entity;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `PlayerMixin.onPlayerInteractEntity` |
| [[40-Interfaces/net.minecraft.world.item.ItemStack|ItemStack]].`use` | `(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/InteractionResult;` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/world/item/Item;use(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/InteractionResult;` (exact) | both | 1000 (default) | `ItemStackMixin.handleUseEvent` |
| [[40-Interfaces/net.minecraft.world.item.ItemStack|ItemStack]].`useOn` | `(Lnet/minecraft/world/item/context/UseOnContext;)Lnet/minecraft/world/InteractionResult;` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/world/item/Item;useOn(Lnet/minecraft/world/item/context/UseOnContext;)Lnet/minecraft/world/InteractionResult;` (exact) | both | 1000 (default) | `ItemStackMixin.handleUseOnEvent` |
| [[40-Interfaces/net.minecraft.world.level.block.state.BlockBehaviour_BlockStateBase|BlockBehaviour$BlockStateBase]].`useItemOn` | `(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/InteractionHand;Lnet/minecraft/world/phys/BlockHitResult;)Lnet/minecraft/world/InteractionResult;` | name_only | @Inject | HEAD | both | 1000 (default) | `BlockBehaviourBlockStateBaseMixin.callUseItemOnEvent` |
| [[40-Interfaces/net.minecraft.world.level.block.state.BlockBehaviour_BlockStateBase|BlockBehaviour$BlockStateBase]].`useWithoutItem` | `(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/phys/BlockHitResult;)Lnet/minecraft/world/InteractionResult;` | name_only | @Inject | HEAD | both | 1000 (default) | `BlockBehaviourBlockStateBaseMixin.callUseWithoutItemEvent` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.block.BlockAttackInteractionAware|BlockAttackInteractionAware]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.entity.FakePlayer|FakePlayer]] (class, 15 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.client.player.ClientHotbarScrollEvents|ClientHotbarScrollEvents]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.client.player.ClientPlayerBlockBreakEvents|ClientPlayerBlockBreakEvents]] (class, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.client.player.ClientPreAttackCallback|ClientPreAttackCallback]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.player.AttackBlockCallback|AttackBlockCallback]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.player.AttackEntityCallback|AttackEntityCallback]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.player.BlockEvents|BlockEvents]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.player.ItemEvents|ItemEvents]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.player.PlayerBlockBreakEvents|PlayerBlockBreakEvents]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.player.PlayerPickItemEvents|PlayerPickItemEvents]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.player.UseBlockCallback|UseBlockCallback]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.player.UseEntityCallback|UseEntityCallback]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.player.UseItemCallback|UseItemCallback]] (interface, 2 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
