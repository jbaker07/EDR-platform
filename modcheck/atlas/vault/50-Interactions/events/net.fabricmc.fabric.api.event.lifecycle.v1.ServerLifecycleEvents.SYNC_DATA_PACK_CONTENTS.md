---
type: "event"
event: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SYNC_DATA_PACK_CONTENTS"
callback: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents$SyncDataPackContents"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SYNC_DATA_PACK_CONTENTS

Callback interface: `net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents$SyncDataPackContents`

Module: [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `PlayerListMixin.hookOnPlayerConnect` @11 | [[40-Interfaces/net.minecraft.server.players.PlayerList|PlayerList]].`placeNewPlayer` @Inject NEW `net/minecraft/network/protocol/game/ClientboundUpdateRecipesPacket` | unknown | static_inference |
| `PlayerListMixin.hookOnDataPacksReloaded` @43 | [[40-Interfaces/net.minecraft.server.players.PlayerList|PlayerList]].`reloadResources` @Inject INVOKE `Lnet/minecraft/network/protocol/common/ClientboundUpdateTagsPacket;<init>(Ljava/util/Map;)V` | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. An analyst-stated contract exists: [[_authored/contracts/net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SYNC_DATA_PACK_CONTENTS|read it]].
