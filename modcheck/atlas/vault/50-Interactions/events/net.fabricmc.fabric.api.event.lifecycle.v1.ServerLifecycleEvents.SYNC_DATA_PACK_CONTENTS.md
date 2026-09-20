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

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `PlayerListMixin.hookOnPlayerConnect` | `PlayerList.placeNewPlayer` @Inject at NEW net/minecraft/network/protocol/game/ClientboundUpdateRecipesPacket | both | static_inference |
| `PlayerListMixin.hookOnDataPacksReloaded` | `PlayerList.reloadResources` @Inject at INVOKE Lnet/minecraft/network/protocol/common/ClientboundUpdateTagsPacket;<init>(Ljava/util/Map;)V | both | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. An analyst-stated contract exists: [[_authored/contracts/net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SYNC_DATA_PACK_CONTENTS|read it]].
