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

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `ServerGamePacketListenerImplMixin.handleInteract` | `ServerGamePacketListenerImpl.handleInteract` @Inject at INVOKE Lnet/minecraft/server/level/ServerPlayer;getItemInHand(Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/item/ItemStack; | both | static_inference |
| `MinecraftMixin.injectUseEntityCallback` | `Minecraft.startUseItem` @Inject at INVOKE Lnet/minecraft/client/multiplayer/MultiPlayerGameMode;interact(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/EntityHitResult;Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/InteractionResult; | client | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
