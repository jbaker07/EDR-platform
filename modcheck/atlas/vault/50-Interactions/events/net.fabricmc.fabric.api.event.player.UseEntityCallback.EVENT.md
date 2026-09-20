---
type: "event"
event: "net.fabricmc.fabric.api.event.player.UseEntityCallback.EVENT"
callback: "net.fabricmc.fabric.api.event.player.UseEntityCallback"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.player.UseEntityCallback.EVENT

Callback interface: `net.fabricmc.fabric.api.event.player.UseEntityCallback`

Module: [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `ServerGamePacketListenerImplMixin.handleInteract` @60 | [[40-Interfaces/net.minecraft.server.network.ServerGamePacketListenerImpl|ServerGamePacketListenerImpl]].`handleInteract` @Inject INVOKE `Lnet/minecraft/server/level/ServerPlayer;getItemInHand(Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/item/ItemStack;` | unknown | static_inference |
| `MinecraftMixin.injectUseEntityCallback` @25 | [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`startUseItem` @Inject INVOKE `Lnet/minecraft/client/multiplayer/MultiPlayerGameMode;interact(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/EntityHitResult;Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/InteractionResult;` | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
