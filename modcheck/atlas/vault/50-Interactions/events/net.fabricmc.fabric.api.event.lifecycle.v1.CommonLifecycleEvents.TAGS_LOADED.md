---
type: "event"
event: "net.fabricmc.fabric.api.event.lifecycle.v1.CommonLifecycleEvents.TAGS_LOADED"
callback: "net.fabricmc.fabric.api.event.lifecycle.v1.CommonLifecycleEvents$TagsLoaded"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.lifecycle.v1.CommonLifecycleEvents.TAGS_LOADED

Callback interface: `net.fabricmc.fabric.api.event.lifecycle.v1.CommonLifecycleEvents$TagsLoaded`

Module: [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `ReloadableServerResourcesMixin.hookRefresh` @14 | [[40-Interfaces/net.minecraft.server.ReloadableServerResources|ReloadableServerResources]].`updateComponentsAndStaticRegistryTags` @Inject TAIL | unknown | static_inference |
| `ClientConfigurationPacketListenerImplMixin.invokeTagsLoaded` @11 | [[40-Interfaces/net.minecraft.client.multiplayer.ClientConfigurationPacketListenerImpl|ClientConfigurationPacketListenerImpl]].`handleConfigurationFinished` @Inject INVOKE `Lnet/minecraft/network/Connection;setupInboundProtocol(Lnet/minecraft/network/ProtocolInfo;Lnet/minecraft/network/PacketListener;)V` | unknown | static_inference |
| `ClientPacketListenerMixin.invokeTagsLoaded` @14 | [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]].`handleUpdateTags` @Inject INVOKE `Lnet/minecraft/world/item/CreativeModeTabs;searchTab()Lnet/minecraft/world/item/CreativeModeTab;` | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
